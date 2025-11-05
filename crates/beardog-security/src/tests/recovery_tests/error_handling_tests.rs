//! Recovery Error_handling Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 8: Recovery Error Handling
    ///
    /// Tests error handling for recovery operations:
    /// - Invalid shard combinations
    /// - Expired recovery attempts
    /// - Insufficient permissions
    /// - Policy violations
    #[test]
    fn test_recovery_error_handling() {
        // Test invalid shard configuration
        let result = ShardConfig::new(2, 3); // threshold > total
        assert!(result.is_err());

        let result = ShardConfig::new(0, 0); // zero shards
        assert!(result.is_err());

        let result = ShardConfig::new(5, 1); // valid
        assert!(result.is_ok());

        // Test insufficient shards for reconstruction
        let config = ShardConfig::new(5, 3).unwrap();
        let secret = b"test_secret";
        let shards = create_shards(secret, &config).unwrap();

        // Single shard should fail (need at least 2 in our simple implementation)
        let insufficient = vec![shards[0].clone()];
        let result = reconstruct_from_shards(&insufficient);
        assert!(result.is_err(), "Single shard should be insufficient");
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(msg.contains("Insufficient") || msg.contains("need at least"));
        }

        // Test expired recovery session
        let expired_session = RecoverySession::new("user_test", std::time::Duration::from_nanos(1));
        std::thread::sleep(std::time::Duration::from_millis(10));

        let result = expired_session.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("expired"));
        }

        // Test policy violation - too many attempts
        let policy = RecoveryPolicy::new().with_max_attempts(2);
        let mut tracker = RecoveryAttemptTracker::new();

        tracker.record_attempt("user_limit");
        tracker.record_attempt("user_limit");
        tracker.record_attempt("user_limit"); // 3rd attempt

        assert!(!tracker.under_attempt_limit("user_limit", &policy));

        // Test invalid guardian approval
        let guardian = Guardian::new("guardian_1", "test@example.com");
        let mut request = RecoveryRequest::new("user_test");

        // First approval succeeds
        assert!(request.add_approval(&guardian).is_ok());

        // Duplicate approval fails
        let result = request.add_approval(&guardian);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                e.to_string().contains("already approved") || e.to_string().contains("duplicate")
            );
        }
    }
}
