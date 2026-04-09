// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical Capability Definitions for BearDog
//!
//! Provides structured capability types for security, performance, compliance,
//! and ecosystem integration. This module defines the capability-based discovery
//! system that eliminates hardcoded dependencies.
//!
//! ## Capability-Based Architecture
//!
//! Instead of hardcoding which services exist, BearDog discovers capabilities
//! dynamically through:
//!
//! 1. **Vendor Capabilities** - External services (AWS KMS, Azure KeyVault, etc.)
//! 2. **Primal Capabilities** - Other primals in the ecosystem
//! 3. **Cross-Cutting Capabilities** - Monitoring, logging, metrics
//! 4. **Specialized Capabilities** - Biometrics, quantum crypto, zero-knowledge
//!
//! ## Example
//!
//! ```rust
//! use beardog_types::canonical::capabilities::CapabilityType;
//!
//! // Define a capability type
//! let capability = CapabilityType::KeyManagement;
//! println!("Capability: {:?}", capability);
//! ```

#[cfg(test)]
#[path = "../capabilities_tests.rs"]
mod capabilities_tests;

mod capability_type;
mod compliance_discovery;
mod discovery;
mod infrastructure;

pub use capability_type::{CapabilityType, ServiceCapabilityType};
pub use compliance_discovery::{
    CapabilityDiscoveryRequest, CapabilityDiscoveryResponse, ComplianceCapabilities,
    ComplianceLevel, DiscoveryMetadata, EncryptionRequirements,
};
pub use discovery::{
    AuthConfig, AuthType, CircuitBreakerConfig, EndpointConfig, HealthStatus, PerformanceMetrics,
    ProviderInfo, SecurityCapabilities, SecurityLevel, UniversalCapability,
};
pub use infrastructure::{
    CapabilityRequirements, ComputeCapabilities, EnvironmentalCapabilities,
    HumanEntropyCapabilities, NetworkCapabilities, PerformanceCapabilities, StorageCapabilities,
    SystemCapabilities,
};
