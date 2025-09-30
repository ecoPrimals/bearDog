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
// use std::time::Duration; // Currently unused

/// **CONSOLIDATED SECURITY CONFIGURATION** - Single source of truth for all security settings
///
/// This structure consolidates all security-related configurations across the `BearDog` ecosystem,
/// eliminating fragmentation and providing a unified security configuration interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedSecurityConfiguration {
    /// **AUTHENTICATION & AUTHORIZATION**
    pub authentication: AuthenticationConfiguration,
    pub authorization: AuthorizationConfiguration,
    pub access_control: AccessControlConfiguration,
    
    /// **CRYPTOGRAPHIC OPERATIONS**
    pub encryption: EncryptionConfiguration,
    pub key_management: KeyManagementConfiguration,
    pub crypto_provider: CryptoProviderConfiguration,
    
    /// **SECURITY MONITORING & AUDITING**
    pub audit: AuditConfiguration,
    pub monitoring: SecurityMonitoringConfiguration,
    pub rate_limiting: RateLimitConfiguration,
    
    /// **THREAT DETECTION & RESPONSE**
    pub threat_detection: ThreatDetectionConfiguration,
    pub threat_response: ThreatResponseConfiguration,
    
    /// **ADVANCED SECURITY FEATURES**
    pub genetic_security: GeneticSecurityConfiguration,
    pub ecosystem_membership: EcosystemMembershipConfiguration,
    pub trust_computation: TrustComputationConfiguration,
    
    /// **COMPLIANCE & GOVERNANCE**
    pub compliance: SecurityComplianceConfiguration,
    pub data_sovereignty: DataSovereigntyConfiguration,
}

/// Authentication configuration - consolidates `AuthConfig` and related structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfiguration {
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum login attempts before lockout
    pub max_login_attempts: u32,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Password hash rounds
    pub hash_rounds: u32,
    /// JWT token expiry duration
    pub jwt_expiry_seconds: u64,
    /// Enable cross-node authentication
    pub enable_cross_node_auth: bool,
    /// Authentication provider type
    pub provider_type: String,
}

impl Default for AuthenticationConfiguration {
    fn default() -> Self {
        Self {
            session_timeout_seconds: 3600, // 1 hour
            max_login_attempts: 5,
            enable_mfa: true,
            hash_rounds: 12,
            jwt_expiry_seconds: 3600,
            enable_cross_node_auth: true,
            provider_type: "unified".to_string(),
        }
    }
}

/// Authorization configuration - consolidates authorization-related configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfiguration {
    /// Enable role-based access control
    pub enable_rbac: bool,
    /// Enable attribute-based access control
    pub enable_abac: bool,
    /// Default user role
    pub default_role: String,
    /// Permission cache timeout seconds
    pub permission_cache_timeout: u64,
    /// Enable capability-based authorization
    pub enable_capability_auth: bool,
    /// Consensus configuration for distributed authorization
    pub consensus: ConsensusConfiguration,
}

/// Consensus configuration for distributed systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfiguration {
    /// Consensus algorithm type
    pub algorithm: String,
    /// Minimum consensus threshold
    pub threshold: f64,
    /// Consensus timeout in seconds
    pub timeout_seconds: u64,
    /// Enable Byzantine fault tolerance
    pub enable_bft: bool,
}

impl Default for ConsensusConfiguration {
    fn default() -> Self {
        Self {
            algorithm: "raft".to_string(),
            threshold: 0.67, // 2/3 majority
            timeout_seconds: 30,
            enable_bft: true,
        }
    }
}

/// Access control configuration - consolidates `MembershipConfig` and related structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfiguration {
    /// Enable ecosystem membership model
    pub enable_ecosystem_membership: bool,
    /// Default membership level
    pub default_membership_level: String,
    /// Auto-evolution settings
    pub auto_evolution: AutoEvolutionConfiguration,
    /// Legacy integration settings
    pub legacy_integration: LegacyIntegrationConfiguration,
    /// Health monitoring for access control
    pub health_monitoring: HealthMonitoringConfiguration,
}

/// Auto-evolution configuration for adaptive access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEvolutionConfiguration {
    /// Enable automatic evolution of access patterns
    pub enable_auto_evolution: bool,
    /// Evolution threshold
    pub evolution_threshold: f64,
    /// Evolution interval in seconds
    pub evolution_interval_seconds: u64,
    /// Maximum evolution steps per interval
    pub max_evolution_steps: u32,
}

impl Default for AutoEvolutionConfiguration {
    fn default() -> Self {
        Self {
            enable_auto_evolution: true,
            evolution_threshold: 0.8,
            evolution_interval_seconds: 3600, // 1 hour
            max_evolution_steps: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
        }
    }
}

/// Legacy integration configuration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyIntegrationConfiguration {
    /// Enable legacy system integration
    pub enable_legacy_integration: bool,
    /// Legacy system timeout
    pub legacy_timeout_seconds: u64,
    /// Legacy authentication methods
    pub legacy_auth_methods: Vec<String>,
    /// Migration timeline
    pub migration_deadline: Option<String>,
}

/// Health monitoring configuration for security systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfiguration {
    /// Enable security health monitoring
    pub enable_monitoring: bool,
    /// Health check interval in seconds
    pub check_interval_seconds: u64,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Enable automated remediation
    pub enable_auto_remediation: bool,
}

/// Encryption configuration - consolidates `EncryptionConfig` and `SafeCryptoConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key size in bits
    pub key_size_bits: u32,
    /// Enable hardware acceleration
    pub enable_hardware_acceleration: bool,
    /// Safe crypto configuration
    pub safe_crypto: SafeCryptoConfiguration,
    /// Context-aware key configuration
    pub context_aware_keys: ContextAwareKeyConfiguration,
}

/// Safe cryptographic operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeCryptoConfiguration {
    /// Enable safe crypto mode
    pub enable_safe_mode: bool,
    /// Validation level (strict, moderate, lenient)
    pub validation_level: String,
    /// Enable constant-time operations
    pub enable_constant_time: bool,
    /// Memory protection level
    pub memory_protection_level: String,
}

impl Default for SafeCryptoConfiguration {
    fn default() -> Self {
        Self {
            enable_safe_mode: true,
            validation_level: "strict".to_string(),
            enable_constant_time: true,
            memory_protection_level: "high".to_string(),
        }
    }
}

/// Context-aware key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfiguration {
    /// Enable context-aware key management
    pub enable_context_aware: bool,
    /// Context factors to consider
    pub context_factors: Vec<String>,
    /// Key rotation based on context
    pub context_rotation_threshold: f64,
    /// Context validation timeout
    pub context_validation_timeout: u64,
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfiguration {
    /// Key rotation interval in seconds
    pub rotation_interval_seconds: u64,
    /// Enable automatic key rotation
    pub enable_auto_rotation: bool,
    /// Key backup configuration
    pub backup_enabled: bool,
    /// Key escrow configuration
    pub escrow_enabled: bool,
    /// Genetic key renewal settings
    pub genetic_renewal: GeneticRenewalConfiguration,
}

/// Genetic key renewal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRenewalConfiguration {
    /// Enable genetic key renewal
    pub enable_genetic_renewal: bool,
    /// Genetic algorithm parameters
    pub genetic_parameters: HashMap<String, f64>,
    /// Renewal trigger conditions
    pub renewal_triggers: Vec<String>,
    /// Renewal frequency
    pub renewal_frequency_hours: u64,
}

impl Default for GeneticRenewalConfiguration {
    fn default() -> Self {
        let mut genetic_parameters = HashMap::new();
        genetic_parameters.insert("mutation_rate".to_string(), 0.1);
        genetic_parameters.insert("crossover_rate".to_string(), 0.8);
        genetic_parameters.insert("selection_pressure".to_string(), 0.7);
        
        Self {
            enable_genetic_renewal: true,
            genetic_parameters,
            renewal_triggers: vec![
                "time_based".to_string(),
                "usage_based".to_string(),
                "threat_based".to_string(),
            ],
            renewal_frequency_hours: 168, // Weekly
        }
    }
}

/// Crypto provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoProviderConfiguration {
    /// Primary crypto provider
    pub primary_provider: String,
    /// Fallback providers
    pub fallback_providers: Vec<String>,
    /// Provider-specific settings
    pub provider_settings: HashMap<String, serde_json::Value>,
    /// Enable provider failover
    pub enable_failover: bool,
}

/// Audit configuration - consolidates `AuditConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfiguration {
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention days
    pub retention_days: u32,
    /// Audit log format
    pub log_format: String,
    /// Enable real-time audit monitoring
    pub enable_realtime_monitoring: bool,
    /// Audit storage backend
    pub storage_backend: String,
    /// Compliance requirements
    pub compliance_standards: Vec<String>,
}

impl Default for AuditConfiguration {
    fn default() -> Self {
        Self {
            enable_audit_logging: true,
            retention_days: 2555, // 7 years
            log_format: "json".to_string(),
            enable_realtime_monitoring: true,
            storage_backend: "encrypted_file".to_string(),
            compliance_standards: vec![
                "SOX".to_string(),
                "GDPR".to_string(),
                "CCPA".to_string(),
            ],
        }
    }
}

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfiguration {
    /// Enable security event monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval seconds
    pub monitoring_interval_seconds: u64,
    /// Alert configuration
    pub alerts: SecurityAlertConfiguration,
    /// Metrics collection
    pub metrics_enabled: bool,
    /// Integration with external SIEM
    pub siem_integration: Option<SiemIntegrationConfiguration>,
}

/// Security alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlertConfiguration {
    /// Enable security alerts
    pub enabled: bool,
    /// Alert channels (email, slack, webhook, etc.)
    pub channels: Vec<String>,
    /// Alert severity levels
    pub severity_levels: HashMap<String, u8>,
    /// Alert rate limiting
    pub rate_limit_per_hour: u32,
}

/// SIEM integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemIntegrationConfiguration {
    /// SIEM system type
    pub siem_type: String,
    /// Connection endpoint
    pub endpoint: String,
    /// Authentication credentials
    pub credentials: HashMap<String, String>,
    /// Event format
    pub event_format: String,
}

/// Rate limiting configuration - consolidates `RateLimitConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfiguration {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute limit
    pub requests_per_minute: u32,
    /// Burst capacity
    pub burst_capacity: u32,
    /// Rate limiting algorithm
    pub algorithm: String,
    /// Whitelist for rate limiting exemptions
    pub whitelist: Vec<String>,
    /// Rate limit enforcement level
    pub enforcement_level: String,
}

impl Default for RateLimitConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u32,
            burst_capacity: 100,
            algorithm: "token_bucket".to_string(),
            whitelist: vec![crate::constants::domains::network::addresses::LOCALHOST_IPV4.to_string()],
            enforcement_level: "strict".to_string(),
        }
    }
}

/// **THREAT DETECTION CONFIGURATION** - Consolidated threat detection settings
///
/// This structure consolidates threat detection configs from:
/// - `beardog-threat/src/threat/handlers/analysis.rs`
/// - `beardog-threat/src/threat/types/modules/core.rs`
/// - `beardog-threat/src/threat/types/config.rs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfiguration {
    /// Enable the threat detection system
    pub enabled: bool,
    /// Enable real-time threat detection
    pub realtime_detection: bool,
    /// Maximum concurrent threat analyses
    pub max_concurrent_analyses: usize,
    /// Detection sensitivity level (0.0 - 1.0)
    pub detection_sensitivity: f64,
    /// Enable threat intelligence feeds
    pub enable_threat_feeds: bool,
    /// Threat intelligence feed URLs
    pub threat_feeds: Vec<String>,
    /// Enable machine learning enhancement
    pub ml_enhancement: bool,
    /// Maximum active threats to track
    pub max_active_threats: usize,
    /// Enable real-time monitoring
    pub real_time_monitoring: bool,
    /// Enable automated response
    pub auto_response: bool,
    /// Automated quarantine threshold score
    pub quarantine_threshold: f64,
    /// Automated block threshold score
    pub block_threshold: f64,
    /// Threat score threshold (0-100)
    pub threat_threshold: u8,
    /// Alert threshold for notifications (0.0 - 1.0)
    pub alert_threshold: f64,
    /// Maximum alerts per minute
    pub max_alerts_per_minute: u32,
    /// Enable auto-quarantine
    pub auto_quarantine: bool,
    /// Notification endpoints for alerts
    pub notification_endpoints: Vec<String>,
    /// Path to threat detection rules
    pub rules_path: String,
    /// Paths to monitor for threats
    pub monitor_paths: Vec<String>,
    /// Cache size for threat data
    pub cache_size: usize,
    /// Monitoring interval in seconds
    pub monitoring_interval_seconds: u64,
}

impl Default for ThreatDetectionConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            realtime_detection: true,
            max_concurrent_analyses: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            detection_sensitivity: 0.7,
            enable_threat_feeds: true,
            threat_feeds: Vec::new(),
            ml_enhancement: true,
            max_active_threats: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            real_time_monitoring: true,
            auto_response: false,
            quarantine_threshold: 0.8,
            block_threshold: 0.9,
            threat_threshold: 70,
            alert_threshold: 0.8,
            max_alerts_per_minute: 100,
            auto_quarantine: false,
            notification_endpoints: Vec::new(),
            rules_path: "/etc/beardog/threat-rules".to_string(),
            monitor_paths: Vec::new(),
            cache_size: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            monitoring_interval_seconds: 300,
        }
    }
}

/// **THREAT RESPONSE CONFIGURATION** - Automated threat response settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResponseConfiguration {
    /// Enable automated threat response
    pub enabled: bool,
    /// Maximum response level severity
    pub max_response_level: String,
    /// Enable automatic isolation
    pub enable_isolation: bool,
    /// Enable automatic blocking
    pub enable_blocking: bool,
    /// Enable enhanced monitoring on threats
    pub enable_enhanced_monitoring: bool,
    /// Collect forensics data
    pub collect_forensics: bool,
    /// Update threat intelligence
    pub update_intelligence: bool,
}

impl Default for ThreatResponseConfiguration {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for safety
            max_response_level: "medium".to_string(),
            enable_isolation: false,
            enable_blocking: false,
            enable_enhanced_monitoring: true,
            collect_forensics: true,
            update_intelligence: true,
        }
    }
}

/// Genetic security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSecurityConfiguration {
    /// Enable genetic security algorithms
    pub enable_genetic_security: bool,
    /// Genetic algorithm parameters
    pub genetic_parameters: HashMap<String, f64>,
    /// Evolution strategies
    pub evolution_strategies: Vec<String>,
    /// Fitness evaluation criteria
    pub fitness_criteria: Vec<String>,
}

/// Ecosystem membership configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemMembershipConfiguration {
    /// Enable ecosystem membership model
    pub enabled: bool,
    /// Membership levels and permissions
    pub membership_levels: HashMap<String, Vec<String>>,
    /// Membership evolution rules
    pub evolution_rules: Vec<MembershipEvolutionRule>,
    /// Integration with genetics system
    pub genetics_integration: GeneticsIntegrationConfiguration,
}

/// Membership evolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipEvolutionRule {
    /// Rule name
    pub name: String,
    /// Trigger conditions
    pub conditions: Vec<String>,
    /// Target membership level
    pub target_level: String,
    /// Evolution probability
    pub probability: f64,
}

/// Genetics integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsIntegrationConfiguration {
    /// Enable genetics integration
    pub enabled: bool,
    /// Genetic factors for security decisions
    pub genetic_factors: Vec<String>,
    /// Integration strength (0.0 to 1.0)
    pub integration_strength: f64,
    /// Genetic validation timeout
    pub validation_timeout_seconds: u64,
}

/// Trust computation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustComputationConfiguration {
    /// Enable trust computation
    pub enabled: bool,
    /// Trust computation algorithm
    pub algorithm: String,
    /// Trust factors and weights
    pub trust_factors: HashMap<String, f64>,
    /// Trust decay parameters
    pub trust_decay: TrustDecayConfiguration,
    /// Evaluation configuration
    pub evaluation: EvaluationConfiguration,
}

/// Trust decay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustDecayConfiguration {
    /// Enable trust decay
    pub enabled: bool,
    /// Decay rate (0.0 to 1.0)
    pub decay_rate: f64,
    /// Decay interval in seconds
    pub decay_interval_seconds: u64,
    /// Minimum trust threshold
    pub minimum_trust: f64,
}

/// Evaluation configuration for trust and security decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfiguration {
    /// Evaluation algorithm
    pub algorithm: String,
    /// Evaluation criteria
    pub criteria: Vec<String>,
    /// Evaluation weights
    pub weights: HashMap<String, f64>,
    /// Evaluation timeout
    pub timeout_seconds: u64,
}

/// Security compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityComplianceConfiguration {
    /// Compliance standards to adhere to
    pub standards: Vec<String>,
    /// Compliance validation settings
    pub validation: ComplianceValidationConfiguration,
    /// Reporting configuration
    pub reporting: ComplianceReportingConfiguration,
    /// Audit requirements
    pub audit_requirements: Vec<String>,
}

/// Compliance validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceValidationConfiguration {
    /// Enable compliance validation
    pub enabled: bool,
    /// Validation frequency
    pub validation_frequency_hours: u64,
    /// Validation strictness level
    pub strictness_level: String,
    /// Automated remediation
    pub auto_remediation: bool,
}

/// Compliance reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportingConfiguration {
    /// Enable compliance reporting
    pub enabled: bool,
    /// Report generation frequency
    pub report_frequency_days: u32,
    /// Report formats
    pub report_formats: Vec<String>,
    /// Report recipients
    pub recipients: Vec<String>,
}

/// Data sovereignty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfiguration {
    /// Enable data sovereignty protection
    pub enabled: bool,
    /// Data residency requirements
    pub residency_requirements: Vec<String>,
    /// Cross-border data transfer rules
    pub transfer_rules: Vec<DataTransferRule>,
    /// Sovereignty validation
    pub validation: SovereigntyValidationConfiguration,
}

/// Data transfer rule for sovereignty compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransferRule {
    /// Rule name
    pub name: String,
    /// Source jurisdictions
    pub source_jurisdictions: Vec<String>,
    /// Target jurisdictions
    pub target_jurisdictions: Vec<String>,
    /// Transfer conditions
    pub conditions: Vec<String>,
    /// Approval required
    pub approval_required: bool,
}

/// Sovereignty validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyValidationConfiguration {
    /// Enable sovereignty validation
    pub enabled: bool,
    /// Validation frequency
    pub validation_frequency_hours: u64,
    /// Validation criteria
    pub criteria: Vec<String>,
    /// Enforcement level
    pub enforcement_level: String,
}

impl Default for ConsolidatedSecurityConfiguration {
    fn default() -> Self {
        Self {
            authentication: AuthenticationConfiguration::default(),
            authorization: AuthorizationConfiguration {
                enable_rbac: true,
                enable_abac: true,
                default_role: "user".to_string(),
                permission_cache_timeout: 300,
                enable_capability_auth: true,
                consensus: ConsensusConfiguration::default(),
            },
            access_control: AccessControlConfiguration {
                enable_ecosystem_membership: true,
                default_membership_level: "learning_participant".to_string(),
                auto_evolution: AutoEvolutionConfiguration::default(),
                legacy_integration: LegacyIntegrationConfiguration {
                    enable_legacy_integration: false,
                    legacy_timeout_seconds: 30,
                    legacy_auth_methods: vec!["basic".to_string()],
                    migration_deadline: Some("2025-12-31".to_string()),
                },
                health_monitoring: HealthMonitoringConfiguration {
                    enable_monitoring: true,
                    check_interval_seconds: 60,
                    alert_thresholds: HashMap::new(),
                    enable_auto_remediation: true,
                },
            },
            encryption: EncryptionConfiguration {
                default_algorithm: "AES-256-GCM".to_string(),
                key_size_bits: 256,
                enable_hardware_acceleration: true,
                safe_crypto: SafeCryptoConfiguration::default(),
                context_aware_keys: ContextAwareKeyConfiguration {
                    enable_context_aware: true,
                    context_factors: vec![
                        "time".to_string(),
                        "location".to_string(),
                        "user_behavior".to_string(),
                    ],
                    context_rotation_threshold: 0.8,
                    context_validation_timeout: 30,
                },
            },
            key_management: KeyManagementConfiguration {
                rotation_interval_seconds: 86400, // 24 hours
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
            monitoring: SecurityMonitoringConfiguration {
                enable_monitoring: true,
                monitoring_interval_seconds: 60,
                alerts: SecurityAlertConfiguration {
                    enabled: true,
                    channels: vec!["log".to_string()],
                    severity_levels: HashMap::new(),
                    rate_limit_per_hour: 100,
                },
                metrics_enabled: true,
                siem_integration: None,
            },
            rate_limiting: RateLimitConfiguration::default(),
            threat_detection: ThreatDetectionConfiguration::default(),
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
                    integration_strength: 0.7,
                    validation_timeout_seconds: 30,
                },
            },
            trust_computation: TrustComputationConfiguration {
                enabled: true,
                algorithm: "weighted_average".to_string(),
                trust_factors: HashMap::new(),
                trust_decay: TrustDecayConfiguration {
                    enabled: true,
                    decay_rate: 0.01,
                    decay_interval_seconds: 3600,
                    minimum_trust: 0.1,
                },
                evaluation: EvaluationConfiguration {
                    algorithm: "multi_criteria".to_string(),
                    criteria: vec!["history".to_string(), "behavior".to_string()],
                    weights: HashMap::new(),
                    timeout_seconds: 30,
                },
            },
            compliance: SecurityComplianceConfiguration {
                standards: vec!["SOX".to_string(), "GDPR".to_string()],
                validation: ComplianceValidationConfiguration {
                    enabled: true,
                    validation_frequency_hours: 24,
                    strictness_level: "high".to_string(),
                    auto_remediation: true,
                },
                reporting: ComplianceReportingConfiguration {
                    enabled: true,
                    report_frequency_days: 30,
                    report_formats: vec!["json".to_string()],
                    recipients: vec![],
                },
                audit_requirements: vec!["access_logs".to_string(), "change_logs".to_string()],
            },
            data_sovereignty: DataSovereigntyConfiguration {
                enabled: true,
                residency_requirements: vec!["user_controlled".to_string()],
                transfer_rules: vec![],
                validation: SovereigntyValidationConfiguration {
                    enabled: true,
                    validation_frequency_hours: 24,
                    criteria: vec!["data_residency".to_string(), "user_consent".to_string()],
                    enforcement_level: "strict".to_string(),
                },
            },
        }
    }
}

impl ConsolidatedSecurityConfiguration {
    /// Validate the entire security configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate authentication settings
        if self.authentication.session_timeout_seconds == 0 {
            return Err(BearDogError::configuration("Session timeout cannot be zero"));
        }
        
        if self.authentication.max_login_attempts == 0 {
            return Err(BearDogError::configuration("Max login attempts cannot be zero"));
        }
        
        // Validate encryption settings
        if self.encryption.key_size_bits < 128 {
            return Err(BearDogError::configuration("Key size must be at least 128 bits"));
        }
        
        // Validate rate limiting
        if self.rate_limiting.enabled && self.rate_limiting.requests_per_minute == 0 {
            return Err(BearDogError::configuration("Rate limit cannot be zero when enabled"));
        }
        
        // Validate trust computation
        if self.trust_computation.enabled && self.trust_computation.trust_decay.decay_rate > 1.0 {
            return Err(BearDogError::configuration("Trust decay rate cannot exceed 1.0"));
        }
        
        Ok(())
    }
    
    /// Create a development-friendly configuration
    pub fn development() -> Self {
        let mut config = Self::default();
        
        // Relaxed settings for development
        config.authentication.session_timeout_seconds = 86400; // 24 hours
        config.authentication.max_login_attempts = crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32;
        config.rate_limiting.requests_per_minute = 10000;
        config.audit.retention_days = 30;
        config.compliance.validation.strictness_level = "moderate".to_string();
        
        config
    }
    
    /// Create a production-hardened configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        
        // Strict settings for production
        config.authentication.session_timeout_seconds = 3600; // 1 hour
        config.authentication.max_login_attempts = 3;
        config.rate_limiting.requests_per_minute = crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u32;
        config.audit.retention_days = 2555; // 7 years
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
    fn test_development_config() {
        let config = ConsolidatedSecurityConfiguration::development();
        assert_eq!(config.authentication.session_timeout_seconds, 86400);
        assert_eq!(config.authentication.max_login_attempts, crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let config = ConsolidatedSecurityConfiguration::production();
        assert_eq!(config.authentication.session_timeout_seconds, 3600);
        assert_eq!(config.authentication.max_login_attempts, 3);
        assert_eq!(config.compliance.validation.strictness_level, "strict");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_validation() {
        let mut config = ConsolidatedSecurityConfiguration::default();
        config.authentication.session_timeout_seconds = 0;
        assert!(config.validate().is_err());
    }
} 