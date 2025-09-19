

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

#[derive(Debug, Clone)]
    /// The usage policy value
    pub usage_policy: KeyUsagePolicy,
    /// The key type value
    pub key_type: KeyType,
}

#[derive(Debug, Clone)]
    /// Whether can_decrypt is enabled
    pub can_decrypt: bool,
    /// Whether can_sign is enabled
    pub can_sign: bool,
    /// Whether can_verify is enabled
    pub can_verify: bool,
    /// Whether exportable is enabled
    pub exportable: bool,
}

impl Default for KeyUsagePolicy {
    fn default(true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            exportable: false,
        }
    }
}

pub trait HsmCapabilityDetector: Send + Sync {
    fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError>> + Send;
    /// Checks if hsm available
    fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError>;
    fn recommend_hsm_tier(&SecurityRequirements,
    ) -> Result<HsmTier, BearDogError>;
}

pub trait HsmHealthMonitor: Send + Sync {
    /// Starts monitoring
    fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> Result<(), BearDogError>;
    /// Gets health_status
    fn get_health_status(Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> Result<Vec<impl HsmProvider + Send + Sync + 'static>, BearDogError>> + Send;
}

pub trait HsmFailoverManager: Send + Sync {
    /// Handles provider_failure
    fn handle_provider_failure(
        provider: &(impl HsmProvider + Send + Sync + 'static),
        error: &BearDogError,
    ) -> Result<(), BearDogError>;
    
    /// Gets failover_provider
    
    fn get_failover_provider(
        failed_provider: &(impl HsmProvider + Send + Sync + 'static),
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError>;
    
    
    fn perform_with_failover<T, F>(
        operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(Send + 'static;
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct SecurityRequirements {
    /// The security level value
    pub security_level: SecurityLevel,
    /// Whether user_interaction_required is enabled
    pub user_interaction_required: bool,
    /// Whether attestation_required is enabled
    pub attestation_required: bool,
    /// Whether hardware_backed_required is enabled
    pub hardware_backed_required: bool,
    /// Collection of compliance requirements
    pub compliance_requirements: Vec<ComplianceStandard>,
    pub performance_requirements: PerformanceRequirements,
}

pub struct PerformanceRequirements {
    /// Optional max latency ms
    pub max_latency_ms: Option<u64>,
    /// Optional min throughput ops per sec
    pub min_throughput_ops_per_sec: Option<u64>,
    /// Whether cost_optimization is enabled
    pub cost_optimization: bool,
}

#[derive(Debug, Clone)]
    pub session_id: Option<String>,
    /// The operation type value
    pub operation_type: OperationType,
    pub timestamp: DateTime<Utc>,
}
/// Types of operation
pub enum OperationType {
    /// Represents key generation variant
    KeyGeneration,
    /// Represents key import variant
    KeyImport,
    /// Represents encryption variant
    Encryption,
    /// Represents decryption variant
    Decryption,
    /// Currently signing
    Signing,
    /// Represents verification variant
    Verification,
    /// Represents key derivation variant
    KeyDerivation,
    /// Represents key backup variant
    KeyBackup,
    /// Represents key restoration variant
    KeyRestoration,
}

impl SecurityRequirements {

/// New operation.
    /// Creates a new instance
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
            performance_requirements: PerformanceRequirements::default(SecurityLevel::Medium,
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
            min_throughput_ops_per_sec: Some(false,
        }
    }
}

impl Default for OperationContext {
    fn default(None,
            session_id: None,
            operation_type: OperationType::KeyGeneration,
            timestamp: Utc::now(),
        }
    }
}

pub use android_strongbox::AndroidStrongBoxHsm;
pub use manager::HsmManager;
pub use software_hsm::RustSoftwareHsm;
