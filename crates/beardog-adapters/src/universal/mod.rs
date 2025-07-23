//! Universal adapters for cross-ecosystem integration
//!
//! This module provides name-agnostic adapters that allow BearDog
//! to integrate with any service mesh or orchestrator following
//! the Universal Primal Architecture Standard.

pub mod bridge_adapter;
pub mod capability_adapter;
pub mod http_adapter;
pub mod protocol_adapter;
pub mod security_provider_bridge;

// New modular architecture
pub mod commercial_extraction;
pub mod service_registration;

// Re-export key types
pub use capability_adapter::{BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector};

pub use bridge_adapter::BridgeAdapter;
pub use http_adapter::HttpAdapter;
pub use protocol_adapter::ProtocolAdapter;
pub use security_provider_bridge::{BridgeConfig, SecurityProviderBridge};

// Re-export from modular components
pub use commercial_extraction::*;
pub use service_registration::*;
