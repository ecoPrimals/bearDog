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
    AdvancedCapabilityMatcher, CapabilityManager, CapabilityManagerConfig, CapabilityMonitor,
    DependencyResolver, EmergentCapability, EmergentCapabilityEngine, GeneticCapabilityProfile,
};
pub use discovery::{
    EcosystemDiscovery, EcosystemDiscoveryConfig, EcosystemService, EcosystemServiceHealth,
    EcosystemServiceType, ServiceSearchCriteria,
};
pub use manager::{EcosystemManagerStatus, UniversalEcosystemManager};
pub use registry::{CapabilityMatch, CapabilityRegistry, CapabilitySearchCriteria, RegistryStats};
pub use traits::{
    Capability, CapabilityAttribute, CapabilityCategory, Dependency, EcosystemRegistration,
    HealthImpact, HealthStatus, MonitoringConfig, NetworkConfig, PrimalProvider, ProviderConfig,
    QualityOfService, ResourceRequirements, ServiceEndpoints, ServiceRequest, ServiceResponse,
};
// Re-export songbird_handoff types for universal usage
pub use songbird_handoff::{
    AdvertisedService, LoadBalancerConfig,
    ServiceEndpoint as SongBirdServiceEndpoint, SongBirdHandoffConfig,
    UniversalSongBirdHandoffManager,
};
// BearDog provider implementation
pub use beardog_provider::BearDogPrimalProvider;

/// Universal adapter version
pub const UNIVERSAL_ADAPTER_VERSION: &str = "1.0.0";

/// Standard ecosystem component identifiers
pub mod ecosystem_ids {
    /// ToadStool compute orchestrator identifier
    pub const TOADSTOOL: &str = "toadstool";
    /// SongBird discovery service identifier
    pub const SONGBIRD: &str = "songbird";
    /// NestGate data management identifier
    pub const NESTGATE: &str = "nestgate";
    /// Squirrel storage service identifier
    pub const SQUIRREL: &str = "squirrel";
    /// BiomeOS operating system identifier
    pub const BIOMEOS: &str = "biomeos";
    /// BearDog security provider identifier
    pub const BEARDOG: &str = "beardog";
}

/// Standard capability identifiers
pub mod capability_ids {
    /// Storage provision capability identifier
    pub const STORAGE_PROVISION: &str = "storage.provision";
    /// Storage backup capability identifier
    pub const STORAGE_BACKUP: &str = "storage.backup";
    /// Storage persist capability identifier
    pub const STORAGE_PERSIST: &str = "storage.persist";
    /// Compute execute capability identifier
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    /// Compute orchestrate capability identifier
    pub const COMPUTE_ORCHESTRATE: &str = "compute.orchestrate";
    /// Security encrypt capability identifier
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    /// Security authorize capability identifier
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";
    /// Security authenticate capability identifier
    pub const SECURITY_AUTHENTICATE: &str = "security.authenticate";
    /// Security audit capability identifier
    pub const SECURITY_AUDIT: &str = "security.audit";
    /// Security monitor capability identifier
    pub const SECURITY_MONITOR: &str = "security.monitor";
    /// AI inference capability identifier
    pub const AI_INFERENCE: &str = "ai.inference";
    /// AI training capability identifier
    pub const AI_TRAINING: &str = "ai.training";
    /// Communication discovery capability identifier
    pub const COMMUNICATION_DISCOVERY: &str = "communication.discovery";
    /// Communication routing capability identifier
    pub const COMMUNICATION_ROUTING: &str = "communication.routing";
    /// Communication discovery capability identifier (alternative name)
    pub const COMM_DISCOVERY: &str = "communication.discovery";
    /// Monitoring metrics capability identifier
    pub const MONITORING_METRICS: &str = "monitoring.metrics";
    /// Monitoring logging capability identifier
    pub const MONITORING_LOGGING: &str = "monitoring.logging";
    /// Monitoring alerting capability identifier
    pub const MONITORING_ALERTING: &str = "monitoring.alerting";
    /// Integration adapt capability identifier
    pub const INTEGRATION_ADAPT: &str = "integration.adapt";
    /// Integration transform capability identifier
    pub const INTEGRATION_TRANSFORM: &str = "integration.transform";
}

/// Standard service request types
pub mod request_types {
    /// Storage store request type
    pub const STORAGE_STORE: &str = "storage.store";
    /// Storage retrieve request type
    pub const STORAGE_RETRIEVE: &str = "storage.retrieve";
    /// Storage delete request type
    pub const STORAGE_DELETE: &str = "storage.delete";
    /// Compute execute request type
    pub const COMPUTE_EXECUTE: &str = "compute.execute";
    /// Compute schedule request type
    pub const COMPUTE_SCHEDULE: &str = "compute.schedule";
    /// Security encrypt request type
    pub const SECURITY_ENCRYPT: &str = "security.encrypt";
    /// Security decrypt request type
    pub const SECURITY_DECRYPT: &str = "security.decrypt";
    /// Security authorize request type
    pub const SECURITY_AUTHORIZE: &str = "security.authorize";
    /// Security authenticate request type
    pub const SECURITY_AUTHENTICATE: &str = "security.authenticate";
    /// AI predict request type
    pub const AI_PREDICT: &str = "ai.predict";
    /// AI train request type
    pub const AI_TRAIN: &str = "ai.train";
    /// Communication send request type
    pub const COMMUNICATION_SEND: &str = "communication.send";
    /// Communication receive request type
    pub const COMMUNICATION_RECEIVE: &str = "communication.receive";
    /// Monitoring alert request type
    pub const MONITORING_ALERT: &str = "monitoring.alert";
    /// Health check request type
    pub const HEALTH_CHECK: &str = "health.check";
    /// Status get request type
    pub const STATUS_GET: &str = "status.get";
}

/// Universal capability examples for testing and development
pub mod capability_examples {
    use super::*;
    use std::collections::HashMap;

    /// Create a sample storage capability
    pub fn storage_capability() -> Capability {
        let mut attributes = HashMap::new();
        attributes.insert(
            "type".to_string(),
            CapabilityAttribute {
                value: "block".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("Storage type".to_string()),
            },
        );
        attributes.insert(
            "capacity_gb".to_string(),
            CapabilityAttribute {
                value: "1000".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::Integer,
                required: true,
                description: Some("Storage capacity in GB".to_string()),
            },
        );

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
        attributes.insert(
            "architecture".to_string(),
            CapabilityAttribute {
                value: "x86_64".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("CPU architecture".to_string()),
            },
        );
        attributes.insert(
            "cores".to_string(),
            CapabilityAttribute {
                value: "8".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::Integer,
                required: true,
                description: Some("Number of CPU cores".to_string()),
            },
        );

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
        attributes.insert(
            "algorithms".to_string(),
            CapabilityAttribute {
                value: "AES256,RSA2048".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("Supported encryption algorithms".to_string()),
            },
        );
        attributes.insert(
            "key_management".to_string(),
            CapabilityAttribute {
                value: "HSM".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("Key management system".to_string()),
            },
        );

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
        attributes.insert(
            "model_type".to_string(),
            CapabilityAttribute {
                value: "transformer".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("AI model type".to_string()),
            },
        );
        attributes.insert(
            "inference_speed".to_string(),
            CapabilityAttribute {
                value: "fast".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: false,
                description: Some("Inference speed category".to_string()),
            },
        );

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
        attributes.insert(
            "protocol".to_string(),
            CapabilityAttribute {
                value: "https".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::String,
                required: true,
                description: Some("Communication protocol".to_string()),
            },
        );
        attributes.insert(
            "discovery".to_string(),
            CapabilityAttribute {
                value: "true".to_string(),
                data_type: crate::adapters::universal::traits::AttributeDataType::Boolean,
                required: true,
                description: Some("Service discovery enabled".to_string()),
            },
        );

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
