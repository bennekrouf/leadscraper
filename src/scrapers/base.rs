use crate::errors::{Result, ScrapingError};
use crate::extractors::DataExtractor;
use crate::models::{Lead, ScrapedData, Source};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Base scraper with common functionality shared across all scrapers
#[derive(Clone)]
pub struct BaseScraper {
    pub client: Client,
    pub extractor: DataExtractor,
}

impl BaseScraper {
    pub fn new(client: Client, extractor: DataExtractor) -> Self {
        Self { client, extractor }
    }

    /// Fetch HTML content from a URL with error handling
    pub async fn fetch_html(&self, url: &str) -> Result<String> {
        debug!("Fetching HTML from: {}", url);

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

        debug!("Successfully fetched {} bytes from {}", html.len(), url);
        Ok(html)
    }

    /// Fetch GitHub README content via API
    pub async fn fetch_github_readme(&self, url: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct GitHubContent {
            content: String,
        }

        debug!("Fetching GitHub README from: {}", url);

        let response = self
            .client
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "Lead-Scraper/1.0")
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

        let content = String::from_utf8(decoded)
            .map_err(|e| ScrapingError::ParseError(format!("Invalid UTF-8 in README: {}", e)))?;

        debug!(
            "Successfully decoded GitHub README: {} bytes",
            content.len()
        );
        Ok(content)
    }

    /// Extract company data from HTML element using common selectors
    pub fn extract_company_data(&self, element: &ElementRef) -> Option<ScrapedData> {
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
            debug!("No name found in element");
            return None;
        }

        let raw_text = element.text().collect::<String>();
        let html = element.html();

        // Try to extract website from links
        let website = self.extractor.extract_website(&html, None);

        debug!("Extracted company data: '{}' -> {:?}", name, website);

        Some(ScrapedData {
            name,
            website,
            raw_text: self.extractor.clean_text(&raw_text),
            html,
        })
    }

    /// Create a Lead from scraped data with all extracted information
    pub async fn create_lead_from_scraped_data(&self, data: ScrapedData, source: Source) -> Lead {
        let email = self.extractor.extract_email(&data.raw_text, &data.html);
        let country = self
            .extractor
            .extract_country(&data.raw_text, data.website.as_deref());
        let (linkedin, twitter) = self
            .extractor
            .extract_social_media(&data.raw_text, &data.html);

        let mut lead = Lead::new(data.name, source)
            .with_website(data.website)
            .with_email(email)
            .with_linkedin(linkedin)
            .with_twitter(twitter)
            .with_country(country)
            .with_description(Some(data.raw_text));

        // Extract GitHub commit emails if it's a GitHub project
        if let Some(ref website) = lead.website {
            if website.contains("github.com") {
                let commit_emails = self.extractor.extract_github_commit_emails(website).await;
                if !commit_emails.is_empty() {
                    lead.github_email = commit_emails.into_iter().next();
                }
            }
        }

        lead
    }

    /// Apply rate limiting between requests
    pub async fn rate_limit(&self, delay_ms: u64) {
        if delay_ms > 0 {
            sleep(Duration::from_millis(delay_ms)).await;
        }
    }

    /// Clean project name by removing emojis and extra characters
    pub fn clean_project_name(&self, name: &str) -> String {
        name.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "-_.".contains(*c))
            .collect::<String>()
            .trim()
            .to_string()
    }

    /// Check if a URL is a valid project link (not navigation/category link)
    pub fn is_valid_project_link(&self, name: &str, url: &Option<String>) -> bool {
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

    /// Determine proper source based on project URL
    pub fn determine_source_from_url(&self, website: &Option<String>) -> Source {
        match website {
            Some(url) if url.contains("github.com") => Source::Website { url: url.clone() },
            Some(url) => Source::Website { url: url.clone() },
            None => Source::Website {
                url: "unknown".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PatternsConfig;

    #[test]
    fn test_clean_project_name() {
        let client = reqwest::Client::new();
        let extractor = DataExtractor::new(&PatternsConfig::default(), client.clone(), None)
            .expect("Failed to create extractor");
        let base = BaseScraper::new(client, extractor);

        assert_eq!(
            base.clean_project_name("🚀 Awesome Project 🎉"),
            "Awesome Project"
        );
        assert_eq!(
            base.clean_project_name("  spaced-name_123  "),
            "spaced-name_123"
        );
    }

    #[test]
    fn test_is_valid_project_link() {
        let client = reqwest::Client::new();
        let extractor = DataExtractor::new(&PatternsConfig::default(), client.clone(), None)
            .expect("Failed to create extractor");
        let base = BaseScraper::new(client, extractor);

        // Valid links
        assert!(base.is_valid_project_link(
            "MyProject",
            &Some("https://github.com/user/project".to_string())
        ));
        assert!(base.is_valid_project_link("CoolApp", &Some("https://example.com".to_string())));

        // Invalid links
        assert!(!base.is_valid_project_link("contents", &Some("https://example.com".to_string())));
        assert!(!base.is_valid_project_link("MyProject", &Some("#anchor".to_string())));
        assert!(!base.is_valid_project_link("MyProject", &Some("x".to_string())));
        assert!(!base.is_valid_project_link("", &Some("https://example.com".to_string())));
        assert!(!base.is_valid_project_link("MyProject", &None));
    }
}
