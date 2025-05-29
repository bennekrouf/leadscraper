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

