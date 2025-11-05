// Unified HSM Configuration System
//
// This module consolidates ALL HSM configuration patterns from across the BearDog
// ecosystem into a single, canonical, maintainable system. It replaces fragmented configs
// from multiple locations.
//
// ## Consolidation Strategy
//
// This unifies HSM configs from:
// - `beardog-tunnel/src/tunnel/hsm/types/config.rs` (HardwareHsmConfig, AndroidHsmConfig, etc.)
// - `beardog-tunnel/src/tunnel/hsm/software_hsm/mod.rs` (SoftwareHsmConfig)
// - `beardog-types/src/canonical/hsm/config.rs` (Multiple HSM configs)
// - `beardog-types/src/canonical/crypto.rs` (HsmConfig)
// - `beardog-core/src/ecosystem_integration/universal_hsm_provider.rs` (UniversalHsmConfig)
// - Various other HSM configs scattered across crates
//
// ## Architecture Principles
//
// - **Single Source of Truth**: All HSM config in one canonical place
// - **Platform Organization**: Logical grouping by HSM platform/type
// - **Zero Fragmentation**: No duplicate HSM config types
// - **Extensible Design**: Easy addition of new HSM platforms
// - **Security First**: Comprehensive security configuration options

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Domain-specific HSM configuration modules
/// Cloud module
pub mod cloud;
/// Core module
/// Core functionality
/// Core functionality
pub mod core;
/// Discovery module
pub mod discovery;
/// Hardware module
pub mod hardware;
/// Mobile module
pub mod mobile;
pub mod performance;
/// Security module
pub mod security;
/// Software module
pub mod software;

// Re-export all configuration types for easy access
pub use cloud::*;
pub use core::*;
pub use discovery::*;
pub use hardware::*;
pub use mobile::*;
pub use performance::*;
pub use security::*;
pub use software::*;

///
/// This configuration consolidates all HSM concerns into a unified, hierarchical
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHsmConfig {
    /// **GLOBAL SETTINGS**
    /// Whether HSM functionality is globally enabled
    /// Whether feature is enabled
    pub enabled: bool,
    pub default_timeout: Duration,
    /// Size of the HSM connection pool
    /// Number of `connection_pool_size`
    pub connection_pool_size: u32,
    /// The retry policy value
    pub retry_policy: HsmRetryPolicy,

    /// **HSM PLATFORMS**
    /// Hardware HSM configuration (PKCS#11, network-attached)
    /// The hardware value
    pub hardware: UnifiedHardwareHsmConfig,
    /// Software HSM configuration (in-memory, file-based)
    /// The software value
    pub software: UnifiedSoftwareHsmConfig,
    /// Mobile HSM configuration (Android Keystore, iOS Secure Enclave)
    /// The mobile value
    pub mobile: UnifiedMobileHsmConfig,
    /// Cloud HSM configuration (`universal_cloud` `universal_hsm`, `universal_cloud` Dedicated HSM, etc.)
    /// The cloud value
    pub cloud: UnifiedCloudHsmConfig,

    /// **DISCOVERY AND MANAGEMENT**
    /// HSM discovery and auto-configuration settings
    /// The discovery value
    pub discovery: UnifiedHsmDiscoveryConfig,
    /// HSM tier management and failover configuration
    /// The tier management value
    pub tier_management: HsmTierManagementConfig,

    /// **SECURITY CONFIGURATION**
    /// HSM security policies and access controls
    /// The security value
    pub security: UnifiedHsmSecurityConfig,
    /// Compliance and regulatory configuration
    /// The compliance value
    pub compliance: HsmComplianceConfig,

    /// **PERFORMANCE AND MONITORING**
    pub performance: UnifiedHsmPerformanceConfig,
    /// HSM monitoring and metrics configuration
    /// The monitoring value
    pub monitoring: HsmMonitoringConfig,
    /// The health checks value
    pub health_checks: HsmHealthCheckConfig,

    /// **INTEGRATION SETTINGS**
    /// Integration with external systems and protocols
    /// The integration value
    pub integration: HsmIntegrationConfig,
    /// Backup and disaster recovery configuration
    /// The backup value
    pub backup: HsmBackupConfig,
}

impl Default for UnifiedHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_DEFAULT_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
            connection_pool_size: std::env::var("BEARDOG_HSM_CONNECTION_POOL_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            retry_policy: HsmRetryPolicy::default(),
            hardware: UnifiedHardwareHsmConfig::default(),
            software: UnifiedSoftwareHsmConfig::default(),
            mobile: UnifiedMobileHsmConfig::default(),
            cloud: UnifiedCloudHsmConfig::default(),
            discovery: UnifiedHsmDiscoveryConfig::default(),
            tier_management: HsmTierManagementConfig::default(),
            security: UnifiedHsmSecurityConfig::default(),
            compliance: HsmComplianceConfig::default(),
            performance: UnifiedHsmPerformanceConfig::default(),
            monitoring: HsmMonitoringConfig::default(),
            health_checks: HsmHealthCheckConfig::default(),
            integration: HsmIntegrationConfig::default(),
            backup: HsmBackupConfig::default(),
        }
    }
}

/// HSM retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmRetryPolicy {
    /// Whether retry functionality is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Maximum number of retry attempts
    /// Number of `max_retries`
    pub max_retries: u32,
    /// The initial delay value
    pub initial_delay: Duration,
    /// Maximum delay between retries
    /// The max delay value
    pub max_delay: Duration,
    /// The backoff multiplier value
    pub backoff_multiplier: f64,
    /// Whether to retry on timeout errors
    pub retry_on_timeout: bool,
    /// Whether to retry on connection errors
    /// Whether `retry_on_connection_error` is enabled
    pub retry_on_connection_error: bool,
}

impl Default for HsmRetryPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: std::env::var("BEARDOG_HSM_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var("BEARDOG_HSM_RETRY_INITIAL_DELAY_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            max_delay: Duration::from_secs(
                std::env::var("BEARDOG_HSM_RETRY_MAX_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            backoff_multiplier: 2.0,
            retry_on_timeout: true,
            retry_on_connection_error: true,
        }
    }
}

/// HSM tier management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmTierManagementConfig {
    /// Whether HSM tier management is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether to automatically assign HSMs to tiers
    /// Whether `auto_tier_assignment` is enabled
    pub auto_tier_assignment: bool,
    /// The tier evaluation interval value
    pub tier_evaluation_interval: Duration,
    /// Mapping of tier criteria
    pub tier_criteria: HashMap<String, TierCriteria>,
}

impl Default for HsmTierManagementConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_tier_assignment: true,
            tier_evaluation_interval: Duration::from_secs(
                std::env::var("BEARDOG_HSM_TIER_EVALUATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(300),
            ),
            tier_criteria: HashMap::new(),
        }
    }
}

/// Tier assignment criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierCriteria {
    /// The security level value
    pub security_level: SecurityLevel,
    pub performance_requirements: PerformanceRequirements,
    /// Collection of compliance requirements
    pub compliance_requirements: Vec<ComplianceStandard>,
    /// The availability requirements value
    pub availability_requirements: AvailabilityRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    High,
    Critical,
    UltraHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Minimum operations per second required
    /// Number of `min_operations_per_second`
    pub min_operations_per_second: u32,
    /// Maximum acceptable latency in milliseconds
    /// Number of `max_latency_ms`
    pub max_latency_ms: u32,
    /// Minimum availability percentage required (0.0-100.0)
    /// The min availability percent value
    pub min_availability_percent: f64,
}

/// Compliance standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStandard {
    /// FIPS 140-2 compliance with specified level
    FIPS140_2(FipsLevel),
    /// Common Criteria compliance with evaluation level
    CommonCriteria(CcEvaluationLevel),
    /// NIST compliance standards
    NIST,
    ISO27001,
    SOC2,
    PciDss,
    HIPAA,
    /// Custom compliance standard with description
    Custom(String),
}

/// FIPS 140-2 levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FipsLevel {
    /// FIPS 140-2 Level 1 (lowest security)
    Level1,
    /// FIPS 140-2 Level 2 (software and firmware security)
    Level2,
    /// FIPS 140-2 Level 3 (tamper-evident physical security)
    Level3,
    /// FIPS 140-2 Level 4 (tamper-active physical security)
    Level4,
}

/// Common Criteria evaluation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CcEvaluationLevel {
    /// Evaluation Assurance Level 1 (functionally tested)
    EAL1,
    /// Evaluation Assurance Level 2 (structurally tested)
    EAL2,
    /// Evaluation Assurance Level 3 (methodically tested and checked)
    EAL3,
    /// Evaluation Assurance Level 4 (methodically designed, tested, and reviewed)
    EAL4,
    EAL5,
    EAL6,
    EAL7,
}

/// Availability requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequirements {
    /// Required uptime SLA percentage (0.0-100.0)
    pub uptime_sla_percent: f64,
    /// Maximum allowable downtime per month
    pub max_downtime_per_month: Duration,
    /// The disaster recovery rto value
    pub disaster_recovery_rto: Duration,
    /// The disaster recovery rpo value
    pub disaster_recovery_rpo: Duration,
}

/// HSM compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmComplianceConfig {
    /// Whether HSM compliance checking is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// List of required compliance standards
    /// Collection of required standards
    pub required_standards: Vec<ComplianceStandard>,
    /// Whether compliance audit logging is enabled
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Whether compliance reporting is enabled
    /// Whether `compliance_reporting` is enabled
    pub compliance_reporting: bool,
    /// Whether automated compliance checks are enabled
    /// Whether `automated_compliance_checks` is enabled
    pub automated_compliance_checks: bool,
}

impl Default for HsmComplianceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            required_standards: vec![
                ComplianceStandard::FIPS140_2(FipsLevel::Level2),
                ComplianceStandard::CommonCriteria(CcEvaluationLevel::EAL4),
            ],
            audit_logging: true,
            compliance_reporting: true,
            automated_compliance_checks: true,
        }
    }
}

/// HSM monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmMonitoringConfig {
    /// Whether HSM monitoring is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether to collect HSM metrics
    /// Whether `metrics_collection` is enabled
    pub metrics_collection: bool,
    pub performance_monitoring: bool,
    /// Whether to monitor HSM security events
    /// Whether `security_monitoring` is enabled
    pub security_monitoring: bool,
    /// Whether to track HSM usage statistics
    /// Whether `usage_tracking` is enabled
    pub usage_tracking: bool,
    /// HSM alerting configuration
    /// The alerting value
    pub alerting: HsmAlertingConfig,
}

impl Default for HsmMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_collection: true,
            performance_monitoring: true,
            security_monitoring: true,
            usage_tracking: true,
            alerting: HsmAlertingConfig::default(),
        }
    }
}

/// HSM alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAlertingConfig {
    /// Whether HSM alerting is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether to alert on HSM failures
    /// Whether `alert_on_failure` is enabled
    pub alert_on_failure: bool,
    pub alert_on_degraded_performance: bool,
    /// Whether to alert on HSM security events
    /// Whether `alert_on_security_events` is enabled
    pub alert_on_security_events: bool,
    /// List of alert channels (email, slack, etc.)
    /// Collection of alert channels
    pub alert_channels: Vec<String>,
}

impl Default for HsmAlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alert_on_failure: true,
            alert_on_degraded_performance: true,
            alert_on_security_events: true,
            alert_channels: Vec::new(),
        }
    }
}

/// HSM health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthCheckConfig {
    /// Whether HSM health checks are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Interval between health checks
    /// The check interval value
    pub check_interval: Duration,
    pub timeout: Duration,
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Number of successes needed to mark as healthy again
    /// Number of `recovery_threshold`
    pub recovery_threshold: u32,
    /// Collection of health check types
    pub health_check_types: Vec<HealthCheckType>,
}

impl Default for HsmHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_HSM_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
            ),
            failure_threshold: std::env::var("BEARDOG_HSM_HEALTH_FAILURE_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            recovery_threshold: std::env::var("BEARDOG_HSM_HEALTH_RECOVERY_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2),
            health_check_types: vec![
                HealthCheckType::Connectivity,
                HealthCheckType::Authentication,
                HealthCheckType::BasicOperations,
            ],
        }
    }
}

/// Health check types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of health check
pub enum HealthCheckType {
    /// Check HSM connectivity and network access
    Connectivity,
    /// Check HSM authentication and authorization
    Authentication,
    /// Check basic HSM operations (encrypt/decrypt)
    BasicOperations,
    PerformanceBenchmark,
    /// Validate HSM security configurations
    SecurityValidation,
    /// Check HSM compliance status
    ComplianceCheck,
}

/// HSM integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmIntegrationConfig {
    /// Whether HSM integration is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The load balancing value
    pub load_balancing: LoadBalancingConfig,
    /// The failover value
    pub failover: FailoverConfig,
    /// Session management configuration
    /// The session management value
    pub session_management: SessionManagementConfig,
}

impl Default for HsmIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            load_balancing: LoadBalancingConfig::default(),
            failover: FailoverConfig::default(),
            session_management: SessionManagementConfig::default(),
        }
    }
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Whether load balancing is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Load balancing strategy to use
    /// The strategy value
    pub strategy: LoadBalancingStrategy,
    /// Whether `health_check` is enabled
    pub health_check_enabled: bool,
    /// Whether `weight_adjustment` is enabled
    pub weight_adjustment: bool,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check_enabled: true,
            weight_adjustment: true,
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Distribute requests in round-robin fashion
    RoundRobin,
    /// Route to HSM with least active connections
    LeastConnections,
    /// Round-robin with weighted distribution
    WeightedRoundRobin,
    PerformanceBased,
    /// Random distribution of requests
    Random,
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Whether failover is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether to automatically failover on HSM failure
    /// Whether `automatic_failover` is enabled
    pub automatic_failover: bool,
    pub failover_timeout: Duration,
    /// Whether to automatically failback when primary recovers
    /// Whether failback is enabled
    pub failback_enabled: bool,
    /// The failback delay value
    pub failback_delay: Duration,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            automatic_failover: true,
            failover_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_FAILOVER_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
            failback_enabled: true,
            failback_delay: Duration::from_secs(
                std::env::var("BEARDOG_HSM_FAILBACK_DELAY_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(60),
            ),
        }
    }
}

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManagementConfig {
    /// Whether session management is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    pub session_timeout: Duration,
    /// Maximum number of concurrent HSM sessions
    /// Number of `max_concurrent_sessions`
    pub max_concurrent_sessions: u32,
    /// Whether `session_pooling` is enabled
    pub session_pooling: bool,
    /// Whether to encrypt HSM session data
    /// Whether `session_encryption` is enabled
    pub session_encryption: bool,
}

impl Default for SessionManagementConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            session_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_SESSION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
            ),
            max_concurrent_sessions: std::env::var("BEARDOG_HSM_MAX_CONCURRENT_SESSIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            session_pooling: true,
            session_encryption: true,
        }
    }
}

/// HSM backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmBackupConfig {
    /// Whether HSM backup is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Interval between HSM backups
    /// The backup interval value
    pub backup_interval: Duration,
    /// How long to retain HSM backups
    /// The retention period value
    pub retention_period: Duration,
    /// Whether to encrypt HSM backup data
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Whether to store backups remotely
    /// Whether `remote_backup` is enabled
    pub remote_backup: bool,
    /// Whether to verify backup integrity
    /// Whether `backup_verification` is enabled
    pub backup_verification: bool,
}

impl Default for HsmBackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backup_interval: Duration::from_secs(
                std::env::var("BEARDOG_HSM_BACKUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(86400), // 24 hours default
            ),
            retention_period: Duration::from_secs(
                std::env::var("BEARDOG_HSM_BACKUP_RETENTION_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(86400 * 30), // 30 days default
            ),
            encryption_enabled: true,
            remote_backup: false,
            backup_verification: true,
        }
    }
}

pub trait HsmConfigValidation {
    /// Validate the HSM configuration
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError>;

    /// Check configuration compatibility
    /// Checks if compatible with
    fn is_compatible_with(&self, other_version: u32) -> bool;
}

impl HsmConfigValidation for UnifiedHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // Validate timeout values
        if self.default_timeout.is_zero() {
            return Err(BearDogError::business(
                "Default timeout must be greater than zero".to_string(),
            ));
        }

        if self.connection_pool_size == 0 {
            return Err(BearDogError::business(
                "Connection pool size must be greater than zero".to_string(),
            ));
        }

        // Validate retry policy
        if self.retry_policy.enabled && self.retry_policy.max_retries == 0 {
            return Err(BearDogError::business(
                "Max retries must be greater than zero when retry policy is enabled".to_string(),
            ));
        }

        // Validate health check configuration
        if self.health_checks.enabled {
            if self.health_checks.check_interval.is_zero() {
                return Err(BearDogError::business(
                    "Health check interval must be greater than zero".to_string(),
                ));
            }

            if self.health_checks.timeout.is_zero() {
                return Err(BearDogError::business(
                    "Health check timeout must be greater than zero".to_string(),
                ));
            }

            if self.health_checks.failure_threshold == 0 {
                return Err(BearDogError::business(
                    "Health check failure threshold must be greater than zero".to_string(),
                ));
            }
        }

        // Validate sub-configurations
        self.hardware.validate()?;
        self.software.validate()?;
        self.mobile.validate()?;
        self.cloud.validate()?;
        self.discovery.validate()?;
        self.security.validate()?;
        self.performance.validate()?;

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
