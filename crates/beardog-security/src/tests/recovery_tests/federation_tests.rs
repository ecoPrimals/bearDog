//! Recovery Federation Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 5: Federation Recovery
    ///
    /// Tests federated recovery across multiple systems:
    /// - Multi-system coordination
    /// - Cross-system verification
    /// - Federation consensus
    /// - Distributed recovery
    #[test]
    fn test_federation_recovery() {
        // Create federation members
        let members = vec![
            FederationMember::new("member_1", "node1.example.com"),
            FederationMember::new("member_2", "node2.example.com"),
            FederationMember::new("member_3", "node3.example.com"),
            FederationMember::new("member_4", "node4.example.com"),
            FederationMember::new("member_5", "node5.example.com"),
        ];

        // Create federation with 3-of-5 consensus
        let federation = Federation::new(members.clone(), 3);
        assert_eq!(federation.member_count(), 5);
        assert_eq!(federation.consensus_threshold(), 3);

        // Initiate federated recovery
        let recovery = FederatedRecovery::new("user_123", &federation);
        assert_eq!(recovery.confirmations(), 0);
        assert!(!recovery.has_consensus(&federation));

        // Member 1 confirms
        let mut recovery = recovery;
        recovery.add_confirmation(&members[0]).unwrap();
        assert_eq!(recovery.confirmations(), 1);

        // Member 3 confirms
        recovery.add_confirmation(&members[2]).unwrap();
        assert_eq!(recovery.confirmations(), 2);

        // Member 5 confirms - consensus reached!
        recovery.add_confirmation(&members[4]).unwrap();
        assert_eq!(recovery.confirmations(), 3);
        assert!(recovery.has_consensus(&federation));

        // Verify quorum reached
        assert!(recovery.is_complete(&federation));

        // Test that duplicate confirmation fails
        let result = recovery.add_confirmation(&members[0]);
        assert!(result.is_err());

        // Test with different thresholds
        let strict_federation = Federation::new(members.clone(), 5);
        let strict_recovery = FederatedRecovery::new("user_456", &strict_federation);

        // Need all 5 confirmations
        assert!(!strict_recovery.has_consensus(&strict_federation));

        let mut strict_recovery = strict_recovery;
        for member in &members {
            strict_recovery.add_confirmation(member).unwrap();
        }
        assert!(strict_recovery.has_consensus(&strict_federation));
    }
}
