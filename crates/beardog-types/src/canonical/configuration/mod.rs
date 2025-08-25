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


pub mod integration;
pub mod monitoring;
pub mod network;
pub mod performance;
pub mod providers;
/// # Canonical Configuration Module
///
/// This module provides canonical configuration types that eliminate fragmentation
/// across the BearDog ecosystem. All configuration should use these canonical types.
pub mod security;
pub mod storage;
pub mod workflows;
// CANONICAL MODERNIZATION: Adding missing configuration domains
pub mod compliance;
pub mod production;
pub mod encryption;

// Re-export all configuration types
pub use integration::*;
pub use monitoring::{UnifiedMonitoringConfig, MetricCollectionConfig, AlertProcessingConfig, PrometheusConfig, SecurityMonitoringConfig, PerformanceMonitoringConfig};
pub use network::*;
pub use performance::*;
pub use providers::*;
pub use security::*;
pub use storage::*;
// Re-exports from workflows module when needed
// Re-export new canonical configuration modules
pub use compliance::{ComplianceConfig, ComplianceStandard, ReportingConfig, PrivacyAuditConfig, DataSovereigntyConfig};
pub use production::{ProductionConfig, Environment, ClusterConfig, NodeConfig, BackupConfig, MaintenanceConfig, CircuitBreakerConfig, HealthMonitoringConfig, ResourceLimitsConfig};
pub use encryption::{EncryptionAlgorithm, EncryptionMode, KeyDerivationConfig, ContextAwareKeyConfig, EntropyAdjustmentConfig, KeyRotationConfig};

// **WORKFLOW CONFIGURATION UNIFICATION** ✅
// Re-export unified workflow configurations - eliminates 8+ duplicate config structs
pub use workflows::{
    WorkflowConfig, WorkflowEngineConfig, WorkflowPolicyConfig, WorkflowProcessorConfig,
    WorkflowNotificationConfig, WorkflowRetryConfig, ZeroCostWorkflowConfig,
    WorkflowStorageConfig, WorkflowMonitoringConfig, ApprovalRequirements, ApprovalTier,
    SecurityProcessorConfig, KeyManagementConfig, SystemProcessorConfig, PolicyProcessorConfig,
    RegistryProcessorConfig, UserManagementProcessorConfig, EmailNotificationSettings,
    WebhookNotificationSettings, AutoApprovalRule, ZeroCostMemoryPoolConfig,
    WorkflowStorageBackend, WorkflowRetentionPolicy,
};

// Type aliases for commonly used configurations - eliminate duplication
pub use crate::config::security_unified::UnifiedSecurityConfig as SecurityConfig;
/// MFA method enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// TOTP (Time-based One-Time Password)
    Totp,
    /// SMS authentication
    Sms,
    /// Email authentication
    Email,
    /// Hardware token
    HardwareToken,
    /// Biometric authentication
    Biometric,
    /// Backup codes
    BackupCodes,
}
// ============================================================================
// UNIFIED CONFIGURATION TYPES - Main aggregation structures
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// **CANONICAL BEARDOG CONFIGURATION** - Main application configuration
/// This is the primary configuration structure that aggregates all domain-specific
/// configurations into a single, unified system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct BearDogConfig {
    /// Performance and optimization settings
    pub performance: PerformanceConfig,
    /// Security and encryption settings
    pub security: SecurityConfig,
    /// Network and communication settings
    pub network: NetworkConfig,
    /// Storage and caching settings
    pub storage: StorageConfig,
    /// Provider integration settings
    pub providers: ProviderConfig,
    /// Workflow processing settings
    pub workflows: WorkflowConfig,
    /// Monitoring and observability settings
    pub monitoring: UnifiedMonitoringConfig,
    /// Compliance and regulatory settings
    pub compliance: ComplianceConfig,
    /// Production deployment settings
    pub production: ProductionConfig,
    /// Encryption and cryptographic settings
    pub encryption: encryption::EncryptionConfig,
}


/// **CANONICAL APPLICATION CONFIGURATION** - Application-level settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Application environment (dev, staging, prod)
    pub environment: String,
    /// Debug mode enabled
    pub debug: bool,
    /// Log level
    pub log_level: String,
    /// Configuration file path
    pub config_path: Option<String>,
}

/// **CANONICAL FEATURE CONFIGURATION** - Feature flag management
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureConfig {
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Feature rollout percentages
    pub rollout_percentages: HashMap<String, f64>,
    /// Feature dependencies
    pub dependencies: HashMap<String, Vec<String>>,
}

/// **CANONICAL ENVIRONMENT CONFIGURATION** - Environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Environment variables
    pub variables: HashMap<String, String>,
    /// Environment-specific overrides
    pub overrides: HashMap<String, serde_json::Value>,
    /// Environment validation rules
    pub validation_rules: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Maximum concurrent sessions
    pub max_concurrent: u32,
    /// Session storage type
    pub storage: String,
    /// Maximum failed login attempts
    pub max_failed_attempts: u32,
    /// Account lockout duration
    pub lockout_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
    /// Rate limit window
    pub window: Duration,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    /// Authentication method
    pub method: String,
    /// Multi-factor authentication settings
    pub mfa: MfaConfig,
    /// Password policy
    pub password_policy: PasswordPolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub mode: String,
    pub padding: Option<String>,
    pub key_derivation_iterations: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MfaConfig {
    /// Whether MFA is required
    pub required: bool,
    /// Available MFA methods
    pub methods: Vec<MfaMethod>,
    /// TOTP configuration
    pub totp: TotpConfig,
}
