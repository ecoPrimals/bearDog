// SPDX-License-Identifier: AGPL-3.0-only

//! Inference serving, caching, load balancing, and cache eviction types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    /// Round robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round robin
    WeightedRoundRobin,
    /// Random
    Random,
}

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least recently used
    Lru,
    /// Least frequently used
    Lfu,
    /// First in, first out
    Fifo,
    /// Time-based expiration
    Ttl,
}

/// Serving configuration for production model deployment
///
/// Configures how models handle inference requests in production,
/// including concurrency limits, timeouts, and load distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingConfig {
    /// Maximum number of inference requests processed simultaneously
    pub max_concurrent_requests: u32,
    /// Maximum time allowed for processing a single inference request
    pub request_timeout: Duration,
    /// Strategy for distributing requests across multiple model instances
    pub load_balancing: LoadBalancingStrategy,
}

impl Default for ServingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            request_timeout: Duration::from_secs(30),
            load_balancing: LoadBalancingStrategy::RoundRobin,
        }
    }
}

impl ServingConfig {
    /// Load serving limits from `BEARDOG_AI_*` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            max_concurrent_requests: std::env::var("BEARDOG_AI_SERVING_MAX_CONCURRENT_REQUESTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            request_timeout: Duration::from_secs(
                std::env::var("BEARDOG_AI_ROUTING_REQUEST_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            load_balancing: LoadBalancingStrategy::RoundRobin,
        }
    }
}

/// Caching configuration for inference result storage
///
/// Configures caching of model predictions to improve response time
/// and reduce computational load for repeated identical requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Maximum number of cached prediction results to store in memory
    pub max_cache_size: u64,
    /// Time-to-live duration after which cached results expire and are removed
    pub ttl: Duration,
    /// Policy for removing entries when cache reaches capacity (LRU, LFU, FIFO, TTL)
    pub eviction_policy: EvictionPolicy,
}
