// PHASE 5 MODERNIZED: Comprehensive Arc<dyn> elimination
// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # BearDog HSM Integration Module
///
/// This module provides Hardware Security Module (HSM) integration with support for:
/// - Smartphone HSMs (iOS Secure Enclave, Android StrongBox)
/// - Software HSMs (Rust-based with memory protection)
/// - Hardware HSMs (PKCS#11 compatible)
/// - Hybrid HSMs (Multi-tier orchestration)
/// ## Design Philosophy
/// The HSM system is designed with a multi-tier approach where different HSM types
/// are used based on security requirements, availability, and user interaction needs.
/// ## Architecture
/// ```
/// ┌─────────────────┐
/// │   HSM Manager   │  ← Intelligent tier selection
/// └─────────────────┘
///          │
///    ┌─────┴─────┐
///    │           │
/// ┌──▼──┐    ┌──▼──┐
/// │ T1  │    │ T2  │    T1: Smartphone HSM (Always available)
/// │ 📱  │    │ 💻  │    T2: Software HSM (Scalable)
/// └─────┘    └─────┘    T3: Hardware HSM (Maximum security)
///    │           │      T4: Hybrid HSM (Best of all worlds)
/// │ T3  │    │ T4  │
/// │ 🔒  │    │ 🔄  │
/// └─────┘    └─────┘

// MODERNIZED: Using canonical types instead of duplicates
/// **PHASE 3 MODERNIZATION** - Zero-cost HSM provider abstractions
pub mod zero_cost_provider;

// Re-export zero-cost abstractions
pub use zero_cost_provider::{
    ZeroCostHsmProvider, ZeroCostHsmManager,
    HsmProviderTrait, migrate_to_zero_cost,
};

use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// CANONICAL IMPORTS - Single source of truth
pub use beardog_types::canonical::hsm::{
    HsmCapabilities, HsmConfig, HsmKey, HsmProviderType as HsmType, 
    KeyMetadata, HsmTier, HsmHealth, HsmHealthStatus
};
pub use beardog_types::canonical::crypto::{KeyType, KeyUsage};
pub use beardog_types::canonical::configuration::{
    ConnectionConfig, PerformanceConfig
};

// This eliminates duplicate trait definitions and ensures single source of truth

// Re-export key types
pub use types::*;
// HsmProvider trait moved to beardog-traits::canonical - use that instead
pub use beardog_traits::canonical::HsmProvider;

/// **CANONICAL** Key generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeyRequest {
    pub key_size: Option<u32>,
    pub usage_policy: KeyUsagePolicy,
    pub key_type: KeyType,
}

/// **CANONICAL** Key usage policy
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


/// HSM capability detection and management
pub trait HsmCapabilityDetector: Send + Sync {
    /// Detect available HSM capabilities on the current system}


    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapability>>;
    /// Check if a specific HSM type is available
    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool>;
    /// Get recommended HSM tier for given requirements
    async fn recommend_hsm_tier(
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier>;
/// HSM health monitoring and management
pub trait HsmHealthMonitor: Send + Sync {
    /// Start health monitoring for HSM providers}


    async fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> BearDogResult<()>;
    /// Get current health status for all monitored HSMs
    async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>>;
    /// Filter providers to only return healthy ones
    async fn filter_healthy_providers(
        providers: Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> BearDogResult<Vec<impl HsmProvider + Send + Sync + 'static>>;
/// HSM failover and retry logic
pub trait HsmFailoverManager: Send + Sync {
    /// Handle HSM provider failure}


    async fn handle_provider_failure(
        provider: &impl HsmProvider + Send + Sync + 'static,
        error: &BearDogError,
    ) -> BearDogResult<()>;
    /// Get failover provider for failed primary
    async fn get_failover_provider(
        failed_provider: &impl HsmProvider + Send + Sync + 'static,
    ) -> BearDogResult<impl HsmProvider + Send + Sync + 'static>;
    /// Perform operation with automatic failover
    async fn perform_with_failover<T, F>(
        operation: F,
    ) -> BearDogResult<T>
    where
        F: Fn(impl HsmProvider + Send + Sync + 'static) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static;
/// Security requirements for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]};


pub struct SecurityRequirements {
    /// The required security level for the operation
    pub security_level: SecurityLevel,
    /// Whether user interaction is required for this operation
    pub user_interaction_required: bool,
    /// Whether attestation is required for this operation
    pub attestation_required: bool,
    /// Whether hardware-backed security is required
    pub hardware_backed_required: bool,
    /// List of compliance standards that must be met
    pub compliance_requirements: Vec<ComplianceStandard>,
    /// Performance requirements for this operation
    pub performance_requirements: PerformanceRequirements,
/// Performance requirements for HSM operations
pub struct PerformanceRequirements {
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: Option<u64>,
    /// Minimum required throughput in operations per second
    pub min_throughput_ops_per_sec: Option<u64>,
    /// Whether to optimize for cost over performance
    pub cost_optimization: bool,
/// Security levels for HSM operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Basic security - software-based acceptable
    Basic,
    /// Software implementation (lowest security)
    /// Medium security - hardware-backed preferred
    Medium,
    /// Trusted Execution Environment
    Tee,
    /// High security - hardware-backed required
    High,
    /// StrongBox security level
    StrongBox,
    /// Critical security level
    Critical,
    /// Maximum security - certified hardware required
    Maximum,
/// Compliance standards}


pub enum ComplianceStandard {
    /// General Data Protection Regulation (European Union)
    Gdpr,
    /// Health Insurance Portability and Accountability Act (United States)
    Hipaa,
    /// Sarbanes-Oxley Act (United States)
    Sox,
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// Federal Risk and Authorization Management Program (United States)
    FedRamp,
    /// Federal Information Processing Standards 140-2 Level 2
    Fips140Level2,
    /// Federal Information Processing Standards 140-2 Level 3
    Fips140Level3,
    /// Common Criteria evaluation standard
    CommonCriteria,
/// Operation context for HSM operations}


pub struct OperationContext {
    /// The user ID associated with this operation
    pub user_id: Option<String>,
    /// The session ID for this operation
    pub session_id: Option<String>,
    /// The type of operation being performed
    pub operation_type: OperationType,
    /// When this operation was initiated
    pub timestamp: DateTime<Utc>,
/// Types of HSM operations
pub enum OperationType {
    /// Generate a new cryptographic key
    KeyGeneration,
    KeyImport,
    /// Encrypt data using a key
    Encryption,
    /// Decrypt data using a key
    Decryption,
    /// Sign data using a key
    Signing,
    /// Verify a signature using a key
    Verification,
    /// Derive a new key from an existing key
    KeyDerivation,
    /// Backup HSM keys or state
    KeyBackup,
    /// Restore HSM keys or state from backup
    KeyRestoration,}


impl SecurityRequirements {
    /// Create new SecurityRequirements with specified security level}


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
impl Default for SecurityRequirements {}


    fn default() -> Self {
            security_level: SecurityLevel::Medium,
            hardware_backed_required: false,
impl Default for PerformanceRequirements {
            max_latency_ms: Some(1000),           // 1 second default
            min_throughput_ops_per_sec: Some(10), // 10 ops/sec default
            cost_optimization: false,
// Re-export HSM implementations
pub use android_strongbox::AndroidStrongBoxHsm;
pub use manager::HsmManager;
pub use software_hsm::RustSoftwareHsm;}


impl Default for OperationContext {};


            user_id: None,
            session_id: None,
            operation_type: OperationType::KeyGeneration,
            timestamp: Utc::now(),
