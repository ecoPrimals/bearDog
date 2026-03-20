// SPDX-License-Identifier: AGPL-3.0-only

//! Service Discovery Capability Trait
//!
//! Vendor-agnostic service discovery abstraction that eliminates hardcoded
//! infrastructure dependencies (Kubernetes, Consul, etcd, etc.).
//!
//! # Philosophy: Infant Discovery
//!
//! BearDog starts knowing only itself and discovers infrastructure capabilities
//! at runtime. This trait enables zero-knowledge bootstrap - the system detects
//! what's available (K8s, Consul, DNS) and uses it, with graceful fallbacks.
//!
//! # Architecture
//!
//! ```text
//! ServiceDiscoveryCapability (trait)
//! ├── KubernetesDiscovery    (if K8s available)
//! ├── ConsulDiscovery        (if Consul available)
//! ├── EtcdDiscovery          (if etcd available)
//! └── DnsHttpDiscovery       (fallback - always available)
//! ```
//!
//! # Examples
//!
//! ## Auto-Detection
//!
//! ```rust,ignore
//! use beardog_types::canonical::discovery::service_discovery_capability::*;
//! use beardog_types::canonical::capabilities::ServiceCapabilityType;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), DiscoveryError> {
//! // Automatically detects and uses best available discovery service
//! let discovery = create_service_discovery().await?;
//!
//! // Now use it - no need to know if it's K8s, Consul, or DNS!
//! let services = discovery.discover_by_capability(
//!     ServiceCapabilityType::ServiceMesh
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Explicit Provider
//!
//! ```rust,ignore
//! use beardog_types::canonical::discovery::service_discovery_capability::*;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), DiscoveryError> {
//! // Try K8s first, fall back to DNS
//! let discovery = if let Ok(k8s) = KubernetesDiscovery::try_create().await {
//!     Arc::new(k8s) as Arc<dyn ServiceDiscoveryCapability>
//! } else {
//!     Arc::new(DnsHttpDiscovery::new()) as Arc<dyn ServiceDiscoveryCapability>
//! };
//! # Ok(())
//! # }
//! ```

pub mod core;
pub mod kubernetes;
pub mod providers;

mod factory;

pub use core::{
    DiscoveryCapabilities, DiscoveryError, DiscoveryHealthStatus, ServiceDescriptor,
    ServiceDiscoveryCapability, ServiceHealth, ServiceProtocol,
};
pub use factory::create_service_discovery;
pub use kubernetes::KubernetesDiscovery;
pub use providers::{ConsulDiscovery, DnsHttpDiscovery, EtcdDiscovery};

#[cfg(test)]
mod tests;
