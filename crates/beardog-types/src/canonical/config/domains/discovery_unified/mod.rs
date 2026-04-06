// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified Discovery Configuration
//!
//! This module provides a single, comprehensive discovery configuration struct
//! that consolidates and replaces two competing "consolidated" discovery configs:
//! - `beardog_types::canonical::config::discovery::ConsolidatedDiscoveryConfig`
//! - `beardog_types::canonical::config::domains::discovery_config::ConsolidatedDiscoveryConfig`
//!
//! ## Design Principles:
//! - **Single Source of Truth**: Eliminates duplication between two "consolidated" configs
//! - **Comprehensive**: Covers all discovery methods (HTTP, DNS, mDNS, Consul, etcd, K8s, Quantum)
//! - **Protocol-Agnostic**: Pluggable discovery mechanisms
//! - **Flexible Configuration**: Environment overrides, builder pattern, validation
//! - **Backward Compatibility**: Type aliases for smooth migration
//!
//! ## Configuration Hierarchy:
//! Discovery config is resolved in the following priority order:
//! 1. Environment variables (when using `from_env()` or `builder().from_env()`)
//! 2. Configuration file values (when loaded via `BearDogConfig::from_file()`)
//! 3. Explicit values (when using `builder().field(value)`)
//! 4. Sensible defaults (always available via `Default`)
//!
//! ## Environment Variables:
//! - `BEARDOG_DISCOVERY_ENABLED`
//! - `BEARDOG_DISCOVERY_SERVICE_ID`
//! - `BEARDOG_DISCOVERY_PROTOCOLS` (comma-separated: http,dns,mdns,consul,etcd,k8s)
//! - `BEARDOG_REGISTRY_BACKEND` (etcd, consul, zookeeper, redis)
//! - `BEARDOG_REGISTRY_ENDPOINTS` (comma-separated URLs)
//! - `BEARDOG_REGISTRY_SERVICE_TTL_SECS`
//! - `BEARDOG_REGISTRY_HEALTH_CHECK_INTERVAL_SECS`
//! - `BEARDOG_REGISTRY_CLEANUP_INTERVAL_SECS`
//! - `BEARDOG_DISCOVERY_PORTS` (comma-separated port numbers)
//! - `BEARDOG_DISCOVERY_TIMEOUT_SECS`
//! - `BEARDOG_DISCOVERY_CACHE_ENABLED`
//! - `BEARDOG_DISCOVERY_CACHE_SIZE`
//! - `BEARDOG_DISCOVERY_CACHE_TTL_SECS`
//! - `BEARDOG_QUANTUM_DISCOVERY_ENABLED`
//! - `BEARDOG_QUANTUM_COHERENCE_TIME_MS`
//! - `BEARDOG_DISCOVERY_SECURITY_ENABLED`
//! - `BEARDOG_DISCOVERY_AUTH_REQUIRED`
//! - `BEARDOG_DISCOVERY_ENCRYPTION_REQUIRED`
//!
//! # Examples
//!
//! ```rust
//! use beardog_types::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfig;
//! use std::sync::Arc;
//!
//! // Load with defaults
//! let default_config = UnifiedDiscoveryConfig::default();
//! assert!(default_config.enabled);
//!
//! // Custom configuration
//! let mut custom_config = UnifiedDiscoveryConfig::default();
//! custom_config.enabled = true;
//! custom_config.service_id = Arc::from("my-service");
//! ```

pub mod types;

pub use types::*;

mod builder;
mod config_impls;

pub use builder::UnifiedDiscoveryConfigBuilder;

#[cfg(test)]
mod tests;
