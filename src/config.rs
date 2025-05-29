use crate::errors::{Result, ScrapingError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub scraper: ScraperConfig,
    pub sources: SourcesConfig,
    pub patterns: PatternsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScraperConfig {
    pub timeout_seconds: u64,
    pub max_concurrent_requests: usize,
    pub user_agent: String,
    pub github_token: Option<String>, // NEW: Optional GitHub token for higher rate limits
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourcesConfig {
    pub ycombinator: YCombinatorConfig,
    pub github_awesome: GitHubAwesomeConfig,
    pub betalist: BetaListConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YCombinatorConfig {
    pub enabled: bool,
    pub base_url: String,
    pub endpoints: Vec<String>,
    pub selectors: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubAwesomeConfig {
    pub enabled: bool,
    pub repositories: Vec<String>,
    pub api_base: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BetaListConfig {
    pub enabled: bool,
    pub base_url: String,
    pub endpoints: Vec<String>,
    pub selectors: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatternsConfig {
    pub email: EmailPatterns,
    pub location: LocationPatterns,
    pub tld_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailPatterns {
    pub mailto: String,
    pub contact: String,
    pub generic: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationPatterns {
    pub country_indicators: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| {
            ScrapingError::ConfigError(format!("Failed to read config file '{}': {}", path, e))
        })?;

        let config: Config = serde_yaml::from_str(&content).map_err(|e| {
            ScrapingError::ConfigError(format!("Failed to parse config file '{}': {}", path, e))
        })?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scraper: ScraperConfig {
                timeout_seconds: 30,
                max_concurrent_requests: 10,
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
                github_token: None,
            },
            sources: SourcesConfig {
                ycombinator: YCombinatorConfig {
                    enabled: true,
                    base_url: "https://www.ycombinator.com".to_string(),
                    endpoints: vec!["/companies".to_string()],
                    selectors: HashMap::new(),
                },
                github_awesome: GitHubAwesomeConfig {
                    enabled: true,
                    repositories: vec!["awesome-startup-tools".to_string()],
                    api_base: "https://api.github.com/repos".to_string(),
                },
                betalist: BetaListConfig {
                    enabled: true,
                    base_url: "https://betalist.com".to_string(),
                    endpoints: vec!["/startups".to_string()],
                    selectors: HashMap::new(),
                },
            },
            patterns: PatternsConfig {
                email: EmailPatterns {
                    mailto: "mailto:([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,})".to_string(),
                    contact: "contact@|info@|hello@".to_string(),
                    generic: "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}".to_string(),
                },
                location: LocationPatterns {
                    country_indicators: vec!["based in ([A-Za-z\\s]+)".to_string()],
                },
                tld_mapping: HashMap::new(),
            },
        }
    }
}

