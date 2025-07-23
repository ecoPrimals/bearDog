//! Core BearDog orchestration engine
//!
//! Manages the lifecycle and health of all BearDog security components.

pub mod components;
pub mod lifecycle;
pub mod operations;
pub mod primal_provider;

// Tests have been moved to integration tests directory

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// Import from our refactored modules
use crate::node_registry::{BasicNodeRegistry, BasicProofVerifier};
use crate::types::{ComponentStatus, CoreState};

use crate::universal_primal_provider::{PrimalCapability, PrimalMetadata};
use beardog_auth::auth::{CrossNodeAuthEngine, NodeRegistry, ProofVerifier};
use beardog_compliance::compliance::ComplianceEngine;
use beardog_compliance::AuditEngine;
use beardog_config::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::encryption::EncryptionEngine;
use beardog_security::BearDogSecurityProvider;
use beardog_threat::threat::types::engine::ThreatDetectionEngine;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_workflows::workflows::InMemoryApprovalStore;
use beardog_workflows::workflows::InMemoryWorkflowStore;
use beardog_workflows::workflows::MultiPartyWorkflowEngine;

/// Core BearDog orchestration engine
#[derive(Clone)]
pub struct BearDogCore {
    pub(crate) config: Arc<BearDogConfig>,
    pub(crate) state: Arc<RwLock<CoreState>>,
    pub(crate) workflow_engine: Arc<MultiPartyWorkflowEngine>,
    pub(crate) encryption_engine: Arc<EncryptionEngine>,
    pub(crate) audit_engine: Arc<AuditEngine>,
    pub(crate) threat_detection_engine: Arc<ThreatDetectionEngine>,
    pub(crate) compliance_engine: Arc<ComplianceEngine>,
    pub(crate) security_provider: Arc<BearDogSecurityProvider>,
    pub(crate) hsm_manager: Arc<HsmManager>,
    pub(crate) cross_node_auth_engine: Arc<CrossNodeAuthEngine>,
    pub(crate) startup_time: std::time::Instant,
    pub(crate) component_status: HashMap<String, ComponentStatus>,
    // Universal Primal Provider integration
    pub(crate) primal_metadata: PrimalMetadata,
    pub(crate) primal_capabilities: Vec<PrimalCapability>,
}

impl BearDogCore {
    /// Create a new BearDog core instance
    pub async fn new(config: BearDogConfig) -> BearDogResult<Self> {
        info!("🐻 Initializing BearDog Security Manager");

        // Convert SecurityEncryptionConfig to EncryptionConfig
        let encryption_config = beardog_config::security::EncryptionConfig {
            default_algorithm: config.encryption.default_algorithm.clone(),
            key_derivation_iterations: 100_000,
            key_rotation_days: 30,
            key_rotation_interval: std::time::Duration::from_secs(
                config.encryption.key_rotation_interval,
            ),
            hsm: beardog_config::security::HsmConfig::default(),
            key_derivation: beardog_config::security::KeyDerivationConfig::default(),
        };

        // Initialize encryption engine
        let encryption_engine = Arc::new(EncryptionEngine::new(encryption_config).await?);

        // Initialize audit engine
        let audit_engine = Arc::new(AuditEngine::new().await);

        // Convert config types to module-specific types
        let threat_config = beardog_threat::threat::types::ThreatDetectionConfig {
            real_time_detection: config.threat_detection.enabled,
            threat_threshold: 80,
            automated_response: true,
            max_alerts_per_minute: 10,
            ml_enhancement: false,
            threat_feeds: vec![],
            auto_quarantine: false,
            notification_endpoints: vec![],
            enabled: config.threat_detection.enabled,
            rules_path: "rules/".to_string(),
            monitor_paths: vec![],
            alert_threshold: 0.8,
            cache_size: beardog_config::constants::performance::DEFAULT_CACHE_SIZE,
            monitoring_interval: 30,
        };

        let compliance_config = beardog_compliance::compliance::ComplianceConfig {
            enabled_standards: vec![
                beardog_compliance::compliance::ComplianceStandard::GDPR,
                beardog_compliance::compliance::ComplianceStandard::SOX,
                beardog_compliance::compliance::ComplianceStandard::PCI_DSS,
            ],
            monitoring_interval: chrono::Duration::minutes(5),
            audit_retention: chrono::Duration::days(config.compliance.audit_retention_days as i64),
            dashboard_refresh_interval: chrono::Duration::minutes(1),
            reporting: beardog_compliance::compliance::ReportingConfig::default(),
        };

        // Initialize threat detection engine
        let threat_detection_engine = Arc::new(ThreatDetectionEngine::new(threat_config));

        // Initialize compliance engine
        let compliance_engine = Arc::new(ComplianceEngine::new(compliance_config).await?);

        // Initialize workflow engine
        let workflow_engine = Arc::new(MultiPartyWorkflowEngine::new(
            config.workflows.clone(),
            Arc::new(InMemoryApprovalStore::new()),
            Arc::new(InMemoryWorkflowStore::new()),
            Arc::new(
                beardog_workflows::workflows::notification::NotificationEngine::new(
                    beardog_config::integration::NotificationConfig::default(),
                ),
            ),
        ));

        // Initialize security provider with proper configuration
        let security_config = beardog_security::SecurityProviderConfig::default();

        // Enable memory key manager if in standalone mode
        if config.app.standalone_mode {
            info!("Running in standalone mode with memory key manager");
        } else {
            info!("Running in distributed mode");
        }

        let security_provider = Arc::new(
            beardog_security::BearDogSecurityProvider::new_with_config(security_config).await?,
        );

        // Initialize HSM manager
        let hsm_manager = Arc::new(HsmManager::new());

        // Initialize cross-node authorization engine
        // Create basic implementations for NodeRegistry and ProofVerifier
        let node_registry = Box::new(BasicNodeRegistry::new());
        let proof_verifier = Box::new(BasicProofVerifier::new());
        let cross_node_auth_engine =
            Arc::new(CrossNodeAuthEngine::new(node_registry, proof_verifier));

        let core = Self {
            config: Arc::new(config),
            encryption_engine,
            audit_engine,
            threat_detection_engine,
            compliance_engine,
            workflow_engine,
            security_provider,
            hsm_manager,
            cross_node_auth_engine,
            startup_time: std::time::Instant::now(),
            component_status: HashMap::new(),
            state: Arc::new(RwLock::new(CoreState::default())),
            // Initialize primal provider metadata
            primal_metadata: PrimalMetadata::default(),
            primal_capabilities: vec![
                PrimalCapability::Security,
                PrimalCapability::AI,
                PrimalCapability::Monitoring,
                PrimalCapability::Compliance,
                PrimalCapability::ThreatDetection,
                PrimalCapability::KeyManagement,
                PrimalCapability::Workflow,
            ],
        };

        info!("✅ BearDog Security Manager initialized successfully");
        Ok(core)
    }

    /// Get configuration reference
    pub fn config(&self) -> &BearDogConfig {
        &self.config
    }

    /// Get startup time
    pub fn get_startup_time(&self) -> std::time::Instant {
        self.startup_time
    }

    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> u64 {
        self.startup_time.elapsed().as_secs()
    }

    /// Get component status map
    pub fn get_component_status(&self) -> &HashMap<String, ComponentStatus> {
        &self.component_status
    }

    /// Get component status by name
    pub fn get_component_status_by_name(&self, name: &str) -> Option<&ComponentStatus> {
        self.component_status.get(name)
    }

    /// Get workflow engine reference
    pub fn workflow_engine(&self) -> &MultiPartyWorkflowEngine {
        &self.workflow_engine
    }

    /// Get encryption engine reference
    pub fn encryption_engine(&self) -> &EncryptionEngine {
        &self.encryption_engine
    }

    /// Get audit engine reference
    pub fn audit_engine(&self) -> &AuditEngine {
        &self.audit_engine
    }

    /// Get threat detection engine reference
    pub fn threat_detection_engine(&self) -> &ThreatDetectionEngine {
        &self.threat_detection_engine
    }

    /// Get compliance engine reference
    pub fn compliance_engine(&self) -> &ComplianceEngine {
        &self.compliance_engine
    }

    /// Get security provider reference
    pub fn security_provider(&self) -> &BearDogSecurityProvider {
        &self.security_provider
    }

    /// Get HSM manager reference
    pub fn hsm_manager(&self) -> &HsmManager {
        &self.hsm_manager
    }

    /// Get cross-node authorization engine
    pub fn cross_node_auth(&self) -> BearDogResult<&CrossNodeAuthEngine> {
        // Return the actual cross-node authorization engine
        Ok(&self.cross_node_auth_engine)
    }

    /// Get node registry
    pub fn node_registry(&self) -> BearDogResult<&dyn NodeRegistry> {
        // Node registry is handled by SongBird orchestration layer
        // For now, return a placeholder error indicating this should be handled by SongBird
        Err(BearDogError::ServiceUnavailable {
            service: "node_registry".to_string(),
            message: "Node registry managed by SongBird orchestration layer".to_string(),
        })
    }

    /// Get proof verifier
    pub fn proof_verifier(&self) -> BearDogResult<&dyn ProofVerifier> {
        // For now, return a placeholder error since proof verifier is still being implemented
        Err(BearDogError::ServiceUnavailable {
            service: "proof_verifier".to_string(),
            message: "Proof verification delegated to SongBird orchestration layer".to_string(),
        })
    }
}
