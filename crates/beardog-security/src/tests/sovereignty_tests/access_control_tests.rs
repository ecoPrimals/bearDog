//! Access Control Sovereignty Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/sovereignty/access
//! `TEST_PRIORITY`: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests sovereignty-based access control:
    /// - Geographic access restrictions
    /// - Identity sovereignty
    /// - Attribute-based access control (ABAC)
    /// - Sovereignty-aware permissions
    #[test]
    fn test_access_control_sovereignty() {
        // Create sovereignty-aware access policy
        let access_policy = SovereignAccessPolicy::new()
            .with_allowed_region("EU")
            .with_allowed_region("US")
            .with_identity_verification(IdentityLevel::High);

        assert_eq!(access_policy.allowed_regions().len(), 2);
        assert!(access_policy.allows_region("EU"));
        assert!(access_policy.allows_region("US"));
        assert!(!access_policy.allows_region("CN"));

        // Test geographic access control
        let eu_request = AccessRequest::new("user_1", "EU", "resource_1");
        assert!(access_policy.validate_access(&eu_request).is_ok());

        let cn_request = AccessRequest::new("user_2", "CN", "resource_1");
        assert!(access_policy.validate_access(&cn_request).is_err());

        // Test identity sovereignty
        let verified_identity = Identity::new("user_3")
            .with_level(IdentityLevel::High)
            .with_issuer("TrustedAuthority")
            .with_region("US");

        assert!(access_policy.validate_identity(&verified_identity).is_ok());

        let unverified_identity = Identity::new("user_4")
            .with_level(IdentityLevel::Low)
            .with_issuer("Unknown");

        assert!(access_policy
            .validate_identity(&unverified_identity)
            .is_err());

        // Test attribute-based access control
        let mut attributes = AttributeSet::new();
        attributes.add("clearance", "top_secret");
        attributes.add("department", "security");
        attributes.add("region", "EU");

        let abac_policy = ABACPolicy::new()
            .require_attribute("clearance", "top_secret")
            .require_attribute("region", "EU");

        assert!(abac_policy.evaluate(&attributes).is_ok());

        // Missing required attribute
        let mut incomplete_attrs = AttributeSet::new();
        incomplete_attrs.add("clearance", "top_secret");
        assert!(abac_policy.evaluate(&incomplete_attrs).is_err());

        // Wrong attribute value
        let mut wrong_attrs = AttributeSet::new();
        wrong_attrs.add("clearance", "public");
        wrong_attrs.add("region", "EU");
        assert!(abac_policy.evaluate(&wrong_attrs).is_err());

        // Test sovereignty delegation
        let delegator = Identity::new("admin")
            .with_level(IdentityLevel::High)
            .with_region("EU");

        let delegatee = Identity::new("operator")
            .with_level(IdentityLevel::Medium)
            .with_region("EU");

        let delegation = Delegation::new(&delegator, &delegatee)
            .with_permission("read")
            .with_expiration(std::time::Duration::from_secs(3600));

        assert!(!delegation.is_expired());
        assert!(delegation.has_permission("read"));
        assert!(!delegation.has_permission("write"));

        // Same-region delegation should succeed
        assert!(delegation.is_valid());

        // Cross-region delegation test
        let cross_region_delegatee = Identity::new("foreign_operator")
            .with_level(IdentityLevel::Medium)
            .with_region("US");

        let cross_delegation =
            Delegation::new(&delegator, &cross_region_delegatee).with_permission("read");

        // This should be allowed (both EU and US are in allowed regions)
        assert!(cross_delegation.is_valid());
    }
}
