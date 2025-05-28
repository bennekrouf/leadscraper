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
