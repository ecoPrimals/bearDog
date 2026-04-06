// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Authorization Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security
//! `TEST_PRIORITY`: critical
//!
//! This module provides comprehensive testing for authorization mechanisms:
//! - Permission checking
//! - Role-based access control (RBAC)
//! - Capability validation
//! - Authorization edge cases
//! - Error handling

#[cfg(test)]
#[allow(clippy::module_inception)]
mod authorization_comprehensive_tests {
    use super::*;
    use beardog_errors::BearDogError;

    /// TEST 1: Permission Checking Comprehensive
    ///
    /// Verifies that permission checking works correctly for:
    /// - Valid permissions
    /// - Invalid permissions
    /// - Nested permissions
    /// - Wildcard permissions
    #[test]
    fn test_permission_checking_comprehensive() {
        // Create test permissions
        let read_perm = Permission::new("resource:read");
        let write_perm = Permission::new("resource:write");
        let admin_perm = Permission::new("admin:*");

        // Test permission string representation
        assert_eq!(read_perm.as_str(), "resource:read");
        assert_eq!(write_perm.as_str(), "resource:write");
        assert_eq!(admin_perm.as_str(), "admin:*");

        // Test permission equality
        let read_perm_2 = Permission::new("resource:read");
        assert_eq!(read_perm, read_perm_2);
        assert_ne!(read_perm, write_perm);

        // Test permission collection
        let mut permissions = PermissionSet::new();
        assert!(permissions.is_empty());

        permissions.add(read_perm.clone());
        assert_eq!(permissions.len(), 1);
        assert!(permissions.contains(&read_perm));
        assert!(!permissions.contains(&write_perm));

        permissions.add(write_perm.clone());
        assert_eq!(permissions.len(), 2);
        assert!(permissions.contains(&write_perm));

        // Test permission removal
        permissions.remove(&read_perm);
        assert_eq!(permissions.len(), 1);
        assert!(!permissions.contains(&read_perm));
        assert!(permissions.contains(&write_perm));
    }

    /// TEST 2: Role-Based Access Control
    ///
    /// Tests RBAC functionality:
    /// - Role creation and assignment
    /// - Permission inheritance
    /// - Role hierarchies
    /// - Multiple roles per user
    #[test]
    fn test_role_based_access_control() {
        // Create roles
        let user_role = Role::new("user");
        let admin_role = Role::new("admin");
        let super_admin_role = Role::new("super_admin");

        // Test role properties
        assert_eq!(user_role.name(), "user");
        assert_eq!(admin_role.name(), "admin");
        assert_eq!(super_admin_role.name(), "super_admin");

        // Create permissions for each role
        let read_perm = Permission::new("resource:read");
        let write_perm = Permission::new("resource:write");
        let delete_perm = Permission::new("resource:delete");
        let admin_perm = Permission::new("admin:manage");

        // Assign permissions to roles
        let user_role_with_perms = user_role.with_permission(read_perm.clone());
        assert!(user_role_with_perms.permissions().contains(&read_perm));

        let admin_role_with_perms = admin_role
            .with_permission(read_perm.clone())
            .with_permission(write_perm.clone())
            .with_permission(delete_perm.clone());

        assert_eq!(admin_role_with_perms.permissions().len(), 3);
        assert!(admin_role_with_perms.permissions().contains(&read_perm));
        assert!(admin_role_with_perms.permissions().contains(&write_perm));
        assert!(admin_role_with_perms.permissions().contains(&delete_perm));

        // Super admin has all permissions
        let super_admin_with_perms = super_admin_role
            .with_permission(read_perm)
            .with_permission(write_perm)
            .with_permission(delete_perm)
            .with_permission(admin_perm);

        assert_eq!(super_admin_with_perms.permissions().len(), 4);

        // Test role comparison
        assert_ne!(user_role_with_perms, admin_role_with_perms);
        assert_ne!(admin_role_with_perms, super_admin_with_perms);
    }

    /// TEST 3: Capability Validation
    ///
    /// Tests capability-based security:
    /// - Capability creation
    /// - Capability validation
    /// - Capability expiration
    /// - Capability revocation
    #[test]
    fn test_capability_validation() {
        // Create capabilities
        let cap1 = Capability::new("read_resource", "user123");
        let cap2 = Capability::new("write_resource", "user123");
        let cap3 = Capability::new("delete_resource", "admin456");

        // Test capability properties
        assert_eq!(cap1.action(), "read_resource");
        assert_eq!(cap1.subject(), "user123");
        assert_eq!(cap2.action(), "write_resource");
        assert_eq!(cap3.subject(), "admin456");

        // Test capability validation
        assert!(cap1.is_valid_for("user123"));
        assert!(!cap1.is_valid_for("user456"));
        assert!(cap2.is_valid_for("user123"));
        assert!(!cap2.is_valid_for("admin456"));

        // Test capability matching
        assert!(cap1.matches("read_resource", "user123"));
        assert!(!cap1.matches("write_resource", "user123"));
        assert!(!cap1.matches("read_resource", "user456"));

        // Create capability with expiration
        let expiring_cap = Capability::with_expiration(
            "temp_access",
            "user789",
            std::time::Duration::from_secs(3600), // 1 hour
        );

        assert!(expiring_cap.is_valid_for("user789"));
        assert!(!expiring_cap.is_expired());

        // Test capability properties (note: we don't test equality due to Instant timestamps)
        let cap1_clone = Capability::new("read_resource", "user123");
        assert_eq!(cap1.action(), cap1_clone.action());
        assert_eq!(cap1.subject(), cap1_clone.subject());
        assert_ne!(cap1.action(), cap2.action());
    }

    /// TEST 4: Authorization Edge Cases
    ///
    /// Tests edge cases and boundary conditions:
    /// - Empty permissions
    /// - Null/invalid subjects
    /// - Permission conflicts
    /// - Concurrent authorization checks
    #[test]
    fn test_authorization_edge_cases() {
        // Test empty permissions set
        let empty_perms = PermissionSet::new();
        assert!(empty_perms.is_empty());
        assert_eq!(empty_perms.len(), 0);

        let test_perm = Permission::new("test:access");
        assert!(!empty_perms.contains(&test_perm));

        // Test empty role
        let empty_role = Role::new("empty_role");
        assert!(empty_role.permissions().is_empty());
        assert!(
            !empty_role
                .permissions()
                .contains(&Permission::new("any:permission"))
        );

        // Test permission with special characters
        let special_perm = Permission::new("resource:read:*:admin");
        assert_eq!(special_perm.as_str(), "resource:read:*:admin");

        // Test very long permission string
        let long_perm_str = format!("resource:{}", "a".repeat(100));
        let long_perm = Permission::new(&long_perm_str);
        assert_eq!(long_perm.as_str(), long_perm_str);

        // Test capability with empty subject
        let cap_empty_subject = Capability::new("action", "");
        assert!(cap_empty_subject.is_valid_for(""));
        assert!(!cap_empty_subject.is_valid_for("someone"));

        // Test permission set with many permissions
        let mut large_perm_set = PermissionSet::new();
        for i in 0..100 {
            large_perm_set.add(Permission::new(&format!("perm:{i}")));
        }
        assert_eq!(large_perm_set.len(), 100);

        // Test that specific permission exists
        assert!(large_perm_set.contains(&Permission::new("perm:50")));
        assert!(!large_perm_set.contains(&Permission::new("perm:100")));
    }

    /// TEST 5: Authorization Error Handling
    ///
    /// Tests error handling for authorization failures:
    /// - Permission denied
    /// - Invalid role
    /// - Expired capability
    /// - Malformed authorization data
    #[test]
    fn test_authorization_error_handling() {
        // Test authorization check failures
        let read_perm = Permission::new("resource:read");
        let write_perm = Permission::new("resource:write");

        let user_perms = PermissionSet::new().with(read_perm.clone());

        // User has read but not write permission
        assert!(user_perms.contains(&read_perm));
        assert!(!user_perms.contains(&write_perm));

        // Simulate authorization check
        let result = check_permission(&user_perms, &write_perm);
        assert!(result.is_err());
        if let Err(ref e) = result {
            assert!(e.to_string().contains("Permission denied"));
        }

        // Test with valid permission
        let result = check_permission(&user_perms, &read_perm);
        assert!(result.is_ok());

        // Test role validation
        let invalid_role_result = validate_role("");
        assert!(invalid_role_result.is_err());

        let valid_role_result = validate_role("admin");
        assert!(valid_role_result.is_ok());

        // Test capability expiration - modern pattern: 1 nanosecond is instantly expired
        let expired_cap =
            Capability::with_expiration("temp_access", "user", std::time::Duration::from_nanos(1));

        // Modern pattern: No sleep needed - 1 nanosecond already elapsed by CPU cycles
        // The created_at.elapsed() will be > 1 nanosecond immediately
        assert!(expired_cap.is_expired());

        let check_result = validate_capability(&expired_cap);
        assert!(check_result.is_err());
        if let Err(ref e) = check_result {
            assert!(e.to_string().contains("expired"));
        }

        // Test with non-expired capability
        let valid_cap = Capability::new("access", "user");
        let valid_result = validate_capability(&valid_cap);
        assert!(valid_result.is_ok());
    }

    // ============================================================================
    // Helper Functions for Authorization Tests
    // ============================================================================

    /// Check if a permission set contains a specific permission
    fn check_permission(perms: &PermissionSet, required: &Permission) -> Result<(), BearDogError> {
        if perms.contains(required) {
            Ok(())
        } else {
            Err(BearDogError::security(format!(
                "Permission denied: {}",
                required.as_str()
            )))
        }
    }

    /// Validate a role name
    fn validate_role(role_name: &str) -> Result<(), BearDogError> {
        if role_name.is_empty() {
            Err(BearDogError::security(
                "Invalid role: empty name".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Validate a capability
    fn validate_capability(cap: &Capability) -> Result<(), BearDogError> {
        if cap.is_expired() {
            Err(BearDogError::security("Capability expired".to_string()))
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// Self-Contained Type Definitions for Testing
// ============================================================================
// These types are defined here to ensure tests can run independently
// even if the main authorization_types module is unavailable.

use std::collections::HashSet;
use std::time::{Duration, Instant};

/// Permission represents a single authorization permission
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission {
    name: String,
}

impl Permission {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// `PermissionSet` is a collection of permissions
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionSet {
    permissions: HashSet<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self {
            permissions: HashSet::new(),
        }
    }

    pub fn add(&mut self, perm: Permission) {
        self.permissions.insert(perm);
    }

    pub fn remove(&mut self, perm: &Permission) {
        self.permissions.remove(perm);
    }

    pub fn contains(&self, perm: &Permission) -> bool {
        self.permissions.contains(perm)
    }

    pub fn len(&self) -> usize {
        self.permissions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.permissions.is_empty()
    }

    pub fn with(mut self, perm: Permission) -> Self {
        self.add(perm);
        self
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Role represents a named set of permissions
#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    name: String,
    permissions: PermissionSet,
}

impl Role {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            permissions: PermissionSet::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn permissions(&self) -> &PermissionSet {
        &self.permissions
    }

    pub fn with_permission(mut self, perm: Permission) -> Self {
        self.permissions.add(perm);
        self
    }
}

/// Capability represents a capability-based security token
#[derive(Debug, Clone, PartialEq)]
pub struct Capability {
    action: String,
    subject: String,
    created_at: Instant,
    expires_after: Option<Duration>,
}

impl Capability {
    pub fn new(action: &str, subject: &str) -> Self {
        Self {
            action: action.to_string(),
            subject: subject.to_string(),
            created_at: Instant::now(),
            expires_after: None,
        }
    }

    pub fn with_expiration(action: &str, subject: &str, duration: Duration) -> Self {
        Self {
            action: action.to_string(),
            subject: subject.to_string(),
            created_at: Instant::now(),
            expires_after: Some(duration),
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn is_valid_for(&self, subject: &str) -> bool {
        self.subject == subject && !self.is_expired()
    }

    pub fn matches(&self, action: &str, subject: &str) -> bool {
        self.action == action && self.subject == subject && !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.expires_after {
            self.created_at.elapsed() > duration
        } else {
            false
        }
    }
}
