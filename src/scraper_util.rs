use crate::{
    config::Config,
    errors::{Result, ScrapingError},
    extractors::DataExtractor,
    models::{Lead, ScrapedData, Source},
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

        let extractor = DataExtractor::new(&config.patterns).map_err(|e| {
            ScrapingError::ExtractionError(format!("Failed to initialize data extractor: {}", e))
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
            info!("Scraping Y Combinator...");
            match self.scrape_ycombinator().await {
                Ok(mut leads) => {
                    info!("Y Combinator: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("Y Combinator scraping failed: {}", e),
            }
        }

        // GitHub Awesome
        if self.config.sources.github_awesome.enabled {
            info!("Scraping GitHub Awesome repositories...");
            match self.scrape_github_awesome().await {
                Ok(mut leads) => {
                    info!("GitHub Awesome: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("GitHub Awesome scraping failed: {}", e),
            }
        }

        // BetaList
        if self.config.sources.betalist.enabled {
            info!("Scraping BetaList...");
            match self.scrape_betalist().await {
                Ok(mut leads) => {
                    info!("BetaList: {} leads extracted", leads.len());
                    all_leads.append(&mut leads);
                }
                Err(e) => error!("BetaList scraping failed: {}", e),
            }
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
                        let source = Source::GitHubAwesome {
                            repository: repo.clone(),
                        };
                        let lead = self.create_lead_from_scraped_data(data, source).await;
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
                                let source = Source::GitHubAwesome {
                                    repository: repo.clone(),
                                };
                                let lead = self.create_lead_from_scraped_data(data, source).await;
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

        // YC has various selectors depending on the page
        let selectors = [".company", ".startup-item", "[data-company]"];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(data) = self.extract_company_data(&element) {
                        scraped_data.push(data);
                    }
                }
            }
        }

        Ok(scraped_data)
    }

    fn parse_betalist_page(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = Html::parse_document(html);
        let mut scraped_data = Vec::new();

        let selectors = [".startup", ".startup-item", "[data-startup]"];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(data) = self.extract_company_data(&element) {
                        scraped_data.push(data);
                    }
                }
            }
        }

        Ok(scraped_data)
    }

    fn parse_github_awesome_content(&self, content: &str) -> Result<Vec<ScrapedData>> {
        let mut scraped_data = Vec::new();

        // Parse markdown links with pattern: [Name](URL) - Description
        let link_regex =
            regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)(?:\s*-\s*(.+))?").map_err(|e| {
                ScrapingError::RegexError(format!("Failed to compile GitHub regex: {}", e))
            })?;

        for line in content.lines() {
            if let Some(caps) = link_regex.captures(line) {
                let name = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                let url = caps.get(2).map(|m| m.as_str().to_string());

                let description = caps
                    .get(3)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "No description".to_string());

                if !name.is_empty() && url.is_some() {
                    let url_ref = url.as_ref().map(|u| u.as_str()).unwrap_or("");
                    scraped_data.push(ScrapedData {
                        name: name.clone(),
                        website: url.clone(),
                        raw_text: description.clone(),
                        html: format!("<a href='{}'>{}</a> - {}", url_ref, name, description),
                    });
                }
            }
        }

        Ok(scraped_data)
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
        let mut email = self.extractor.extract_email(&data.raw_text, &data.html);
        let country = self
            .extractor
            .extract_country(&data.raw_text, data.website.as_deref());

        // If no email found in README and we have a website, try to fetch it from the website
        if email.is_none() && data.website.is_some() {
            email = self
                .fetch_email_from_website(data.website.as_ref().unwrap())
                .await;
        }

        Lead::new(data.name, source)
            .with_website(data.website)
            .with_email(email)
            .with_country(country)
            .with_description(Some(data.raw_text))
    }

    async fn fetch_email_from_website(&self, website_url: &str) -> Option<String> {
        // Try common contact page patterns
        let contact_paths = [
            "/contact",
            "/contact-us",
            "/about",
            "/team",
            "/support",
            "", // Homepage
        ];

        for path in &contact_paths {
            let url = if path.is_empty() {
                website_url.to_string()
            } else {
                format!("{}{}", website_url.trim_end_matches('/'), path)
            };

            if let Ok(html) = self.fetch_html(&url).await {
                if let Some(email) = self.extractor.extract_email(&html, &html) {
                    debug!("Found email {} on page: {}", email, url);
                    return Some(email);
                }
            }

            // Small delay to be respectful
            sleep(Duration::from_millis(200)).await;
        }

        None
    }

    pub async fn save_leads(&self, leads: &[Lead], output_path: &str) -> Result<()> {
        use std::fs;

        // Save as JSON
        let json_output = format!("{}/leads.json", output_path);
        fs::create_dir_all(output_path).map_err(|e| {
            ScrapingError::IoError(format!(
                "Failed to create output directory '{}': {}",
                output_path, e
            ))
        })?;

        let json_content = serde_json::to_string_pretty(leads).map_err(|e| {
            ScrapingError::IoError(format!("Failed to serialize leads to JSON: {}", e))
        })?;

        fs::write(&json_output, json_content).map_err(|e| {
            ScrapingError::IoError(format!(
                "Failed to write JSON file '{}': {}",
                json_output, e
            ))
        })?;

        // Save as CSV for easy analysis
        let csv_output = format!("{}/leads.csv", output_path);
        let mut csv_content =
            String::from("name,website,email,source,country,description,scraped_at\n");

        for lead in leads {
            csv_content.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                lead.name.replace('"', "'"),
                lead.website.as_deref().unwrap_or(""),
                lead.email.as_deref().unwrap_or(""),
                lead.source,
                lead.country.as_deref().unwrap_or(""),
                lead.description.as_deref().unwrap_or("").replace('"', "'"),
                lead.scraped_at.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        fs::write(&csv_output, csv_content).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write CSV file '{}': {}", csv_output, e))
        })?;

        info!("Results saved:");
        info!("  JSON: {}", json_output);
        info!("  CSV: {}", csv_output);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_scraper_initialization() {
        let config = Config::default();
        let scraper = LeadScraper::new(config).await;
        assert!(scraper.is_ok());
    }

    #[tokio::test]
    async fn test_github_content_parsing() {
        let config = Config::default();
        let scraper = LeadScraper::new(config)
            .await
            .expect("Failed to create scraper");
        let content = r#"
# Awesome Startups

- [StartupOne](https://startupone.com) - AI-powered analytics
- [StartupTwo](https://startuptwo.io) - Blockchain solutions
        "#;

        let results = scraper
            .parse_github_awesome_content(content)
            .expect("Failed to parse content");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "StartupOne");
        assert_eq!(
            results[0].website,
            Some("https://startupone.com".to_string())
        );
    }
}
