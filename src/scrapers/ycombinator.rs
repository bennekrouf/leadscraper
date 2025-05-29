use super::{base::BaseScraper, SourceScraper};
use crate::{
    config::YCombinatorConfig,
    errors::{Result, ScrapingError},
    extractors::DataExtractor,
    models::{Lead, ScrapedData, Source},
};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::{debug, info, warn};

/// Y Combinator scraper implementation
pub struct YCombinatorScraper {
    config: YCombinatorConfig,
    base: BaseScraper,
}

impl YCombinatorScraper {
    pub fn new(config: YCombinatorConfig, client: Client, extractor: DataExtractor) -> Self {
        Self {
            config,
            base: BaseScraper::new(client, extractor),
        }
    }

    /// Parse Y Combinator page and extract company data
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

                    if let Some(data) = self.base.extract_company_data(&element) {
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
}

#[async_trait]
impl SourceScraper for YCombinatorScraper {
    async fn scrape(&self) -> Result<Vec<Lead>> {
        info!("🚀 Starting Y Combinator scraping...");
        let mut leads = Vec::new();

        for endpoint in &self.config.endpoints {
            let url = format!("{}{}", self.config.base_url, endpoint);
            debug!("Fetching Y Combinator endpoint: {}", url);

            match self.base.fetch_html(&url).await {
                Ok(html) => {
                    let scraped_data = self.parse_ycombinator_page(&html)?;
                    for data in scraped_data {
                        let lead = self
                            .base
                            .create_lead_from_scraped_data(data, Source::YCombinator)
                            .await;
                        leads.push(lead);
                    }
                    debug!(
                        "✅ Processed endpoint {}, total leads: {}",
                        endpoint,
                        leads.len()
                    );
                }
                Err(e) => {
                    warn!("❌ Failed to fetch Y Combinator endpoint {}: {}", url, e);
                    // Continue with other endpoints
                }
            }

            // Rate limiting between endpoints
            self.base.rate_limit(500).await;
        }

        info!("✅ Y Combinator scraping complete: {} leads", leads.len());
        Ok(leads)
    }

    fn source_name(&self) -> &'static str {
        "Y Combinator"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn expected_leads_count(&self) -> Option<usize> {
        // Rough estimate: 50-200 companies per batch
        Some(self.config.endpoints.len() * 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PatternsConfig;
    use std::collections::HashMap;

    fn create_test_scraper() -> YCombinatorScraper {
        let config = YCombinatorConfig {
            enabled: true,
            base_url: "https://www.ycombinator.com".to_string(),
            endpoints: vec!["/companies?batch=W24".to_string()],
            selectors: HashMap::new(),
        };
        let client = reqwest::Client::new();
        let extractor = DataExtractor::new(&PatternsConfig::default(), client.clone(), None)
            .expect("Failed to create extractor");

        YCombinatorScraper::new(config, client, extractor)
    }

    #[test]
    fn test_source_name() {
        let scraper = create_test_scraper();
        assert_eq!(scraper.source_name(), "Y Combinator");
    }

    #[test]
    fn test_is_enabled() {
        let scraper = create_test_scraper();
        assert!(scraper.is_enabled());
    }

    #[tokio::test]
    async fn test_parse_ycombinator_page() {
        let scraper = create_test_scraper();

        let sample_html = r#"
            <div class="company-row">
                <h2>Test Company</h2>
                <p>A great startup doing amazing things</p>
                <a href="https://testcompany.com">Website</a>
            </div>
        "#;

        let result = scraper.parse_ycombinator_page(sample_html);
        assert!(result.is_ok());

        let scraped_data = result.unwrap();
        assert!(!scraped_data.is_empty());
        assert_eq!(scraped_data[0].name, "Test Company");
    }
}
