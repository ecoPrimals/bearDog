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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serving_config_default_clone_debug() {
        let c = ServingConfig::default();
        assert_eq!(c.max_concurrent_requests, 100);
        assert_eq!(c.request_timeout, Duration::from_secs(30));
        assert_eq!(c.load_balancing, LoadBalancingStrategy::RoundRobin);
        let _ = format!("{c:?}");
        let c2 = c.clone();
        assert_eq!(c.max_concurrent_requests, c2.max_concurrent_requests);
        assert_eq!(c.request_timeout, c2.request_timeout);
        assert_eq!(c.load_balancing, c2.load_balancing);
    }

    #[test]
    fn serving_config_from_env_without_mutation() {
        let c = ServingConfig::from_env();
        assert!(c.max_concurrent_requests > 0);
        assert_eq!(c.load_balancing, LoadBalancingStrategy::RoundRobin);
    }

    #[test]
    fn load_balancing_and_eviction_serde_roundtrip() {
        for strat in [
            LoadBalancingStrategy::RoundRobin,
            LoadBalancingStrategy::LeastConnections,
            LoadBalancingStrategy::WeightedRoundRobin,
            LoadBalancingStrategy::Random,
        ] {
            let j = serde_json::to_string(&strat).expect("serialize strategy");
            let back: LoadBalancingStrategy = serde_json::from_str(&j).expect("deserialize");
            assert_eq!(strat, back);
        }
        for pol in [
            EvictionPolicy::Lru,
            EvictionPolicy::Lfu,
            EvictionPolicy::Fifo,
            EvictionPolicy::Ttl,
        ] {
            let j = serde_json::to_string(&pol).expect("serialize policy");
            let back: EvictionPolicy = serde_json::from_str(&j).expect("deserialize");
            assert_eq!(pol, back);
        }
    }

    #[test]
    fn serving_config_serde_roundtrip() {
        let c = ServingConfig {
            max_concurrent_requests: 42,
            request_timeout: Duration::from_secs(7),
            load_balancing: LoadBalancingStrategy::LeastConnections,
        };
        let j = serde_json::to_string(&c).expect("serialize ServingConfig");
        let back: ServingConfig = serde_json::from_str(&j).expect("deserialize ServingConfig");
        assert_eq!(c.max_concurrent_requests, back.max_concurrent_requests);
        assert_eq!(c.request_timeout, back.request_timeout);
        assert_eq!(c.load_balancing, back.load_balancing);
    }

    #[test]
    fn caching_config_serde_roundtrip() {
        let c = CachingConfig {
            max_cache_size: 1000,
            ttl: Duration::from_secs(120),
            eviction_policy: EvictionPolicy::Lfu,
        };
        let j = serde_json::to_string(&c).expect("serialize CachingConfig");
        let back: CachingConfig = serde_json::from_str(&j).expect("deserialize CachingConfig");
        assert_eq!(c.max_cache_size, back.max_cache_size);
        assert_eq!(c.ttl, back.ttl);
        assert_eq!(c.eviction_policy, back.eviction_policy);
    }
}
