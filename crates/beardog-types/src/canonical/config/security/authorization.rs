// SPDX-License-Identifier: AGPL-3.0-only

// Authorization Configuration
//
// Canonical authorization configuration for RBAC, permissions, and access control policies.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL AUTHORIZATION CONFIGURATION**
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAuthorizationConfig {
    /// Enable role-based access control (RBAC)
    /// Whether `enable_rbac` is enabled
    pub enable_rbac: bool,

    /// The default role value
    pub default_role: String,

    /// Role definitions
    /// Mapping of roles
    pub roles: HashMap<String, RoleConfig>,

    /// Permission definitions
    /// Mapping of permissions
    pub permissions: HashMap<String, PermissionConfig>,

    /// Resource-based access control rules
    /// Mapping of resource rules
    pub resource_rules: HashMap<String, ResourceRuleConfig>,

    /// Enable attribute-based access control (ABAC)
    /// Whether `enable_abac` is enabled
    pub enable_abac: bool,

    /// ABAC policy rules
    /// Collection of abac policies
    pub abac_policies: Vec<AbacPolicyConfig>,

    /// Cache authorization decisions
    /// Whether `enable_auth_cache` is enabled
    pub enable_auth_cache: bool,

    /// Authorization cache TTL in seconds
    /// Number of `auth_cache_ttl_seconds`
    pub auth_cache_ttl_seconds: u64,

    /// Require explicit permissions (deny by default)
    /// Whether `require_explicit_permissions` is enabled
    pub require_explicit_permissions: bool,
}

impl Default for CanonicalAuthorizationConfig {
    fn default() -> Self {
        Self {
            enable_rbac: true,
            default_role: "user".to_string(),
            roles: HashMap::new(),
            permissions: HashMap::new(),
            resource_rules: HashMap::new(),
            enable_abac: false,
            abac_policies: Vec::new(),
            enable_auth_cache: true,
            auth_cache_ttl_seconds: 300, // 5 minutes
            require_explicit_permissions: true,
        }
    }
}

impl CanonicalAuthorizationConfig {
    /// Create production authorization configuration
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self {
            auth_cache_ttl_seconds: 60, // Shorter cache for production
            require_explicit_permissions: true,
            ..Self::default()
        };

        // Add default production roles
        config.roles.insert(
            "admin".to_string(),
            RoleConfig {
                name: "admin".to_string(),
                description: "System administrator with full access".to_string(),
                permissions: vec!["*".to_string()],
                inherits_from: Vec::new(),
                enabled: true,
            },
        );

        config.roles.insert(
            "user".to_string(),
            RoleConfig {
                name: "user".to_string(),
                description: "Standard user with basic access".to_string(),
                permissions: vec!["read:own".to_string(), "write:own".to_string()],
                inherits_from: Vec::new(),
                enabled: true,
            },
        );

        config
    }

    /// Validate authorization configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.default_role.is_empty() {
            return Err(BearDogError::security(
                "Default role cannot be empty".to_string(),
            ));
        }

        if !self.roles.contains_key(&self.default_role) {
            return Err(BearDogError::security(format!(
                "Default role {} must be defined in roles",
                self.default_role
            )));
        }

        // Validate role configurations
        for (role_name, role_config) in &self.roles {
            if role_config.name != *role_name {
                return Err(BearDogError::security(format!(
                    "Role name mismatch: key {} != config name {}",
                    role_name, role_config.name
                )));
            }
            role_config.validate()?;
        }

        Ok(())
    }
}

/// Role configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    /// Role name
    /// Name of the item
    pub name: String,

    /// Role description
    /// The description value
    pub description: String,

    /// Permissions granted to this role
    /// Collection of permissions
    pub permissions: Vec<String>,

    /// Roles this role inherits from
    /// Collection of inherits from
    pub inherits_from: Vec<String>,

    /// Whether this role is enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

impl RoleConfig {
    /// Validate role configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.name.is_empty() {
            return Err(BearDogError::security(
                "Role name cannot be empty".to_string(),
            ));
        }

        if self.permissions.is_empty() && self.inherits_from.is_empty() {
            return Err(BearDogError::security(format!(
                "Role {} must have either permissions or inherit from other roles",
                self.name
            )));
        }

        Ok(())
    }
}

/// Permission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    /// Permission name
    /// Name of the item
    pub name: String,

    /// Permission description
    /// The description value
    pub description: String,

    /// Resource type this permission applies to
    /// The resource type value
    pub resource_type: String,

    /// Actions allowed by this permission
    /// Collection of actions
    pub actions: Vec<String>,

    /// Collection of conditions
    pub conditions: Vec<String>,
}

/// Resource access rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRuleConfig {
    /// Resource pattern (e.g., "/api/users/*")
    /// The resource pattern value
    pub resource_pattern: String,

    /// Required permissions
    /// Collection of required permissions
    pub required_permissions: Vec<String>,

    /// HTTP methods this rule applies to
    /// Collection of methods
    pub methods: Vec<String>,

    /// Additional conditions
    /// Mapping of conditions
    pub conditions: HashMap<String, String>,
}

/// Attribute-based access control policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacPolicyConfig {
    /// Policy name
    /// Name of the item
    pub name: String,

    /// Policy description
    /// The description value
    pub description: String,

    /// Subject attributes (user properties)
    /// Mapping of subject attributes
    pub subject_attributes: HashMap<String, String>,

    /// Resource attributes
    /// Mapping of resource attributes
    pub resource_attributes: HashMap<String, String>,

    /// The action value
    pub action: String,

    /// Environment attributes (time, location, etc.)
    /// Mapping of environment attributes
    pub environment_attributes: HashMap<String, String>,

    /// Policy decision (allow/deny)
    /// The decision value
    pub decision: String,
}
