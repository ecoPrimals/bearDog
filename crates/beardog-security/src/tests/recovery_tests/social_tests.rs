//! Recovery Social Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 2: Social Recovery Workflow
    ///
    /// Tests social recovery mechanisms:
    /// - Guardian selection
    /// - Recovery initiation
    /// - Guardian approval collection
    /// - Recovery completion
    #[test]
    fn test_social_recovery_workflow() {
        // Setup guardians
        let guardians = vec![
            Guardian::new("guardian_1", "alice@example.com"),
            Guardian::new("guardian_2", "bob@example.com"),
            Guardian::new("guardian_3", "carol@example.com"),
            Guardian::new("guardian_4", "dave@example.com"),
            Guardian::new("guardian_5", "eve@example.com"),
        ];

        // Create social recovery config (3 of 5 guardians)
        let recovery_config = SocialRecoveryConfig::new(guardians.clone(), 3);
        assert_eq!(recovery_config.guardian_count(), 5);
        assert_eq!(recovery_config.threshold(), 3);

        // Initiate recovery
        let recovery_request = RecoveryRequest::new("user_123");
        assert_eq!(recovery_request.user_id(), "user_123");
        assert_eq!(recovery_request.approvals(), 0);
        assert!(!recovery_request.is_complete(&recovery_config));

        // Guardian 1 approves
        let mut request = recovery_request;
        request.add_approval(&guardians[0]).unwrap();
        assert_eq!(request.approvals(), 1);
        assert!(!request.is_complete(&recovery_config));

        // Guardian 3 approves
        request.add_approval(&guardians[2]).unwrap();
        assert_eq!(request.approvals(), 2);
        assert!(!request.is_complete(&recovery_config));

        // Guardian 5 approves - threshold reached!
        request.add_approval(&guardians[4]).unwrap();
        assert_eq!(request.approvals(), 3);
        assert!(request.is_complete(&recovery_config));

        // Verify cannot approve twice
        let result = request.add_approval(&guardians[0]);
        assert!(result.is_err());

        // Test with 2-of-3 config
        let simple_config = SocialRecoveryConfig::new(guardians[0..3].to_vec(), 2);
        let mut simple_request = RecoveryRequest::new("user_456");

        simple_request.add_approval(&guardians[0]).unwrap();
        assert!(!simple_request.is_complete(&simple_config));

        simple_request.add_approval(&guardians[1]).unwrap();
        assert!(simple_request.is_complete(&simple_config));
    }
}
