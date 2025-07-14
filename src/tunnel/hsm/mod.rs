//! # BearDog HSM Integration Module
//!
//! This module provides Hardware Security Module (HSM) integration with support for:
//! - Smartphone HSMs (iOS Secure Enclave, Android StrongBox)
//! - Software HSMs (Rust-based with memory protection)
//! - Hardware HSMs (PKCS#11 compatible)
//! - Hybrid HSMs (Multi-tier orchestration)
//!
//! ## Design Philosophy
//!
//! The HSM system is designed with a multi-tier approach where different HSM types
//! are used based on security requirements, availability, and user interaction needs.
//!
//! ## Architecture
//!
//! ```
//! ┌─────────────────┐
//! │   HSM Manager   │  ← Intelligent tier selection
//! └─────────────────┘
//!          │
//!    ┌─────┴─────┐
//!    │           │
//! ┌──▼──┐    ┌──▼──┐
//! │ T1  │    │ T2  │    T1: Smartphone HSM (Always available)
//! │ 📱  │    │ 💻  │    T2: Software HSM (Scalable)
//! └─────┘    └─────┘    T3: Hardware HSM (Maximum security)
//!    │           │      T4: Hybrid HSM (Best of all worlds)
//! ┌──▼──┐    ┌──▼──┐
//! │ T3  │    │ T4  │
//! │ 🔒  │    │ 🔄  │
//! └─────┘    └─────┘
//! ```

use crate::error::{BearDogError, BearDogResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod android_strongbox;
pub mod manager;
pub mod software_hsm;
pub mod types;

// Re-export common types
pub use android_strongbox::AndroidStrongBoxHsm;
pub use manager::{
    HsmManager, HsmManagerConfig, SimpleHsmTier, HsmProviderSelection,
    DefaultHsmHealthMonitor, DefaultHsmFailoverManager, CircuitBreaker, CircuitBreakerState,
    DefaultHsmCapabilityDetector, HsmPerformanceTracker, OperationMetrics,
    // Re-export all config types
    HealthConfig, FailoverConfig, PerformanceConfig
};
pub use software_hsm::RustSoftwareHsm;
pub use types::*;

/// Universal HSM interface for all HSM types
#[async_trait]
pub trait HsmProvider: Send + Sync {
    /// Initialize the HSM connection
    async fn initialize(&self, config: HsmConfig) -> BearDogResult<()>;

    /// Generate a new key in the HSM
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey>;

    /// Import an existing key into the HSM
    async fn import_key(&self, key_data: &[u8], metadata: KeyMetadata) -> BearDogResult<HsmKey>;

    /// Encrypt data using HSM key
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Decrypt data using HSM key
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Sign data using HSM key
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Verify signature using HSM key
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool>;

    /// Derive key using HSM-based KDF
    async fn derive_key(
        &self,
        master_key_id: &str,
        derivation_data: &[u8],
    ) -> BearDogResult<HsmKey>;

    /// Get HSM information and capabilities
    async fn get_info(&self) -> BearDogResult<HsmInfo>;

    /// List keys stored in HSM
    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>>;

    /// Delete key from HSM
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()>;

    /// Backup HSM state (if supported)
    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>>;

    /// Restore HSM state (if supported)
    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()>;

    /// Get HSM health status
    async fn health_check(&self) -> BearDogResult<HsmHealthStatus>;
}

/// HSM capability detection and management
#[async_trait]
pub trait HsmCapabilityDetector: Send + Sync {
    /// Detect available HSM capabilities on the current system
    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapability>>;

    /// Check if a specific HSM type is available
    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool>;

    /// Get recommended HSM tier for given requirements
    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier>;
}

/// HSM health monitoring and management
#[async_trait]
pub trait HsmHealthMonitor: Send + Sync {
    /// Start health monitoring for HSM providers
    async fn start_monitoring(&self, providers: Vec<Arc<dyn HsmProvider>>) -> BearDogResult<()>;

    /// Get current health status for all monitored HSMs
    async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>>;

    /// Filter providers to only return healthy ones
    async fn filter_healthy_providers(
        &self,
        providers: Vec<Arc<dyn HsmProvider>>,
    ) -> BearDogResult<Vec<Arc<dyn HsmProvider>>>;
}

/// HSM failover and retry logic
#[async_trait]
pub trait HsmFailoverManager: Send + Sync {
    /// Handle HSM provider failure
    async fn handle_provider_failure(
        &self,
        provider: &Arc<dyn HsmProvider>,
        error: &BearDogError,
    ) -> BearDogResult<()>;

    /// Get failover provider for failed primary
    async fn get_failover_provider(
        &self,
        failed_provider: &Arc<dyn HsmProvider>,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Arc<dyn HsmProvider>>;

    /// Perform operation with automatic failover
    async fn perform_with_failover<T, F>(
        &self,
        operation: F,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<T>
    where
        F: Fn(Arc<dyn HsmProvider>) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static;
}

/// Security requirements for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub security_level: SecurityLevel,
    pub user_interaction_required: bool,
    pub attestation_required: bool,
    pub hardware_backed_required: bool,
    pub compliance_requirements: Vec<ComplianceStandard>,
    pub performance_requirements: PerformanceRequirements,
}

/// Performance requirements for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_latency_ms: Option<u64>,
    pub min_throughput_ops_per_sec: Option<u64>,
    pub cost_optimization: bool,
}

/// Security levels for HSM operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Basic security - software-based acceptable
    Basic,
    /// Medium security - hardware-backed preferred
    Medium,
    /// High security - hardware-backed required
    High,
    /// Maximum security - certified hardware required
    Maximum,
}

/// Compliance standards
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Operation context for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub operation_type: OperationType,
    pub user_interaction_required: bool,
    pub timestamp: DateTime<Utc>,
}

/// Types of HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Create new SecurityRequirements with specified security level
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
            user_interaction_required: false,
            timestamp: Utc::now(),
        }
    }
}
