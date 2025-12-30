//! Configuration for BscScan API client

use crate::error::{Error, Result};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.etherscan.io/v2/api";
const DEFAULT_CHAIN_ID: u64 = 1; // Ethereum Mainnet

/// Configuration for Etherscan API client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Etherscan API keys (supports multiple for rotation)
    pub api_keys: Vec<String>,

    /// Base URL for Etherscan API (default: v2 api)
    pub base_url: String,

    /// Chain ID (default: 1 for Mainnet)
    pub chain_id: u64,

    /// Rate limit in requests per second (default: 5 for free tier)
    pub rate_limit_per_second: u32,

    /// HTTP request timeout in seconds
    pub timeout_seconds: u64,

    /// Cache TTL in seconds (0 = no cache)
    pub cache_ttl_seconds: u64,

    /// Maximum cache size (number of entries)
    pub cache_max_size: u64,
}

impl ClientConfig {
    /// Create a new configuration with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_keys: vec![api_key.into()],
            base_url: DEFAULT_BASE_URL.to_string(),
            chain_id: DEFAULT_CHAIN_ID,
            rate_limit_per_second: 5,
            timeout_seconds: 30,
            cache_ttl_seconds: 300, // 5 minutes
            cache_max_size: 1000,
        }
    }

    /// Create configuration for Ethereum Sepolia testnet
    pub fn testnet(api_key: impl Into<String>) -> Self {
        Self {
            api_keys: vec![api_key.into()],
            base_url: DEFAULT_BASE_URL.to_string(),
            chain_id: 11155111, // Sepolia
            rate_limit_per_second: 5,
            timeout_seconds: 30,
            cache_ttl_seconds: 300,
            cache_max_size: 1000,
        }
    }

    /// Load configuration from environment variables
    ///
    /// Environment variables:
    /// - `ETHERSCAN_API_KEYS`: Comma-separated list of API keys (required)
    /// - `ETHERSCAN_BASE_URL`: Base URL (optional, defaults to v2 api)
    /// - `ETHERSCAN_CHAIN_ID`: Chain ID (optional, defaults to 1)
    /// - `ETHERSCAN_RATE_LIMIT`: Rate limit per second (optional, default: 5)
    /// - `ETHERSCAN_TIMEOUT`: Timeout in seconds (optional, default: 30)
    /// - `ETHERSCAN_CACHE_TTL`: Cache TTL in seconds (optional, default: 300)
    pub fn from_env() -> Result<Self> {
        let api_keys = std::env::var("ETHERSCAN_API_KEYS")
            .map_err(|_| Error::InvalidConfig("ETHERSCAN_API_KEYS not set".to_string()))?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if api_keys.is_empty() {
            return Err(Error::InvalidConfig(
                "ETHERSCAN_API_KEYS cannot be empty".to_string(),
            ));
        }

        let base_url = std::env::var("ETHERSCAN_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        let chain_id = std::env::var("ETHERSCAN_CHAIN_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_CHAIN_ID);

        let rate_limit_per_second = std::env::var("ETHERSCAN_RATE_LIMIT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let timeout_seconds = std::env::var("ETHERSCAN_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        let cache_ttl_seconds = std::env::var("ETHERSCAN_CACHE_TTL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300);

        let cache_max_size = std::env::var("ETHERSCAN_CACHE_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);

        Ok(Self {
            api_keys,
            base_url,
            chain_id,
            rate_limit_per_second,
            timeout_seconds,
            cache_ttl_seconds,
            cache_max_size,
        })
    }

    /// Create a builder for ClientConfig
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }

    /// Get request timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }

    /// Get cache TTL as Duration
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_seconds)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.api_keys.is_empty() {
            return Err(Error::InvalidConfig("At least one API key required".to_string()));
        }

        for key in &self.api_keys {
            if key.is_empty() {
                return Err(Error::InvalidConfig("API key cannot be empty".to_string()));
            }
        }

        if self.base_url.is_empty() {
            return Err(Error::InvalidConfig("Base URL cannot be empty".to_string()));
        }

        if self.rate_limit_per_second == 0 {
            return Err(Error::InvalidConfig(
                "Rate limit must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for ClientConfig
#[derive(Debug, Default)]
pub struct ClientConfigBuilder {
    api_keys: Vec<String>,
    base_url: Option<String>,
    chain_id: Option<u64>,
    rate_limit_per_second: Option<u32>,
    timeout_seconds: Option<u64>,
    cache_ttl_seconds: Option<u64>,
    cache_max_size: Option<u64>,
}

impl ClientConfigBuilder {
    /// Add an API key
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_keys.push(key.into());
        self
    }

    /// Add multiple API keys
    pub fn api_keys(mut self, keys: Vec<String>) -> Self {
        self.api_keys = keys;
        self
    }

    /// Set base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set chain ID
    pub fn chain_id(mut self, id: u64) -> Self {
        self.chain_id = Some(id);
        self
    }

    /// Use testnet
    pub fn testnet(mut self) -> Self {
        self.chain_id = Some(11155111); // Sepolia
        self
    }

    /// Set rate limit per second
    pub fn rate_limit(mut self, limit: u32) -> Self {
        self.rate_limit_per_second = Some(limit);
        self
    }

    /// Set request timeout in seconds
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Set cache TTL in seconds
    pub fn cache_ttl(mut self, seconds: u64) -> Self {
        self.cache_ttl_seconds = Some(seconds);
        self
    }

    /// Set cache max size
    pub fn cache_max_size(mut self, size: u64) -> Self {
        self.cache_max_size = Some(size);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<ClientConfig> {
        if self.api_keys.is_empty() {
            return Err(Error::InvalidConfig(
                "At least one API key is required".to_string(),
            ));
        }

        let config = ClientConfig {
            api_keys: self.api_keys,
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            chain_id: self.chain_id.unwrap_or(DEFAULT_CHAIN_ID),
            rate_limit_per_second: self.rate_limit_per_second.unwrap_or(5),
            timeout_seconds: self.timeout_seconds.unwrap_or(30),
            cache_ttl_seconds: self.cache_ttl_seconds.unwrap_or(300),
            cache_max_size: self.cache_max_size.unwrap_or(1000),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = ClientConfig::new("test-key");
        assert_eq!(config.api_keys.len(), 1);
        assert_eq!(config.api_keys[0], "test-key");
        assert_eq!(config.rate_limit_per_second, 5);
    }

    #[test]
    fn test_testnet_config() {
        let config = ClientConfig::testnet("test-key");
        assert_eq!(config.chain_id, 11155111);
    }

    #[test]
    fn test_builder() {
        let config = ClientConfig::builder()
            .api_key("key1")
            .api_key("key2")
            .rate_limit(10)
            .timeout(60)
            .build()
            .unwrap();

        assert_eq!(config.api_keys.len(), 2);
        assert_eq!(config.rate_limit_per_second, 10);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_builder_testnet() {
        let config = ClientConfig::builder()
            .api_key("test-key")
            .testnet()
            .build()
            .unwrap();

        assert_eq!(config.chain_id, 11155111);
    }

    #[test]
    fn test_validation_fails_without_api_key() {
        let result = ClientConfig::builder().build();
        assert!(result.is_err());
    }
}
