# Lead Scraper - Modular Architecture

Aggressive lead extraction system for startup data mining. Built in Rust with a **modular scraper architecture** for maximum maintainability and extensibility.

## 🚀 What's New in v0.2.0

- **Modular Scrapers**: Each source has its own dedicated scraper module
- **Trait-Based Design**: Easy to add new sources by implementing `SourceScraper`
- **Better Error Handling**: Individual scraper failures don't crash the entire process
- **Enhanced Testing**: Each scraper can be tested independently
- **Cleaner Code**: Separation of concerns with clear boundaries

## ✨ Features

- **Multi-Source Scraping**: Y Combinator, GitHub Awesome lists, BetaList
- **Smart Data Extraction**: Email, website, social media, country detection
- **GitHub Integration**: Extract real emails from commit history
- **Modular Architecture**: Easy to extend with new sources
- **Configurable**: YAML-based configuration for all parameters
- **Multiple Outputs**: JSON, CSV exports with lead categorization
- **Rate Limited**: Respectful scraping with configurable delays
- **Error Resilient**: Continue scraping even if individual sources fail

## 🏗️ Architecture

```
src/
├── main.rs              # CLI interface
├── lib.rs               # Library exports + legacy compatibility
├── config.rs            # YAML configuration loading
├── models.rs            # Lead and Source data structures
├── extractors.rs        # Email/country/website extraction logic
├── scraper_util.rs      # Main orchestrator (simplified)
└── scrapers/            # 🆕 Modular scraper architecture
    ├── mod.rs           # SourceScraper trait + factory
    ├── base.rs          # Common scraper functionality
    ├── ycombinator.rs   # Y Combinator scraper
    ├── github_awesome.rs # GitHub Awesome lists scraper
    └── betalist.rs      # BetaList scraper
```

## 🔧 Quick Start

```bash
# Default run (uses config/scraper.yaml, outputs to results/)
cargo run

# Custom configuration
cargo run -- --config custom.yaml --output my_output

# With verbose logging
cargo run -- --config config/scraper.yaml --output results/ --verbose
```

## ⚙️ Configuration

Edit `config/scraper.yaml`:

```yaml
scraper:
  timeout_seconds: 30
  max_concurrent_requests: 10
  github_token: null  # Optional: "ghp_your_token" for higher rate limits

sources:
  ycombinator:
    enabled: true
    base_url: "https://www.ycombinator.com"
    endpoints:
      - "/companies?batch=W24"
      - "/companies?batch=S24"
      
  github_awesome:
    enabled: true
    repositories:
      - "sindresorhus/awesome"
      - "awesome-selfhosted/awesome-selfhosted"
      
  betalist:
    enabled: true
    base_url: "https://betalist.com"
    endpoints:
      - "/startups"

patterns:
  email:
    mailto: "mailto:([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,})"
    generic: "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
  # ... more patterns
```

## 📊 Output Structure

Results are saved in multiple formats for different use cases:

```
results/
├── contactable_leads.json    # Leads with email/contact info (priority)
├── research_leads.json       # Leads requiring more research
├── all_leads.json           # Complete dataset (legacy format)
├── all_leads.csv            # Spreadsheet-friendly format
└── stats.json              # Comprehensive statistics
```

## 🧩 Adding New Scrapers

The modular architecture makes it easy to add new sources:

1. **Create scraper module**: `src/scrapers/newsource.rs`
2. **Implement trait**:
```rust
use async_trait::async_trait;
use super::{base::BaseScraper, SourceScraper};

pub struct NewSourceScraper {
    config: NewSourceConfig,
    base: BaseScraper,
}

#[async_trait]
impl SourceScraper for NewSourceScraper {
    async fn scrape(&self) -> Result<Vec<Lead>> {
        // Your scraping logic here
    }
    
    fn source_name(&self) -> &'static str {
        "New Source"
    }
    
    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
```
3. **Register in factory**: Add to `create_scrapers()` in `scrapers/mod.rs`
4. **Add configuration**: Update `config.rs` and YAML schema

## 📈 Lead Data Structure

```rust
struct Lead {
    name: String,                    // Company name
    website: Option<String>,         // Primary website URL
    email: Option<String>,           // Direct contact email
    github_email: Option<String>,    // Email from GitHub commits
    linkedin: Option<String>,        // LinkedIn profile
    twitter: Option<String>,         // Twitter/X profile
    source: Source,                  // Where data was scraped from
    country: Option<String>,         // Detected country
    description: Option<String>,     // Company description
    scraped_at: DateTime<Utc>,      // Timestamp
}
```

## 🧪 Testing

Each scraper has dedicated tests:

```bash
# Test all modules
cargo test

# Test specific scraper
cargo test scrapers::ycombinator::tests

# Test with output
cargo test -- --nocapture
```

## 🔍 Key Improvements

### Before (Monolithic)
- All scraping logic in one 400+ line file
- Hard to maintain and extend
- Difficult to test individual sources
- Single point of failure

### After (Modular)
- ✅ **Separation of Concerns**: Each scraper handles one source
- ✅ **Easy Extension**: Add new sources with minimal code changes  
- ✅ **Independent Testing**: Test each scraper in isolation
- ✅ **Error Isolation**: One scraper failure doesn't affect others
- ✅ **Clean Architecture**: Clear interfaces and boundaries
- ✅ **Maintainable**: Much easier to debug and enhance

## 🚀 Performance & Reliability

- **Concurrent Scraping**: Multiple sources processed in parallel
- **Rate Limiting**: Respectful delays between requests
- **Error Recovery**: Continue processing if individual sources fail
- **GitHub Integration**: Extract real emails from commit history (not just public profiles)
- **Smart Filtering**: Remove bot/automated emails and prioritize business contacts

## 📋 Sample Output

```json
{
  "total_leads": 157,
  "contactable_leads": 23,
  "research_leads": 134,
  "contact_rate": 14.6,
  "sources_breakdown": {
    "Y Combinator": {"total": 50, "with_contact": 12, "contact_rate": 24.0},
    "GitHub Awesome": {"total": 87, "with_contact": 8, "contact_rate": 9.2},
    "BetaList": {"total": 20, "with_contact": 3, "contact_rate": 15.0}
  }
}
```

## 🛠️ Development

Built with Rust best practices:
- Generic types over dynamic dispatch
- Comprehensive error handling with custom error types
- No `.unwrap()` calls - all errors properly handled
- YAML configuration with clear structure
- Structured logging with `tracing`
- Modular architecture for maintainability

## 📄 License

MIT License - See LICENSE file for details.

---

**Ready to scale your lead generation? Clone, configure, and start scraping! 🚀**
