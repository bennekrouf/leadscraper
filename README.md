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
# Create project
cargo new minicrm-scraper --bin
cd minicrm-scraper

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
