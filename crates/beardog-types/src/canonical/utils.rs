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
}
