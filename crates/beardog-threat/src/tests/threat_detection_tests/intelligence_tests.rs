// SPDX-License-Identifier: AGPL-3.0-only

//! Threat Intelligence and Incident Response Tests
//!
//! `TEST_CATEGORY`: unit + integration
//! `TEST_DOMAIN`: threat-detection/intelligence
//! `TEST_PRIORITY`: critical

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float_assert::near_f64;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// TEST 7: Threat Intelligence Integration
    ///
    /// Tests integration with threat intelligence:
    /// - IOC (Indicators of Compromise) lookup
    /// - Reputation scoring
    /// - Threat feed integration
    /// - Intelligence enrichment
    #[test]
    fn test_threat_intelligence() {
        let mut intel = ThreatIntelligence::new();

        // Add known malicious indicators
        intel.add_ioc(Ioc::new(
            IocType::IPAddress,
            "10.0.0.1",
            ThreatSeverity::High,
            "Known botnet C&C",
        ));

        intel.add_ioc(Ioc::new(
            IocType::Domain,
            "evil.com",
            ThreatSeverity::Critical,
            "Malware distribution",
        ));

        intel.add_ioc(Ioc::new(
            IocType::FileHash,
            "abc123def456",
            ThreatSeverity::Critical,
            "Known malware",
        ));

        // Test IOC lookup
        assert!(intel.is_malicious(IocType::IPAddress, "10.0.0.1"));
        assert!(intel.is_malicious(IocType::Domain, "evil.com"));
        assert!(!intel.is_malicious(IocType::IPAddress, "192.168.1.1"));

        // Test reputation scoring
        let bad_ip_score = intel.get_reputation_score(IocType::IPAddress, "10.0.0.1");
        assert!(bad_ip_score < 30.0); // Low score = bad reputation

        let good_ip_score = intel.get_reputation_score(IocType::IPAddress, "8.8.8.8");
        assert!(good_ip_score > 70.0); // High score = good reputation

        // Test threat enrichment
        let mut threat = Threat::new(ThreatType::UnusualAccess, ThreatSeverity::Medium)
            .with_metadata("source_ip", "10.0.0.1");

        intel.enrich_threat(&mut threat);

        assert!(threat.enriched());
        assert!(!threat.ioc_matches().is_empty());
        assert_eq!(threat.severity(), ThreatSeverity::High); // Elevated due to IOC match

        // Test threat feed integration
        let feed = ThreatFeed::new("test_feed");

        // Simulate feed update
        let feed_data = vec![
            Ioc::new(
                IocType::IPAddress,
                "192.0.2.1",
                ThreatSeverity::Medium,
                "Scanning source",
            ),
            Ioc::new(
                IocType::Domain,
                "phishing.example",
                ThreatSeverity::High,
                "Phishing site",
            ),
        ];

        for ioc in feed_data {
            intel.import_from_feed(&feed, ioc);
        }

        assert!(intel.is_malicious(IocType::IPAddress, "192.0.2.1"));
        assert!(intel.is_malicious(IocType::Domain, "phishing.example"));

        // Test feed staleness
        assert!(!feed.is_stale(Duration::from_secs(3600)));

        // Test IOC expiration
        // Test expiration - modern pattern: Use instant expiry (1 nanosecond)
        let expiring_ioc = Ioc::new_with_expiry(
            IocType::IPAddress,
            "temp.bad.ip",
            ThreatSeverity::Low,
            "Temporary threat",
            Duration::from_nanos(1), // Instant expiry
        );

        intel.add_ioc(expiring_ioc);
        // IOC added but already expired by CPU cycles

        // Cleanup expired IOCs
        intel.cleanup_expired();

        // After cleanup, expired IOC should be removed
        assert!(!intel.is_malicious(IocType::IPAddress, "temp.bad.ip"));

        // Test bulk lookup
        let ips_to_check = vec!["10.0.0.1", "192.168.1.1", "8.8.8.8"];
        let results = intel.bulk_lookup(IocType::IPAddress, &ips_to_check);

        assert_eq!(results.len(), 3);
        assert!(results[0].is_malicious); // 10.0.0.1
        assert!(!results[1].is_malicious); // 192.168.1.1
        assert!(!results[2].is_malicious); // 8.8.8.8
    }

    /// TEST 8: Incident Response Triggers
    ///
    /// Tests automated incident response:
    /// - Response rule engine
    /// - Automated mitigation actions
    /// - Escalation workflows
    /// - Response effectiveness tracking
    #[test]
    fn test_incident_response() {
        let mut response_engine = IncidentResponseEngine::new();

        // Define response rules
        response_engine.add_rule(ResponseRule::new(
            "block_brute_force",
            ThreatType::BruteForce,
            ThreatSeverity::High,
            vec![
                ResponseAction::BlockIP,
                ResponseAction::NotifyAdmin,
                ResponseAction::LogIncident,
            ],
        ));

        response_engine.add_rule(ResponseRule::new(
            "quarantine_malware",
            ThreatType::Malware,
            ThreatSeverity::Critical,
            vec![
                ResponseAction::QuarantineFile,
                ResponseAction::IsolateHost,
                ResponseAction::EscalateToSOC,
            ],
        ));

        // Test rule matching
        let brute_force = Threat::new(ThreatType::BruteForce, ThreatSeverity::High)
            .with_metadata("source_ip", "10.0.0.1");

        let actions = response_engine.get_response_actions(&brute_force);
        assert_eq!(actions.len(), 3);
        assert!(actions.contains(&ResponseAction::BlockIP));
        assert!(actions.contains(&ResponseAction::NotifyAdmin));

        // Test response execution
        let mut executor = ResponseExecutor::new();
        let responses = Arc::new(Mutex::new(Vec::new()));

        let responses_clone = responses.clone();
        executor.set_action_handler(ResponseAction::BlockIP, move |threat: &Threat| {
            responses_clone.lock().unwrap().push(format!(
                "Blocked IP: {}",
                threat
                    .metadata()
                    .get("source_ip")
                    .map_or("unknown", |s| s.as_str())
            ));
        });

        let responses_clone2 = responses.clone();
        executor.set_action_handler(ResponseAction::NotifyAdmin, move |_threat: &Threat| {
            responses_clone2
                .lock()
                .unwrap()
                .push("Admin notified".to_string());
        });

        executor.execute_responses(&brute_force, &actions);

        let executed = responses.lock().unwrap();
        assert!(executed.len() >= 2);
        assert!(executed.iter().any(|r| r.contains("Blocked IP")));
        assert!(executed.iter().any(|r| r.contains("Admin notified")));

        // Test escalation workflow
        let mut escalation = EscalationWorkflow::new();

        escalation.add_level(EscalationLevel::new(
            "Level1",
            ThreatSeverity::Medium,
            vec!["security_team@example.com"],
            Duration::from_secs(300), // 5 minutes
        ));

        escalation.add_level(EscalationLevel::new(
            "Level2",
            ThreatSeverity::High,
            vec!["soc_team@example.com", "manager@example.com"],
            Duration::from_secs(900), // 15 minutes
        ));

        escalation.add_level(EscalationLevel::new(
            "Level3",
            ThreatSeverity::Critical,
            vec!["ciso@example.com", "exec_team@example.com"],
            Duration::from_secs(1800), // 30 minutes
        ));

        // Test appropriate escalation level
        let medium_threat = Threat::new(ThreatType::SuspiciousActivity, ThreatSeverity::Medium);
        let level = escalation.determine_level(&medium_threat);
        assert_eq!(level.name(), "Level1");

        let critical_threat = Threat::new(ThreatType::DataBreach, ThreatSeverity::Critical);
        let level = escalation.determine_level(&critical_threat);
        assert_eq!(level.name(), "Level3");

        // Test response effectiveness tracking
        let mut tracker = ResponseTracker::new();

        let incident_id = tracker.create_incident(&brute_force);
        tracker.record_response(&incident_id, ResponseAction::BlockIP, true);
        tracker.record_response(&incident_id, ResponseAction::NotifyAdmin, true);

        assert_eq!(tracker.response_count(&incident_id), 2);
        near_f64(tracker.success_rate(&incident_id), 100.0);

        // Test mitigation time tracking - modern pattern: test behavior, not timing
        tracker.mark_mitigated(&incident_id);

        let mitigation_time = tracker.mitigation_time(&incident_id);
        // Verify mitigation time is recorded (actual duration doesn't matter in test)
        assert!(mitigation_time.is_some());

        // Test automated response metrics
        let metrics = tracker.get_metrics();
        assert_eq!(metrics.total_incidents, 1);
        assert_eq!(metrics.total_responses, 2);
        assert_eq!(metrics.successful_responses, 2);
        near_f64(metrics.success_rate(), 100.0);
    }
}

// ============================================================================
// Self-Contained Type Definitions for Testing
// ============================================================================
