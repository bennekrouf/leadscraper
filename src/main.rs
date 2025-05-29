// Update src/main.rs to generate timestamped folders

use chrono::{DateTime, Utc};
use clap::Parser;
use leadscraper::errors::Result as ScrapingResult;
use leadscraper::{Config, LeadScraper};
use std::path::Path;
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

    /// Use custom folder name instead of timestamp (optional)
    #[arg(long)]
    folder_name: Option<String>,

    /// Skip timestamp folder creation and use output path directly
    #[arg(long)]
    no_timestamp: bool,
}

#[tokio::main]
async fn main() -> ScrapingResult<()> {
    let cli = Cli::parse();

    // Initialize tracing
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

    // Generate timestamped output directory
    let output_dir = if cli.no_timestamp {
        cli.output.clone()
    } else {
        generate_output_directory(&cli.output, cli.folder_name.as_deref())?
    };

    info!("📁 Output directory: {}", output_dir);

    // Initialize scraper
    let scraper = LeadScraper::new(config).await?;

    // Execute scraping
    info!("🔥 Beginning aggressive lead extraction...");
    let start_time = Utc::now();
    let leads = scraper.scrape_all_sources().await?;
    let end_time = Utc::now();
    let duration = end_time - start_time;

    info!(
        "✅ Extracted {} total leads in {:.1}s",
        leads.len(),
        duration.num_milliseconds() as f64 / 1000.0
    );

    // Save results with run metadata
    scraper
        .save_leads_with_metadata(&leads, &output_dir, start_time, end_time)
        .await?;
    info!("💾 Results saved to: {}", output_dir);

    // Print summary
    print_run_summary(&output_dir, &leads, duration);

    Ok(())
}

/// Generate timestamped output directory
fn generate_output_directory(base_path: &str, custom_name: Option<&str>) -> ScrapingResult<String> {
    let timestamp = Utc::now();

    let folder_name = match custom_name {
        Some(name) => {
            // Custom name with timestamp prefix
            format!("{}_{}", timestamp.format("%Y%m%d_%H%M%S"), name)
        }
        None => {
            // Pure timestamp folder
            timestamp.format("%Y%m%d_%H%M%S_scrape").to_string()
        }
    };

    let output_path = Path::new(base_path).join(folder_name);
    let output_str = output_path.to_string_lossy().to_string();

    // Create the directory
    std::fs::create_dir_all(&output_path).map_err(|e| {
        leadscraper::ScrapingError::IoError(format!(
            "Failed to create timestamped output directory '{}': {}",
            output_str, e
        ))
    })?;

    Ok(output_str)
}

/// Print comprehensive run summary
fn print_run_summary(output_dir: &str, leads: &[leadscraper::Lead], duration: chrono::Duration) {
    info!("🎯 Run Summary:");
    info!("   📊 Total leads: {}", leads.len());
    info!(
        "   ⏱️  Duration: {:.2}s",
        duration.num_milliseconds() as f64 / 1000.0
    );
    info!("   📁 Results: {}", output_dir);

    // Quick stats
    let contactable = leads.iter().filter(|l| l.has_contact()).count();
    let contact_rate = if !leads.is_empty() {
        (contactable as f64 / leads.len() as f64) * 100.0
    } else {
        0.0
    };

    info!("   📧 Contactable: {} ({:.1}%)", contactable, contact_rate);

    // Source breakdown
    let mut source_counts = std::collections::HashMap::new();
    for lead in leads {
        let source_name = match &lead.source {
            leadscraper::Source::YCombinator => "YC",
            leadscraper::Source::GitHubAwesome { .. } => "GitHub",
            leadscraper::Source::BetaList => "BetaList",
            leadscraper::Source::Website { .. } => "Website",
        };
        *source_counts.entry(source_name).or_insert(0) += 1;
    }

    info!("   🔍 Sources: {:?}", source_counts);
}

