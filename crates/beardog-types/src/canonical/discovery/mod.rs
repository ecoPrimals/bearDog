// Discovery Types Module
//
// This module provides universal discovery types that eliminate all hardcoded
// vendor and primal dependencies through capability-based interfaces.

/// Universal discovery types without hardcoded vendor/primal names
pub mod universal;

// Re-export key types for easier access
pub use universal::{
    AuthenticationMethod, ComputeAbility, DiscoveryMetadata, NetworkFunction, OrchestrationFeature,
    PerformanceProfile, PerformanceRequirements, SecurityLevel, SecurityRequirements,
    SecurityService, ServiceEndpoint, StorageCharacteristic, UniversalCapabilityType,
    UniversalDiscoveryRequest, UniversalDiscoveryResponse, UniversalServiceDescriptor,
};
