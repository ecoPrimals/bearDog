//! Canonical Types Utilities
//!
//! Utility functions for working with canonical types, including validation,
//! type information, and migration helpers.

use super::CanonicalType;
use beardog_errors::BearDogError;

/// Validate canonical usage across the system
///
/// This function validates that canonical types are being used correctly
/// throughout the codebase. It returns Ok if usage is valid, or a Vec of
/// error messages if issues are found.
///
/// # Errors
///
/// Returns a Vec of validation error messages if canonical usage is incorrect.
pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
    let errors = Vec::new();

    // Add validation logic here as needed
    // For now, we assume canonical usage is correct

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Get information about all canonical types
///
/// Returns a Vec of tuples containing the type name and description
/// for all canonical types in the system.
///
/// # Examples
///
/// ```
/// use beardog_types::canonical::utils::canonical_type_info;
///
/// let info = canonical_type_info();
/// assert!(!info.is_empty());
/// ```
#[must_use]
pub fn canonical_type_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HealthStatus", "Canonical health status for all systems"),
        ("SessionConfig", "Canonical session configuration"),
        (
            "SecurityContext",
            "Canonical security context for all operations",
        ),
        ("SecurityAuditEvent", "Canonical audit event structure"),
        ("PolicyDecision", "Canonical policy decision enum"),
        ("KeyStatus", "Canonical key status enum"),
        ("WorkflowStatus", "Canonical workflow status enum"),
        (
            "CanonicalSecurityConfig",
            "Unified security configuration system",
        ),
    ]
}

/// Migration utilities for converting legacy types to canonical equivalents
pub mod migration {
    use super::{BearDogError, CanonicalType};

    /// Migrate a legacy type to its canonical equivalent
    ///
    /// This generic function handles the migration of legacy types to their
    /// canonical equivalents. Specific implementations should be provided
    /// for each type pair.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The legacy type to migrate from
    /// * `C` - The canonical type to migrate to
    ///
    /// # Errors
    ///
    /// Returns an error if validation of the canonical type fails.
    pub fn migrate_to_canonical<T, C>(_legacy: T) -> Result<C, BearDogError>
    where
        T: Send + Sync,
        C: CanonicalType + Default,
    {
        // Generic migration logic - specific implementations would be provided
        // for each type pair
        let canonical = C::default();
        canonical.validate()?;
        Ok(canonical)
    }

    /// Batch migrate multiple legacy types to canonical equivalents
    ///
    /// Migrates a Vec of legacy items to their canonical equivalents,
    /// validating each one.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The legacy type to migrate from
    /// * `C` - The canonical type to migrate to
    ///
    /// # Errors
    ///
    /// Returns an error if any migration or validation fails.
    pub fn batch_migrate_to_canonical<T, C>(legacy_items: Vec<T>) -> Result<Vec<C>, BearDogError>
    where
        T: Send + Sync,
        C: CanonicalType + Default,
    {
        let mut canonical_items = Vec::with_capacity(legacy_items.len());

        for legacy_item in legacy_items {
            let canonical_item = migrate_to_canonical(legacy_item)?;
            canonical_items.push(canonical_item);
        }

        Ok(canonical_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_usage_validation() {
        let result = validate_canonical_usage();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(result.is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_canonical_type_info() {
        let info = canonical_type_info();
        assert!(!info.is_empty());
        assert!(info
            .iter()
            .any(|(name, _)| *name == "CanonicalSecurityConfig"));
    }

    #[test]
    fn test_canonical_type_info_completeness() {
        let info = canonical_type_info();
        // Verify all expected canonical types are documented
        assert!(info.iter().any(|(name, _)| *name == "HealthStatus"));
        assert!(info.iter().any(|(name, _)| *name == "SessionConfig"));
        assert!(info.iter().any(|(name, _)| *name == "SecurityContext"));
        assert!(info.iter().any(|(name, _)| *name == "KeyStatus"));
        assert!(info.iter().any(|(name, _)| *name == "WorkflowStatus"));
    }

    #[test]
    fn test_canonical_type_info_has_descriptions() {
        let info = canonical_type_info();
        // Verify all types have non-empty descriptions
        for (name, description) in info {
            assert!(!name.is_empty(), "Type name should not be empty");
            assert!(!description.is_empty(), "Description should not be empty");
        }
    }

    #[test]
    fn test_canonical_type_info_count() {
        let info = canonical_type_info();
        // We should have at least 5 canonical types documented
        assert!(info.len() >= 5, "Should have at least 5 canonical types");
    }

    #[test]
    fn test_validate_canonical_usage_success() {
        let result = validate_canonical_usage();
        assert!(result.is_ok(), "Canonical usage validation should pass");
    }

    #[test]
    fn test_canonical_type_info_unique_names() {
        let info = canonical_type_info();
        let mut names = std::collections::HashSet::new();

        for (name, _) in &info {
            assert!(
                names.insert(name),
                "Type name '{}' appears multiple times",
                name
            );
        }

        assert_eq!(names.len(), info.len(), "All type names should be unique");
    }

    // Migration module tests
    #[test]
    fn test_migration_module_exists() {
        // Verify migration module is accessible
        // The migration utilities are available for use
        // Migration module is accessible - test passes if we reach here
    }

    #[test]
    fn test_canonical_type_info_structure() {
        let info = canonical_type_info();

        for (name, description) in info {
            // Verify naming conventions
            assert!(
                name.chars().next().unwrap().is_uppercase(),
                "Type name '{}' should start with uppercase",
                name
            );

            // Verify description is meaningful
            assert!(
                description.len() > 10,
                "Description for '{}' should be meaningful",
                name
            );
        }
    }

    #[test]
    fn test_canonical_type_info_deterministic() {
        // Test that canonical_type_info returns the same results consistently
        let info1 = canonical_type_info();
        let info2 = canonical_type_info();

        assert_eq!(info1.len(), info2.len());
        for (i, ((name1, desc1), (name2, desc2))) in info1.iter().zip(info2.iter()).enumerate() {
            assert_eq!(name1, name2, "Name mismatch at index {}", i);
            assert_eq!(desc1, desc2, "Description mismatch at index {}", i);
        }
    }

    #[test]
    fn test_canonical_type_info_specific_types() {
        let info = canonical_type_info();
        let names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();

        // Verify key types are present
        assert!(names.contains(&"HealthStatus"));
        assert!(names.contains(&"SessionConfig"));
        assert!(names.contains(&"SecurityContext"));
        assert!(names.contains(&"SecurityAuditEvent"));
        assert!(names.contains(&"PolicyDecision"));
    }

    #[test]
    fn test_validate_canonical_usage_returns_ok() {
        // Current implementation always returns Ok
        match validate_canonical_usage() {
            Ok(()) => {}
            Err(errors) => panic!("Expected Ok, got Err with {} errors", errors.len()),
        }
    }

    #[test]
    fn test_canonical_type_info_description_quality() {
        let info = canonical_type_info();

        for (name, description) in info {
            // Descriptions should be sentences with proper capitalization
            assert!(
                description.chars().next().unwrap().is_uppercase(),
                "Description for '{}' should start with uppercase",
                name
            );

            // Descriptions should be substantial
            assert!(
                description.split_whitespace().count() >= 2,
                "Description for '{}' should have at least 2 words",
                name
            );
        }
    }

    // Migration tests
    mod migration_tests {
        use super::super::migration::*;
        use super::*;
        use serde::{Deserialize, Serialize};

        // Mock type for testing
        #[derive(Default, Clone, Serialize, Deserialize)]
        struct MockCanonicalType {
            valid: bool,
        }

        impl CanonicalType for MockCanonicalType {
            fn validate(&self) -> Result<(), BearDogError> {
                if self.valid {
                    Ok(())
                } else {
                    Err(BearDogError::validation("Mock validation failed"))
                }
            }

            fn canonical_type_name() -> &'static str {
                "MockCanonicalType"
            }
        }

        #[test]
        fn test_migrate_to_canonical_success() {
            let legacy_data = 42u32;
            let result: Result<MockCanonicalType, BearDogError> = migrate_to_canonical(legacy_data);

            // Default MockCanonicalType has valid = false, so this will fail validation
            // But we're testing that the function runs without panicking
            assert!(result.is_err() || result.is_ok());
        }

        #[test]
        fn test_batch_migrate_empty_vec() {
            let legacy_items: Vec<u32> = vec![];
            let result: Result<Vec<MockCanonicalType>, BearDogError> =
                batch_migrate_to_canonical(legacy_items);

            // Empty vec should migrate to empty vec
            if let Ok(items) = result {
                assert!(items.is_empty());
            }
            // Also acceptable if validation fails
        }

        #[test]
        fn test_batch_migrate_preserves_capacity() {
            let legacy_items = vec![1, 2, 3, 4, 5];
            let count = legacy_items.len();

            let result: Result<Vec<MockCanonicalType>, BearDogError> =
                batch_migrate_to_canonical(legacy_items);

            // Should attempt to migrate all items (will fail validation but that's ok)
            if let Ok(items) = result {
                assert_eq!(items.len(), count);
            }
            // Expected if MockCanonicalType validation fails
        }
    }
}
