// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability taxonomy: [`CapabilityType`] and stable wire identifiers.

use serde::{Deserialize, Serialize};

/// Capability Type - Types of capabilities in the `BearDog` ecosystem
///
/// Defines all discoverable capabilities that can be provided by:
/// - External vendors (AWS, Azure, Google Cloud)
/// - Other primals in the ecosystem
/// - Internal `BearDog` services
///
/// ## Categories
///
/// - **Vendor Capabilities**: External cloud services
/// - **Primal Capabilities**: Ecosystem services
/// - **Cross-Cutting**: Monitoring, logging, metrics
/// - **Specialized**: Advanced cryptographic and AI features
///
/// ## Usage
///
/// Capabilities are discovered dynamically rather than hardcoded:
///
/// ```rust
/// # use beardog_types::canonical::capabilities::CapabilityType;
/// // Query for capability providers
/// let cap = CapabilityType::HardwareSecurityModule;
/// println!("Looking for: {}", cap.name());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityType {
    // === VENDOR CAPABILITIES ===
    /// Key Management Service (discovered through capability-based detection)
    KeyManagement,
    /// Hardware Security Module (discovered through capability-based detection)
    HardwareSecurityModule,
    /// Secrets Management (discovered through capability-based detection)
    SecretsManagement,
    /// Authentication Service (OAuth, SAML, OIDC providers)
    Authentication,
    /// Cloud Storage (discovered through capability-based detection)
    CloudStorage,
    /// Database Service (discovered through capability-based detection)
    DatabaseService,
    /// Load Balancing (discovered through capability-based detection)
    LoadBalancing,
    /// Content Delivery Network (discovered through capability-based detection)
    ContentDeliveryNetwork,

    // === PRIMAL CAPABILITIES ===
    /// Service Mesh and Network Routing
    ServiceMesh,
    /// Compute and AI Intelligence
    ComputeIntelligence,
    /// Data Storage and Management
    DataStorage,
    /// Distributed AI and ML
    DistributedIntelligence,
    /// Container Orchestration
    ContainerOrchestration,
    /// Security and Cryptography
    Security,

    // === CROSS-CUTTING CAPABILITIES ===
    /// Monitoring and Observability
    Monitoring,
    /// Centralized Logging
    Logging,
    /// Metrics Collection and Analysis
    Metrics,
    /// Health Checking and Status
    HealthChecking,
    /// Configuration Management
    Configuration,
    /// Network Communication
    Network,
    /// Message Queuing
    MessageQueue,
    /// Event Streaming
    EventStreaming,
    /// Workflow Orchestration
    WorkflowOrchestration,

    // === COMPATIBILITY ALIASES ===
    /// Legacy alias for storage-related capability discovery (prefer [`Self::DataStorage`] in new code).
    Storage,
    /// Legacy alias for general compute capability (prefer [`Self::ComputeIntelligence`] where applicable).
    Compute,
    /// Legacy alias for network capability (maps to the same discovery class as [`Self::Network`]).
    Networking,
    /// Legacy alias for AI / ML capability surfaces.
    ArtificialIntelligence,
    /// Legacy alias for orchestration platforms (containers, workflows).
    Orchestration,

    // === SPECIALIZED CAPABILITIES ===
    /// Biometric Authentication
    BiometricAuth,
    /// Quantum-Resistant Cryptography
    QuantumCrypto,
    /// Zero-Knowledge Proofs
    ZeroKnowledgeProofs,
    /// Genetic Algorithm Processing
    GeneticAlgorithms,
    /// Threat Detection and Response
    ThreatDetection,
    /// Compliance and Audit
    ComplianceAudit,

    /// Custom capability with name
    Custom(String),
}

impl CapabilityType {
    /// Returns a short human-readable label for this capability (not the stable wire id).
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::KeyManagement => "Key Management".to_string(),
            Self::HardwareSecurityModule => "Hardware Security Module".to_string(),
            Self::SecretsManagement => "Secrets Management".to_string(),
            Self::Authentication => "Authentication".to_string(),
            Self::CloudStorage => "Cloud Storage".to_string(),
            Self::DatabaseService => "Database Service".to_string(),
            Self::LoadBalancing => "Load Balancing".to_string(),
            Self::ContentDeliveryNetwork => "Content Delivery Network".to_string(),
            Self::ServiceMesh => "Service Mesh".to_string(),
            Self::ComputeIntelligence => "Compute Intelligence".to_string(),
            Self::DataStorage => "Data Storage".to_string(),
            Self::DistributedIntelligence => "Distributed Intelligence".to_string(),
            Self::ContainerOrchestration => "Container Orchestration".to_string(),
            Self::Security => "Security".to_string(),
            Self::Monitoring => "Monitoring".to_string(),
            Self::Logging => "Logging".to_string(),
            Self::Metrics => "Metrics".to_string(),
            Self::HealthChecking => "Health Checking".to_string(),
            Self::Configuration => "Configuration".to_string(),
            Self::Network => "Network".to_string(),
            Self::MessageQueue => "Message Queue".to_string(),
            Self::EventStreaming => "Event Streaming".to_string(),
            Self::WorkflowOrchestration => "Workflow Orchestration".to_string(),
            Self::Storage => "Storage".to_string(),
            Self::Compute => "Compute".to_string(),
            Self::Networking => "Networking".to_string(),
            Self::ArtificialIntelligence => "Artificial Intelligence".to_string(),
            Self::Orchestration => "Orchestration".to_string(),
            Self::BiometricAuth => "Biometric Authentication".to_string(),
            Self::QuantumCrypto => "Quantum-Resistant Cryptography".to_string(),
            Self::ZeroKnowledgeProofs => "Zero-Knowledge Proofs".to_string(),
            Self::GeneticAlgorithms => "Genetic Algorithms".to_string(),
            Self::ThreatDetection => "Threat Detection".to_string(),
            Self::ComplianceAudit => "Compliance and Audit".to_string(),
            Self::Custom(name) => name.clone(),
        }
    }

    /// Stable capability identifier string for registries, ACLs, and serialized manifests.
    #[must_use]
    pub fn as_capability_id(&self) -> String {
        match self {
            Self::Security => "capability:security".to_string(),
            Self::Storage => "capability:storage".to_string(),
            Self::Compute => "capability:compute".to_string(),
            Self::Networking => "capability:networking".to_string(),
            Self::ArtificialIntelligence => "capability:ai".to_string(),
            Self::Orchestration => "capability:orchestration".to_string(),
            Self::Monitoring => "capability:monitoring".to_string(),
            Self::ComputeIntelligence => "capability:compute-intelligence".to_string(),
            Self::DistributedIntelligence => "capability:distributed-intelligence".to_string(),
            Self::ServiceMesh => "capability:service-mesh".to_string(),
            Self::DataStorage => "capability:data-storage".to_string(),
            Self::Custom(name) => format!("capability:custom:{name}"),
            _ => format!(
                "capability:{}",
                self.name().to_lowercase().replace(' ', "-")
            ),
        }
    }

    /// Token for `CAPABILITY_{TOKEN}_ENDPOINT` environment discovery (hyphenated, lowercase).
    ///
    /// Aligns with `beardog_discovery::discovered_services_from_environment`: the capability
    /// string passed there should match this token (e.g. `ipc`, `security`, `compute-intelligence`).
    #[must_use]
    pub fn discovery_env_token(&self) -> String {
        let id = self.as_capability_id();
        if let Some(rest) = id.strip_prefix("capability:custom:") {
            rest.to_string()
        } else if let Some(rest) = id.strip_prefix("capability:") {
            rest.to_string()
        } else {
            id
        }
    }

    /// Returns `true` if this capability class is typically fulfilled by an external vendor API.
    #[must_use]
    #[inline]
    pub const fn is_vendor_capability(&self) -> bool {
        matches!(
            self,
            Self::KeyManagement
                | Self::HardwareSecurityModule
                | Self::SecretsManagement
                | Self::Authentication
                | Self::CloudStorage
                | Self::DatabaseService
                | Self::LoadBalancing
                | Self::ContentDeliveryNetwork
        )
    }

    /// Returns `true` if this capability class is typically fulfilled by another primal in the mesh.
    #[must_use]
    #[inline]
    pub const fn is_primal_capability(&self) -> bool {
        matches!(
            self,
            Self::ServiceMesh
                | Self::ComputeIntelligence
                | Self::DataStorage
                | Self::DistributedIntelligence
                | Self::ContainerOrchestration
                | Self::Security
        )
    }

    /// Get the primal name associated with this capability (if any)
    /// DEPRECATED: Removed hardcoded primal mappings to achieve true capability-based discovery
    #[deprecated(note = "Use capability-based discovery instead of hardcoded primal names")]
    #[must_use]
    #[inline]
    pub const fn associated_primal(&self) -> Option<&'static str> {
        // EVOLUTION: No longer return hardcoded primal names
        // Each primal should discover capabilities dynamically through universal adapter
        // No capability available
        None
    }
}

impl std::fmt::Display for CapabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<CapabilityType> for String {
    fn from(capability: CapabilityType) -> Self {
        capability.as_capability_id()
    }
}

/// Use `CapabilityType` directly in new code
pub type ServiceCapabilityType = CapabilityType;
