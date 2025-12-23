//! Compliance Sovereignty Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/sovereignty/compliance
//! `TEST_PRIORITY`: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 1: Compliance Sovereignty Checks
    ///
    /// Tests compliance-based sovereignty enforcement:
    /// - Data residency requirements
    /// - Jurisdictional compliance
    /// - Regulatory framework validation
    /// - Cross-border data transfer rules
    #[test]
    fn test_compliance_sovereignty_checks() {
        // Create sovereignty policy for EU/GDPR
        let eu_policy = SovereigntyPolicy::new("EU")
            .with_jurisdiction(Jurisdiction::EuropeanUnion)
            .with_regulation(Regulation::GDPR)
            .with_data_residency(DataResidency::StrictLocal);

        assert_eq!(eu_policy.region(), "EU");
        assert_eq!(eu_policy.jurisdiction(), Jurisdiction::EuropeanUnion);
        assert!(eu_policy.requires_local_storage());

        // Test data residency validation
        let eu_data = DataLocation::new("user_data", "EU", "Frankfurt");
        assert!(eu_policy.validate_data_location(&eu_data).is_ok());

        // Test cross-border transfer (should fail for strict policy)
        let us_data = DataLocation::new("user_data", "US", "Virginia");
        let result = eu_policy.validate_data_location(&us_data);
        assert!(result.is_err(), "Cross-border transfer should be blocked");

        // Create US policy with different requirements
        let us_policy = SovereigntyPolicy::new("US")
            .with_jurisdiction(Jurisdiction::UnitedStates)
            .with_regulation(Regulation::CCPA)
            .with_data_residency(DataResidency::FlexibleRegional);

        assert_eq!(us_policy.jurisdiction(), Jurisdiction::UnitedStates);
        assert!(!us_policy.requires_local_storage());

        // US policy should allow regional storage
        let canada_data = DataLocation::new("user_data", "CA", "Toronto");
        assert!(us_policy.validate_data_location(&canada_data).is_ok());

        // Test multi-region policy
        let global_policy = SovereigntyPolicy::new("Global")
            .with_jurisdiction(Jurisdiction::International)
            .with_regulation(Regulation::ISO27001)
            .with_data_residency(DataResidency::Global);

        assert!(global_policy.validate_data_location(&eu_data).is_ok());
        assert!(global_policy.validate_data_location(&us_data).is_ok());

        // Test compliance framework requirements
        let frameworks = eu_policy.required_frameworks();
        assert!(frameworks.contains(&"GDPR"));

        // Test audit requirements
        assert!(eu_policy.requires_audit_trail());
        assert_eq!(eu_policy.audit_retention_days(), 2555); // 7 years for GDPR
    }
}
