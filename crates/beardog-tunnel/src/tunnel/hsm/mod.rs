

pub mod zero_cost_provider;

pub use zero_cost_provider::{
    ZeroCostHsmProvider, ZeroCostHsmManager,
    HsmProviderTrait, migrate_to_zero_cost,
};

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub use beardog_types::canonical::hsm::{
    HsmCapabilities, HsmConfig, HsmKey, HsmProviderType as HsmType, 
    KeyMetadata, HsmTier, HsmHealth, HsmHealthStatus
};
pub use beardog_types::canonical::crypto::{KeyType, KeyUsage};
pub use beardog_types::canonical::configuration::{
    ConnectionConfig, PerformanceConfig
};

pub use types::*;

pub use beardog_traits::canonical::HsmProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeyRequest {
    pub key_size: Option<u32>,
    pub usage_policy: KeyUsagePolicy,
    pub key_type: KeyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsagePolicy {
    pub can_encrypt: bool,
    pub can_decrypt: bool,
    pub can_sign: bool,
    pub can_verify: bool,
    pub exportable: bool,
}

impl Default for KeyUsagePolicy {
    fn default() -> Self {
        Self {
            can_encrypt: true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            exportable: false,
        }
    }
}

pub trait HsmCapabilityDetector: Send + Sync {
    async fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError>;
    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError>;
    async fn recommend_hsm_tier(
        requirements: &SecurityRequirements,
    ) -> Result<HsmTier, BearDogError>;
}

pub trait HsmHealthMonitor: Send + Sync {
    async fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> Result<(), BearDogError>;
    async fn get_health_status(&self) -> Result<HashMap<String, HsmHealthStatus>, BearDogError>;
    async fn filter_healthy_providers(
        providers: Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> Result<Vec<impl HsmProvider + Send + Sync + 'static>, BearDogError>;
}

pub trait HsmFailoverManager: Send + Sync {
    async fn handle_provider_failure(
        provider: &(impl HsmProvider + Send + Sync + 'static),
        error: &BearDogError,
    ) -> Result<(), BearDogError>;
    
    async fn get_failover_provider(
        failed_provider: &(impl HsmProvider + Send + Sync + 'static),
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError>;
    
    async fn perform_with_failover<T, F>(
        operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(impl HsmProvider + Send + Sync + 'static) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static;
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct SecurityRequirements {
    pub security_level: SecurityLevel,
    pub user_interaction_required: bool,
    pub attestation_required: bool,
    pub hardware_backed_required: bool,
    pub compliance_requirements: Vec<ComplianceStandard>,
    pub performance_requirements: PerformanceRequirements,
}

pub struct PerformanceRequirements {
    pub max_latency_ms: Option<u64>,
    pub min_throughput_ops_per_sec: Option<u64>,
    pub cost_optimization: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {

    Basic,

    Medium,

    Tee,

    High,

    StrongBox,
    Critical,
    Maximum,
}

pub enum ComplianceStandard {
    Gdpr,
    Hipaa,
    Sox,
    PciDss,
    FedRamp,
    Fips140Level2,
    Fips140Level3,
    CommonCriteria,
}

pub struct OperationContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub operation_type: OperationType,
    pub timestamp: DateTime<Utc>,
}

pub enum OperationType {
    KeyGeneration,
    KeyImport,
    Encryption,
    Decryption,
    Signing,
    Verification,
    KeyDerivation,
    KeyBackup,
    KeyRestoration,
}

impl SecurityRequirements {

    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            security_level,
            user_interaction_required: false,
            attestation_required: false,
            hardware_backed_required: matches!(
                security_level,
                SecurityLevel::High | SecurityLevel::Maximum
            ),
            compliance_requirements: vec![],
            performance_requirements: PerformanceRequirements::default(),
        }
    }
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::Medium,
            user_interaction_required: false,
            attestation_required: false,
            hardware_backed_required: false,
            compliance_requirements: vec![],
            performance_requirements: PerformanceRequirements::default(),
        }
    }
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_latency_ms: Some(1000),           // 1 second default
            min_throughput_ops_per_sec: Some(10), // 10 ops/sec default
            cost_optimization: false,
        }
    }
}

impl Default for OperationContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            operation_type: OperationType::KeyGeneration,
            timestamp: Utc::now(),
        }
    }
}

pub use android_strongbox::AndroidStrongBoxHsm;
pub use manager::HsmManager;
pub use software_hsm::RustSoftwareHsm;
