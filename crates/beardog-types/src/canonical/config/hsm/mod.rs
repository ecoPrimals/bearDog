// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified HSM configuration for Beardog (hardware, software, mobile, cloud).
//!
//! Consolidates fragmented HSM settings from tunnel, core, and adapter crates into one
//! serde-friendly tree with validation hooks ([`HsmConfigValidation`](crate::canonical::config::hsm::HsmConfigValidation)).

use crate::canonical::traits::RetryStrategy;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Domain-specific HSM configuration modules
/// Cloud module
pub mod cloud;
/// Core module
/// Core functionality
pub mod core;
/// Discovery module
pub mod discovery;
/// Hardware module
pub mod hardware;
/// Mobile module
pub mod mobile;
/// Latency, throughput, and benchmarking knobs applied across HSM backends.
pub mod performance;
/// Security module
pub mod security;
/// Software module
pub mod software;

/// Tiering, compliance, monitoring, integration, and backup knobs not covered by platform submodules.
mod operational_config;

// Re-export all configuration types for easy access
pub use cloud::UnifiedCloudHsmConfig;
pub use core::CoreHsmConfig;
pub use discovery::UnifiedHsmDiscoveryConfig;
pub use hardware::UnifiedHardwareHsmConfig;
pub use mobile::UnifiedMobileHsmConfig;
pub use operational_config::{
    AvailabilityRequirements, CcEvaluationLevel, ComplianceStandard, FailoverConfig, FipsLevel,
    HealthCheckType, HsmAlertingConfig, HsmBackupConfig, HsmComplianceConfig, HsmHealthCheckConfig,
    HsmIntegrationConfig, HsmMonitoringConfig, HsmTierManagementConfig, LoadBalancingConfig,
    LoadBalancingStrategy, PerformanceRequirements, SecurityLevel, SessionManagementConfig,
    TierCriteria,
};
pub use performance::UnifiedHsmPerformanceConfig;
pub use security::UnifiedHsmSecurityConfig;
pub use software::UnifiedSoftwareHsmConfig;

/// Root HSM configuration: platforms, discovery, security, monitoring, and DR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHsmConfig {
    /// **GLOBAL SETTINGS**
    /// Whether HSM functionality is globally enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Default RPC/session timeout for HSM operations. **Default:** `30s`, overridable via `BEARDOG_HSM_DEFAULT_TIMEOUT_SECS`.
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
    /// Cloud HSM configuration (discovered via capability-based providers)
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
    /// Provider throughput limits, batching, and cost controls. **Default:** [`UnifiedHsmPerformanceConfig::default()`].
    pub performance: UnifiedHsmPerformanceConfig,
    /// HSM monitoring and metrics configuration
    /// The monitoring value
    pub monitoring: HsmMonitoringConfig,
    /// Periodic probes and thresholds marking an HSM unhealthy. **Default:** [`HsmHealthCheckConfig::default()`].
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

// Implement RetryStrategy trait for HSM retry policy
impl RetryStrategy for HsmRetryPolicy {
    fn max_attempts(&self) -> u32 {
        self.max_retries
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Exponential backoff with HSM-specific multiplier
        // Note: Precision loss is acceptable for delay calculations (not cryptographic)
        #[expect(
            clippy::cast_possible_truncation,
            reason = "delay millis fit u64 for scheduling"
        )]
        #[expect(
            clippy::cast_precision_loss,
            reason = "acceptable imprecision for non-cryptographic backoff math"
        )]
        #[expect(
            clippy::cast_sign_loss,
            reason = "delay clamped non-negative before f64 to u64"
        )]
        #[expect(
            clippy::cast_possible_wrap,
            reason = "retry attempt capped at 30 for powi exponent"
        )]
        let delay_ms = {
            // Clamp to u64::MAX millis to avoid overflow (still ~584 million years)
            let initial_ms = self.initial_delay.as_millis().min(u128::from(u64::MAX)) as f64;
            let max_ms = self.max_delay.as_millis().min(u128::from(u64::MAX)) as u64;
            let computed = initial_ms * self.backoff_multiplier.powi(attempt.min(30) as i32);
            computed.min(max_ms as f64).max(0.0) as u64
        };
        Duration::from_millis(delay_ms)
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // HSM-specific: only retry if enabled and based on error type
        if !self.enabled {
            return false;
        }

        let error_str = error.to_string().to_lowercase();

        // Check for timeout errors
        if self.retry_on_timeout
            && (error_str.contains("timeout") || error_str.contains("timed out"))
        {
            return true;
        }

        // Check for connection errors
        if self.retry_on_connection_error
            && (error_str.contains("connection")
                || error_str.contains("network")
                || error_str.contains("unreachable")
                || error_str.contains("refused"))
        {
            return true;
        }

        // Don't retry on HSM authentication/authorization failures
        if error_str.contains("auth")
            || error_str.contains("permission")
            || error_str.contains("forbidden")
            || error_str.contains("unauthorized")
        {
            return false;
        }

        // Retry on other HSM errors by default
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        !self.enabled || attempts >= self.max_retries
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        if !self.enabled {
            return Duration::from_secs(0);
        }
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

/// Validates canonical HSM configuration trees before use in production paths.
pub trait HsmConfigValidation {
    /// Validate the HSM configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if timeouts, pool size, retry policy, or health check settings are invalid.
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
