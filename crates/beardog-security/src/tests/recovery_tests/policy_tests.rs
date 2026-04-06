// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recovery Policy Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 4: Recovery Policy Enforcement
    ///
    /// Tests recovery policy validation and enforcement:
    /// - Policy creation
    /// - Policy validation
    /// - Cooldown periods
    /// - Attempt limits
    #[test]
    fn test_recovery_policy_enforcement() {
        // Create recovery policy
        let policy = RecoveryPolicy::new()
            .with_cooldown(std::time::Duration::from_secs(3600)) // 1 hour
            .with_max_attempts(3)
            .with_guardian_threshold(3, 5);

        // Verify policy settings
        assert_eq!(
            policy.cooldown_duration(),
            std::time::Duration::from_secs(3600)
        );
        assert_eq!(policy.max_attempts(), 3);
        assert_eq!(policy.guardian_threshold(), 3);

        // Test policy validation - valid request
        let valid_request = RecoveryRequest::new("user_123");
        assert!(policy.validate_request(&valid_request).is_ok());

        // Test cooldown enforcement
        let mut tracker = RecoveryAttemptTracker::new();

        // First attempt - allowed
        assert!(tracker.can_attempt("user_123", &policy));
        tracker.record_attempt("user_123");

        // Second attempt immediately - blocked by cooldown
        assert!(!tracker.can_attempt("user_123", &policy));

        // Simulate time passing (in real system)
        // For test, we'll just check the logic

        // Test attempt limit
        tracker.record_attempt("user_456");
        assert!(tracker.attempt_count("user_456") == 1);

        tracker.record_attempt("user_456");
        tracker.record_attempt("user_456");
        assert!(tracker.attempt_count("user_456") == 3);

        // Fourth attempt should fail (max 3)
        assert!(!tracker.under_attempt_limit("user_456", &policy));

        // Different user should be independent
        assert!(tracker.can_attempt("user_789", &policy));

        // Test policy with stricter settings
        let strict_policy = RecoveryPolicy::new()
            .with_cooldown(std::time::Duration::from_secs(86400)) // 24 hours
            .with_max_attempts(1)
            .with_guardian_threshold(5, 7);

        assert_eq!(strict_policy.max_attempts(), 1);
        assert_eq!(strict_policy.guardian_threshold(), 5);
    }
}
