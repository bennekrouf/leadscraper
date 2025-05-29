use crate::{
    config::Config,
    errors::{Result, ScrapingError},
    extractors::DataExtractor,
    models::{Lead, LeadStats},
    scrapers::{create_scrapers, SourceScraper},
};
use reqwest::Client;
use std::time::Duration;
use tracing::{error, info, warn};

/// Main lead scraper orchestrator - now simplified and using modular scrapers
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

    /// Scrape all enabled sources using the modular scraper architecture
    pub async fn scrape_all_sources(&self) -> Result<Vec<Lead>> {
        info!("🚀 Starting lead extraction from all enabled sources...");

        let scrapers = create_scrapers(&self.config, &self.client, &self.extractor);

        if scrapers.is_empty() {
            warn!("⚠️  No scrapers enabled in configuration!");
            return Ok(Vec::new());
        }

        info!("📋 Enabled scrapers: {}", scrapers.len());
        for scraper in &scrapers {
            let expected = scraper
                .expected_leads_count()
                .map(|c| format!(" (expected ~{})", c))
                .unwrap_or_default();
            info!("   • {}{}", scraper.source_name(), expected);
        }

        let mut all_leads = Vec::new();
        let mut successful_scrapers = 0;
        let mut failed_scrapers = 0;

        for scraper in scrapers {
            let source_name = scraper.source_name();
            info!("🔥 Starting {} scraper...", source_name);

            match scraper.scrape().await {
                Ok(mut leads) => {
                    let count = leads.len();
                    info!("✅ {}: {} leads extracted", source_name, count);
                    all_leads.append(&mut leads);
                    successful_scrapers += 1;
                }
                Err(e) => {
                    error!("❌ {} scraping failed: {}", source_name, e);
                    failed_scrapers += 1;
                    // Continue with other scrapers - don't fail completely
                }
            }
        }

        // Summary
        info!("🎯 Scraping Summary:");
        info!("   ✅ Successful scrapers: {}", successful_scrapers);
        if failed_scrapers > 0 {
            info!("   ❌ Failed scrapers: {}", failed_scrapers);
        }
        info!("   📊 Total leads extracted: {}", all_leads.len());

        if all_leads.is_empty() {
            warn!(
                "🚨 No leads extracted from any source! Check your config and network connection."
            );
        } else {
            // Log some quick stats
            let contactable = all_leads
                .iter()
                .filter(|l| self.has_contact_info(l))
                .count();
            let contact_rate = if all_leads.len() > 0 {
                (contactable as f32 / all_leads.len() as f32) * 100.0
            } else {
                0.0
            };
            info!(
                "   📧 Contactable leads: {} ({:.1}%)",
                contactable, contact_rate
            );
        }

        Ok(all_leads)
    }

    /// Save leads to output directory with categorization and stats
    pub async fn save_leads(&self, leads: &[Lead], output_path: &str) -> Result<()> {
        use std::fs;

        info!("💾 Saving results to: {}", output_path);

        fs::create_dir_all(output_path).map_err(|e| {
            ScrapingError::IoError(format!(
                "Failed to create output directory '{}': {}",
                output_path, e
            ))
        })?;

        // Separate leads by contact availability
        let (contactable_leads, research_leads): (Vec<_>, Vec<_>) =
            leads.iter().partition(|lead| self.has_contact_info(lead));

        // Save contactable leads (priority targets)
        let contactable_output = format!("{}/contactable_leads.json", output_path);
        let contactable_json = serde_json::to_string_pretty(&contactable_leads).map_err(|e| {
            ScrapingError::IoError(format!("Failed to serialize contactable leads: {}", e))
        })?;
        fs::write(&contactable_output, contactable_json).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write contactable leads file: {}", e))
        })?;

        // Save research leads (need more investigation)
        let research_output = format!("{}/research_leads.json", output_path);
        let research_json = serde_json::to_string_pretty(&research_leads).map_err(|e| {
            ScrapingError::IoError(format!("Failed to serialize research leads: {}", e))
        })?;
        fs::write(&research_output, research_json).map_err(|e| {
            ScrapingError::IoError(format!("Failed to write research leads file: {}", e))
        })?;

        // Create comprehensive stats
        let stats = LeadStats::new(&contactable_leads, &research_leads);
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

        // Save CSV for spreadsheet users
        self.save_csv_format(leads, output_path).await?;

        // Print detailed summary
        info!("✅ Results saved successfully:");
        info!(
            "   📧 Contactable leads: {} -> {}",
            contactable_leads.len(),
            contactable_output
        );
        info!(
            "   🔍 Research leads: {} -> {}",
            research_leads.len(),
            research_output
        );
        info!("   📊 Detailed stats: {}", stats_output);
        info!("   📋 All leads (JSON): {}", all_leads_output);
        info!("   📄 CSV export: {}/all_leads.csv", output_path);

        // Print top-level stats
        info!("🎯 Final Statistics:");
        info!("   Total leads: {}", stats.total_leads);
        info!("   Contact rate: {:.1}%", stats.contact_rate);
        info!(
            "   Top sources: {:?}",
            stats
                .sources_breakdown
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.total))
                .collect::<Vec<_>>()
        );

        Ok(())
    }

    /// Save leads in CSV format for spreadsheet applications
    async fn save_csv_format(&self, leads: &[Lead], output_path: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let csv_path = format!("{}/all_leads.csv", output_path);
        let mut file = File::create(&csv_path)
            .map_err(|e| ScrapingError::IoError(format!("Failed to create CSV file: {}", e)))?;

        // Write CSV header
        writeln!(
            file,
            "Name,Website,Email,GitHub Email,LinkedIn,Twitter,Source,Country,Description,Scraped At,Contact Score"
        ).map_err(|e| ScrapingError::IoError(format!("Failed to write CSV header: {}", e)))?;

        // Write data rows
        for lead in leads {
            let source_str = match &lead.source {
                crate::models::Source::YCombinator => "Y Combinator".to_string(),
                crate::models::Source::GitHubAwesome { repository } => {
                    format!("GitHub/{}", repository)
                }
                crate::models::Source::BetaList => "BetaList".to_string(),
                crate::models::Source::Website { url } => {
                    if url.contains("github.com") {
                        "GitHub Project".to_string()
                    } else {
                        "Website".to_string()
                    }
                }
            };

            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{}",
                Self::csv_escape(&lead.name),
                lead.website.as_deref().unwrap_or(""),
                lead.email.as_deref().unwrap_or(""),
                lead.github_email.as_deref().unwrap_or(""),
                lead.linkedin.as_deref().unwrap_or(""),
                lead.twitter.as_deref().unwrap_or(""),
                Self::csv_escape(&source_str),
                lead.country.as_deref().unwrap_or(""),
                Self::csv_escape(&lead.description.as_deref().unwrap_or("").replace('\n', " ")),
                lead.scraped_at.format("%Y-%m-%d %H:%M:%S UTC"),
                lead.contact_score()
            )
            .map_err(|e| ScrapingError::IoError(format!("Failed to write CSV row: {}", e)))?;
        }

        Ok(())
    }

    /// Escape CSV field content
    fn csv_escape(text: &str) -> String {
        if text.contains(',') || text.contains('"') || text.contains('\n') {
            format!("\"{}\"", text.replace('"', "\"\""))
        } else {
            text.to_string()
        }
    }

    /// Check if lead has any contact information
    fn has_contact_info(&self, lead: &Lead) -> bool {
        lead.email.is_some() || lead.github_email.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Source;
    use chrono::Utc;

    #[test]
    fn test_has_contact_info() {
        let config = Config::default();
        let client = reqwest::Client::new();
        let github_token = None;
        let extractor = DataExtractor::new(&config.patterns, client.clone(), github_token)
            .expect("Failed to create extractor");

        let scraper = LeadScraper {
            client,
            config,
            extractor,
        };

        // Lead with email
        let lead_with_email = Lead {
            name: "Test Company".to_string(),
            website: None,
            email: Some("test@example.com".to_string()),
            github_email: None,
            linkedin: None,
            twitter: None,
            source: Source::BetaList,
            country: None,
            description: None,
            scraped_at: Utc::now(),
        };
        assert!(scraper.has_contact_info(&lead_with_email));

        // Lead without contact
        let lead_no_contact = Lead {
            name: "Test Company 2".to_string(),
            website: Some("https://example.com".to_string()),
            email: None,
            github_email: None,
            linkedin: None,
            twitter: None,
            source: Source::BetaList,
            country: None,
            description: None,
            scraped_at: Utc::now(),
        };
        assert!(!scraper.has_contact_info(&lead_no_contact));
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(LeadScraper::csv_escape("simple"), "simple");
        assert_eq!(LeadScraper::csv_escape("with,comma"), "\"with,comma\"");
        assert_eq!(LeadScraper::csv_escape("with\"quote"), "\"with\"\"quote\"");
        assert_eq!(
            LeadScraper::csv_escape("with\nnewline"),
            "\"with\nnewline\""
        );
    }
}

