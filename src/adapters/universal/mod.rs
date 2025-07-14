//! Universal Ecosystem Adapter Architecture
//!
//! **Universal, domain-agnostic ecosystem integration**
//!
//! This module provides universal adapter patterns that enable any ecosystem component
//! to integrate with the ecoPrimals ecosystem. It follows SongBird's established
//! universal patterns for truly agnostic, interoperable ecosystem components.
//!
//! ## Core Concepts
//!
//! - **Universal PrimalProvider**: Domain-agnostic interface for any ecosystem component
//! - **Capability Advertisement**: Rich capability system with QoS and resource metrics
//! - **Service Discovery**: Automatic ecosystem discovery and registration
//! - **Generic Request/Response**: Universal messaging patterns for inter-service communication
//! - **Health Monitoring**: Comprehensive health and performance monitoring
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                Universal Ecosystem Manager                   │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
//! │  │  ToadStool  │  │  SongBird   │  │  NestGate   │        │
//! │  │ PrimalProvider│ │ PrimalProvider│ │ PrimalProvider│     │
//! │  └─────────────┘  └─────────────┘  └─────────────┘        │
//! │                                                             │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
//! │  │   Squirrel  │  │   biomeOS   │  │   BearDog   │        │
//! │  │ PrimalProvider│ │ PrimalProvider│ │ PrimalProvider│     │
//! │  └─────────────┘  └─────────────┘  └─────────────┘        │
//! └─────────────────────────────────────────────────────────────┘
//! ```

// Core universal trait definitions
pub mod traits;

// Universal capability registry
pub mod registry;

// Universal service discovery
pub mod discovery;

// Universal manager
pub mod manager;

// Comprehensive capability management
pub mod capability_manager;

// SongBird discovery and orchestration handoff
pub mod songbird_handoff;

// BearDog's implementation of universal patterns
pub mod beardog_provider;

// Re-export core universal types (avoid ambiguous conflicts)
pub use capability_manager::*;
pub use discovery::*;
pub use manager::*;
pub use registry::*;
pub use traits::*;
// Note: Exclude songbird_handoff re-exports to avoid ambiguous conflicts
// songbird_handoff contains network discovery functionality that belongs to SongBird
// BearDog is a security provider, not a network discovery service
pub use beardog_provider::*;

/// Universal adapter version
pub const UNIVERSAL_ADAPTER_VERSION: &str = "1.0.0";

/// Standard ecosystem component identifiers
pub mod ecosystem_ids {
    pub const TOADSTOOL: &str = "toadstool";
    pub const SONGBIRD: &str = "songbird";
    pub const NESTGATE: &str = "nestgate";
    pub const SQUIRREL: &str = "squirrel";
    pub const BIOMEOS: &str = "biomeos";
    pub const BEARDOG: &str = "beardog";
}

/// Standard capability identifiers used across the ecosystem
pub mod capability_ids {
    // Compute capabilities
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    pub const COMPUTE_ORCHESTRATE: &str = "compute.orchestrate";
    pub const COMPUTE_SCHEDULE: &str = "compute.schedule";

    // Storage capabilities
    pub const STORAGE_PERSIST: &str = "storage.persist";
    pub const STORAGE_CACHE: &str = "storage.cache";
    pub const STORAGE_BACKUP: &str = "storage.backup";

    // Security capabilities
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    pub const SECURITY_AUTHENTICATE: &str = "security.authenticate";
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";
    pub const SECURITY_AUDIT: &str = "security.audit";
    pub const SECURITY_MONITOR: &str = "security.monitor";

    // AI capabilities
    pub const AI_INFERENCE: &str = "ai.inference";
    pub const AI_TRAINING: &str = "ai.training";
    pub const AI_ANALYSIS: &str = "ai.analysis";

    // Communication capabilities
    pub const COMM_DISCOVERY: &str = "communication.discovery";
    pub const COMM_ROUTING: &str = "communication.routing";
    pub const COMM_MESSAGING: &str = "communication.messaging";

    // Monitoring capabilities
    pub const MONITOR_METRICS: &str = "monitoring.metrics";
    pub const MONITOR_LOGGING: &str = "monitoring.logging";
    pub const MONITOR_ALERTING: &str = "monitoring.alerting";
}

/// Standard request types used across the ecosystem
pub mod request_types {
    // Generic request types
    pub const HEALTH_CHECK: &str = "health.check";
    pub const STATUS_GET: &str = "status.get";
    pub const CONFIG_UPDATE: &str = "config.update";

    // Compute request types
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    pub const COMPUTE_STATUS: &str = "compute.status";
    pub const COMPUTE_CANCEL: &str = "compute.cancel";

    // Storage request types
    pub const STORAGE_STORE: &str = "storage.store";
    pub const STORAGE_RETRIEVE: &str = "storage.retrieve";
    pub const STORAGE_DELETE: &str = "storage.delete";

    // Security request types
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    pub const SECURITY_DECRYPT: &str = "security.decrypt";
    pub const SECURITY_AUTHENTICATE: &str = "security.authenticate";
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";

    // AI request types
    pub const AI_INFERENCE: &str = "ai.inference";
    pub const AI_TRAINING: &str = "ai.training";
    pub const AI_ANALYSIS: &str = "ai.analysis";

    // Communication request types
    pub const COMM_DISCOVER: &str = "communication.discover";
    pub const COMM_ROUTE: &str = "communication.route";
    pub const COMM_MESSAGE: &str = "communication.message";
}

/// Helper functions for creating standard capabilities
pub mod capability_helpers {
    use super::*;
    use std::collections::HashMap;

    /// Create a security capability for BearDog
    pub fn create_security_capability(
        id: &str,
        name: &str,
        description: &str,
        attributes: HashMap<String, CapabilityAttribute>,
    ) -> Capability {
        Capability {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: CapabilityCategory::Security,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a compute capability for ToadStool
    pub fn create_compute_capability(
        id: &str,
        name: &str,
        description: &str,
        attributes: HashMap<String, CapabilityAttribute>,
    ) -> Capability {
        Capability {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: CapabilityCategory::Compute,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a storage capability for NestGate
    pub fn create_storage_capability(
        id: &str,
        name: &str,
        description: &str,
        attributes: HashMap<String, CapabilityAttribute>,
    ) -> Capability {
        Capability {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: CapabilityCategory::Storage,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create an AI capability for Squirrel
    pub fn create_ai_capability(
        id: &str,
        name: &str,
        description: &str,
        attributes: HashMap<String, CapabilityAttribute>,
    ) -> Capability {
        Capability {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: CapabilityCategory::AI,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a communication capability for SongBird
    pub fn create_communication_capability(
        id: &str,
        name: &str,
        description: &str,
        attributes: HashMap<String, CapabilityAttribute>,
    ) -> Capability {
        Capability {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: CapabilityCategory::Communication,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }
}
