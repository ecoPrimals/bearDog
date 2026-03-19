// SPDX-License-Identifier: AGPL-3.0-only

//! Sovereignty Edge Cases Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/sovereignty/edge-cases
//! `TEST_PRIORITY`: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests edge cases and boundary conditions:
    /// - Conflicting policies
    /// - Policy inheritance
    /// - Dynamic policy updates
    /// - Cross-jurisdictional scenarios
    #[test]
    fn test_sovereignty_edge_cases() {
        // Test conflicting policies
        let strict_policy = SovereigntyPolicy::new("Strict")
            .with_data_residency(DataResidency::StrictLocal)
            .with_jurisdiction(Jurisdiction::EuropeanUnion);

        let relaxed_policy = SovereigntyPolicy::new("Relaxed")
            .with_data_residency(DataResidency::Global)
            .with_jurisdiction(Jurisdiction::International);

        // Test policy conflict resolution (strictest wins)
        let resolved = PolicyResolver::resolve(&[strict_policy.clone(), relaxed_policy.clone()]);
        assert_eq!(resolved.data_residency(), DataResidency::StrictLocal);

        // Test empty policy set
        let empty_resolved = PolicyResolver::resolve(&[]);
        assert!(empty_resolved.is_default());

        // Test policy inheritance
        let parent_policy =
            SovereigntyPolicy::new("Parent").with_data_residency(DataResidency::FlexibleRegional);

        let child_policy = SovereigntyPolicy::new("Child")
            .inherit_from(&parent_policy)
            .with_jurisdiction(Jurisdiction::UnitedStates);

        assert_eq!(
            child_policy.data_residency(),
            DataResidency::FlexibleRegional
        );
        assert_eq!(child_policy.jurisdiction(), Jurisdiction::UnitedStates);

        // Test dynamic policy updates
        let mut dynamic_policy =
            SovereigntyPolicy::new("Dynamic").with_data_residency(DataResidency::StrictLocal);

        assert!(dynamic_policy.requires_local_storage());

        dynamic_policy.update_residency(DataResidency::FlexibleRegional);
        assert!(!dynamic_policy.requires_local_storage());

        // Test cross-jurisdictional data flow
        let source_jurisdiction = Jurisdiction::EuropeanUnion;
        let dest_jurisdiction = Jurisdiction::UnitedStates;

        let flow = DataFlow::new(source_jurisdiction, dest_jurisdiction);

        // EU to US transfer requires special handling (Privacy Shield, Standard Contractual Clauses)
        assert!(flow.requires_special_authorization());
        assert!(flow.needs_consent());

        // Same jurisdiction flow
        let same_jurisdiction_flow =
            DataFlow::new(Jurisdiction::EuropeanUnion, Jurisdiction::EuropeanUnion);
        assert!(!same_jurisdiction_flow.requires_special_authorization());

        // Test policy validation with invalid configuration
        let invalid_policy_result = SovereigntyPolicy::new("").validate();
        assert!(invalid_policy_result.is_err());

        let valid_policy =
            SovereigntyPolicy::new("Valid").with_jurisdiction(Jurisdiction::UnitedStates);
        assert!(valid_policy.validate().is_ok());

        // Test maximum policy complexity
        let complex_policy = SovereigntyPolicy::new("Complex")
            .with_jurisdiction(Jurisdiction::EuropeanUnion)
            .with_regulation(Regulation::GDPR)
            .with_regulation(Regulation::ISO27001)
            .with_data_residency(DataResidency::StrictLocal);

        assert_eq!(complex_policy.regulation_count(), 2);
        assert!(complex_policy.has_regulation(Regulation::GDPR));
        assert!(complex_policy.has_regulation(Regulation::ISO27001));

        // Test policy with all features
        let comprehensive_policy = SovereigntyPolicy::new("Comprehensive")
            .with_jurisdiction(Jurisdiction::EuropeanUnion)
            .with_regulation(Regulation::GDPR)
            .with_data_residency(DataResidency::StrictLocal);

        assert!(comprehensive_policy.is_comprehensive());
        assert!(comprehensive_policy.validate().is_ok());
    }
}
