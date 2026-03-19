// SPDX-License-Identifier: AGPL-3.0-only

//! Trust Management Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/sovereignty/trust
//! `TEST_PRIORITY`: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests trust establishment and verification:
    /// - Trust domain creation
    /// - Trust anchor management
    /// - Certificate chain validation
    /// - Trust revocation
    #[test]
    fn test_trust_management() {
        // Create trust domain
        let trust_domain = TrustDomain::new("example.com");
        assert_eq!(trust_domain.name(), "example.com");
        assert_eq!(trust_domain.anchor_count(), 0);

        // Add trust anchors
        let mut domain = trust_domain;
        let root_ca = TrustAnchor::new("RootCA", "root_fingerprint");
        domain.add_anchor(root_ca.clone()).unwrap();
        assert_eq!(domain.anchor_count(), 1);

        let intermediate_ca = TrustAnchor::new("IntermediateCA", "intermediate_fingerprint");
        domain.add_anchor(intermediate_ca.clone()).unwrap();
        assert_eq!(domain.anchor_count(), 2);

        // Test trust verification
        assert!(domain.trusts_anchor(&root_ca));
        assert!(domain.trusts_anchor(&intermediate_ca));

        let unknown_ca = TrustAnchor::new("UnknownCA", "unknown_fingerprint");
        assert!(!domain.trusts_anchor(&unknown_ca));

        // Create certificate chain
        let cert_chain = CertificateChain::new()
            .with_leaf("leaf_cert")
            .with_intermediate("intermediate_cert")
            .with_root("root_cert");

        assert_eq!(cert_chain.length(), 3);
        assert_eq!(cert_chain.leaf(), "leaf_cert");
        assert_eq!(cert_chain.root(), "root_cert");

        // Validate certificate chain
        let valid_chain = CertificateChain::new()
            .with_leaf("leaf")
            .with_intermediate("IntermediateCA")
            .with_root("RootCA");

        assert!(domain.validate_chain(&valid_chain).is_ok());

        // Invalid chain (unknown root)
        let invalid_chain = CertificateChain::new()
            .with_leaf("leaf")
            .with_root("UnknownRoot");

        assert!(domain.validate_chain(&invalid_chain).is_err());

        // Test trust revocation
        domain.revoke_anchor(&intermediate_ca).unwrap();
        assert!(!domain.trusts_anchor(&intermediate_ca));
        assert_eq!(domain.anchor_count(), 1);

        // Test trust expiration
        let expiring_anchor = TrustAnchor::with_expiration(
            "ExpiringCA",
            "expiring_fingerprint",
            std::time::Duration::from_nanos(1),
        );
        domain.add_anchor(expiring_anchor.clone()).unwrap();

        // Modern pattern: 1 nanosecond already elapsed by CPU cycles
        assert!(expiring_anchor.is_expired());
        assert!(!domain.is_anchor_valid(&expiring_anchor));

        // Test trust domain merging
        let other_domain = TrustDomain::new("other.com")
            .with_anchor(TrustAnchor::new("OtherCA", "other_fingerprint"));

        let merged = domain.merge(&other_domain);
        assert!(merged.anchor_count() > domain.anchor_count());
    }
}
