use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub name: String,
    pub website: Option<String>,
    pub email: Option<String>,
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
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::YCombinator => write!(f, "Y Combinator"),
            Source::GitHubAwesome { repository } => write!(f, "GitHub/{}", repository),
            Source::BetaList => write!(f, "BetaList"),
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

    pub fn with_country(mut self, country: Option<String>) -> Self {
        self.country = country;
        self
    }

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
}
