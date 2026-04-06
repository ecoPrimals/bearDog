// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Client Configuration Module
//!
//! This module contains client-side network configuration structs and implementations.

use crate::canonical::traits::RetryStrategy;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Client configuration - consolidates client-side network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfiguration {
    /// Connection timeout seconds
    pub connection_timeout_seconds: u64,
    /// Request timeout seconds  
    pub request_timeout_seconds: u64,
    /// Maximum redirects to follow
    pub max_redirects: u32,
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// User agent string (`Arc<str>` for fast cloning across requests)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub user_agent: Arc<str>,
    /// Default headers
    pub default_headers: HashMap<String, String>,
    /// Retry configuration
    pub retry: RetryConfiguration,
}

/// Retry configuration for network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfiguration {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Enable exponential backoff
    pub enable_exponential_backoff: bool,
    /// Retryable status codes
    pub retryable_status_codes: Vec<u16>,
}

impl Default for ClientConfiguration {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default queue size and timeout millis fit u64 for client retry config"
    )]
    fn default() -> Self {
        Self {
            connection_timeout_seconds: 30,
            request_timeout_seconds: 60,
            max_redirects: 5,
            enable_connection_pooling: true,
            user_agent: Arc::from(
                format!(
                    "BearDog/{}",
                    crate::constants::domains::system::versions::BEARDOG_VERSION
                )
                .as_str(),
            ),
            default_headers: HashMap::new(),
            retry: RetryConfiguration {
                max_attempts: crate::constants::domains::system::defaults::DEFAULT_MAX_RETRIES,
                base_delay_ms: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE
                    as u64,
                max_delay_ms: crate::constants::domains::network::timeouts::REQUEST_TIMEOUT
                    .as_millis() as u64,
                backoff_multiplier:
                    crate::constants::domains::system::defaults::DEFAULT_BACKOFF_MULTIPLIER,
                enable_exponential_backoff: true,
                retryable_status_codes: vec![408, 429, 500, 502, 503, 504],
            },
        }
    }
}

impl ClientConfiguration {
    /// Validate client configuration
    ///
    /// # Errors
    ///
    /// Returns an error if timeouts, user agent, or nested retry settings are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.connection_timeout_seconds == 0 {
            return Err(BearDogError::configuration(
                "Client connection timeout cannot be zero",
            ));
        }

        if self.request_timeout_seconds == 0 {
            return Err(BearDogError::configuration(
                "Client request timeout cannot be zero",
            ));
        }

        if self.user_agent.is_empty() {
            return Err(BearDogError::configuration("User agent cannot be empty"));
        }

        self.retry.validate()?;

        Ok(())
    }
}

impl RetryConfiguration {
    /// Validate retry configuration
    ///
    /// # Errors
    ///
    /// Returns an error if retry attempts, delays, or backoff multiplier are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_attempts == 0 {
            return Err(BearDogError::configuration(
                "Max retry attempts cannot be zero",
            ));
        }

        if self.base_delay_ms == 0 {
            return Err(BearDogError::configuration("Base delay cannot be zero"));
        }

        if self.backoff_multiplier <= 0.0 {
            return Err(BearDogError::configuration(
                "Backoff multiplier must be positive",
            ));
        }

        Ok(())
    }
}

// Implement RetryStrategy trait for network client retry configuration
impl RetryStrategy for RetryConfiguration {
    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if self.enable_exponential_backoff {
            // Exponential backoff with multiplier
            // Note: Precision loss is acceptable for delay calculations (not cryptographic)
            #[expect(
                clippy::cast_possible_truncation,
                reason = "retry delay computed as u64 millis for Duration::from_millis"
            )]
            #[expect(
                clippy::cast_precision_loss,
                reason = "acceptable imprecision for non-cryptographic backoff math"
            )]
            #[expect(
                clippy::cast_sign_loss,
                reason = "delay clamped non-negative before f64 to u64"
            )]
            #[expect(
                clippy::cast_possible_wrap,
                reason = "retry attempt capped at 30 for powi exponent"
            )]
            let delay_ms = {
                let computed = (self.base_delay_ms as f64
                    * self.backoff_multiplier.powi(attempt.min(30) as i32))
                .min(self.max_delay_ms as f64)
                .max(0.0) as u64;
                computed.min(self.max_delay_ms)
            };
            Duration::from_millis(delay_ms)
        } else {
            // Linear backoff
            Duration::from_millis(self.base_delay_ms.min(self.max_delay_ms))
        }
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Network-specific: Could check for HTTP status codes in error message
        // For now, allow retries on most errors
        let error_str = error.to_string().to_lowercase();

        // Don't retry on authentication/authorization errors
        if error_str.contains("401") || error_str.contains("403") {
            return false;
        }

        // Don't retry on client errors (4xx except specific ones)
        if error_str.contains("400") || error_str.contains("404") {
            return false;
        }

        // Retry on network errors, timeouts, and 5xx status codes
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        attempts >= self.max_attempts
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_client_config() {
        let config = ClientConfiguration::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.max_redirects, 5);
        assert!(config.enable_connection_pooling);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(!config.user_agent.is_empty());
    }

    #[test]
    fn test_retry_validation() {
        let config = RetryConfiguration {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            max_attempts: 0,
            base_delay_ms: 100,
            max_delay_ms: 1000,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![500, 502, 503],
        };
        assert!(config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_client_validation() {
        let mut config = ClientConfiguration::default();
        config.user_agent = Arc::from(String::new().as_str());
        assert!(config.validate().is_err());
    }

    #[test]
    fn client_validation_connection_and_request_timeout_zero() {
        let mut c = ClientConfiguration::default();
        c.connection_timeout_seconds = 0;
        assert!(c.validate().is_err());
        c = ClientConfiguration::default();
        c.request_timeout_seconds = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn retry_validation_base_delay_and_backoff() {
        let mut r = RetryConfiguration {
            max_attempts: 1,
            base_delay_ms: 10,
            max_delay_ms: 100,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![],
        };
        assert!(r.validate().is_ok());

        r.base_delay_ms = 0;
        assert!(r.validate().is_err());

        r = RetryConfiguration {
            max_attempts: 1,
            base_delay_ms: 10,
            max_delay_ms: 100,
            backoff_multiplier: 0.0,
            enable_exponential_backoff: false,
            retryable_status_codes: vec![],
        };
        assert!(r.validate().is_err());
    }

    #[test]
    fn retry_strategy_exponential_and_linear_delays() {
        let r = RetryConfiguration {
            max_attempts: 5,
            base_delay_ms: 10,
            max_delay_ms: 1000,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![],
        };
        assert_eq!(r.delay_for_attempt(0), std::time::Duration::from_millis(10));
        assert_eq!(r.delay_for_attempt(1), std::time::Duration::from_millis(20));
        assert_eq!(
            r.total_delay(3),
            r.delay_for_attempt(0) + r.delay_for_attempt(1) + r.delay_for_attempt(2)
        );

        let linear = RetryConfiguration {
            max_attempts: 3,
            base_delay_ms: 50,
            max_delay_ms: 50,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: false,
            retryable_status_codes: vec![],
        };
        assert_eq!(
            linear.delay_for_attempt(0),
            std::time::Duration::from_millis(50)
        );
    }

    #[test]
    fn retry_strategy_should_retry_error_filters_4xx() {
        let r = RetryConfiguration {
            max_attempts: 3,
            base_delay_ms: 50,
            max_delay_ms: 500,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![500],
        };
        assert!(!r.should_retry_error(&std::io::Error::other("HTTP 401 unauthorized")));
        assert!(!r.should_retry_error(&std::io::Error::other("status 403")));
        assert!(!r.should_retry_error(&std::io::Error::other("400 bad")));
        assert!(r.should_retry_error(&std::io::Error::other("500 server")));
    }

    #[test]
    fn retry_strategy_limit_and_backoff_multiplier() {
        let r = RetryConfiguration {
            max_attempts: 2,
            base_delay_ms: 1,
            max_delay_ms: 10,
            backoff_multiplier: 1.5,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![],
        };
        assert!(r.is_limit_reached(2));
        assert!(!r.is_limit_reached(1));
        assert_eq!(r.backoff_multiplier(), 1.5);
    }

    #[test]
    fn client_config_json_roundtrip() {
        let c = ClientConfiguration::default();
        let v = serde_json::to_value(&c).expect("serialize ClientConfiguration");
        let back: ClientConfiguration = serde_json::from_value(v).expect("deserialize");
        assert_eq!(c.max_redirects, back.max_redirects);
        assert_eq!(
            c.connection_timeout_seconds,
            back.connection_timeout_seconds
        );
    }
}
