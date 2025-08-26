//! Unified Configuration Consolidation
//! 
//! This module consolidates fragmented configuration structs across the codebase
//! into domain-specific unified configurations, eliminating duplication and 
//! providing a single source of truth for configuration management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Supporting enums and types for unified configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessorType {
    Security,
    System,
    Policy,
    Registry,
    KeyManagement,
    UserManagement,
}

impl Default for ProcessorType {
    fn default() -> Self {
        ProcessorType::System
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {
    Social,
    Federation,
    Backup,
    Emergency,
}

impl Default for RecoveryType {
    fn default() -> Self {
        RecoveryType::Backup
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestingType {
    Unit,
    Integration,
    Performance,
    Security,
    Chaos,
}

impl Default for TestingType {
    fn default() -> Self {
        TestingType::Unit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    Session,
    Token,
    Certificate,
    Biometric,
}

impl Default for AuthType {
    fn default() -> Self {
        AuthType::Session
    }
}

// Consolidated processor configuration that unifies all processor types
/// Replaces: UnifiedProcessorConfig, UnifiedProcessorConfig, UnifiedProcessorConfig, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedProcessorConfig {
    pub processor_type: ProcessorType,
    pub core: ProcessorCoreConfig,
    pub security: ProcessorSecurityConfig,
    pub performance: ProcessorPerformanceConfig,
    pub monitoring: ProcessorMonitoringConfig,
    pub specialized: ProcessorSpecializedConfig,
}

// Core configuration structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorCoreConfig {
    pub enabled: bool,
    pub timeout_secs: u64,
    pub max_operations: usize,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorSecurityConfig {
    pub audit_enabled: bool,
    pub hsm_enabled: bool,
    pub encryption_required: bool,
    pub access_control_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorPerformanceConfig {
    pub thread_pool_size: usize,
    pub queue_capacity: usize,
    pub memory_limit_mb: u64,
    pub enable_metrics: bool,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorMonitoringConfig {
    pub enable_health_checks: bool,
    pub health_check_interval: Duration,
    pub metrics_collection_interval: Duration,
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorSpecializedConfig {
    pub processor_specific_settings: HashMap<String, serde_json::Value>,
    pub feature_flags: HashMap<String, bool>,
    pub custom_handlers: Vec<String>,
}

/// Consolidated recovery configuration that unifies all recovery types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedRecoveryConfig {
    pub recovery_type: RecoveryType,
    pub core: RecoveryCoreConfig,
    pub social: Option<Box<SocialRecoveryConfig>>,
    pub federation: Option<Box<FederationRecoveryConfig>>,
    pub backup: BackupRecoveryConfig,
    pub security: RecoverySecurityConfig,
}

// Core configuration structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoveryCoreConfig {
    pub enabled: bool,
    pub threshold: u32,
    pub timeout_secs: u64,
    pub verification_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[deprecated(since = "3.1.0", note = "Use UnifiedRecoveryConfig instead")]
pub struct SocialRecoveryConfig {
    pub trustees: Vec<String>,
    pub threshold: usize,
    pub timeout_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[deprecated(since = "3.1.0", note = "Use UnifiedRecoveryConfig instead")]
pub struct FederationRecoveryConfig {
    pub federation_endpoint: String,
    pub member_nodes: Vec<String>,
    pub consensus_threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupRecoveryConfig {
    pub auto_backup: bool,
    pub backup_interval_secs: u64,
    pub max_backups: u32,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoverySecurityConfig {
    pub multi_factor_required: bool,
    pub audit_enabled: bool,
    pub rate_limiting_enabled: bool,
    pub secure_storage_required: bool,
}

/// Consolidated testing configuration that unifies all test types
/// Replaces: UnifiedTestingConfig, UnifiedTestingConfig, PenetrationTestConfiguration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedTestingConfig {
    pub test_type: TestingType,
    pub core: TestingCoreConfig,
    pub load: Option<LoadTestConfig>,
    pub stress: Option<StressTestConfig>,
    pub penetration: Option<PenetrationTestConfig>,
    pub performance: TestingPerformanceConfig,
    pub reporting: TestingReportingConfig,
}

// Core configuration structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingCoreConfig {
    pub enabled: bool,
    pub parallel_execution: bool,
    pub timeout_secs: u64,
    pub max_concurrent_tests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingSecurityConfig {
    pub isolation_enabled: bool,
    pub sandbox_mode: bool,
    pub data_cleanup_enabled: bool,
    pub audit_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestConfig {
    pub load_patterns: Vec<LoadPattern>,
    pub endpoints: Vec<TestEndpoint>,
    pub data_generators: Vec<DataGenerator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StressTestConfig {
    pub stress_multiplier: f64,
    pub failure_thresholds: HashMap<String, f64>,
    pub recovery_time_limit: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PenetrationTestConfig {
    pub attack_vectors: Vec<AttackVector>,
    pub security_boundaries: Vec<SecurityBoundary>,
    pub compliance_checks: Vec<ComplianceCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingPerformanceConfig {
    pub performance_thresholds: HashMap<String, f64>,
    pub memory_limits: HashMap<String, u64>,
    pub timeout_limits: HashMap<String, Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingReportingConfig {
    pub report_formats: Vec<ReportFormat>,
    pub output_directory: String,
    pub real_time_dashboard: bool,
    pub alert_on_failures: bool,
}

/// Consolidated authentication configuration 
/// Replaces: UnifiedAuthConfig, UnifiedAuthConfig, SessionConfig
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedAuthConfig {
    pub core: AuthCoreConfig,
    pub verification: AuthVerificationConfig,
    pub session: AuthSessionConfig,
    pub multi_factor: AuthMultiFactorConfig,
    pub providers: Vec<AuthProviderConfig>,
}

// Core configuration structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthCoreConfig {
    pub enabled: bool,
    pub timeout_secs: u64,
    pub max_sessions: usize,
    pub refresh_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSecurityConfig {
    pub encryption_enabled: bool,
    pub secure_cookies: bool,
    pub csrf_protection: bool,
    pub rate_limiting_enabled: bool,
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    Enhanced,
    Maximum,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Maximum,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Basic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustedContact {
    pub contact_id: String,
    pub contact_method: String,
    pub verification_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationMethod {
    Email,
    Sms,
    Push,
    InApp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FederationNode {
    pub node_id: String,
    pub endpoint: String,
    pub public_key: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupLocation {
    pub location_type: String,
    pub path: String,
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    pub retention_days: u32,
    pub max_backups: u32,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadPattern {
    pub pattern_name: String,
    pub rps_curve: Vec<f64>,
    pub duration_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestEndpoint {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataGenerator {
    pub generator_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttackVector {
    pub vector_name: String,
    pub target_components: Vec<String>,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityBoundary {
    pub boundary_name: String,
    pub protection_level: String,
    pub monitored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceCheck {
    pub check_name: String,
    pub standard: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Json,
    Html,
    Pdf,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    HardwareToken,
    Biometric,
    BackupCodes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthVerificationConfig {
    pub require_email_verification: bool,
    pub require_phone_verification: bool,
    pub verification_timeout: Duration,
    pub verification_code_length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSessionConfig {
    pub session_timeout: Duration,
    pub max_concurrent_sessions: u32,
    pub session_refresh_enabled: bool,
    pub secure_cookies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthMultiFactorConfig {
    pub enabled: bool,
    pub required_factors: u32,
    pub available_methods: Vec<MfaMethod>,
    pub backup_codes_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthProviderConfig {
    pub provider_name: String,
    pub provider_type: String,
    pub configuration: HashMap<String, String>,
    pub priority: u32,
}

// Additional supporting types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserManagementProcessorConfig {
    pub enabled: bool,
    pub max_users: usize,
    pub session_timeout_secs: u64,
    pub audit_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailNotificationSettings {
    pub enabled: bool,
    pub smtp_server: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub rate_limit_per_hour: u32,
}

/// Configuration consolidation registry
pub struct UnifiedConfigRegistry;

impl UnifiedConfigRegistry {
    /// Get list of all unified config types
    pub fn unified_config_types() -> Vec<&'static str> {
        vec![
            "UnifiedProcessorConfig",
            "UnifiedRecoveryConfig", 
            "UnifiedTestingConfig",
            "UnifiedAuthConfig",
        ]
    }
    
    /// Get deprecated config types that should be migrated
    pub fn deprecated_config_types() -> Vec<&'static str> {
        vec![
            "SecurityProcessorConfig",
            "PolicyProcessorConfig", 
            "SystemProcessorConfig",
            "KeyManagementConfig",
            "RegistryProcessorConfig",
            "RecoveryConfig",
            "SocialRecoveryConfig",
            "FederationRecoveryConfig",
            "LoadTestConfiguration",
            "StressTestConfiguration", 
            "PenetrationTestConfiguration",
            "AuthConfig",
            "VerificationConfig",
            "SessionConfig",
        ]
    }
    
    /// Get migration path for deprecated config
    pub fn get_migration_path(deprecated_config: &str) -> Option<&'static str> {
        match deprecated_config {
            "SecurityProcessorConfig" | "PolicyProcessorConfig" | "SystemProcessorConfig" 
            | "KeyManagementConfig" | "RegistryProcessorConfig" => Some("UnifiedProcessorConfig"),
            
            "RecoveryConfig" | "SocialRecoveryConfig" | "FederationRecoveryConfig" 
            => Some("UnifiedRecoveryConfig"),
            
            "LoadTestConfiguration" | "StressTestConfiguration" | "PenetrationTestConfiguration"
            => Some("UnifiedTestingConfig"),
            
            "AuthConfig" | "VerificationConfig" | "SessionConfig" 
            => Some("UnifiedAuthConfig"),
            
            _ => None,
        }
    }
} 