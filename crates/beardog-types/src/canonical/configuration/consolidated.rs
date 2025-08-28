//! Consolidated Configuration Module
//!
//! This is the **SINGLE SOURCE OF TRUTH** for all BearDog configuration.
//! All other configuration modules should be deprecated in favor of this unified approach.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **THE** canonical BearDog configuration struct
/// Replaces: BearDogCanonicalConfig, UnifiedDeploymentConfig, BiomeConfig, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BearDogCanonicalConfig {
    /// Core application configuration
    pub app: AppConfig,

    /// Network and connectivity settings
    pub network: NetworkConfig,

    /// Security and authentication settings
    pub security: SecurityConfig,

    /// HSM and cryptographic configuration
    pub hsm: HsmConfig,

    /// Database and storage configuration
    pub database: DatabaseConfig,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Workflow engine configuration
    pub workflows: WorkflowConfig,

    /// Production and deployment settings
    pub production: ProductionConfig,

    /// Performance tuning configuration
    pub performance: PerformanceConfig,

    /// Configuration management settings
    pub config_manager: ConfigManagerConfig,

    /// Service discovery configuration
    pub discovery: DiscoveryConfig,

    /// Compliance and audit configuration
    pub compliance: ComplianceConfig,

    /// Tunnel and VPN configuration
    pub tunnel: TunnelConfig,

    /// Integration with external systems
    pub integration: IntegrationConfig,

    /// Node registry configuration
    pub node_registry: NodeRegistryConfig,

    /// Platform-specific settings
    pub platform: PlatformConfig,

    /// Load testing configuration
    pub load_testing: LoadTestingConfig,

    /// Secrets management configuration
    pub secrets: SecretsConfig,

    /// Testing and development configuration
    pub testing: TestingConfig,
}

// Core Configuration Structs

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub log_level: LogLevel,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub features: HashMap<String, bool>,
    pub rollout_percentages: HashMap<String, f64>,
    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub enable_tls: bool,
    pub circuit_breaker: crate::canonical::providers::CircuitBreakerConfig,
    pub load_balancing: LoadBalancingConfig,
    pub bandwidth_mbps: u32,
    pub port_ranges: Vec<PortRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    pub authentication: AuthConfig,
    pub authorization: AuthorizationConfig,
    pub encryption: EncryptionConfig,
    pub rate_limiting: RateLimitConfig,
    pub session: SessionConfig,
    pub mfa: MfaConfig,
    pub audit: AuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmConfig {
    pub provider_type: HsmProvider,
    pub connection_config: HsmConnectionConfig,
    pub key_management: KeyManagementConfig,
    pub performance: HsmPerformanceConfig,
    pub monitoring: HsmMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub pool_size: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub migration: MigrationConfig,
    pub backup: BackupConfig,
    pub encryption: DatabaseEncryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub prometheus: PrometheusConfig,
    pub logging: LoggingConfig,
    pub tracing: TracingConfig,
    pub alerts: AlertConfig,
    pub metrics_collection: MetricCollectionConfig,
    pub health_checks: HealthCheckConfig,
    pub security_monitoring: SecurityMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {
    pub engine_type: WorkflowEngineType,
    pub storage_backend: WorkflowStorageBackend,
    pub retry_config: WorkflowRetryConfig,
    pub notification: WorkflowNotificationConfig,
    pub monitoring: WorkflowMonitoringConfig,
    pub approval: ApprovalConfig,
    pub retention: RetentionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionConfig {
    pub cluster: ClusterConfig,
    pub scaling: ScalingConfig,
    pub health_monitoring: HealthMonitoringConfig,
    pub resource_limits: ResourceLimitsConfig,
    pub maintenance: MaintenanceConfig,
    pub disaster_recovery: DisasterRecoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    pub cpu: CpuConfig,
    pub memory: MemoryConfig,
    pub network: NetworkPerformanceConfig,
    pub storage: StoragePerformanceConfig,
    pub caching: CachingConfig,
    pub optimization: OptimizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigManagerConfig {
    pub sources: ConfigSourcesConfig,
    pub validation: ConfigValidationConfig,
    pub caching: ConfigCachingConfig,
    pub reloading: ConfigReloadingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    pub enabled: bool,
    pub discovery_interval: Duration,
    pub service_timeout: Duration,
    pub max_services: usize,
    pub auto_registration: bool,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceConfig {
    pub enabled_standards: Vec<ComplianceStandard>,
    pub audit_retention_days: u32,
    pub reporting_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_retries: u32,
    pub parallel_execution: bool,
    pub max_concurrent_tests: usize,
    pub enable_performance_monitoring: bool,
    pub enable_auto_cleanup: bool,
    pub enable_verbose_logging: bool,
    pub test_data_dir: String,
    pub metrics: TestMetricsConfig,
    pub integration: IntegrationTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMetricsConfig {
    pub collect_metrics: bool,
    pub metrics_interval: Duration,
    pub store_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationTestConfig {
    pub enabled: bool,
    pub test_environment: String,
    pub cleanup_after_test: bool,
    pub reporting: ComplianceReportingConfig,
    pub monitoring_interval: Duration,
    pub audit_retention: Duration,
    pub data_sovereignty: DataSovereigntyConfig,
    pub privacy_audit: PrivacyAuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TunnelConfig {
    pub performance: TunnelPerformanceConfig,
    pub key_management: TunnelKeyManagementConfig,
    pub gaming: GamingOptimizationConfig,
    pub genetic_healing: GeneticHealingConfig,
    pub monitoring: TunnelMonitoringConfig,
    pub alert_thresholds: TunnelAlertThresholds,
    pub prefer_human_entropy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationConfig {
    pub external_services: HashMap<String, ExternalServiceConfig>,
    pub adapters: HashMap<String, AdapterConfig>,
    pub timeouts: TimeoutConfig,
    pub retry_policy: RetryPolicy,
    pub circuit_breaker: crate::canonical::providers::CircuitBreakerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeRegistryConfig {
    pub enabled: bool,
    pub registry_url: String,
    pub heartbeat_interval: Duration,
    pub node_timeout: Duration,
    pub max_nodes: usize,
    pub health_check_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformConfig {
    pub platform_type: PlatformType,
    pub resource_limits: ResourceLimits,
    pub security_level: SecurityLevel,
    pub optimization_flags: Vec<String>,
    pub compatibility_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestingConfig {
    pub enabled: bool,
    pub concurrent_users: usize,
    pub duration: Duration,
    pub ramp_up_time: Duration,
    pub target_rps: u32,
    pub scenarios: Vec<LoadTestScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretsConfig {
    pub provider: SecretsProvider,
    pub encryption_key: String,
    pub rotation_interval: Duration,
    pub backup_enabled: bool,
    pub audit_enabled: bool,
}

// Supporting Enums and Types

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Environment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HsmProvider {
    #[default]
    Software,
    Hardware,
    Cloud,
    AndroidStrongBox,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorkflowEngineType {
    #[default]
    Simple,
    Advanced,
    Distributed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorkflowStorageBackend {
    #[default]
    Memory,
    Database,
    FileSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ComplianceStandard {
    #[default]
    Gdpr,
    Sox,
    PciDss,
    Hipaa,
    IsoIec27001,
    Nist,
    FedRamp,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PlatformType {
    #[default]
    Linux,
    Windows,
    MacOS,
    Android,
    Ios,
    Embedded,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    #[default]
    Standard,
    High,
    Critical,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecretsProvider {
    #[default]
    Memory,
    File,
    Vault,
    Cloud,
}

// Supporting Configuration Structs (simplified for brevity)

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub method: String,
    pub timeout: Duration,
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    pub enabled: bool,
    pub policy_engine: String,
    pub cache_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub rotation_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    pub timeout: Duration,
    pub max_sessions: u32,
    pub secure_cookies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MfaConfig {
    pub enabled: bool,
    pub methods: Vec<MfaMethod>,
    pub backup_codes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_level: LogLevel,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub strategy: String,
    pub health_check_interval: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

// Authentication and Processing Types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AuthType {
    #[default]
    Session,
    Token,
    Certificate,
    Biometric,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProcessorType {
    Security,
    #[default]
    System,
    Policy,
    Registry,
    KeyManagement,
    UserManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RecoveryType {
    Social,
    Federation,
    #[default]
    Backup,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TestingType {
    #[default]
    Unit,
    Integration,
    Performance,
    Security,
    Chaos,
}

// Unified Configuration Structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedAuthConfig {
    pub auth_type: AuthType,
    pub core: AuthCoreConfig,
    pub security: AuthSecurityConfig,
    pub session: SessionConfig,
    pub token: TokenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthCoreConfig {
    pub enabled: bool,
    pub timeout_secs: u64,
    pub max_attempts: u32,
    pub retry_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSecurityConfig {
    pub audit_enabled: bool,
    pub encryption_enabled: bool,
    pub secure_cookies: bool,
    pub csrf_protection: bool,
}

// SessionConfig already defined above with different fields
// Using the existing SessionConfig with timeout, max_sessions, secure_cookies

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionManagementConfig {
    pub concurrent_session_limit: u32,
    pub idle_timeout_minutes: u32,
    pub absolute_timeout_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenConfig {
    pub expiry_minutes: u32,
    pub refresh_enabled: bool,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedProcessorConfig {
    pub processor_type: ProcessorType,
    pub core: ProcessorCoreConfig,
    pub security: ProcessorSecurityConfig,
    pub performance: ProcessorPerformanceConfig,
    pub monitoring: ProcessorMonitoringConfig,
    pub specialized: ProcessorSpecializedConfig,
}

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
    pub encryption_required: bool,
    pub access_control_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorPerformanceConfig {
    pub batch_size: usize,
    pub parallel_processing: bool,
    pub cache_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorMonitoringConfig {
    pub metrics_enabled: bool,
    pub health_check_interval_secs: u64,
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorSpecializedConfig {
    pub custom_settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedRecoveryConfig {
    pub recovery_type: RecoveryType,
    pub core: RecoveryCoreConfig,
    pub security: RecoverySecurityConfig,
    pub backup: BackupRecoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoveryCoreConfig {
    pub enabled: bool,
    pub max_recovery_attempts: u32,
    pub recovery_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoverySecurityConfig {
    pub verification_required: bool,
    pub multi_factor_auth: bool,
    pub audit_recovery_attempts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupRecoveryConfig {
    pub backup_interval_hours: u32,
    pub retention_days: u32,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedTestingConfig {
    pub testing_type: TestingType,
    pub core: TestingCoreConfig,
    pub security: TestingSecurityConfig,
    pub performance: TestingPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingCoreConfig {
    pub enabled: bool,
    pub test_data_cleanup: bool,
    pub mock_external_services: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingSecurityConfig {
    pub security_tests_enabled: bool,
    pub penetration_testing: bool,
    pub vulnerability_scanning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingPerformanceConfig {
    pub load_testing_enabled: bool,
    pub stress_testing_enabled: bool,
    pub benchmark_comparisons: bool,
}

// Email notification settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailNotificationSettings {
    pub enabled: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub use_tls: bool,
    pub from_address: String,
    pub admin_emails: Vec<String>,
}

// User Management Processor Config
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserManagementProcessorConfig {
    pub user_registration_enabled: bool,
    pub password_policy: PasswordPolicy,
    pub session_management: SessionManagementConfig,
    pub role_based_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PasswordPolicy {
    pub min_length: u32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub expiry_days: u32,
}

// SessionManagementConfig already defined above

// Additional supporting structs would be defined here...
// (Simplified for brevity - in practice, each would have full definitions)

macro_rules! impl_default_config {
    ($($struct_name:ident),*) => {
        $(
            #[derive(Debug, Clone, Serialize, Deserialize, Default)]
            pub struct $struct_name {
                pub enabled: bool,
            }
        )*
    };
}

// Generate default implementations for remaining config structs
impl_default_config!(
    HsmConnectionConfig,
    KeyManagementConfig,
    HsmPerformanceConfig,
    HsmMonitoringConfig,
    MigrationConfig,
    BackupConfig,
    DatabaseEncryptionConfig,
    PrometheusConfig,
    LoggingConfig,
    TracingConfig,
    AlertConfig,
    MetricCollectionConfig,
    HealthCheckConfig,
    SecurityMonitoringConfig,
    WorkflowRetryConfig,
    WorkflowNotificationConfig,
    WorkflowMonitoringConfig,
    ApprovalConfig,
    RetentionConfig,
    ClusterConfig,
    ScalingConfig,
    HealthMonitoringConfig,
    ResourceLimitsConfig,
    MaintenanceConfig,
    DisasterRecoveryConfig,
    CpuConfig,
    MemoryConfig,
    NetworkPerformanceConfig,
    StoragePerformanceConfig,
    CachingConfig,
    OptimizationConfig,
    ConfigSourcesConfig,
    ConfigValidationConfig,
    ConfigCachingConfig,
    ConfigReloadingConfig,
    ComplianceReportingConfig,
    DataSovereigntyConfig,
    PrivacyAuditConfig,
    TunnelPerformanceConfig,
    TunnelKeyManagementConfig,
    GamingOptimizationConfig,
    GeneticHealingConfig,
    TunnelMonitoringConfig,
    TunnelAlertThresholds,
    ExternalServiceConfig,
    AdapterConfig,
    TimeoutConfig,
    RetryPolicy,
    ResourceLimits,
    LoadTestScenario,
    MfaMethod
);

// Configuration Builder and Validator

#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: BearDogCanonicalConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: BearDogCanonicalConfig::default(),
        }
    }

    pub fn app(mut self, app: AppConfig) -> Self {
        self.config.app = app;
        self
    }

    pub fn network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    pub fn build(self) -> BearDogCanonicalConfig {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &BearDogCanonicalConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if config.app.name.is_empty() {
            errors.push("App name cannot be empty".to_string());
        }

        if config.network.port == 0 {
            errors.push("Network port must be specified".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigMigrator;

impl ConfigMigrator {
    pub fn migrate_from_legacy() -> BearDogCanonicalConfig {
        // Migration logic would be implemented here
        BearDogCanonicalConfig::default()
    }
}
