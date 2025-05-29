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

#[derive(Clone)]
pub struct DataExtractor {
    email_patterns: EmailPatterns,
    location_patterns: LocationPatterns,
    tld_mapping: HashMap<String, String>,
    client: Client,
    github_token: Option<String>,
}

// Also need to make the inner structs cloneable
#[derive(Clone)]
struct EmailPatterns {
    mailto: Regex,
    generic: Regex,
}

#[derive(Clone)]
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

