// SPDX-License-Identifier: AGPL-3.0-only

// Security configuration types for BearDog
// Provides comprehensive security settings, authentication, and access control

use crate::canonical::capabilities::SecurityLevel;
use crate::constants::time;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    /// Name of the item
    pub name: String,
    /// Policy description
    /// The description value
    pub description: String,
    /// Policy version
    /// The version value
    pub version: String,
    /// Policy rules
    /// Collection of rules
    pub rules: Vec<SecurityRule>,
    pub enforcement_level: EnforcementLevel,
    /// Policy metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            name: "Default Security Policy".to_string(),
            description: "Default security policy for BearDog".to_string(),
            version: "1.0.0".to_string(),
            rules: Vec::new(),
            enforcement_level: EnforcementLevel::Strict,
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    /// Permissive mode (warnings only)
    Permissive,
    Standard,
    Strict,
    Maximum,
}

impl Default for EnforcementLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Security rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    /// Name of the item
    pub name: String,
    /// Rule description
    /// The description value
    pub description: String,
    /// Rule condition
    /// The condition value
    pub condition: String,
    /// Rule action
    /// The action value
    pub action: SecurityAction,
    /// Rule enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Rule priority
    /// Number of priority
    pub priority: u32,
}

impl Default for SecurityRule {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Rule".to_string(),
            description: "Default security rule".to_string(),
            condition: "true".to_string(),
            action: SecurityAction::Allow,
            enabled: true,
            priority: 100,
        }
    }
}

/// Security actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityAction {
    /// Allow the action
    Allow,
    /// Deny the action
    Deny,
    /// Log the action
    Log,
    /// Alert on the action
    Alert,
    /// Block and alert
    BlockAndAlert,
}

impl Default for SecurityAction {
    fn default() -> Self {
        Self::Allow
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Authentication methods enabled
    /// Collection of methods
    pub methods: Vec<AuthMethod>,
    /// Multi-factor authentication required
    /// Whether mfa_required is enabled
    pub mfa_required: bool,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum failed attempts
    /// Number of max_failed_attempts
    pub max_failed_attempts: u32,
    /// Account lockout duration in seconds
    /// Number of lockout_duration_seconds
    pub lockout_duration_seconds: u64,
    /// Password policy
    /// The password policy value
    pub password_policy: PasswordPolicy,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            methods: vec![AuthMethod::Password, AuthMethod::Mfa],
            mfa_required: true,
            session_timeout_seconds: time::SECONDS_PER_HOUR,
            max_failed_attempts: 3,
            lockout_duration_seconds: 900,
            password_policy: PasswordPolicy::default(),
        }
    }
}

/// Authentication methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Password authentication
    Password,
    /// Multi-factor authentication
    Mfa,
    /// Biometric authentication
    Biometric,
    /// Certificate authentication
    Certificate,
    /// Token-based authentication
    Token,
    /// SSO authentication
    Sso,
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Minimum password length
    /// Number of min_length
    pub min_length: u32,
    /// Require uppercase letters
    /// Whether require_uppercase is enabled
    pub require_uppercase: bool,
    /// Require lowercase letters
    /// Whether require_lowercase is enabled
    pub require_lowercase: bool,
    /// Require numbers
    /// Whether require_numbers is enabled
    pub require_numbers: bool,
    /// Require special characters
    /// Whether require_special is enabled
    pub require_special: bool,
    /// Password history count
    /// Number of history
    pub history_count: u32,
    /// Password expiry in days
    /// Optional expiry days
    pub expiry_days: Option<u32>,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            history_count: 5,
            expiry_days: Some(90),
        }
    }
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Role-based access control enabled
    /// Whether rbac is enabled
    pub rbac_enabled: bool,
    /// Attribute-based access control enabled
    /// Whether abac is enabled
    pub abac_enabled: bool,
    /// Default access policy
    /// The default policy value
    pub default_policy: AccessPolicy,
    /// Resource permissions
    /// Mapping of resource permissions
    pub resource_permissions: HashMap<String, Vec<Permission>>,
    /// Role definitions
    /// Mapping of roles
    pub roles: HashMap<String, Role>,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            rbac_enabled: true,
            abac_enabled: false,
            default_policy: AccessPolicy::Deny,
            resource_permissions: HashMap::new(),
            roles: HashMap::new(),
        }
    }
}

/// Access policies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPolicy {
    /// Allow by default
    Allow,
    /// Deny by default
    Deny,
    /// Require explicit permission
    Explicit,
}

/// Permission definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    /// Permission name
    /// Name of the item
    pub name: String,
    /// Permission description
    /// The description value
    pub description: String,
    /// Resource type
    /// The resource type value
    pub resource_type: String,
    /// Actions allowed
    /// Collection of actions
    pub actions: Vec<String>,
    /// Conditions
    /// Collection of conditions
    pub conditions: Vec<String>,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role name
    /// Name of the item
    pub name: String,
    /// Role description
    /// The description value
    pub description: String,
    /// Permissions granted to this role
    /// Collection of permissions
    pub permissions: Vec<String>,
    /// Role inheritance
    /// Collection of inherits from
    pub inherits_from: Vec<String>,
    /// Role enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

impl Default for Role {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default role".to_string(),
            permissions: Vec::new(),
            inherits_from: Vec::new(),
            enabled: false,
        }
    }
}

/// Security token configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// Token type
    /// The token type value
    pub token_type: TokenType,
    /// Token issuer
    /// The issuer value
    pub issuer: String,
    /// Token audience
    /// The audience value
    pub audience: String,
    /// Token expiry duration
    /// The expiry duration value
    pub expiry_duration: Duration,
    /// Token refresh enabled
    /// Whether refresh is enabled
    pub refresh_enabled: bool,
    /// Signing algorithm
    /// The signing algorithm value
    pub signing_algorithm: String,
    /// Token claims
    /// Mapping of claims
    pub claims: HashMap<String, String>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            token_type: TokenType::Jwt,
            issuer: "beardog ".to_string(),
            audience: "beardog-api".to_string(),
            expiry_duration: Duration::hours(1),
            refresh_enabled: true,
            signing_algorithm: "HS256".to_string(),
            claims: HashMap::new(),
        }
    }
}

/// Token types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of token
pub enum TokenType {
    /// JSON Web Token
    Jwt,
    /// Opaque token
    Opaque,
    /// Bearer token
    Bearer,
    /// API key
    ApiKey,
}

/// Comprehensive security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Security policy
    /// The policy value
    pub policy: SecurityPolicy,
    /// Authentication configuration
    /// The authentication value
    pub authentication: AuthenticationConfig,
    /// Access control configuration
    /// The access control value
    pub access_control: AccessControlConfig,
    /// Token configuration
    /// The token value
    pub token: TokenConfig,
    /// Audit logging enabled
    /// Whether audit_logging is enabled
    pub audit_logging: bool,
    /// Encryption at rest enabled
    /// Whether encryption_at_rest is enabled
    pub encryption_at_rest: bool,
    /// Encryption in transit enabled
    /// Whether encryption_in_transit is enabled
    pub encryption_in_transit: bool,
    /// Security headers enabled
    /// Whether security_headers is enabled
    pub security_headers: bool,
    /// Rate limiting enabled
    /// Whether rate_limiting is enabled
    pub rate_limiting: bool,
    /// Rate limit per minute
    /// Number of rate_limit_per_minute
    pub rate_limit_per_minute: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            policy: SecurityPolicy::default(),
            authentication: AuthenticationConfig::default(),
            access_control: AccessControlConfig::default(),
            token: TokenConfig::default(),
            audit_logging: true,
            encryption_at_rest: true,
            encryption_in_transit: true,
            security_headers: true,
            rate_limiting: true,
            rate_limit_per_minute: 1000,
        }
    }
}

impl SecurityConfig {
    /// Create a new security configuration
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_security() -> Self {
        let mut config = Self::default();
        config.policy.enforcement_level = EnforcementLevel::Maximum;
        config.authentication.mfa_required = true;
        config.authentication.max_failed_attempts = 1;
        config.authentication.lockout_duration_seconds = time::SECONDS_PER_HOUR;
        config.access_control.default_policy = AccessPolicy::Deny;
        config.rate_limiting = true;
        config.rate_limit_per_minute = 100;
        config
    }

    pub fn development() -> Self {
        let mut config = Self::default();
        config.policy.enforcement_level = EnforcementLevel::Permissive;
        config.authentication.mfa_required = false;
        config.authentication.session_timeout_seconds = time::SECONDS_PER_DAY;
        config.rate_limiting = false;
        config
    }

    /// Validate the security configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.authentication.max_failed_attempts == 0 {
            return Err("Max failed attempts must be greater than 0".to_string());
        }

        if self.authentication.session_timeout_seconds == 0 {
            return Err("Session timeout must be greater than 0".to_string());
        }

        if self.rate_limiting && self.rate_limit_per_minute == 0 {
            return Err(
                "Rate limit per minute must be greater than 0 when rate limiting is enabled"
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Check if the configuration meets security standards
    pub fn meets_security_standards(&self) -> bool {
        self.authentication.mfa_required
            && self.encryption_at_rest
            && self.encryption_in_transit
            && self.audit_logging
            && self.authentication.password_policy.min_length >= 8
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityContext {
    /// User identity
    pub user_id: String,
    pub session_id: String,
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Security clearance level
    /// The clearance level value
    pub clearance_level: SecurityLevel,
    /// Additional context metadata
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: "anonymous".to_string(),
            session_id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            clearance_level: SecurityLevel::Basic,
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum PolicyDecision {
    /// Allow the operation
    Allow,
    /// Deny the operation
    #[default]
    /// Represents deny variant
    Deny,
    /// Allow with conditions
    Conditional,
}
