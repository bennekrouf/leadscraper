This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
```
config/
  scraper.yaml
results/
  all_leads.json
  contactable_leads.json
  research_leads.json
  stats.json
src/
  config.rs
  errors.rs
  extractors.rs
  lib.rs
  main.rs
  models.rs
  scraper_util.rs
.gitignore
Cargo.toml
README.md
```

# Files

## File: results/all_leads.json
````json
[
  {
    "name": "Trading Places AI",
    "website": "https://betalist.com/startups/trading-places-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Trading Places AI Find high-probability trade setups in seconds",
    "scraped_at": "2025-05-29T08:01:55.209660Z"
  },
  {
    "name": "Bill Club",
    "website": "https://betalist.com/startups/bill-club",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Bill Club Split restaurant bills with your friends instantly, no download needed",
    "scraped_at": "2025-05-29T08:01:55.209686Z"
  },
  {
    "name": "Onuro AI",
    "website": "https://betalist.com/startups/onuro-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Onuro AI The Apple of Code Assistants",
    "scraped_at": "2025-05-29T08:01:55.209709Z"
  },
  {
    "name": "FlightsGPT",
    "website": "https://betalist.com/startups/flightsgpt",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "FlightsGPT Because clicking 60 times to find a flight is so 2020",
    "scraped_at": "2025-05-29T08:01:55.209732Z"
  },
  {
    "name": "ShotSearch",
    "website": "https://betalist.com/startups/shotsearch",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "ShotSearch Reverse Search Twitter/X Screenshots",
    "scraped_at": "2025-05-29T08:01:55.209754Z"
  },
  {
    "name": "Feedbackgrove",
    "website": "https://betalist.com/startups/feedbackgrove",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Feedbackgrove Collect your Product feedback Autonomously",
    "scraped_at": "2025-05-29T08:01:55.209778Z"
  },
  {
    "name": "Evntaly",
    "website": "https://betalist.com/startups/evntaly",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Evntaly Ship smarter, Track everything",
    "scraped_at": "2025-05-29T08:01:55.209800Z"
  },
  {
    "name": "VibeCode",
    "website": "https://betalist.com/startups/vibecode",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "VibeCode The mobile app that builds mobile apps",
    "scraped_at": "2025-05-29T08:01:55.209823Z"
  },
  {
    "name": "Vaiz",
    "website": "https://betalist.com/startups/vaiz",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Vaiz One platform for Tasks & Documents",
    "scraped_at": "2025-05-29T08:01:55.209844Z"
  },
  {
    "name": "AI Manga Translator",
    "website": "https://betalist.com/startups/ai-manga-translator",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "AI Manga Translator Experience the future of manga with AI Manga Translator",
    "scraped_at": "2025-05-29T08:01:55.209869Z"
  },
  {
    "name": "Databuddy",
    "website": "https://betalist.com/startups/databuddy",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Databuddy A privacy-centric analytics platform that prioritizes user experience izadoesdev",
    "scraped_at": "2025-05-29T08:01:55.209893Z"
  },
  {
    "name": "threadify",
    "website": "https://betalist.com/startups/threadify",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "threadify Split long text into threads for X(twitter), threads, LinkedIn",
    "scraped_at": "2025-05-29T08:01:55.209916Z"
  },
  {
    "name": "Draft Alpha",
    "website": "https://betalist.com/startups/draft-alpha",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Draft Alpha Draft Alpha helps you create your first brand identity and then apply it",
    "scraped_at": "2025-05-29T08:01:55.209939Z"
  },
  {
    "name": "Time 'N Track",
    "website": "https://betalist.com/startups/time-n-track",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Time 'N Track Time tracking app for freelancers, creatives, and solo business owners",
    "scraped_at": "2025-05-29T08:01:55.209962Z"
  },
  {
    "name": "q32 CV Match",
    "website": "https://betalist.com/startups/q32-cv-match",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "q32 CV Match Upload resumes, get a ranked shortlist with bullet fit-summaries.",
    "scraped_at": "2025-05-29T08:01:55.209985Z"
  },
  {
    "name": "CheepCode",
    "website": "https://betalist.com/startups/cheepcode",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "CheepCode Linear Ticket -> GitHub PR for $1 per task",
    "scraped_at": "2025-05-29T08:01:55.210007Z"
  },
  {
    "name": "Fortect macOS Antivirus",
    "website": "https://betalist.com/startups/fortect-macos-antivirus",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Fortect macOS Antivirus Protect your Mac with a cloud-backed AI Antivirus",
    "scraped_at": "2025-05-29T08:01:55.210032Z"
  },
  {
    "name": "Unblockly.AI",
    "website": "https://betalist.com/startups/unblockly-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Unblockly.AI AI-powered LinkedIn prospecting for Google Sheets",
    "scraped_at": "2025-05-29T08:01:55.210056Z"
  },
  {
    "name": "Color Explorer",
    "website": "https://betalist.com/startups/color-explorer",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Color Explorer From inspiration to actionable insights",
    "scraped_at": "2025-05-29T08:01:55.210079Z"
  },
  {
    "name": "taskLearn.ai",
    "website": "https://betalist.com/startups/tasklearn-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "taskLearn.ai Task-based learning app with 24/7 AI Mentor",
    "scraped_at": "2025-05-29T08:01:55.210102Z"
  }
]
````

## File: results/contactable_leads.json
````json
[]
````

## File: results/research_leads.json
````json
[
  {
    "name": "Trading Places AI",
    "website": "https://betalist.com/startups/trading-places-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Trading Places AI Find high-probability trade setups in seconds",
    "scraped_at": "2025-05-29T08:01:55.209660Z"
  },
  {
    "name": "Bill Club",
    "website": "https://betalist.com/startups/bill-club",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Bill Club Split restaurant bills with your friends instantly, no download needed",
    "scraped_at": "2025-05-29T08:01:55.209686Z"
  },
  {
    "name": "Onuro AI",
    "website": "https://betalist.com/startups/onuro-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Onuro AI The Apple of Code Assistants",
    "scraped_at": "2025-05-29T08:01:55.209709Z"
  },
  {
    "name": "FlightsGPT",
    "website": "https://betalist.com/startups/flightsgpt",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "FlightsGPT Because clicking 60 times to find a flight is so 2020",
    "scraped_at": "2025-05-29T08:01:55.209732Z"
  },
  {
    "name": "ShotSearch",
    "website": "https://betalist.com/startups/shotsearch",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "ShotSearch Reverse Search Twitter/X Screenshots",
    "scraped_at": "2025-05-29T08:01:55.209754Z"
  },
  {
    "name": "Feedbackgrove",
    "website": "https://betalist.com/startups/feedbackgrove",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Feedbackgrove Collect your Product feedback Autonomously",
    "scraped_at": "2025-05-29T08:01:55.209778Z"
  },
  {
    "name": "Evntaly",
    "website": "https://betalist.com/startups/evntaly",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Evntaly Ship smarter, Track everything",
    "scraped_at": "2025-05-29T08:01:55.209800Z"
  },
  {
    "name": "VibeCode",
    "website": "https://betalist.com/startups/vibecode",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "VibeCode The mobile app that builds mobile apps",
    "scraped_at": "2025-05-29T08:01:55.209823Z"
  },
  {
    "name": "Vaiz",
    "website": "https://betalist.com/startups/vaiz",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Vaiz One platform for Tasks & Documents",
    "scraped_at": "2025-05-29T08:01:55.209844Z"
  },
  {
    "name": "AI Manga Translator",
    "website": "https://betalist.com/startups/ai-manga-translator",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "AI Manga Translator Experience the future of manga with AI Manga Translator",
    "scraped_at": "2025-05-29T08:01:55.209869Z"
  },
  {
    "name": "Databuddy",
    "website": "https://betalist.com/startups/databuddy",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Databuddy A privacy-centric analytics platform that prioritizes user experience izadoesdev",
    "scraped_at": "2025-05-29T08:01:55.209893Z"
  },
  {
    "name": "threadify",
    "website": "https://betalist.com/startups/threadify",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "threadify Split long text into threads for X(twitter), threads, LinkedIn",
    "scraped_at": "2025-05-29T08:01:55.209916Z"
  },
  {
    "name": "Draft Alpha",
    "website": "https://betalist.com/startups/draft-alpha",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Draft Alpha Draft Alpha helps you create your first brand identity and then apply it",
    "scraped_at": "2025-05-29T08:01:55.209939Z"
  },
  {
    "name": "Time 'N Track",
    "website": "https://betalist.com/startups/time-n-track",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Time 'N Track Time tracking app for freelancers, creatives, and solo business owners",
    "scraped_at": "2025-05-29T08:01:55.209962Z"
  },
  {
    "name": "q32 CV Match",
    "website": "https://betalist.com/startups/q32-cv-match",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "q32 CV Match Upload resumes, get a ranked shortlist with bullet fit-summaries.",
    "scraped_at": "2025-05-29T08:01:55.209985Z"
  },
  {
    "name": "CheepCode",
    "website": "https://betalist.com/startups/cheepcode",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "CheepCode Linear Ticket -> GitHub PR for $1 per task",
    "scraped_at": "2025-05-29T08:01:55.210007Z"
  },
  {
    "name": "Fortect macOS Antivirus",
    "website": "https://betalist.com/startups/fortect-macos-antivirus",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Fortect macOS Antivirus Protect your Mac with a cloud-backed AI Antivirus",
    "scraped_at": "2025-05-29T08:01:55.210032Z"
  },
  {
    "name": "Unblockly.AI",
    "website": "https://betalist.com/startups/unblockly-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Unblockly.AI AI-powered LinkedIn prospecting for Google Sheets",
    "scraped_at": "2025-05-29T08:01:55.210056Z"
  },
  {
    "name": "Color Explorer",
    "website": "https://betalist.com/startups/color-explorer",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "Color Explorer From inspiration to actionable insights",
    "scraped_at": "2025-05-29T08:01:55.210079Z"
  },
  {
    "name": "taskLearn.ai",
    "website": "https://betalist.com/startups/tasklearn-ai",
    "email": null,
    "github_email": null,
    "linkedin": null,
    "twitter": null,
    "source": "BetaList",
    "country": null,
    "description": "taskLearn.ai Task-based learning app with 24/7 AI Mentor",
    "scraped_at": "2025-05-29T08:01:55.210102Z"
  }
]
````

## File: results/stats.json
````json
{
  "total_leads": 20,
  "contactable_leads": 0,
  "research_leads": 20,
  "contact_rate": 0.0,
  "sources_breakdown": {
    "BetaList": {
      "total": 20,
      "with_contact": 0,
      "contact_rate": 0.0
    }
  },
  "countries_breakdown": {},
  "email_types": {
    "direct_emails": 0,
    "github_emails": 0,
    "no_emails": 20
  },
  "generated_at": "2025-05-29T08:01:56.015041Z"
}
````

## File: src/errors.rs
````rust
use std::fmt;

#[derive(Debug)]
pub enum ScrapingError {
    ConfigError(String),
    NetworkError(String),
    ParseError(String),
    ExtractionError(String),
    IoError(String),
    RegexError(String),
}

impl fmt::Display for ScrapingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapingError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ScrapingError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ScrapingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ScrapingError::ExtractionError(msg) => write!(f, "Extraction error: {}", msg),
            ScrapingError::IoError(msg) => write!(f, "IO error: {}", msg),
            ScrapingError::RegexError(msg) => write!(f, "Regex error: {}", msg),
        }
    }
}

impl std::error::Error for ScrapingError {}

impl From<std::io::Error> for ScrapingError {
    fn from(err: std::io::Error) -> Self {
        ScrapingError::IoError(err.to_string())
    }
}

impl From<serde_yaml::Error> for ScrapingError {
    fn from(err: serde_yaml::Error) -> Self {
        ScrapingError::ConfigError(err.to_string())
    }
}

impl From<reqwest::Error> for ScrapingError {
    fn from(err: reqwest::Error) -> Self {
        ScrapingError::NetworkError(err.to_string())
    }
}

impl From<regex::Error> for ScrapingError {
    fn from(err: regex::Error) -> Self {
        ScrapingError::RegexError(err.to_string())
    }
}

impl From<serde_json::Error> for ScrapingError {
    fn from(err: serde_json::Error) -> Self {
        ScrapingError::ParseError(err.to_string())
    }
}

impl From<base64::DecodeError> for ScrapingError {
    fn from(err: base64::DecodeError) -> Self {
        ScrapingError::ParseError(format!("Base64 decode error: {}", err))
    }
}

impl From<std::string::FromUtf8Error> for ScrapingError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ScrapingError::ParseError(format!("UTF-8 conversion error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, ScrapingError>;
````

## File: .gitignore
````
/target
````

## File: Cargo.toml
````toml
[package]
name = "leadscraper"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.98"
base64 = "0.22.1"
chrome = "0.1.0"
chrono = { version = "0.4.41", features = ["serde"] }
clap = { version = "4.5.39", features = ["derive"] }
regex = "1.11.1"
reqwest = { version = "0.12.15", features = ["json"] }
scraper = "0.23.1"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
serde_yaml = "0.9.34"
tokio = { version = "1.45.1", features = ["full"] }
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.19", features = ["env-filter"] }
url = "2.5.4"
````

## File: config/scraper.yaml
````yaml
scraper:
  timeout_seconds: 30
  max_concurrent_requests: 10
  user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
  github_token: null  # Set to "ghp_your_token_here" for higher rate limits
  
sources:
  ycombinator:
    enabled: true
    base_url: "https://www.ycombinator.com"
    endpoints:
      - "/companies?batch=W24"
      - "/companies?batch=S24"  
    selectors:
      company_name: "[data-company-name], .company-name, h3, h2"
      website: "a[href*='http']:not([href*='ycombinator'])"
      description: ".company-description, p"    

  github_awesome:
    enabled: true
    repositories:
      - "sindresorhus/awesome"
      - "awesome-selfhosted/awesome-selfhosted" 
      - "ripienaar/free-for-dev"
    api_base: "https://api.github.com/repos"
    
  betalist:
    enabled: true
    base_url: "https://betalist.com"
    endpoints:
      - "/startups"
    selectors:
      startup_name: ".startup-name, h2 a"
      website: ".startup-link, .website"
      description: ".startup-pitch, .description"

patterns:
  email:
    mailto: "mailto:([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,})"
    contact: "contact@|info@|hello@|support@|admin@"
    generic: "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    
  location:
    country_indicators:
      - "based in ([A-Za-z\\s]+)"
      - "located in ([A-Za-z\\s]+)"
      - "from ([A-Za-z\\s]+)"
    
  tld_mapping:
    ".fr": "France"
    ".de": "Germany"
    ".uk": "United Kingdom"
    ".ca": "Canada"
    ".au": "Australia"
    ".jp": "Japan"
    ".kr": "South Korea"
    ".sg": "Singapore"
    ".nl": "Netherlands"
    ".se": "Sweden"
    ".ch": "Switzerland"
    ".be": "Belgium"
    ".it": "Italy"
    ".es": "Spain"
````

## File: src/config.rs
````rust
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
````

## File: src/extractors.rs
````rust
use crate::config::PatternsConfig;
use crate::errors::{Result, ScrapingError};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};
use url::Url;

type ExtractorResult<T> = std::result::Result<T, ScrapingError>;

// GitHub API structures
#[derive(Debug, Deserialize)]
struct GitHubRepo {
    name: String,
    full_name: String,
    fork: bool,
    #[serde(rename = "default_branch")]
    default_branch: String,
    owner: GitHubOwner,
}

#[derive(Debug, Deserialize)]
struct GitHubOwner {
    login: String,
    #[serde(rename = "type")]
    owner_type: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCommit {
    commit: CommitDetails,
    author: Option<GitHubUser>,
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    author: CommitAuthor,
    committer: CommitAuthor,
}

#[derive(Debug, Deserialize)]
struct CommitAuthor {
    name: Option<String>,
    email: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    email: Option<String>,
}

pub struct DataExtractor {
    email_patterns: EmailPatterns,
    location_patterns: LocationPatterns,
    tld_mapping: HashMap<String, String>,
    client: Client,
    github_token: Option<String>,
}

struct EmailPatterns {
    mailto: Regex,
    generic: Regex,
}

struct LocationPatterns {
    country_indicators: Vec<Regex>,
}

impl DataExtractor {
    pub fn new(
        patterns: &PatternsConfig,
        client: Client,
        github_token: Option<String>,
    ) -> Result<Self> {
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
            client,
            github_token,
        })
    }

    pub fn extract_email(&self, text: &str, html: &str) -> Option<String> {
        // Try mailto links first (highest priority)
        if let Some(caps) = self.email_patterns.mailto.captures(html) {
            if let Some(email) = caps.get(1) {
                return Some(email.as_str().to_string());
            }
        }

        // Fallback to generic email pattern
        if let Some(caps) = self.email_patterns.generic.captures(text) {
            let email = caps.get(0)?.as_str();
            // Filter out obvious false positives
            if !email.contains("example.") && !email.contains("placeholder") {
                return Some(email.to_string());
            }
        }

        None
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
            return scheme == "http" || scheme == "https";
        }
        false
    }

    pub fn extract_social_media(&self, text: &str, html: &str) -> (Option<String>, Option<String>) {
        let linkedin = self.extract_linkedin(text, html);
        let twitter = self.extract_twitter(text, html);
        (linkedin, twitter)
    }

    fn extract_linkedin(&self, text: &str, html: &str) -> Option<String> {
        let linkedin_patterns = [
            r"https?://(?:www\.)?linkedin\.com/in/([a-zA-Z0-9-]+)",
            r"https?://(?:www\.)?linkedin\.com/company/([a-zA-Z0-9-]+)",
            r"linkedin\.com/in/([a-zA-Z0-9-]+)",
            r"linkedin\.com/company/([a-zA-Z0-9-]+)",
        ];

        for pattern in &linkedin_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(caps) = regex.captures(html) {
                    if let Some(profile) = caps.get(0) {
                        let url = profile.as_str();
                        if url.starts_with("http") {
                            return Some(url.to_string());
                        } else {
                            return Some(format!("https://{}", url));
                        }
                    }
                }
                // Try in text too
                if let Some(caps) = regex.captures(text) {
                    if let Some(profile) = caps.get(0) {
                        let url = profile.as_str();
                        if url.starts_with("http") {
                            return Some(url.to_string());
                        } else {
                            return Some(format!("https://{}", url));
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_twitter(&self, text: &str, html: &str) -> Option<String> {
        let twitter_patterns = [
            r"https?://(?:www\.)?(?:twitter\.com|x\.com)/([a-zA-Z0-9_]+)",
            r"(?:twitter\.com|x\.com)/([a-zA-Z0-9_]+)",
            r"@([a-zA-Z0-9_]+)",
        ];

        for pattern in &twitter_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(caps) = regex.captures(html) {
                    if let Some(handle) = caps.get(1) {
                        let username = handle.as_str();
                        if !username.is_empty() && username != "twitter" && username != "x" {
                            return Some(format!("https://twitter.com/{}", username));
                        }
                    } else if let Some(full_match) = caps.get(0) {
                        let url = full_match.as_str();
                        if url.starts_with("http") {
                            return Some(url.to_string());
                        }
                    }
                }
                // Try in text too
                if let Some(caps) = regex.captures(text) {
                    if let Some(handle) = caps.get(1) {
                        let username = handle.as_str();
                        if !username.is_empty() && username != "twitter" && username != "x" {
                            return Some(format!("https://twitter.com/{}", username));
                        }
                    }
                }
            }
        }
        None
    }

    // NEW: Extract real emails from GitHub repository commits
    pub async fn extract_github_commit_emails(&self, github_url: &str) -> Vec<String> {
        match self.get_repo_commit_emails(github_url).await {
            Ok(emails) => {
                if !emails.is_empty() {
                    debug!(
                        "Found {} real commit emails for {}: {:?}",
                        emails.len(),
                        github_url,
                        emails
                    );
                }
                emails
            }
            Err(e) => {
                debug!("Failed to extract commit emails from {}: {}", github_url, e);
                vec![]
            }
        }
    }

    async fn get_repo_commit_emails(&self, github_url: &str) -> ExtractorResult<Vec<String>> {
        let repo_path = self.parse_github_url(github_url)?;

        // Check if it's a fork - skip forks
        let repo_info = self.get_repo_info(&repo_path).await?;
        if repo_info.fork {
            debug!("Skipping fork repository: {}", repo_path);
            return Ok(vec![]);
        }

        // Get recent commits from main branch
        let commits = self
            .get_recent_commits(&repo_path, &repo_info.default_branch)
            .await?;

        // Extract unique valid emails
        let emails = self.extract_valid_emails_from_commits(commits);

        Ok(emails)
    }

    fn parse_github_url(&self, url: &str) -> ExtractorResult<String> {
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(path) = parsed_url.path_segments() {
                let segments: Vec<&str> = path.collect();
                if segments.len() >= 2 {
                    return Ok(format!("{}/{}", segments[0], segments[1]));
                }
            }
        }
        Err(ScrapingError::ParseError(format!(
            "Invalid GitHub URL: {}",
            url
        )))
    }

    async fn get_repo_info(&self, repo_path: &str) -> ExtractorResult<GitHubRepo> {
        let url = format!("https://api.github.com/repos/{}", repo_path);

        let mut request = self.client.get(&url);
        if let Some(ref token) = self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }
        request = request.header("User-Agent", "Lead-Scraper/1.0");

        let response = request
            .send()
            .await
            .map_err(|e| ScrapingError::NetworkError(format!("Failed to get repo info: {}", e)))?;

        if !response.status().is_success() {
            return Err(ScrapingError::NetworkError(format!(
                "GitHub API error: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| ScrapingError::ParseError(format!("Failed to parse repo info: {}", e)))
    }

    async fn get_recent_commits(
        &self,
        repo_path: &str,
        branch: &str,
    ) -> ExtractorResult<Vec<GitHubCommit>> {
        let url = format!(
            "https://api.github.com/repos/{}/commits?sha={}&per_page=15",
            repo_path, branch
        );

        let mut request = self.client.get(&url);
        if let Some(ref token) = self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }
        request = request.header("User-Agent", "Lead-Scraper/1.0");

        let response = request
            .send()
            .await
            .map_err(|e| ScrapingError::NetworkError(format!("Failed to get commits: {}", e)))?;

        if !response.status().is_success() {
            warn!(
                "Failed to get commits for {}: {}",
                repo_path,
                response.status()
            );
            return Ok(vec![]);
        }

        response
            .json()
            .await
            .map_err(|e| ScrapingError::ParseError(format!("Failed to parse commits: {}", e)))
    }

    fn extract_valid_emails_from_commits(&self, commits: Vec<GitHubCommit>) -> Vec<String> {
        let mut emails = HashSet::new();
        let invalid_domains = [
            "users.noreply.github.com",
            "noreply.github.com",
            "example.com",
            "localhost",
            "test.com",
            "placeholder.com",
        ];

        let invalid_emails = [
            "noreply@github.com",
            "action@github.com",
            "actions@github.com",
            "github-actions@github.com",
            "github-actions[bot]@users.noreply.github.com",
            "dependabot[bot]@users.noreply.github.com",
            "renovate[bot]@users.noreply.github.com",
            "snyk-bot@users.noreply.github.com",
        ];

        for commit in commits {
            // Check author email
            let author_email = &commit.commit.author.email;
            if self.is_valid_commit_email(author_email, &invalid_domains, &invalid_emails) {
                emails.insert(author_email.clone());
            }

            // Check committer email (different from author sometimes)
            let committer_email = &commit.commit.committer.email;
            if self.is_valid_commit_email(committer_email, &invalid_domains, &invalid_emails) {
                emails.insert(committer_email.clone());
            }
        }

        // Convert to sorted vec, prioritize company emails
        let mut email_vec: Vec<String> = emails.into_iter().collect();
        email_vec.sort_by(|a, b| {
            let a_priority = self.email_priority(a);
            let b_priority = self.email_priority(b);
            b_priority.cmp(&a_priority)
        });

        // Limit to top 2 emails
        email_vec.truncate(2);
        email_vec
    }

    fn is_valid_commit_email(
        &self,
        email: &str,
        invalid_domains: &[&str],
        invalid_emails: &[&str],
    ) -> bool {
        // Basic email validation
        if !email.contains('@') || !email.contains('.') {
            return false;
        }

        // Check against specific invalid emails
        for invalid_email in invalid_emails {
            if email.eq_ignore_ascii_case(invalid_email) {
                return false;
            }
        }

        // Check against invalid domains
        for invalid_domain in invalid_domains {
            if email.ends_with(invalid_domain) {
                return false;
            }
        }

        // Skip bot emails
        if email.contains("[bot]") || email.contains("bot@") {
            return false;
        }

        // Must have valid TLD
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }

        let domain = parts[1];
        if domain.split('.').count() < 2 {
            return false;
        }

        true
    }

    fn email_priority(&self, email: &str) -> u8 {
        // Lowest priority - no-reply and bot emails
        if email.contains("noreply")
            || email.contains("[bot]")
            || email.contains("action")
            || email.contains("automated")
        {
            return 0;
        }

        // Low priority - personal emails
        if email.contains("@gmail.com")
            || email.contains("@yahoo.com")
            || email.contains("@hotmail.com")
            || email.contains("@outlook.com")
            || email.contains("@protonmail.com")
            || email.contains("@icloud.com")
        {
            return 2;
        }

        // Medium priority - known tech company emails
        if email.contains("@google.com")
            || email.contains("@microsoft.com")
            || email.contains("@apple.com")
            || email.contains("@facebook.com")
            || email.contains("@meta.com")
            || email.contains("@amazon.com")
        {
            return 4;
        }

        // Highest priority - startup/company emails (custom domains)
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_extraction() {
        let patterns = crate::config::PatternsConfig::default();
        let client = reqwest::Client::new();
        let extractor =
            DataExtractor::new(&patterns, client, None).expect("Failed to create extractor");

        let html = r#"<a href="mailto:test@example.com">Contact</a>"#;
        let text = "Contact us at test@example.com";

        let email = extractor.extract_email(text, html);
        assert_eq!(email, Some("test@example.com".to_string()));
    }
}
````

## File: src/lib.rs
````rust
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
````

## File: src/main.rs
````rust
use clap::Parser;

use leadscraper::errors::Result as ScrapingResult;
use leadscraper::Config;
use leadscraper::LeadScraper;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "leadscraper")]
#[command(about = "Aggressive lead scraper for startup data extraction")]
struct Cli {
    #[arg(short, long, default_value = "config/scraper.yaml")]
    config: String,

    #[arg(short, long, default_value = "results")]
    output: String,

    #[arg(short, long, default_value = "true")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> ScrapingResult<()> {
    let cli = Cli::parse();

    // Initialize tracing - always verbose by default now
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    info!("🚀 Starting Lead Scraper");

    // Load configuration
    let config = Config::load(&cli.config)?;
    info!("📋 Configuration loaded from: {}", cli.config);

    // Initialize scraper
    let scraper = LeadScraper::new(config).await?;

    // Execute scraping
    info!("🔥 Beginning aggressive lead extraction...");
    let leads = scraper.scrape_all_sources().await?;

    info!("✅ Extracted {} total leads", leads.len());

    // Save results
    scraper.save_leads(&leads, &cli.output).await?;
    info!("💾 Results saved to directory: {}", cli.output);

    Ok(())
}
````

## File: src/models.rs
````rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub name: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub github_email: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub source: Source,
    pub country: Option<String>,
    pub description: Option<String>,
    pub scraped_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    YCombinator,
    GitHubAwesome { repository: String },
    BetaList,
    Website { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadStats {
    pub total_leads: usize,
    pub contactable_leads: usize,
    pub research_leads: usize,
    pub contact_rate: f32,
    pub sources_breakdown: HashMap<String, SourceStats>,
    pub countries_breakdown: HashMap<String, usize>,
    pub email_types: EmailTypeStats,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceStats {
    pub total: usize,
    pub with_contact: usize,
    pub contact_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTypeStats {
    pub direct_emails: usize,
    pub github_emails: usize,
    pub no_emails: usize,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::YCombinator => write!(f, "Y Combinator"),
            Source::GitHubAwesome { repository } => write!(f, "GitHub/{}", repository),
            Source::BetaList => write!(f, "BetaList"),
            Source::Website { url } => write!(f, "Website: {}", url),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScrapedData {
    pub name: String,
    pub website: Option<String>,
    pub raw_text: String,
    pub html: String,
}

impl Lead {
    pub fn new(name: String, source: Source) -> Self {
        Self {
            name,
            website: None,
            email: None,
            github_email: None,
            linkedin: None,
            twitter: None,
            source,
            country: None,
            description: None,
            scraped_at: Utc::now(),
        }
    }

    pub fn with_website(mut self, website: Option<String>) -> Self {
        self.website = website;
        self
    }

    pub fn with_email(mut self, email: Option<String>) -> Self {
        self.email = email;
        self
    }

    pub fn with_github_email(mut self, github_email: Option<String>) -> Self {
        self.github_email = github_email;
        self
    }

    pub fn with_linkedin(mut self, linkedin: Option<String>) -> Self {
        self.linkedin = linkedin;
        self
    }

    pub fn with_twitter(mut self, twitter: Option<String>) -> Self {
        self.twitter = twitter;
        self
    }

    pub fn with_country(mut self, country: Option<String>) -> Self {
        self.country = country;
        self
    }

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    // NEW: Contact scoring methods
    pub fn has_contact(&self) -> bool {
        self.email.is_some() || self.github_email.is_some()
    }

    pub fn contact_score(&self) -> u8 {
        let mut score = 0;
        if self.email.is_some() {
            score += 3;
        }
        if self.github_email.is_some() {
            score += 2;
        }
        if self.linkedin.is_some() {
            score += 1;
        }
        if self.twitter.is_some() {
            score += 1;
        }
        score
    }
}

impl LeadStats {
    pub fn new(contactable_leads: &[&Lead], research_leads: &[&Lead]) -> Self {
        let total_leads = contactable_leads.len() + research_leads.len();
        let contactable_count = contactable_leads.len();
        let research_count = research_leads.len();

        let contact_rate = if total_leads > 0 {
            (contactable_count as f32 / total_leads as f32) * 100.0
        } else {
            0.0
        };

        // Sources breakdown
        let mut sources_breakdown = HashMap::new();
        let all_leads: Vec<&Lead> = contactable_leads
            .iter()
            .chain(research_leads.iter())
            .copied()
            .collect();

        for lead in &all_leads {
            let source_name = match &lead.source {
                Source::YCombinator => "Y Combinator".to_string(),
                Source::GitHubAwesome { repository } => format!("GitHub/{}", repository),
                Source::BetaList => "BetaList".to_string(),
                Source::Website { url } => {
                    if url.contains("github.com") {
                        "GitHub Project".to_string()
                    } else {
                        "Website".to_string()
                    }
                }
            };

            let stats = sources_breakdown.entry(source_name).or_insert(SourceStats {
                total: 0,
                with_contact: 0,
                contact_rate: 0.0,
            });

            stats.total += 1;
            if lead.has_contact() {
                stats.with_contact += 1;
            }
        }

        // Calculate contact rates for each source
        for stats in sources_breakdown.values_mut() {
            stats.contact_rate = if stats.total > 0 {
                (stats.with_contact as f32 / stats.total as f32) * 100.0
            } else {
                0.0
            };
        }

        // Countries breakdown
        let mut countries_breakdown = HashMap::new();
        for lead in &all_leads {
            if let Some(ref country) = lead.country {
                *countries_breakdown.entry(country.clone()).or_insert(0) += 1;
            }
        }

        // Email types
        let mut direct_emails = 0;
        let mut github_emails = 0;
        let mut no_emails = 0;

        for lead in &all_leads {
            match (&lead.email, &lead.github_email) {
                (Some(_), _) => direct_emails += 1,
                (None, Some(_)) => github_emails += 1,
                (None, None) => no_emails += 1,
            }
        }

        Self {
            total_leads,
            contactable_leads: contactable_count,
            research_leads: research_count,
            contact_rate,
            sources_breakdown,
            countries_breakdown,
            email_types: EmailTypeStats {
                direct_emails,
                github_emails,
                no_emails,
            },
            generated_at: Utc::now(),
        }
    }
}
````

## File: src/scraper_util.rs
````rust
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
````

## File: README.md
````markdown
# MiniCRM Lead Scraper

Aggressive lead extraction system for startup data mining. Built in Rust for maximum performance and reliability.

## Features

- **Multi-Source Scraping**: Y Combinator, GitHub Awesome lists, BetaList
- **Smart Data Extraction**: Email, website, country detection using regex patterns and TLD mapping
- **Zero Dependencies**: Pure Rust implementation with minimal external dependencies
- **Configurable**: YAML-based configuration for all scraping parameters
- **Output Formats**: JSON and CSV export
- **Rate Limited**: Respectful scraping with configurable delays
- **Error Resilient**: Continue scraping even if individual sources fail

## Quick Start

```bash

# Default run (uses config/scraper.yaml, outputs to results/, verbose mode)
cargo run

# Custom configuration
cargo run -- --config custom.yaml --output my_output

# Copy source files and dependencies
# Run scraper
cargo run -- --config config/scraper.yaml --output results/

# With verbose logging
cargo run -- --config config/scraper.yaml --output results/ --verbose
```

## Configuration

Edit `config/scraper.yaml` to customize:
- Source endpoints and selectors
- Email/location extraction patterns
- TLD country mappings
- Rate limiting settings

## Output

Results saved in two formats:
- `leads.json` - Full structured data
- `leads.csv` - Spreadsheet-friendly format

## Architecture

```
src/
├── main.rs          # CLI interface
├── lib.rs           # Library exports
├── config.rs        # YAML configuration loading
├── models.rs        # Lead and Source data structures
├── extractors.rs    # Email/country/website extraction logic
└── scraper.rs       # Main scraping orchestration
```

## Lead Data Structure

```rust
struct Lead {
    name: String,                    // Company name
    website: Option<String>,         // Primary website URL
    email: Option<String>,           // Contact email
    source: Source,                  // Where data was scraped from
    country: Option<String>,         // Detected country
    description: Option<String>,     // Company description
    scraped_at: DateTime<Utc>,      // Timestamp
}
```

## License

MIT
````
