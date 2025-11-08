//! Recovery Session Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Shared mutex for tests that use timing/sleep
    static TIMING_TEST_LOCK: Mutex<()> = Mutex::new(());

    /// TEST 7: Session-Based Recovery
    ///
    /// Tests session-based recovery mechanisms:
    /// - Session creation
    /// - Session validation
    /// - Session expiration
    /// - Session revocation
    #[test]
    fn test_session_based_recovery() {
        // Use a lock to ensure this test runs serially with other timing-dependent tests
        let _guard = TIMING_TEST_LOCK.lock().unwrap();

        // Create recovery session
        let session = RecoverySession::new("user_123", std::time::Duration::from_secs(1800));
        assert_eq!(session.user_id(), "user_123");
        assert!(session.is_active());
        assert!(!session.is_expired());

        // Verify session ID is unique
        let session2 = RecoverySession::new("user_123", std::time::Duration::from_secs(1800));
        assert_ne!(session.session_id(), session2.session_id());

        // Test session validation
        assert!(session.validate().is_ok());

        // Test session expiration
        let expiring_session =
            RecoverySession::new("user_expire", std::time::Duration::from_nanos(1));
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(expiring_session.is_expired());
        assert!(!expiring_session.is_active());
        assert!(expiring_session.validate().is_err());

        // Test session revocation
        let mut active_session =
            RecoverySession::new("user_revoke", std::time::Duration::from_secs(3600));
        assert!(active_session.is_active());

        active_session.revoke();
        assert!(!active_session.is_active());
        assert!(active_session.validate().is_err());

        // Test session refresh
        let mut refreshable_session =
            RecoverySession::new("user_refresh", std::time::Duration::from_secs(1800));
        let original_expiry = refreshable_session.expires_at();

        refreshable_session.refresh(std::time::Duration::from_secs(3600));
        assert!(refreshable_session.expires_at() > original_expiry);
        assert!(refreshable_session.is_active());
    }
}
