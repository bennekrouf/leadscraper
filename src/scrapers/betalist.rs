use super::{base::BaseScraper, SourceScraper};
use crate::{
    config::BetaListConfig,
    errors::Result,
    extractors::DataExtractor,
    models::{Lead, ScrapedData, Source},
};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, info, warn};

/// BetaList scraper implementation
pub struct BetaListScraper {
    config: BetaListConfig,
    base: BaseScraper,
}

impl BetaListScraper {
    pub fn new(config: BetaListConfig, client: Client, extractor: DataExtractor) -> Self {
        Self {
            config,
            base: BaseScraper::new(client, extractor),
        }
    }

    /// Parse BetaList page and extract startup data
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

    /// Extract startup data from BetaList HTML element
    fn extract_betalist_data(&self, element: &ElementRef) -> Option<ScrapedData> {
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
        let cleaned_text = self.base.extractor.clean_text(&raw_text);

        // Extract website from any external links
        let html = element.html();
        let external_website = self.base.extractor.extract_website(&html, None);

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
}

#[async_trait]
impl SourceScraper for BetaListScraper {
    async fn scrape(&self) -> Result<Vec<Lead>> {
        info!("🚀 Starting BetaList scraping...");
        let mut leads = Vec::new();

        for endpoint in &self.config.endpoints {
            let url = format!("{}{}", self.config.base_url, endpoint);
            debug!("Fetching BetaList endpoint: {}", url);

            match self.base.fetch_html(&url).await {
                Ok(html) => {
                    let scraped_data = self.parse_betalist_page(&html)?;
                    for data in scraped_data {
                        let lead = self
                            .base
                            .create_lead_from_scraped_data(data, Source::BetaList)
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
                    warn!("❌ Failed to fetch BetaList endpoint {}: {}", url, e);
                    // Continue with other endpoints
                }
            }

            // Rate limiting between endpoints (BetaList is more restrictive)
            self.base.rate_limit(800).await;
        }

        info!("✅ BetaList scraping complete: {} leads", leads.len());
        Ok(leads)
    }

    fn source_name(&self) -> &'static str {
        "BetaList"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn expected_leads_count(&self) -> Option<usize> {
        // Rough estimate: 20-50 startups per page
        Some(self.config.endpoints.len() * 30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PatternsConfig;
    use std::collections::HashMap;

    fn create_test_scraper() -> BetaListScraper {
        let config = BetaListConfig {
            enabled: true,
            base_url: "https://betalist.com".to_string(),
            endpoints: vec!["/startups".to_string()],
            selectors: HashMap::new(),
        };
        let client = reqwest::Client::new();
        let extractor = DataExtractor::new(&PatternsConfig::default(), client.clone(), None)
            .expect("Failed to create extractor");

        BetaListScraper::new(config, client, extractor)
    }

    #[test]
    fn test_source_name() {
        let scraper = create_test_scraper();
        assert_eq!(scraper.source_name(), "BetaList");
    }

    #[test]
    fn test_is_enabled() {
        let scraper = create_test_scraper();
        assert!(scraper.is_enabled());
    }

    #[tokio::test]
    async fn test_parse_betalist_page() {
        let scraper = create_test_scraper();

        let sample_html = r#"
            <div id="startup-123456">
                <a href="/startups/test-startup">
                    <div class="font-medium">Test Startup</div>
                </a>
                <p>This is a test startup that does amazing things</p>
                <a href="https://teststartup.com">Visit Website</a>
            </div>
        "#;

        let result = scraper.parse_betalist_page(sample_html);
        assert!(result.is_ok());

        let scraped_data = result.unwrap();
        assert!(!scraped_data.is_empty());
        assert_eq!(scraped_data[0].name, "Test Startup");
    }

    #[test]
    fn test_extract_betalist_data() {
        let scraper = create_test_scraper();
        let html = Html::parse_document(
            r#"
            <div id="startup-123">
                <a href="/startups/cool-app">
                    <span class="font-medium">Cool App</span>
                </a>
                <p>A revolutionary app for cool people</p>
                <a href="https://coolapp.io">Website</a>
            </div>
        "#,
        );

        if let Some(element) = html.root_element().first_child() {
            if let Some(element_ref) = scraper::ElementRef::wrap(element) {
                let result = scraper.extract_betalist_data(&element_ref);
                assert!(result.is_some());

                let data = result.unwrap();
                assert_eq!(data.name, "Cool App");
                assert!(data.website.is_some());
            }
        }
    }
}
