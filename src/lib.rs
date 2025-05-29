pub mod config;
pub mod errors;
pub mod extractors;
pub mod models;
pub mod scraper_util;

pub use config::Config;
pub use errors::{Result, ScrapingError};
pub use models::{Lead, Source};
pub use scraper_util::LeadScraper;

use regex::Regex;
use std::collections::HashMap;
use url::Url;

use crate::config::PatternsConfig;
pub struct DataExtractor {
    email_patterns: EmailPatterns,
    location_patterns: LocationPatterns,
    tld_mapping: HashMap<String, String>,
}

struct EmailPatterns {
    mailto: Regex,
    generic: Regex,
}

struct LocationPatterns {
    country_indicators: Vec<Regex>,
}

impl DataExtractor {
    pub fn new(patterns: &PatternsConfig) -> Result<Self> {
        let email_patterns = EmailPatterns {
            mailto: Regex::new(&patterns.email.mailto)?,
            generic: Regex::new(&patterns.email.generic)?,
        };

        let mut country_indicators = Vec::new();
        for pattern in &patterns.location.country_indicators {
            country_indicators.push(Regex::new(pattern)?);
        }

        let location_patterns = LocationPatterns { country_indicators };

        Ok(Self {
            email_patterns,
            location_patterns,
            tld_mapping: patterns.tld_mapping.clone(),
        })
    }

    pub fn extract_country(&self, text: &str, website: Option<&str>) -> Option<String> {
        // Try location indicators in text first
        for pattern in &self.location_patterns.country_indicators {
            if let Some(caps) = pattern.captures(text) {
                if let Some(country) = caps.get(1) {
                    return Some(country.as_str().trim().to_string());
                }
            }
        }

        // Fallback to TLD mapping
        if let Some(website_url) = website {
            if let Ok(url) = Url::parse(website_url) {
                if let Some(host) = url.host_str() {
                    for (tld, country) in &self.tld_mapping {
                        if host.ends_with(tld) {
                            return Some(country.clone());
                        }
                    }
                }
            }
        }

        None
    }

    pub fn extract_website(&self, html: &str, _base_url: Option<&str>) -> Option<String> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);

        // Try common selectors for websites
        let selectors = [
            "a[href*='http']:not([href*='twitter']):not([href*='linkedin']):not([href*='facebook'])",
            ".website a",
            ".url a",
            "a.external",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(href) = element.value().attr("href") {
                        if self.is_valid_website_url(href) {
                            return Some(href.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    fn is_valid_website_url(&self, url: &str) -> bool {
        if let Ok(parsed) = Url::parse(url) {
            let scheme = parsed.scheme();
            if scheme == "http" || scheme == "https" {
                return scheme == "http" || scheme == "https";
            }
        }
        false
    }

    pub fn clean_text(&self, text: &str) -> String {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .filter(|c| c.is_ascii() || c.is_whitespace())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_extraction() {
        let patterns = crate::config::PatternsConfig::default();
        let extractor = DataExtractor::new(&patterns).expect("Failed to create extractor");

        let html = r#"<a href="mailto:test@example.com">Contact</a>"#;
        let text = "Contact us at test@example.com";

        let email = extractor.extract_email(text, html);
        assert_eq!(email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_website_extraction() {
        let patterns = crate::config::PatternsConfig::default();
        let extractor = DataExtractor::new(&patterns).expect("Failed to create extractor");

        let html = r#"<a href="https://example.com">Visit Website</a>"#;
        let website = extractor.extract_website(html, None);

        assert_eq!(website, Some("https://example.com".to_string()));
    }
}
