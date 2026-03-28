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

#[cfg(test)]
mod tests {
    use super::*;

    fn json_eq<T: serde::Serialize>(a: &T, b: &T) {
        let ja = serde_json::to_value(a).expect("serialize a");
        let jb = serde_json::to_value(b).expect("serialize b");
        assert_eq!(ja, jb, "JSON representations must match");
    }

    #[test]
    fn canonical_authorization_default_debug_clone() {
        let c = CanonicalAuthorizationConfig::default();
        let _ = format!("{c:?}");
        assert_eq!(c.default_role, "user");
    }

    #[test]
    fn canonical_authorization_production_validates() {
        let p = CanonicalAuthorizationConfig::production();
        p.validate().expect("production preset must validate");
        assert!(p.roles.contains_key("admin"));
        assert_eq!(p.auth_cache_ttl_seconds, 60);
    }

    #[test]
    fn validate_rejects_empty_default_role() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.default_role.clear();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_missing_default_role_in_roles() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.default_role = "ghost".to_string();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_role_name_mismatch() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.roles.insert(
            "key".to_string(),
            RoleConfig {
                name: "other".to_string(),
                description: "d".to_string(),
                permissions: vec!["p".to_string()],
                inherits_from: vec![],
                enabled: true,
            },
        );
        c.default_role = "key".to_string();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_role_without_permissions_or_inheritance() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.roles.insert(
            "user".to_string(),
            RoleConfig {
                name: "user".to_string(),
                description: "d".to_string(),
                permissions: vec![],
                inherits_from: vec![],
                enabled: true,
            },
        );
        assert!(c.validate().is_err());
    }

    #[test]
    fn role_config_validate_errors() {
        let bad = RoleConfig {
            name: String::new(),
            description: "d".to_string(),
            permissions: vec!["a".to_string()],
            inherits_from: vec![],
            enabled: true,
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn serde_roundtrip_authorization_and_nested() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.roles.insert(
            "user".to_string(),
            RoleConfig {
                name: "user".to_string(),
                description: "u".to_string(),
                permissions: vec!["read".to_string()],
                inherits_from: vec![],
                enabled: true,
            },
        );
        let v = serde_json::to_value(&c).expect("to value");
        let back: CanonicalAuthorizationConfig = serde_json::from_value(v).expect("from value");
        json_eq(&c, &back);

        let perm = PermissionConfig {
            name: "p1".to_string(),
            description: "d".to_string(),
            resource_type: "r".to_string(),
            actions: vec!["get".to_string()],
            conditions: vec!["c".to_string()],
        };
        json_eq(
            &perm,
            &serde_json::from_value(serde_json::to_value(&perm).unwrap()).unwrap(),
        );

        let rule = ResourceRuleConfig {
            resource_pattern: "/x/*".to_string(),
            required_permissions: vec!["p".to_string()],
            methods: vec!["GET".to_string()],
            conditions: HashMap::from([("k".to_string(), "v".to_string())]),
        };
        json_eq(
            &rule,
            &serde_json::from_value(serde_json::to_value(&rule).unwrap()).unwrap(),
        );

        let abac = AbacPolicyConfig {
            name: "pol".to_string(),
            description: "d".to_string(),
            subject_attributes: HashMap::from([("s".to_string(), "1".to_string())]),
            resource_attributes: HashMap::new(),
            action: "read".to_string(),
            environment_attributes: HashMap::new(),
            decision: "allow".to_string(),
        };
        json_eq(
            &abac,
            &serde_json::from_value(serde_json::to_value(&abac).unwrap()).unwrap(),
        );
    }

    #[test]
    fn valid_config_with_matching_role_passes() {
        let mut c = CanonicalAuthorizationConfig::default();
        c.roles.insert(
            c.default_role.clone(),
            RoleConfig {
                name: c.default_role.clone(),
                description: "d".to_string(),
                permissions: vec!["x".to_string()],
                inherits_from: vec![],
                enabled: true,
            },
        );
        c.validate().expect("valid authz config");
    }

    #[test]
    fn production_has_shorter_auth_cache_than_default() {
        let d = CanonicalAuthorizationConfig::default();
        let p = CanonicalAuthorizationConfig::production();
        assert!(p.auth_cache_ttl_seconds < d.auth_cache_ttl_seconds);
    }
}
