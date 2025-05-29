use crate::{
    config::Config,
    errors::{Result, ScrapingError},
    extractors::DataExtractor,
    models::{Lead, LeadStats, ScrapedData, Source},
};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

pub struct LeadScraper {
    client: Client,
    config: Config,
    extractor: DataExtractor,
}

impl LeadScraper {
    pub async fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent(&config.scraper.user_agent)
            .timeout(Duration::from_secs(config.scraper.timeout_seconds))
            .build()
            .map_err(|e| {
                ScrapingError::NetworkError(format!("Failed to create HTTP client: {}", e))
            })?;

        let github_token = config.scraper.github_token.clone();
        let extractor = DataExtractor::new(&config.patterns, client.clone(), github_token)
            .map_err(|e| {
                ScrapingError::ExtractionError(format!(
                    "Failed to initialize data extractor: {}",
                    e
                ))
            })?;

        Ok(Self {
            client,
            config,
            extractor,
        })
    }

    pub async fn scrape_all_sources(&self) -> Result<Vec<Lead>> {
        let mut all_leads = Vec::new();

        // Y Combinator
        if self.config.sources.ycombinator.enabled {
            info!("🔥 Scraping Y Combinator...");
            match self.scrape_ycombinator().await {
                Ok(mut leads) => {
                    info!("✅ Y Combinator: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("❌ Y Combinator scraping failed: {}", e),
            }
        } else {
            info!("⏭️  Y Combinator disabled in config");
        }

        // GitHub Awesome
        if self.config.sources.github_awesome.enabled {
            info!("🔥 Scraping GitHub Awesome repositories...");
            match self.scrape_github_awesome().await {
                Ok(mut leads) => {
                    info!("✅ GitHub Awesome: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("❌ GitHub Awesome scraping failed: {}", e),
            }
        } else {
            info!("⏭️  GitHub Awesome disabled in config");
        }

        // BetaList
        if self.config.sources.betalist.enabled {
            info!("🔥 Scraping BetaList...");
            match self.scrape_betalist().await {
                Ok(mut leads) => {
                    info!("✅ BetaList: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("❌ BetaList scraping failed: {}", e),
            }
        } else {
            info!("⏭️  BetaList disabled in config");
        }

        if all_leads.is_empty() {
            warn!(
                "🚨 No leads extracted from any source! Check your config and network connection."
            );
        }

        Ok(all_leads)
    }

    async fn scrape_ycombinator(&self) -> Result<Vec<Lead>> {
        let mut leads = Vec::new();
        let config = &self.config.sources.ycombinator;

        for endpoint in &config.endpoints {
            let url = format!("{}{}", config.base_url, endpoint);
            debug!("Fetching: {}", url);

            match self.fetch_html(&url).await {
                Ok(html) => {
                    let scraped_data = self.parse_ycombinator_page(&html)?;
                    for data in scraped_data {
                        let lead = self
                            .create_lead_from_scraped_data(data, Source::YCombinator)
                            .await;
                        leads.push(lead);
                    }
                }
                Err(e) => warn!("Failed to fetch {}: {}", url, e),
            }

            // Rate limiting
            sleep(Duration::from_millis(500)).await;
        }

        Ok(leads)
    }

    async fn scrape_github_awesome(&self) -> Result<Vec<Lead>> {
        let mut leads = Vec::new();
        let config = &self.config.sources.github_awesome;

        for repo in &config.repositories {
            let url = format!("{}/{}/contents/readme.md", config.api_base, repo);
            debug!("Fetching GitHub repo: {}", url);

            match self.fetch_github_readme(&url).await {
                Ok(content) => {
                    let scraped_data = self.parse_github_awesome_content(&content)?;
                    for data in scraped_data {
                        // Create source based on actual project URL, not awesome list
                        let source = self.determine_source_from_url(&data.website);
                        let mut lead = self.create_lead_from_scraped_data(data, source).await;

                        // Extract real GitHub emails from commits if it's a GitHub project
                        if let Some(ref website) = lead.website {
                            if website.contains("github.com") {
                                let commit_emails =
                                    self.extractor.extract_github_commit_emails(website).await;
                                if !commit_emails.is_empty() {
                                    // Use first (highest priority) email as github_email
                                    lead.github_email = commit_emails.into_iter().next();
                                }
                            }
                        }

                        leads.push(lead);
                    }
                }
                Err(e) => {
                    // Try README.md as fallback
                    let fallback_url = format!("{}/{}/contents/README.md", config.api_base, repo);
                    debug!("Trying fallback URL: {}", fallback_url);

                    match self.fetch_github_readme(&fallback_url).await {
                        Ok(content) => {
                            let scraped_data = self.parse_github_awesome_content(&content)?;
                            for data in scraped_data {
                                let source = self.determine_source_from_url(&data.website);
                                let mut lead =
                                    self.create_lead_from_scraped_data(data, source).await;

                                if let Some(ref website) = lead.website {
                                    if website.contains("github.com") {
                                        let commit_emails = self
                                            .extractor
                                            .extract_github_commit_emails(website)
                                            .await;
                                        if !commit_emails.is_empty() {
                                            lead.github_email = commit_emails.into_iter().next();
                                        }
                                    }
                                }

                                leads.push(lead);
                            }
                        }
                        Err(_) => warn!("Failed to fetch {}: {}", url, e),
                    }
                }
            }

            // Rate limiting for GitHub API
            sleep(Duration::from_secs(1)).await;
        }

        Ok(leads)
    }

    // NEW: Determine proper source based on project URL
    fn determine_source_from_url(&self, website: &Option<String>) -> Source {
        match website {
            Some(url) if url.contains("github.com") => Source::Website { url: url.clone() },
            Some(url) => Source::Website { url: url.clone() },
            None => Source::Website {
                url: "unknown".to_string(),
            },
        }
    }

    async fn scrape_betalist(&self) -> Result<Vec<Lead>> {
        let mut leads = Vec::new();
        let config = &self.config.sources.betalist;

        for endpoint in &config.endpoints {
            let url = format!("{}{}", config.base_url, endpoint);
            debug!("Fetching: {}", url);

            match self.fetch_html(&url).await {
                Ok(html) => {
                    let scraped_data = self.parse_betalist_page(&html)?;
                    for data in scraped_data {
                        let lead = self
                            .create_lead_from_scraped_data(data, Source::BetaList)
                            .await;
                        leads.push(lead);
                    }
                }
                Err(e) => warn!("Failed to fetch {}: {}", url, e),
            }

            // Rate limiting
            sleep(Duration::from_millis(800)).await;
        }

        Ok(leads)
    }

    async fn fetch_html(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await.map_err(|e| {
            ScrapingError::NetworkError(format!("HTTP request failed for '{}': {}", url, e))
        })?;

        if !response.status().is_success() {
            return Err(ScrapingError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                url
            )));
        }

        let html = response.text().await.map_err(|e| {
            ScrapingError::NetworkError(format!(
                "Failed to read response body from '{}': {}",
                url, e
            ))
        })?;
        Ok(html)
    }

    async fn fetch_github_readme(&self, url: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct GitHubContent {
            content: String,
        }

        let response = self
            .client
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| {
                ScrapingError::NetworkError(format!(
                    "GitHub API request failed for '{}': {}",
                    url, e
                ))
            })?;

        if !response.status().is_success() {
            return Err(ScrapingError::NetworkError(format!(
                "GitHub API error {}: {}",
                response.status(),
                url
            )));
        }

        let github_content: GitHubContent = response.json().await.map_err(|e| {
            ScrapingError::ParseError(format!("Failed to parse GitHub API response: {}", e))
        })?;

        use base64::{engine::general_purpose, Engine as _};

        // Clean the base64 content - remove whitespace and newlines
        let cleaned_content = github_content
            .content
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let decoded = general_purpose::STANDARD
            .decode(&cleaned_content)
            .map_err(|e| {
                ScrapingError::ParseError(format!("Failed to decode base64 content: {}", e))
            })?;

        String::from_utf8(decoded)
            .map_err(|e| ScrapingError::ParseError(format!("Invalid UTF-8 in README: {}", e)))
    }

    fn parse_ycombinator_page(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = Html::parse_document(html);
        let mut scraped_data = Vec::new();

        debug!("Parsing Y Combinator page, HTML length: {}", html.len());

        // YC has various selectors depending on the page
        let selectors = [
            ".company-row",
            ".company",
            ".startup-item",
            "[data-company]",
            "tr", // Table rows for company listings
        ];

        let mut found_elements = 0;

        for selector_str in &selectors {
            debug!("Trying YC selector: {}", selector_str);

            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    found_elements += 1;

                    if let Some(data) = self.extract_company_data(&element) {
                        debug!("✅ Extracted YC data: {}", data.name);
                        scraped_data.push(data);
                    }
                }
            }

            // If we found data with this selector, don't try others
            if !scraped_data.is_empty() {
                break;
            }
        }

        debug!(
            "YC parsing complete: {} elements found, {} leads extracted",
            found_elements,
            scraped_data.len()
        );
        Ok(scraped_data)
    }

    fn parse_betalist_page(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = Html::parse_document(html);
        let mut scraped_data = Vec::new();

        debug!("Parsing BetaList page, HTML length: {}", html.len());

        // BetaList uses different selectors - based on actual HTML structure
        let selectors = [
            "#startup-[0-9]+",        // div with id like "startup-124183"
            ".block[id^='startup-']", // block divs with startup IDs
            "div[id*='startup']",     // any div containing "startup" in ID
        ];

        let mut found_elements = 0;

        for selector_str in &selectors {
            debug!("Trying BetaList selector: {}", selector_str);

            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    found_elements += 1;
                    debug!(
                        "Found BetaList element #{}: {:?}",
                        found_elements,
                        element.value().id()
                    );

                    if let Some(data) = self.extract_betalist_data(&element) {
                        debug!("✅ Extracted BetaList data: {}", data.name);
                        scraped_data.push(data);
                    } else {
                        debug!("❌ Failed to extract data from element");
                    }
                }
            } else {
                warn!("Invalid selector: {}", selector_str);
            }

            // If we found data with this selector, don't try others
            if !scraped_data.is_empty() {
                break;
            }
        }

        debug!(
            "BetaList parsing complete: {} elements found, {} leads extracted",
            found_elements,
            scraped_data.len()
        );
        Ok(scraped_data)
    }

    // NEW: Specific BetaList data extraction
    fn extract_betalist_data(&self, element: &scraper::ElementRef) -> Option<ScrapedData> {
        // Look for startup name in links
        let name_selectors = [
            "a[href*='/startups/'] .font-medium",
            "a[href*='/startups/']",
            ".font-medium",
            "h3",
            "h2",
            ".name",
        ];

        let mut name = String::new();
        let mut website_link = None;

        for selector_str in &name_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(name_element) = element.select(&selector).next() {
                    let potential_name = name_element.text().collect::<String>().trim().to_string();
                    if !potential_name.is_empty() && potential_name.len() > 2 {
                        name = potential_name;

                        // Try to get the href for website
                        if let Some(href) = name_element.value().attr("href") {
                            if href.contains("/startups/") {
                                website_link = Some(format!("https://betalist.com{}", href));
                            }
                        }
                        break;
                    }
                }
            }
        }

        if name.is_empty() {
            debug!("No name found in BetaList element");
            return None;
        }

        // Get description from text content
        let raw_text = element.text().collect::<String>();
        let cleaned_text = self.extractor.clean_text(&raw_text);

        // Extract website from any external links
        let html = element.html();
        let external_website = self.extractor.extract_website(&html, None);

        // Prefer external website over BetaList link
        let final_website = external_website.or(website_link);

        debug!("BetaList extracted: '{}' -> {:?}", name, final_website);

        Some(ScrapedData {
            name,
            website: final_website,
            raw_text: cleaned_text,
            html,
        })
    }

    fn parse_github_awesome_content(&self, content: &str) -> Result<Vec<ScrapedData>> {
        let mut scraped_data = Vec::new();
        debug!(
            "Parsing GitHub awesome content, total lines: {}",
            content.lines().count()
        );

        // Enhanced regex to handle various markdown link formats
        let link_patterns = [
            // Standard: [Name](URL) - Description
            r"\[([^\]]+)\]\(([^)]+)\)(?:\s*[-–—]\s*(.+))?",
            // With emoji: - 🚀 [Name](URL) - Description
            r"[-*]\s*(?:[^\[\]]*\s+)?\[([^\]]+)\]\(([^)]+)\)(?:\s*[-–—]\s*(.+))?",
            // Simple: [Name](URL)
            r"\[([^\]]+)\]\(([^)]+)\)",
        ];

        let mut processed_count = 0;
        let mut valid_count = 0;

        for pattern in &link_patterns {
            let link_regex = regex::Regex::new(pattern).map_err(|e| {
                ScrapingError::RegexError(format!("Failed to compile GitHub regex: {}", e))
            })?;

            for (line_num, line) in content.lines().enumerate() {
                // Skip navigation links, headers, and non-project links
                if line.trim().starts_with('#')
                    || line.contains("contents")
                    || line.contains("awesome-")
                    || line.contains("github.com/sindresorhus")
                {
                    continue;
                }

                if let Some(caps) = link_regex.captures(line) {
                    processed_count += 1;

                    let name = caps
                        .get(1)
                        .map(|m| m.as_str().trim().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let url = caps.get(2).map(|m| m.as_str().trim().to_string());

                    let description = caps
                        .get(3)
                        .map(|m| m.as_str().trim().to_string())
                        .unwrap_or_else(|| "No description".to_string());

                    debug!(
                        "Line {}: Found potential project '{}' -> {:?}",
                        line_num + 1,
                        name,
                        url
                    );

                    // Filter out generic/navigation links
                    if self.is_valid_project_link(&name, &url) {
                        valid_count += 1;
                        let url_ref = url.as_ref().map(|u| u.as_str()).unwrap_or("");
                        scraped_data.push(ScrapedData {
                            name: self.clean_project_name(&name),
                            website: url.clone(),
                            raw_text: description.clone(),
                            html: format!("<a href='{}'>{}</a> - {}", url_ref, name, description),
                        });
                        debug!("✅ Added valid project: {}", name);
                    } else {
                        debug!("❌ Filtered out: {}", name);
                    }
                }
            }

            // If we found projects with this pattern, don't try other patterns to avoid duplicates
            if !scraped_data.is_empty() {
                break;
            }
        }

        debug!(
            "Parsing complete: {} processed, {} valid projects found",
            processed_count, valid_count
        );
        Ok(scraped_data)
    }

    // NEW: Validate if this is a real project link vs navigation
    fn is_valid_project_link(&self, name: &str, url: &Option<String>) -> bool {
        // Must have a URL
        if url.is_none() || name.is_empty() {
            return false;
        }

        let url_str = url.as_ref().unwrap();

        // Skip anchor links (start with #)
        if url_str.starts_with('#') || url_str.contains("#readme") {
            return false;
        }

        // Skip obvious navigation/category links by name
        let skip_names = [
            "platforms",
            "programming languages",
            "front-end development",
            "back-end development",
            "computer science",
            "big data",
            "theory",
            "books",
            "editors",
            "gaming",
            "development environment",
            "entertainment",
            "databases",
            "media",
            "learn",
            "security",
            "content management systems",
            "hardware",
            "business",
            "work",
            "networking",
            "decentralized systems",
            "health and social science",
            "events",
            "testing",
            "miscellaneous",
            "related",
            "contents",
        ];

        let name_lower = name.to_lowercase();
        for skip in &skip_names {
            if name_lower == *skip || name_lower.contains(&format!("{} ", skip)) {
                debug!("Skipping navigation link: {}", name);
                return false;
            }
        }

        // Skip if URL is just a relative link or too short
        if url_str.len() < 10 {
            debug!("Skipping short URL: {}", url_str);
            return false;
        }

        debug!("Valid project link: {} -> {}", name, url_str);
        true
    }

    // NEW: Clean up project names (remove emojis, extra spaces)
    fn clean_project_name(&self, name: &str) -> String {
        name.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "-_.".contains(*c))
            .collect::<String>()
            .trim()
            .to_string()
    }

    fn extract_company_data(&self, element: &scraper::ElementRef) -> Option<ScrapedData> {
        let name_selectors = ["h3", "h2", ".name", ".company-name", ".startup-name"];
        let mut name = String::new();

        for selector_str in &name_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(name_element) = element.select(&selector).next() {
                    name = name_element.text().collect::<String>().trim().to_string();
                    if !name.is_empty() {
                        break;
                    }
                }
            }
        }

        if name.is_empty() {
            return None;
        }

        let raw_text = element.text().collect::<String>();
        let html = element.html();

        // Try to extract website from links
        let website = self.extractor.extract_website(&html, None);

        Some(ScrapedData {
            name,
            website,
            raw_text: self.extractor.clean_text(&raw_text),
            html,
        })
    }

    async fn create_lead_from_scraped_data(&self, data: ScrapedData, source: Source) -> Lead {
        let email = self.extractor.extract_email(&data.raw_text, &data.html);
        let country = self
            .extractor
            .extract_country(&data.raw_text, data.website.as_deref());

        Lead::new(data.name, source)
            .with_website(data.website)
            .with_email(email)
            .with_country(country)
            .with_description(Some(data.raw_text))
    }

    pub async fn save_leads(&self, leads: &[Lead], output_path: &str) -> Result<()> {
        use std::fs;

        fs::create_dir_all(output_path).map_err(|e| {
            ScrapingError::IoError(format!(
                "Failed to create output directory '{}': {}",
                output_path, e
            ))
        })?;

        // Separate leads by contact availability
        let (contactable_leads, no_contact_leads): (Vec<_>, Vec<_>) =
            leads.iter().partition(|lead| self.has_contact_info(lead));

        // Save contactable leads (priority targets)
        let contactable_output = format!("{}/contactable_leads.json", output_path);
        let contactable_json = serde_json::to_string_pretty(&contactable_leads).map_err(|e| {
            ScrapingError::IoError(format!("Failed to serialize contactable leads: {}", e))
        })?;
        fs::write(&contactable_output, contactable_json).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write contactable leads file: {}", e))
        })?;

        // Save no-contact leads (research targets)
        let no_contact_output = format!("{}/research_leads.json", output_path);
        let no_contact_json = serde_json::to_string_pretty(&no_contact_leads).map_err(|e| {
            ScrapingError::IoError(format!("Failed to serialize research leads: {}", e))
        })?;
        fs::write(&no_contact_output, no_contact_json).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write research leads file: {}", e))
        })?;

        // Create summary stats
        let stats = LeadStats::new(&contactable_leads, &no_contact_leads);
        let stats_output = format!("{}/stats.json", output_path);
        let stats_json = serde_json::to_string_pretty(&stats)
            .map_err(|e| ScrapingError::IoError(format!("Failed to serialize stats: {}", e)))?;
        fs::write(&stats_output, stats_json)
            .map_err(|e| ScrapingError::IoError(format!("Failed to write stats file: {}", e)))?;

        // Save legacy all-leads file for compatibility
        let all_leads_output = format!("{}/all_leads.json", output_path);
        let all_leads_json = serde_json::to_string_pretty(leads)
            .map_err(|e| ScrapingError::IoError(format!("Failed to serialize all leads: {}", e)))?;
        fs::write(&all_leads_output, all_leads_json).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write all leads file: {}", e))
        })?;

        info!("✅ Results saved:");
        info!(
            "   📧 Contactable leads: {} -> {}",
            contactable_leads.len(),
            contactable_output
        );
        info!(
            "   🔍 Research leads: {} -> {}",
            no_contact_leads.len(),
            no_contact_output
        );
        info!("   📊 Stats: {}", stats_output);
        info!("   📋 All leads: {}", all_leads_output);

        Ok(())
    }

    // NEW: Check if lead has any contact information
    fn has_contact_info(&self, lead: &Lead) -> bool {
        lead.email.is_some() || lead.github_email.is_some()
    }
}

