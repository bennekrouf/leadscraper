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

// NEW: Run metadata to track scraping sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMetadata {
    pub run_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: f64,
    pub total_leads: usize,
    pub contactable_leads: usize,
    pub contact_rate: f32,
    pub scraper_version: String,
    pub config_summary: ConfigSummary,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSummary {
    pub enabled_sources: Vec<String>,
    pub timeout_seconds: u64,
    pub github_token_configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub leads_per_second: f64,
    pub sources_processed: usize,
    pub successful_sources: usize,
    pub failed_sources: usize,
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

    // Contact scoring methods
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

impl RunMetadata {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>, stats: &LeadStats) -> Self {
        let duration = end_time - start_time;
        let duration_seconds = duration.num_milliseconds() as f64 / 1000.0;

        let leads_per_second = if duration_seconds > 0.0 {
            stats.total_leads as f64 / duration_seconds
        } else {
            0.0
        };

        // Generate a simple run ID based on timestamp
        let run_id = start_time.format("%Y%m%d_%H%M%S").to_string();

        Self {
            run_id,
            start_time,
            end_time,
            duration_seconds,
            total_leads: stats.total_leads,
            contactable_leads: stats.contactable_leads,
            contact_rate: stats.contact_rate,
            scraper_version: env!("CARGO_PKG_VERSION").to_string(),
            config_summary: ConfigSummary {
                enabled_sources: stats.sources_breakdown.keys().cloned().collect(),
                timeout_seconds: 30, // Default value - could be passed from config
                github_token_configured: false, // Could be passed from config
            },
            performance: PerformanceMetrics {
                leads_per_second,
                sources_processed: stats.sources_breakdown.len(),
                successful_sources: stats.sources_breakdown.len(), // Simplification
                failed_sources: 0,                                 // Could be tracked from scraper
            },
        }
    }
}

