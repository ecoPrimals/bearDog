//! Recovery Challenge Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 6: Challenge-Response Recovery
    ///
    /// Tests challenge-response based recovery:
    /// - Challenge generation
    /// - Response validation
    /// - Multi-factor challenges
    /// - Challenge expiration
    #[test]
    fn test_challenge_response_recovery() {
        // Generate recovery challenge
        let challenge = RecoveryChallenge::generate("user_123");
        assert_eq!(challenge.user_id(), "user_123");
        assert!(!challenge.is_answered());
        assert!(!challenge.is_expired());

        // Verify challenge data
        assert_eq!(challenge.challenge_data().len(), 32); // 256-bit challenge

        // Answer challenge correctly
        let correct_response = compute_response(challenge.challenge_data(), b"correct_secret");
        let mut challenge = challenge;
        let result = challenge.submit_response(&correct_response);
        assert!(result.is_ok());
        assert!(challenge.is_answered());
        assert!(challenge.is_valid());

        // Test incorrect response
        let mut new_challenge = RecoveryChallenge::generate("user_456");
        let wrong_response = compute_response(new_challenge.challenge_data(), b"wrong_secret");
        let result = new_challenge.submit_response(&wrong_response);
        assert!(result.is_err());
        assert!(!new_challenge.is_valid());

        // Test challenge expiration
        let short_challenge =
            RecoveryChallenge::with_expiration("user_789", std::time::Duration::from_nanos(1));

        // Modern pattern: 1 nanosecond instantly expired - no sleep needed
        assert!(short_challenge.is_expired());

        // Test multi-factor challenge
        let mut mfa_challenge = MultiFactorChallenge::new("user_mfa");

        // Start with no factors
        assert_eq!(mfa_challenge.factor_count(), 0);
        // Empty challenge is considered complete (no factors to complete)

        // Add two factors
        mfa_challenge.add_factor(ChallengeFactor::Password);
        mfa_challenge.add_factor(ChallengeFactor::Biometric);
        assert_eq!(mfa_challenge.factor_count(), 2);

        // Now with factors added, should not be complete yet
        assert!(
            !mfa_challenge.is_complete(),
            "Should not be complete with 0 of 2 factors"
        );

        // Complete first factor
        mfa_challenge
            .complete_factor(ChallengeFactor::Password)
            .unwrap();
        assert!(
            !mfa_challenge.is_complete(),
            "Should not be complete with only 1 of 2 factors"
        );

        // Complete second factor
        mfa_challenge
            .complete_factor(ChallengeFactor::Biometric)
            .unwrap();
        assert!(
            mfa_challenge.is_complete(),
            "Should be complete with both factors"
        );
    }
}
