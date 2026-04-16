// SPDX-License-Identifier: AGPL-3.0-or-later

// Discovery Types Module
//
// This module provides universal discovery types that eliminate all hardcoded
// vendor and primal dependencies through capability-based interfaces.
//
// # Infant Discovery Architecture
//
// This module implements the "infant discovery" pattern where BearDog starts
// with zero knowledge and discovers capabilities at runtime.

/// Universal discovery types without hardcoded vendor/primal names
pub mod universal;

/// Service discovery capability trait (vendor-agnostic)
/// Abstracts Kubernetes, Consul, etcd, DNS, etc.
pub mod service_discovery_capability;

/// Key management capability trait (vendor-agnostic)
/// Abstracts AWS KMS, Azure Key Vault, GCP KMS, PKCS#11, etc.
pub mod key_management_capability;

/// Secure software HSM implementation with proper cryptography
pub mod software_hsm_impl;

// Re-export key types for easier access
pub use universal::{
    AuthenticationMethod, ComputeAbility, DiscoveryMetadata, NetworkFunction, OrchestrationFeature,
    PerformanceProfile, PerformanceRequirements, SecurityLevel, SecurityRequirements,
    SecurityService, ServiceEndpoint, StorageCharacteristic, UniversalCapabilityType,
    UniversalDiscoveryRequest, UniversalDiscoveryResponse, UniversalServiceDescriptor,
};

// Re-export capability traits
pub use service_discovery_capability::{
    DiscoveryCapabilities, DiscoveryError, DiscoveryHealthStatus, ServiceDescriptor,
    ServiceDiscoveryBackend, ServiceDiscoveryCapability, ServiceHealth, ServiceProtocol,
    create_service_discovery,
};

pub use key_management_capability::{
    KeyAlgorithm, KeyManagementBackend, KeyManagementCapability, KeyMetadata, KeySpec, KeyState,
    KeyUsage, KmsCapabilities, KmsError, KmsHealthStatus, create_key_management,
};
