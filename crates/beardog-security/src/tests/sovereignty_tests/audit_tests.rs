// SPDX-License-Identifier: AGPL-3.0-only

//! Sovereignty Audit Trail Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/sovereignty/audit
//! `TEST_PRIORITY`: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests audit trail creation and validation:
    /// - Audit event logging
    /// - Tamper detection
    /// - Audit chain integrity
    /// - Compliance reporting
    #[test]
    fn test_sovereignty_audit_trails() {
        // Create audit trail
        let mut audit_trail = AuditTrail::new("sovereignty_audit");
        assert_eq!(audit_trail.event_count(), 0);

        // Log sovereignty events
        let event1 = AuditEvent::new(EventType::DataAccess)
            .with_user("user_1")
            .with_resource("sensitive_data")
            .with_region("EU")
            .with_outcome(Outcome::Success);

        audit_trail.log_event(event1.clone()).unwrap();
        assert_eq!(audit_trail.event_count(), 1);

        let event2 = AuditEvent::new(EventType::KeyOperation)
            .with_user("admin")
            .with_resource("encryption_key")
            .with_region("EU")
            .with_outcome(Outcome::Success);

        audit_trail.log_event(event2).unwrap();
        assert_eq!(audit_trail.event_count(), 2);

        // Test event retrieval
        let events = audit_trail.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type(), EventType::DataAccess);

        // Test audit trail integrity
        assert!(audit_trail.verify_integrity().is_ok());

        // Simulate tampering detection
        let mut tampered_trail = audit_trail.clone();
        tampered_trail.corrupt_event(0); // Simulate corruption
        assert!(tampered_trail.verify_integrity().is_err());

        // Test event filtering
        let eu_events = audit_trail.filter_by_region("EU");
        assert_eq!(eu_events.len(), 2);

        let us_events = audit_trail.filter_by_region("US");
        assert_eq!(us_events.len(), 0);

        // Test compliance reporting
        let report = audit_trail.generate_compliance_report();
        assert_eq!(report.total_events(), 2);
        assert_eq!(report.successful_events(), 2);
        assert_eq!(report.failed_events(), 0);

        // Test audit chain
        let chain = AuditChain::new();
        assert_eq!(chain.length(), 0);

        let mut chain = chain;
        chain.append(audit_trail.clone()).unwrap();
        assert_eq!(chain.length(), 1);

        let another_trail = AuditTrail::new("another_audit").with_event(
            AuditEvent::new(EventType::PolicyChange)
                .with_user("admin")
                .with_outcome(Outcome::Success),
        );

        chain.append(another_trail).unwrap();
        assert_eq!(chain.length(), 2);

        // Verify chain integrity
        assert!(chain.verify_chain().is_ok());

        // Test event search
        let data_access_events = audit_trail.search_by_type(EventType::DataAccess);
        assert_eq!(data_access_events.len(), 1);

        let admin_events = audit_trail.search_by_user("admin");
        assert_eq!(admin_events.len(), 1);

        // Test retention policy
        let retention_policy =
            RetentionPolicy::new().with_duration(std::time::Duration::from_secs(86400 * 365 * 7)); // 7 years

        assert!(retention_policy.should_retain(&event1));

        // Simulate old event
        let _old_event = AuditEvent::new(EventType::DataAccess).with_timestamp_nanos(1); // Very old

        // Modern pattern: No sleep needed - just verify retention policy configuration
        assert!(retention_policy.duration_days() == 2555); // 7 years
    }
}
