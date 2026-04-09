// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated Security Configuration Domain
//!
//! This module consolidates ALL security-related configuration structs across the `BearDog`
//! ecosystem into a single, unified security configuration system. It eliminates fragmentation
//! by providing a canonical security configuration that replaces 15+ scattered security configs.
//!
//! ## 🎯 **Consolidation Impact**
//!
//! This module unifies and replaces:
//! - `SecurityProviderConfig` (beardog-security)
//! - `RateLimitConfig` (beardog-security)
//! - `AuditConfig` (beardog-security)
//! - `EncryptionConfig` (beardog-security)
//! - `SafeCryptoConfig` (beardog-security)
//! - `GeneticRenewalConfig` (beardog-security)
//! - `ContextAwareKeyConfig` (beardog-security)
//! - `MembershipConfig` (beardog-security)
//! - `AutoEvolutionConfig` (beardog-security)
//! - `TrustComputationConfig` (beardog-security)
//! - `CrossNodeAuthConfig` (beardog-auth)
//! - `ConsensusConfig` (beardog-auth)
//! - `AuthConfig` (beardog-auth)
//! - Plus additional scattered security configurations

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Submodules
pub mod advanced;
pub mod auth;
pub mod compliance;
pub mod crypto;
pub mod monitoring;
pub mod threat;

// Re-export all public types for convenience
pub use advanced::{
    EcosystemMembershipConfiguration, EvaluationConfiguration, GeneticSecurityConfiguration,
    GeneticsIntegrationConfiguration, TrustComputationConfiguration, TrustDecayConfiguration,
};
pub use auth::{
    AccessControlConfiguration, AuthenticationConfiguration, AuthorizationConfiguration,
    AutoEvolutionConfiguration, ConsensusConfiguration, HealthMonitoringConfiguration,
    LegacyIntegrationConfiguration,
};
pub use compliance::{
    ComplianceReportingConfiguration, ComplianceValidationConfiguration,
    DataSovereigntyConfiguration, SecurityComplianceConfiguration,
    SovereigntyValidationConfiguration,
};
pub use crypto::{
    CryptoProviderConfiguration, EncryptionConfiguration, GeneticRenewalConfiguration,
    KeyManagementConfiguration,
};
pub use monitoring::{
    AuditConfiguration, SecurityAlertConfiguration, SecurityMonitoringConfiguration,
};
pub use threat::ThreatResponseConfiguration;

// Import canonical threat detection config
use super::threat::CanonicalThreatDetectionConfig;

/// **CONSOLIDATED SECURITY CONFIGURATION** - Single source of truth for all security settings
///
/// This structure consolidates all security-related configurations across the `BearDog` ecosystem,
/// eliminating fragmentation and providing a unified security configuration interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedSecurityConfiguration {
    /// **AUTHENTICATION & AUTHORIZATION**
    /// Identity verification (MFA, sessions). **Default:** [`AuthenticationConfiguration::default()`].
    pub authentication: AuthenticationConfiguration,
    /// RBAC/ABAC policy sources. **Default:** `AuthorizationConfiguration::default()`.
    pub authorization: AuthorizationConfiguration,
    /// Fine-grained resource ACLs. **Default:** `AccessControlConfiguration::default()`.
    pub access_control: AccessControlConfiguration,

    /// **CRYPTOGRAPHIC OPERATIONS**
    /// Data-protection algorithms and key rotation. **Default:** `EncryptionConfiguration::default()`.
    pub encryption: EncryptionConfiguration,
    /// Lifecycle for signing/encryption keys. **Default:** `KeyManagementConfiguration::default()`.
    pub key_management: KeyManagementConfiguration,
    /// Which software/HSM providers may satisfy crypto ops. **Default:** `CryptoProviderConfiguration::default()`.
    pub crypto_provider: CryptoProviderConfiguration,

    /// **SECURITY MONITORING & AUDITING**
    /// Immutable audit sinks and retention. **Default:** [`AuditConfiguration::default()`].
    pub audit: AuditConfiguration,
    /// SIEM hooks and security metrics. **Default:** `SecurityMonitoringConfiguration::default()`.
    pub monitoring: SecurityMonitoringConfiguration,
    /// API and IPC rate limits. **Default:** [`super::network::RateLimitConfig::default()`].
    pub rate_limiting: super::network::RateLimitConfig,

    /// **THREAT DETECTION & RESPONSE**
    /// IDS/IPS style signals. **Default:** [`CanonicalThreatDetectionConfig::default()`].
    pub threat_detection: CanonicalThreatDetectionConfig,
    /// Automated containment workflows. **Default:** [`ThreatResponseConfiguration::default()`].
    pub threat_response: ThreatResponseConfiguration,

    /// **ADVANCED SECURITY FEATURES**
    /// Genetics-aware key evolution policy. **Default:** `GeneticSecurityConfiguration::default()`.
    pub genetic_security: GeneticSecurityConfiguration,
    /// Node membership and attestation. **Default:** `EcosystemMembershipConfiguration::default()`.
    pub ecosystem_membership: EcosystemMembershipConfiguration,
    /// Trust scoring between primals. **Default:** `TrustComputationConfiguration::default()`.
    pub trust_computation: TrustComputationConfiguration,

    /// **COMPLIANCE & GOVERNANCE**
    /// Regulatory mapping (FIPS, SOC2, etc.). **Default:** `SecurityComplianceConfiguration::default()`.
    pub compliance: SecurityComplianceConfiguration,
    /// Data residency and sovereignty flags. **Default:** `DataSovereigntyConfiguration::default()`.
    pub data_sovereignty: DataSovereigntyConfiguration,
}

impl ConsolidatedSecurityConfiguration {
    /// Create default authorization configuration from environment
    fn default_authorization() -> AuthorizationConfiguration {
        AuthorizationConfiguration {
            enable_rbac: std::env::var("BEARDOG_AUTHZ_RBAC_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            enable_abac: std::env::var("BEARDOG_AUTHZ_ABAC_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            default_role: std::env::var("BEARDOG_AUTHZ_DEFAULT_ROLE")
                .unwrap_or_else(|_| "user".to_string()),
            permission_cache_timeout: std::env::var("BEARDOG_AUTHZ_CACHE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
            enable_capability_auth: std::env::var("BEARDOG_AUTHZ_CAPABILITY_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            consensus: ConsensusConfiguration::default(),
        }
    }

    /// Create default access control configuration from environment
    fn default_access_control() -> AccessControlConfiguration {
        AccessControlConfiguration {
            enable_ecosystem_membership: true,
            default_membership_level: "learning_participant".to_string(),
            auto_evolution: AutoEvolutionConfiguration::default(),
            legacy_integration: LegacyIntegrationConfiguration {
                enable_legacy_integration: std::env::var("BEARDOG_LEGACY_INTEGRATION_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(false),
                legacy_timeout_seconds: std::env::var("BEARDOG_LEGACY_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
                legacy_auth_methods: vec!["basic".to_string()],
                migration_deadline: std::env::var("BEARDOG_LEGACY_MIGRATION_DEADLINE")
                    .ok()
                    .or_else(|| Some("2025-12-31".to_string())),
            },
            health_monitoring: HealthMonitoringConfiguration {
                enable_monitoring: std::env::var("BEARDOG_HEALTH_MONITORING_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                check_interval_seconds: std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
                alert_thresholds: HashMap::new(),
                enable_auto_remediation: std::env::var("BEARDOG_HEALTH_AUTO_REMEDIATION_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
            },
        }
    }

    /// Create default encryption configuration from environment (legacy method)
    fn default_encryption() -> EncryptionConfiguration {
        use crate::canonical::config::source::EnvConfigSource;
        EncryptionConfiguration::from_source(&EnvConfigSource::new())
    }

    /// Create default monitoring configuration from environment
    fn default_monitoring() -> SecurityMonitoringConfiguration {
        SecurityMonitoringConfiguration {
            enable_monitoring: std::env::var("BEARDOG_SECURITY_MONITORING_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            monitoring_interval_seconds: std::env::var("BEARDOG_SECURITY_MONITORING_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            alerts: SecurityAlertConfiguration {
                enabled: true,
                channels: vec!["log".to_string()],
                severity_levels: HashMap::new(),
                rate_limit_per_hour: std::env::var("BEARDOG_SECURITY_ALERT_RATE_LIMIT_PER_HOUR")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            },
            metrics_enabled: true,
            siem_integration: None,
        }
    }

    /// Create default trust computation configuration from environment
    fn default_trust_computation() -> TrustComputationConfiguration {
        TrustComputationConfiguration {
            enabled: true,
            algorithm: "weighted_average".to_string(),
            trust_factors: HashMap::new(),
            trust_decay: TrustDecayConfiguration {
                enabled: std::env::var("BEARDOG_TRUST_DECAY_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                decay_rate: std::env::var("BEARDOG_TRUST_DECAY_RATE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.01),
                decay_interval_seconds: std::env::var("BEARDOG_TRUST_DECAY_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
                minimum_trust: std::env::var("BEARDOG_TRUST_MINIMUM")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.1),
            },
            evaluation: EvaluationConfiguration {
                algorithm: std::env::var("BEARDOG_TRUST_EVAL_ALGORITHM")
                    .unwrap_or_else(|_| "multi_criteria".to_string()),
                criteria: vec!["history".to_string(), "behavior".to_string()],
                weights: HashMap::new(),
                timeout_seconds: std::env::var("BEARDOG_TRUST_EVAL_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            },
        }
    }

    /// Create default compliance configuration from environment
    fn default_compliance() -> SecurityComplianceConfiguration {
        SecurityComplianceConfiguration {
            standards: vec!["SOX".to_string(), "GDPR".to_string()],
            validation: ComplianceValidationConfiguration {
                enabled: std::env::var("BEARDOG_COMPLIANCE_VALIDATION_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                validation_frequency_hours: std::env::var(
                    "BEARDOG_COMPLIANCE_VALIDATION_FREQUENCY_HOURS",
                )
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
                strictness_level: std::env::var("BEARDOG_COMPLIANCE_STRICTNESS")
                    .unwrap_or_else(|_| "high".to_string()),
                auto_remediation: std::env::var("BEARDOG_COMPLIANCE_AUTO_REMEDIATION")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
            },
            reporting: ComplianceReportingConfiguration {
                enabled: std::env::var("BEARDOG_COMPLIANCE_REPORTING_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                report_frequency_days: std::env::var("BEARDOG_COMPLIANCE_REPORT_FREQUENCY_DAYS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
                report_formats: vec!["json".to_string()],
                recipients: vec![],
            },
            audit_requirements: vec!["access_logs".to_string(), "change_logs".to_string()],
        }
    }

    /// Create default data sovereignty configuration from environment
    fn default_data_sovereignty() -> DataSovereigntyConfiguration {
        DataSovereigntyConfiguration {
            enabled: true,
            residency_requirements: vec!["user_controlled".to_string()],
            transfer_rules: vec![],
            validation: SovereigntyValidationConfiguration {
                enabled: std::env::var("BEARDOG_SOVEREIGNTY_VALIDATION_ENABLED")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(true),
                validation_frequency_hours: std::env::var(
                    "BEARDOG_SOVEREIGNTY_VALIDATION_FREQUENCY_HOURS",
                )
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
                criteria: vec!["data_residency".to_string(), "user_consent".to_string()],
                enforcement_level: std::env::var("BEARDOG_SOVEREIGNTY_ENFORCEMENT")
                    .unwrap_or_else(|_| "strict".to_string()),
            },
        }
    }
}

impl Default for ConsolidatedSecurityConfiguration {
    fn default() -> Self {
        Self {
            authentication: AuthenticationConfiguration::default(),
            authorization: Self::default_authorization(),
            access_control: Self::default_access_control(),
            encryption: Self::default_encryption(),
            key_management: KeyManagementConfiguration {
                rotation_interval_seconds: std::env::var("BEARDOG_KEY_ROTATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(86400), // 24 hours
                enable_auto_rotation: true,
                backup_enabled: true,
                escrow_enabled: false,
                genetic_renewal: GeneticRenewalConfiguration::default(),
            },
            crypto_provider: CryptoProviderConfiguration {
                primary_provider: "unified_crypto".to_string(),
                fallback_providers: vec!["openssl".to_string()],
                provider_settings: HashMap::new(),
                enable_failover: true,
            },
            audit: AuditConfiguration::default(),
            monitoring: Self::default_monitoring(),
            rate_limiting: super::network::RateLimitConfig::default(),
            threat_detection: CanonicalThreatDetectionConfig::default(),
            threat_response: ThreatResponseConfiguration::default(),
            genetic_security: GeneticSecurityConfiguration {
                enable_genetic_security: true,
                genetic_parameters: HashMap::new(),
                evolution_strategies: vec!["mutation".to_string(), "crossover".to_string()],
                fitness_criteria: vec!["security_strength".to_string(), "performance".to_string()],
            },
            ecosystem_membership: EcosystemMembershipConfiguration {
                enabled: true,
                membership_levels: HashMap::new(),
                evolution_rules: vec![],
                genetics_integration: GeneticsIntegrationConfiguration {
                    enabled: true,
                    genetic_factors: vec!["trust".to_string(), "behavior".to_string()],
                    integration_strength: std::env::var("BEARDOG_GENETICS_INTEGRATION_STRENGTH")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.7),
                    validation_timeout_seconds: std::env::var(
                        "BEARDOG_GENETICS_VALIDATION_TIMEOUT_SECS",
                    )
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
                },
            },
            trust_computation: Self::default_trust_computation(),
            compliance: Self::default_compliance(),
            data_sovereignty: Self::default_data_sovereignty(),
        }
    }
}

impl ConsolidatedSecurityConfiguration {
    /// Validate the entire security configuration
    ///
    /// # Errors
    ///
    /// Returns an error if authentication, encryption, rate limiting, or trust computation settings are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate authentication settings
        if self.authentication.session_timeout_seconds == 0 {
            return Err(BearDogError::configuration(
                "Session timeout cannot be zero",
            ));
        }

        if self.authentication.max_login_attempts == 0 {
            return Err(BearDogError::configuration(
                "Max login attempts cannot be zero",
            ));
        }

        // Validate encryption settings
        if self.encryption.key_size_bits < 128 {
            return Err(BearDogError::configuration(
                "Key size must be at least 128 bits",
            ));
        }

        // Validate rate limiting
        if self.rate_limiting.enabled && self.rate_limiting.max_requests == 0 {
            return Err(BearDogError::configuration(
                "Rate limit cannot be zero when enabled",
            ));
        }

        // Validate trust computation
        if self.trust_computation.enabled && self.trust_computation.trust_decay.decay_rate > 1.0 {
            return Err(BearDogError::configuration(
                "Trust decay rate cannot exceed 1.0",
            ));
        }

        Ok(())
    }

    /// Create a development-friendly configuration
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for login attempt limits"
    )]
    pub fn development() -> Self {
        let mut config = Self::default();

        // Relaxed settings for development
        config.authentication.session_timeout_seconds = 86400; // 24 hours
        config.authentication.max_login_attempts =
            crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32;
        config.rate_limiting.max_requests = std::env::var("BEARDOG_RATE_LIMITING_MAX_REQUESTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10000);
        config.audit.retention_days = std::env::var("BEARDOG_DEV_AUDIT_RETENTION_DAYS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        config.compliance.validation.strictness_level = "moderate".to_string();

        config
    }

    /// Create a production-hardened configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Strict settings for production
        config.authentication.session_timeout_seconds = 3600; // 1 hour
        config.authentication.max_login_attempts = 3;
        config.rate_limiting.max_requests =
            crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64;
        config.audit.retention_days = std::env::var("BEARDOG_PRODUCTION_AUDIT_RETENTION_DAYS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2555); // 7 years
        config.compliance.validation.strictness_level = "strict".to_string();
        config.data_sovereignty.validation.enforcement_level = "strict".to_string();

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_security_config_validation() {
        let config = ConsolidatedSecurityConfiguration::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "DEFAULT_POOL_SIZE fits u32; matches development preset assertion"
    )]
    fn test_development_config() {
        let config = ConsolidatedSecurityConfiguration::development();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.authentication.session_timeout_seconds, 86400);
        assert_eq!(
            config.authentication.max_login_attempts,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let config = ConsolidatedSecurityConfiguration::production();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.authentication.session_timeout_seconds, 3600);
        assert_eq!(config.authentication.max_login_attempts, 3);
        assert_eq!(config.compliance.validation.strictness_level, "strict");
        assert!(config.validate().is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    #[test]
    fn test_invalid_config_validation() {
        let mut config = ConsolidatedSecurityConfiguration::default();
        config.authentication.session_timeout_seconds = 0;
        assert!(config.validate().is_err());
    }
}
