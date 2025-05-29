use super::{base::BaseScraper, SourceScraper};
use crate::{
    config::GitHubAwesomeConfig,
    errors::{Result, ScrapingError},
    extractors::DataExtractor,
    models::{Lead, ScrapedData, Source},
};
use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, info, warn};

/// GitHub Awesome lists scraper implementation
pub struct GitHubAwesomeScraper {
    config: GitHubAwesomeConfig,
    base: BaseScraper,
}

impl GitHubAwesomeScraper {
    pub fn new(config: GitHubAwesomeConfig, client: Client, extractor: DataExtractor) -> Self {
        Self {
            config,
            base: BaseScraper::new(client, extractor),
        }
    }

    /// Parse GitHub awesome list README content and extract project links
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
                    if self.base.is_valid_project_link(&name, &url) {
                        valid_count += 1;
                        let url_ref = url.as_ref().map(|u| u.as_str()).unwrap_or("");
                        scraped_data.push(ScrapedData {
                            name: self.base.clean_project_name(&name),
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

    /// Scrape a single GitHub repository's README
    async fn scrape_repository(&self, repo: &str) -> Result<Vec<Lead>> {
        let mut leads = Vec::new();

        let url = format!("{}/{}/contents/readme.md", self.config.api_base, repo);
        debug!("Fetching GitHub repo: {}", url);

        let content = match self.base.fetch_github_readme(&url).await {
            Ok(content) => content,
            Err(_) => {
                // Try README.md as fallback
                let fallback_url = format!("{}/{}/contents/README.md", self.config.api_base, repo);
                debug!("Trying fallback URL: {}", fallback_url);

                match self.base.fetch_github_readme(&fallback_url).await {
                    Ok(content) => content,
                    Err(e) => {
                        warn!("Failed to fetch README for {}: {}", repo, e);
                        return Ok(leads);
                    }
                }
            }
        };

        let scraped_data = self.parse_github_awesome_content(&content)?;
        debug!(
            "Found {} projects in repository {}",
            scraped_data.len(),
            repo
        );

        for data in scraped_data {
            // Create source based on actual project URL, not awesome list
            let source = self.base.determine_source_from_url(&data.website);
            let mut lead = self.base.create_lead_from_scraped_data(data, source).await;

            // Extract real GitHub emails from commits if it's a GitHub project
            if let Some(ref website) = lead.website {
                if website.contains("github.com") {
                    let commit_emails = self
                        .base
                        .extractor
                        .extract_github_commit_emails(website)
                        .await;
                    if !commit_emails.is_empty() {
                        // Use first (highest priority) email as github_email
                        lead.github_email = commit_emails.into_iter().next();
                    }
                }
            }

            leads.push(lead);
        }

        Ok(leads)
    }
}

#[async_trait]
impl SourceScraper for GitHubAwesomeScraper {
    async fn scrape(&self) -> Result<Vec<Lead>> {
        info!("🚀 Starting GitHub Awesome lists scraping...");
        let mut all_leads = Vec::new();

        for repo in &self.config.repositories {
            info!("📚 Scraping repository: {}", repo);

            match self.scrape_repository(repo).await {
                Ok(mut leads) => {
                    info!("✅ Found {} leads in {}", leads.len(), repo);
                    all_leads.append(&mut leads);
                }
                Err(e) => {
                    warn!("❌ Failed to scrape repository {}: {}", repo, e);
                    // Continue with other repositories
                }
            }

            // Rate limiting for GitHub API
            self.base.rate_limit(1000).await;
        }

        info!(
            "✅ GitHub Awesome scraping complete: {} total leads",
            all_leads.len()
        );
        Ok(all_leads)
    }

    fn source_name(&self) -> &'static str {
        "GitHub Awesome"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn expected_leads_count(&self) -> Option<usize> {
        // Rough estimate: 100-500 projects per awesome list
        Some(self.config.repositories.len() * 200)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PatternsConfig;

    fn create_test_scraper() -> GitHubAwesomeScraper {
        let config = GitHubAwesomeConfig {
            enabled: true,
            repositories: vec!["sindresorhus/awesome".to_string()],
            api_base: "https://api.github.com/repos".to_string(),
        };
        let client = reqwest::Client::new();
        let extractor = DataExtractor::new(&PatternsConfig::default(), client.clone(), None)
            .expect("Failed to create extractor");

        GitHubAwesomeScraper::new(config, client, extractor)
    }

    #[test]
    fn test_source_name() {
        let scraper = create_test_scraper();
        assert_eq!(scraper.source_name(), "GitHub Awesome");
    }

    #[test]
    fn test_is_enabled() {
        let scraper = create_test_scraper();
        assert!(scraper.is_enabled());
    }

    #[test]
    fn test_parse_github_awesome_content() {
        let scraper = create_test_scraper();

        let sample_content = r#"
# Awesome List

## Projects

- [Awesome Project](https://github.com/user/awesome-project) - A really awesome project
- [Cool Tool](https://cooltool.com) - Tool for doing cool things
- [Contents](#contents) - Navigation link (should be filtered)
        "#;

        let result = scraper.parse_github_awesome_content(sample_content);
        assert!(result.is_ok());

        let scraped_data = result.unwrap();
        // Should find 2 valid projects (Contents link should be filtered out)
        assert_eq!(scraped_data.len(), 2);
        assert_eq!(scraped_data[0].name, "Awesome Project");
        assert_eq!(scraped_data[1].name, "Cool Tool");
    }
}
