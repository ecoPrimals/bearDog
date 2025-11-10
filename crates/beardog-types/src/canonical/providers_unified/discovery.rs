//! Discovery Configuration (DEPRECATED)
//!
//! This module is deprecated. Use the canonical discovery configuration instead:
//! - For general discovery: `crate::canonical::config::domains::discovery::DiscoveryConfig`
//! - For service registry: `crate::canonical::providers_unified::service_discovery::ServiceDiscoveryConfig`
//!
//! This file is kept temporarily for backwards compatibility and will be removed in a future version.

// Re-export canonical DiscoveryConfig
pub use crate::canonical::config::domains::discovery::DiscoveryConfig;

// Re-export service registry specific types
pub use super::service_discovery::{ServiceDiscoveryConfig, DiscoveryType};
