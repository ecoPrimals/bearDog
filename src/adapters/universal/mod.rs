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

// SongBird discovery and orchestration handoff (universal patterns)
pub mod songbird_handoff;

// BearDog's implementation of universal patterns
pub mod beardog_provider;

// Re-export core universal types (specific imports to avoid conflicts)
pub use capability_manager::{
    CapabilityManager, GeneticCapabilityProfile, EmergentCapability, CapabilityMonitor,
    CapabilityManagerConfig, AdvancedCapabilityMatcher, DependencyResolver, EmergentCapabilityEngine,
};
pub use discovery::{
    EcosystemDiscovery, EcosystemService, EcosystemServiceType, EcosystemServiceHealth,
    EcosystemDiscoveryConfig, ServiceSearchCriteria,
};
pub use manager::{
    UniversalEcosystemManager, EcosystemManagerStatus,
};
pub use registry::{
    CapabilityRegistry, RegistryStats, CapabilitySearchCriteria, CapabilityMatch,
};
pub use traits::{
    PrimalProvider, Capability, CapabilityCategory, CapabilityAttribute, QualityOfService, ResourceRequirements,
    HealthStatus, HealthImpact, ServiceRequest, ServiceResponse, EcosystemRegistration, ProviderConfig,
    ServiceEndpoints, Dependency, MonitoringConfig, NetworkConfig,
};
// Re-export songbird_handoff types for universal usage
pub use songbird_handoff::{
    SongBirdHandoffConfig, AdvertisedService, ServiceEndpoint as SongBirdServiceEndpoint,
    LoadBalancerConfig, UniversalSongBirdHandoffManager, HandoffManagerFactory,
};
// BearDog provider implementation
pub use beardog_provider::BearDogPrimalProvider;

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

/// Standard capability identifiers
pub mod capability_ids {
    pub const STORAGE_PROVISION: &str = "storage.provision";
    pub const STORAGE_BACKUP: &str = "storage.backup";
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    pub const COMPUTE_ORCHESTRATE: &str = "compute.orchestrate";
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";
    pub const SECURITY_AUDIT: &str = "security.audit";
    pub const AI_INFERENCE: &str = "ai.inference";
    pub const AI_TRAINING: &str = "ai.training";
    pub const COMMUNICATION_DISCOVERY: &str = "communication.discovery";
    pub const COMMUNICATION_ROUTING: &str = "communication.routing";
    pub const MONITORING_METRICS: &str = "monitoring.metrics";
    pub const MONITORING_LOGGING: &str = "monitoring.logging";
    pub const MONITORING_ALERTING: &str = "monitoring.alerting";
    pub const INTEGRATION_ADAPT: &str = "integration.adapt";
    pub const INTEGRATION_TRANSFORM: &str = "integration.transform";
}

/// Standard service request types
pub mod request_types {
    pub const STORAGE_STORE: &str = "storage.store";
    pub const STORAGE_RETRIEVE: &str = "storage.retrieve";
    pub const STORAGE_DELETE: &str = "storage.delete";
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    pub const COMPUTE_SCHEDULE: &str = "compute.schedule";
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    pub const SECURITY_DECRYPT: &str = "security.decrypt";
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";
    pub const AI_PREDICT: &str = "ai.predict";
    pub const AI_TRAIN: &str = "ai.train";
    pub const COMMUNICATION_SEND: &str = "communication.send";
    pub const COMMUNICATION_RECEIVE: &str = "communication.receive";
    pub const MONITORING_COLLECT: &str = "monitoring.collect";
    pub const MONITORING_ALERT: &str = "monitoring.alert";
}

/// Universal capability examples for testing and development
pub mod capability_examples {
    use super::*;
    use std::collections::HashMap;

    /// Create a sample storage capability
    pub fn storage_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert("type".to_string(), CapabilityAttribute {
            value: "block".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("Storage type".to_string()),
        });
        attributes.insert("capacity_gb".to_string(), CapabilityAttribute {
            value: "1000".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::Integer,
            required: true,
            description: Some("Storage capacity in GB".to_string()),
        });

        Capability {
            id: capability_ids::STORAGE_PROVISION.to_string(),
            name: "Universal Storage Provision".to_string(),
            description: "Provides universal storage provisioning capability".to_string(),
            category: CapabilityCategory::Storage,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a sample compute capability
    pub fn compute_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert("architecture".to_string(), CapabilityAttribute {
            value: "x86_64".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("CPU architecture".to_string()),
        });
        attributes.insert("cores".to_string(), CapabilityAttribute {
            value: "8".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::Integer,
            required: true,
            description: Some("Number of CPU cores".to_string()),
        });

        Capability {
            id: capability_ids::COMPUTE_EXECUTE.to_string(),
            name: "Universal Compute Execute".to_string(),
            description: "Provides universal compute execution capability".to_string(),
            category: CapabilityCategory::Compute,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a sample security capability
    pub fn security_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert("algorithms".to_string(), CapabilityAttribute {
            value: "AES256,RSA2048".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("Supported encryption algorithms".to_string()),
        });
        attributes.insert("key_management".to_string(), CapabilityAttribute {
            value: "HSM".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("Key management system".to_string()),
        });

        Capability {
            id: capability_ids::SECURITY_ENCRYPT.to_string(),
            name: "Universal Security Encrypt".to_string(),
            description: "Provides universal encryption capability".to_string(),
            category: CapabilityCategory::Security,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a sample AI capability
    pub fn ai_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert("model_type".to_string(), CapabilityAttribute {
            value: "transformer".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("AI model type".to_string()),
        });
        attributes.insert("inference_speed".to_string(), CapabilityAttribute {
            value: "fast".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: false,
            description: Some("Inference speed category".to_string()),
        });

        Capability {
            id: capability_ids::AI_INFERENCE.to_string(),
            name: "Universal AI Inference".to_string(),
            description: "Provides universal AI inference capability".to_string(),
            category: CapabilityCategory::AI,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create a sample communication capability
    pub fn communication_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert("protocol".to_string(), CapabilityAttribute {
            value: "https".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::String,
            required: true,
            description: Some("Communication protocol".to_string()),
        });
        attributes.insert("discovery".to_string(), CapabilityAttribute {
            value: "true".to_string(),
            data_type: crate::adapters::universal::traits::AttributeDataType::Boolean,
            required: true,
            description: Some("Service discovery enabled".to_string()),
        });

        Capability {
            id: capability_ids::COMMUNICATION_DISCOVERY.to_string(),
            name: "Universal Communication Discovery".to_string(),
            description: "Provides universal service discovery capability".to_string(),
            category: CapabilityCategory::Communication,
            attributes,
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }
}
