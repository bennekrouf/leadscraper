use crate::errors::Result;
use crate::models::Lead;
use async_trait::async_trait;

pub mod base;
pub mod betalist;
pub mod github_awesome;
pub mod ycombinator;

pub use base::BaseScraper;
pub use betalist::BetaListScraper;
pub use github_awesome::GitHubAwesomeScraper;
pub use ycombinator::YCombinatorScraper;

/// Common trait for all source scrapers
#[async_trait]
pub trait SourceScraper {
    /// Scrape leads from this source
    async fn scrape(&self) -> Result<Vec<Lead>>;

    /// Get the human-readable name of this scraper
    fn source_name(&self) -> &'static str;

    /// Check if this scraper is enabled in configuration
    fn is_enabled(&self) -> bool;

    /// Get the expected number of leads (for progress tracking)
    fn expected_leads_count(&self) -> Option<usize> {
        None
    }
}

/// Factory function to create all enabled scrapers
pub fn create_scrapers(
    config: &crate::config::Config,
    client: &reqwest::Client,
    extractor: &crate::extractors::DataExtractor,
) -> Vec<Box<dyn SourceScraper + Send + Sync>> {
    let mut scrapers: Vec<Box<dyn SourceScraper + Send + Sync>> = Vec::new();

    // Y Combinator scraper
    if config.sources.ycombinator.enabled {
        scrapers.push(Box::new(YCombinatorScraper::new(
            config.sources.ycombinator.clone(),
            client.clone(),
            extractor.clone(),
        )));
    }

    // GitHub Awesome scraper
    if config.sources.github_awesome.enabled {
        scrapers.push(Box::new(GitHubAwesomeScraper::new(
            config.sources.github_awesome.clone(),
            client.clone(),
            extractor.clone(),
        )));
    }

    // BetaList scraper
    if config.sources.betalist.enabled {
        scrapers.push(Box::new(BetaListScraper::new(
            config.sources.betalist.clone(),
            client.clone(),
            extractor.clone(),
        )));
    }

    scrapers
}
