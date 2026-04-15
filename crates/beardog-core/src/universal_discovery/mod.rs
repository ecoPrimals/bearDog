// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Discovery Service - Modular Architecture
//!
//! This module provides comprehensive service discovery capabilities
//! across multiple protocols and networks, organized into focused domain modules
//! for maximum maintainability and adherence to best practices.
//!
//! # Modular Architecture
//!
//! Universal discovery is split into focused domain modules:
//!
//! - **`protocols`**: Discovery protocol implementations (Consul, Eureka, etc.)
//! - **`registry`**: Service registration, tracking, and management
//! - **`health`**: Health checking, monitoring, and status management
//! - **`load_balancing`**: Load balancing algorithms and traffic distribution
//! - **`network`**: Network configuration, addressing, and communication
//!
//! # Features
//!
//! - Multi-protocol service discovery (Consul, Eureka, custom protocols)
//! - Health-based routing and failover
//! - Multiple load balancing strategies
//! - Dynamic service registration and deregistration
//! - Real-time health monitoring
//! - Zero-knowledge bootstrap support
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_core::universal_discovery::{DiscoveryProtocol, ServiceRegistry};
//!
//! // Initialize service registry for discovery
//! let registry = ServiceRegistry::new();
//! // Services are discovered and health-checked automatically
//! ```
//!
//! # Architecture
//!
//! The universal discovery system follows the primal sovereignty pattern:
//! - No hardcoded service locations
//! - Capability-based service discovery
//! - Health-based routing decisions
//! - Graceful degradation on failures

use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::providers_unified::traits::other_traits::ServiceInfo;
use chrono::{DateTime, Utc};

mod protocol_handlers;
mod service;
mod types;

// Re-export sub-modules
/// Health checking for discovered services
pub mod health;
/// Load balancing strategies
pub mod load_balancing;
/// Network discovery mechanisms
pub mod network;
/// Protocol support and negotiation
pub mod protocols;
#[cfg(test)]
#[path = "protocols_tests.rs"]
mod protocols_tests;
/// Service registry
pub mod registry;

#[cfg(test)]
mod tests;

// Re-export types for convenience
// HealthCheckConfig is a domain-specific config for universal discovery
pub use health::{HealthCheckConfig, HealthMonitor, ServiceHealthState};
pub use load_balancing::{LoadBalancer, LoadBalancingAlgorithm, LoadBalancingConfig};
#[cfg(feature = "mdns")]
pub use protocol_handlers::MdnsProtocolHandler;
pub use protocol_handlers::{MinimalProtocolHandler, ProtocolHandler};
pub use protocols::ModernServiceDiscovery;
pub use registry::{ExtendedServiceInfo, ServiceRegistry, ServiceRegistryConfig};
pub use service::UniversalServiceDiscovery;
pub use types::{
    DiscoveryEvent, DiscoveryProtocol, DiscoveryStatistics, ProtocolStatistics,
    UniversalDiscoveryConfig,
};
