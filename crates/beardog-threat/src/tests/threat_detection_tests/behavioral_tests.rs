//! Behavioral Analysis Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: threat-detection/behavioral
//! TEST_PRIORITY: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// TEST 3: Behavioral Analysis
    ///
    /// Tests behavioral threat detection:
    /// - User behavior profiling
    /// - Access pattern analysis
    /// - Rate limiting detection
    /// - Suspicious activity identification
    #[test]
    fn test_behavioral_analysis() {
        let mut analyzer = BehaviorAnalyzer::new();

        // Create user profile with normal behavior
        let user_id = "user_123";

        // Establish normal access patterns
        for i in 0..100 {
            analyzer.record_event(
                user_id,
                BehaviorEvent::Login {
                    timestamp: Instant::now(),
                    success: true,
                    source_ip: "192.168.1.100".to_string(),
                },
            );

            // Simulate some time passing
            std::thread::sleep(Duration::from_millis(1));

            if i % 10 == 0 {
                analyzer.record_event(
                    user_id,
                    BehaviorEvent::DataAccess {
                        resource: "normal_resource".to_string(),
                        timestamp: Instant::now(),
                    },
                );
            }
        }

        analyzer.build_profile(user_id);

        assert!(analyzer.has_profile(user_id));

        // Test normal behavior (should not be flagged)
        let normal_login = BehaviorEvent::Login {
            timestamp: Instant::now(),
            success: true,
            source_ip: "192.168.1.100".to_string(),
        };

        let threat = analyzer.analyze_event(user_id, &normal_login);
        assert!(threat.is_none() || threat.unwrap().severity() == ThreatSeverity::Low);

        // Test suspicious behavior: rapid failed logins
        for _ in 0..10 {
            analyzer.record_event(
                user_id,
                BehaviorEvent::Login {
                    timestamp: Instant::now(),
                    success: false,
                    source_ip: "192.168.1.100".to_string(),
                },
            );
        }

        let failed_login = BehaviorEvent::Login {
            timestamp: Instant::now(),
            success: false,
            source_ip: "192.168.1.100".to_string(),
        };

        let threat = analyzer.analyze_event(user_id, &failed_login);
        assert!(threat.is_some());
        assert!(matches!(
            threat.unwrap().threat_type(),
            ThreatType::BruteForce
        ));

        // Test suspicious behavior: login from unusual location
        let unusual_login = BehaviorEvent::Login {
            timestamp: Instant::now(),
            success: true,
            source_ip: "10.0.0.1".to_string(), // Different IP
        };

        let threat = analyzer.analyze_event(user_id, &unusual_login);
        assert!(threat.is_some());
        assert!(matches!(
            threat.unwrap().threat_type(),
            ThreatType::UnusualAccess
        ));

        // Test suspicious behavior: unusual data access
        let unusual_access = BehaviorEvent::DataAccess {
            resource: "sensitive_admin_resource".to_string(),
            timestamp: Instant::now(),
        };

        let threat = analyzer.analyze_event(user_id, &unusual_access);
        assert!(threat.is_some());

        // Test rate limiting detection
        let mut rate_detector = RateLimitDetector::new(10, Duration::from_secs(1));

        // Normal rate (first 10 requests are within limit)
        for _ in 0..10 {
            assert!(!rate_detector.check_rate("client_1"));
        }

        // Exceed rate limit (11th+ requests should be flagged)
        for _ in 0..5 {
            assert!(rate_detector.check_rate("client_1")); // Should be flagged
        }

        // Different client should be unaffected
        assert!(!rate_detector.check_rate("client_2"));

        // Wait for window to reset
        std::thread::sleep(Duration::from_secs(2));
        assert!(!rate_detector.check_rate("client_1")); // Should be normal again
    }
}
