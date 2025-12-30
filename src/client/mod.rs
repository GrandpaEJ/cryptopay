//! Etherscan API client module

use crate::config::ClientConfig;
use crate::error::{Error, Result};
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use moka::future::Cache;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub mod endpoints;
pub mod types;

pub use endpoints::*;
pub use types::*;

/// Etherscan API client with rate limiting and caching
#[derive(Clone)]
pub struct BscScanClient {
    config: Arc<ClientConfig>,
    http_client: Client,
    rate_limiter: Arc<DefaultDirectRateLimiter>,
    cache: Cache<String, Value>,
    api_key_index: Arc<AtomicUsize>,
}

impl BscScanClient {
    /// Create a new Etherscan client with the given API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::new(api_key);
        Self::with_config(config)
    }

    /// Create a new Etherscan client for Sepolia testnet
    pub fn testnet(api_key: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::testnet(api_key);
        Self::with_config(config)
    }

    /// Create a new Etherscan client with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let http_client = Client::builder()
            .timeout(config.timeout())
            .build()
            .map_err(|e| Error::InvalidConfig(format!("Failed to create HTTP client: {}", e)))?;

        // Create rate limiter
        let rate_limit = NonZeroU32::new(config.rate_limit_per_second)
            .ok_or_else(|| Error::InvalidConfig("Rate limit must be greater than 0".to_string()))?;
        let quota = Quota::per_second(rate_limit);
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        // Create cache
        let cache = Cache::builder()
            .max_capacity(config.cache_max_size)
            .time_to_live(config.cache_ttl())
            .build();

        Ok(Self {
            config: Arc::new(config),
            http_client,
            rate_limiter,
            cache,
            api_key_index: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Get the next API key (round-robin rotation)
    fn get_api_key(&self) -> &str {
        let index = self.api_key_index.fetch_add(1, Ordering::Relaxed);
        &self.config.api_keys[index % self.config.api_keys.len()]
    }

    /// Make a cached API request
    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        module: &str,
        action: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        // Create cache key
        let cache_key = format!(
            "{}:{}:{}",
            module,
            action,
            params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&")
        );

        // Check cache if TTL > 0
        if self.config.cache_ttl_seconds > 0 {
            if let Some(cached) = self.cache.get(&cache_key).await {
                return serde_json::from_value(cached)
                    .map_err(|e| Error::Serialization(e));
            }
        }

        // Wait for rate limiter
        self.rate_limiter.until_ready().await;

        // Build request
        let api_key = self.get_api_key();
        let mut url = reqwest::Url::parse(&self.config.base_url)
            .map_err(|e| Error::InvalidConfig(format!("Invalid base URL: {}", e)))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("module", module);
            query_pairs.append_pair("action", action);
            query_pairs.append_pair("apikey", api_key);

            for (key, value) in params {
                query_pairs.append_pair(key, value);
            }
        }

        // Make request
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::HttpRequest(e))?;

        let status = response.status();
        let body: Value = response.json().await.map_err(|e| Error::HttpRequest(e))?;

        // Check for API errors
        if !status.is_success() {
            return Err(Error::api_error(format!(
                "HTTP {}: {}",
                status,
                body.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
            )));
        }

        // Parse Etherscan response format
        let api_status = body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("0");

        let message = body
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        // Status "1" = success, "0" = error
        if api_status == "0" && message != "No transactions found" && message != "NOTOK" {
            return Err(Error::api_error(message));
        }

        // Extract result
        let result = body
            .get("result")
            .ok_or_else(|| Error::api_error("Missing 'result' field in response"))?
            .clone();

        // Cache the result
        if self.config.cache_ttl_seconds > 0 {
            self.cache.insert(cache_key, result.clone()).await;
        }

        serde_json::from_value(result).map_err(|e| Error::Serialization(e))
    }

    /// Make a simple request (for endpoints that return single values)
    pub(crate) async fn request_simple<T: DeserializeOwned>(
        &self,
        module: &str,
        action: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        self.request(module, action, params).await
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        self.cache.invalidate_all();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (u64, u64) {
        (self.cache.entry_count(), self.cache.weighted_size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BscScanClient::new("test-key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_testnet_client() {
        let client = BscScanClient::testnet("test-key");
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(client.config.base_url.contains("testnet"));
    }

    #[test]
    fn test_api_key_rotation() {
        let config = ClientConfig::builder()
            .api_key("key1")
            .api_key("key2")
            .api_key("key3")
            .build()
            .unwrap();

        let client = BscScanClient::with_config(config).unwrap();

        // Test rotation
        assert_eq!(client.get_api_key(), "key1");
        assert_eq!(client.get_api_key(), "key2");
        assert_eq!(client.get_api_key(), "key3");
        assert_eq!(client.get_api_key(), "key1"); // Should wrap around
    }
}
