//! Configuration Types
//!
//! This module contains all types related to security configuration,
//! policy management, provider settings, and system configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderConfig {
    /// Enable rate limiting for security operations
    pub rate_limiting_enabled: bool,
    /// Rate limiting configuration
    pub rate_limit_config: super::crypto_types::RateLimitConfig,
    /// Multi-factor authentication settings
    pub mfa_config: super::auth_types::MfaConfig,
    /// Session management settings
    pub session_config: super::auth_types::SessionConfig,
    /// Maximum failed authentication attempts before lockout
    pub max_failed_attempts: u32,
    /// Account lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Audit logging settings
    pub audit_logging_enabled: bool,
    /// Threat detection engine configuration
    pub threat_detection_enabled: bool,
    /// Compliance checking enabled
    pub compliance_enabled: bool,
    /// Optional threat detection engine
    pub threat_detector: Option<ThreatAnalyzer>,
    /// Optional workflow engine for approval processes
    pub workflow_engine: Option<MultiPartyWorkflowEngine>,
    /// Standalone in-memory key manager for "crypto in your pocket"
    pub memory_key_manager: Option<MemoryKeyManager>,
    /// Recovery system for account unlock and key recovery
    pub recovery_manager: Option<RecoveryManager>,
}

/// Security configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Password policy settings
    pub password_policy: PasswordPolicy,
    /// Account lockout policy
    pub lockout_policy: AccountLockoutPolicy,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Audit settings
    pub audit: AuditConfig,
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Minimum password length
    pub min_length: u8,
    /// Require uppercase letters
    pub require_uppercase: bool,
    /// Require lowercase letters
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special: bool,
    /// Password history to prevent reuse
    pub history_count: u8,
    /// Maximum password age in days
    pub max_age_days: u16,
}

/// Account lockout policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLockoutPolicy {
    /// Enable account lockout
    pub enabled: bool,
    /// Failed attempts before lockout
    pub max_failed_attempts: u8,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Reset failed attempt counter after this many minutes
    pub reset_counter_minutes: u32,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key rotation interval in days
    pub key_rotation_days: u16,
    /// Encryption strength level
    pub strength_level: u8,
    /// Enable hardware security module
    pub hsm_enabled: bool,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Log successful operations
    pub log_successful: bool,
    /// Log failed operations
    pub log_failed: bool,
    /// Retention period in days
    pub retention_days: u16,
    /// Enable real-time alerting
    pub real_time_alerts: bool,
}

/// Security rules engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRules {
    /// List of security rules
    pub rules: Vec<SecurityRule>,
    /// Default policy when no rules match
    pub default_policy: PolicyDecision,
    /// Security context for rule evaluation
    pub context: super::auth_types::SecurityContext,
}

/// Individual security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Unique rule identifier
    pub id: String,
    /// Rule name/description
    pub name: String,
    /// Rule conditions
    pub conditions: Vec<RuleCondition>,
    /// Action to take if rule matches
    pub action: PolicyDecision,
    /// Rule priority (higher = evaluated first)
    pub priority: u32,
    /// Whether the rule is enabled
    pub enabled: bool,
}

/// Rule condition for policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// Field to evaluate
    pub field: String,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Value to compare against
    pub value: String,
}

/// Comparison operators for rule conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Equals
    Equals,
    /// Not equals
    NotEquals,
    /// Contains substring
    Contains,
    /// Matches regex pattern
    Regex,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// In list of values
    In,
    /// Not in list of values
    NotIn,
}

/// Policy decision result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Allow the operation
    Allow,
    /// Deny the operation
    Deny,
    /// Require additional approval
    RequireApproval,
    /// Allow with conditions
    ConditionalAllow {
        /// Required conditions
        conditions: Vec<String>,
    },
}

/// Compliance status for security policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Fully compliant with all policies
    Compliant,
    /// Some minor compliance issues
    PartiallyCompliant,
    /// Major compliance violations
    NonCompliant,
    /// Compliance status unknown
    Unknown,
}

/// Threat analysis and detection system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalyzer {
    /// Enable threat detection
    pub enabled: bool,
    /// Threat detection rules
    pub rules: Vec<ThreatRule>,
    /// Response actions for detected threats
    pub response_actions: HashMap<String, ThreatResponse>,
    /// Update interval for threat intelligence
    pub update_interval_hours: u32,
}

/// Threat detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatRule {
    /// Rule identifier
    pub id: String,
    /// Rule pattern or signature
    pub pattern: String,
    /// Threat severity level
    pub severity: ThreatSeverity,
    /// Whether the rule is enabled
    pub enabled: bool,
}

/// Threat severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatSeverity {
    /// Low severity threat
    Low,
    /// Medium severity threat
    Medium,
    /// High severity threat
    High,
    /// Critical severity threat
    Critical,
}

/// Threat response action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatResponse {
    /// Log the threat
    Log,
    /// Block the request
    Block,
    /// Alert administrators
    Alert,
    /// Quarantine the source
    Quarantine,
}

// Placeholder structs for optional components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiPartyWorkflowEngine {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryKeyManager {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryManager {
    pub enabled: bool,
}

// Default implementations

impl Default for SecurityProviderConfig {
    fn default() -> Self {
        Self {
            rate_limiting_enabled: true,
            rate_limit_config: Default::default(),
            mfa_config: Default::default(),
            session_config: Default::default(),
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
            audit_logging_enabled: true,
            threat_detection_enabled: false,
            compliance_enabled: false,
            threat_detector: None,
            workflow_engine: None,
            memory_key_manager: None,
            recovery_manager: None,
        }
    }
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: false,
            history_count: 5,
            max_age_days: 90,
        }
    }
}

impl Default for AccountLockoutPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
            reset_counter_minutes: 60,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            key_rotation_days: 30,
            strength_level: 3,
            hsm_enabled: false,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_successful: false,
            log_failed: true,
            retention_days: 365,
            real_time_alerts: true,
        }
    }
}
