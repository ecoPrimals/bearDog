

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IntegrationPoliciesConfig {
    pub circuit_breaker: IntegrationCircuitBreaker,
    pub rate_limiting: IntegrationRateLimiting,
    pub retry_policy: NotificationRetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCircuitBreaker {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
}

impl Default for IntegrationCircuitBreaker {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRateLimiting {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_capacity: u32,
}

impl Default for IntegrationRateLimiting {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: 100,
            burst_capacity: 150,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRetryPolicy {
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for NotificationRetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
        }
    }
}
