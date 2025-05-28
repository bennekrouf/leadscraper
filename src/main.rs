use clap::Parser;

use leadscraper::errors::Result as ScrapingResult;
use leadscraper::Config;
use leadscraper::LeadScraper;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "minicrm-scraper")]
#[command(about = "Aggressive lead scraper for MiniCRM")]
struct Cli {
    #[arg(short, long, default_value = "config/scraper.yaml")]
    config: String,

    #[arg(short, long, default_value = "output")]
    output: String,

    #[arg(short, long)]
    verbose: bool,
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

    info!("Starting MiniCRM Lead Scraper");

    // Load configuration
    let config = Config::load(&cli.config)?;
    info!("Configuration loaded from: {}", cli.config);

    // Initialize scraper
    let scraper = LeadScraper::new(config).await?;

    // Execute scraping
    info!("Beginning aggressive lead extraction...");
    let leads = scraper.scrape_all_sources().await?;

    info!("Extracted {} leads", leads.len());

    // Save results
    scraper.save_leads(&leads, &cli.output).await?;
    info!("Results saved to: {}", cli.output);

    Ok(())
}
