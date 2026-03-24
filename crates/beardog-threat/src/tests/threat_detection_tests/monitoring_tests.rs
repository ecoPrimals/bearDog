// SPDX-License-Identifier: AGPL-3.0-only

//! Real-Time Monitoring and Classification Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: threat-detection/monitoring
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
    use std::time::{Duration, Instant};

    /// TEST 4: Real-Time Threat Monitoring
    ///
    /// Tests real-time monitoring capabilities:
    /// - Continuous event stream processing
    /// - Alert generation
    /// - Threat aggregation
    /// - Performance under load
    #[test]
    fn test_realtime_monitoring() {
        let monitor = Arc::new(Mutex::new(ThreatMonitor::new()));
        let alert_handler = Arc::new(Mutex::new(AlertHandler::new()));

        // Register alert handler
        let handler_clone = alert_handler.clone();
        monitor
            .lock()
            .unwrap()
            .set_alert_callback(move |alert: ThreatAlert| {
                handler_clone.lock().unwrap().handle_alert(alert);
            });

        // Simulate real-time event stream
        let monitor_clone = monitor.clone();
        let producer = std::thread::spawn(move || {
            for i in 0..100 {
                let event = if i % 10 == 0 {
                    // Generate threat event
                    ThreatEvent::new(
                        ThreatType::Malware,
                        ThreatSeverity::High,
                        format!("threat_{i}"),
                    )
                } else {
                    // Generate normal event
                    ThreatEvent::new(
                        ThreatType::None,
                        ThreatSeverity::Info,
                        format!("normal_{i}"),
                    )
                };

                monitor_clone.lock().unwrap().process_event(&event);
                // Modern pattern: No sleep needed - test real concurrent processing
            }
        });

        producer.join().unwrap();

        // Verify alerts were generated
        let alert_guard = alert_handler.lock().unwrap();
        let alerts = alert_guard.get_alerts();
        assert!(alerts.len() >= 10); // Should have ~10 threat events
        drop(alert_guard);

        // Verify alert aggregation
        let stats = monitor.lock().unwrap().get_statistics();
        assert_eq!(stats.total_events, 100);
        assert!(stats.threat_events >= 10);
        assert!(stats.benign_events >= 90);

        // Test high-volume scenario
        let high_volume_monitor = Arc::new(Mutex::new(ThreatMonitor::new()));
        let start = Instant::now();

        for _ in 0..10000 {
            high_volume_monitor
                .lock()
                .unwrap()
                .process_event(&ThreatEvent::new(
                    ThreatType::None,
                    ThreatSeverity::Info,
                    "benchmark".to_string(),
                ));
        }

        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(1)); // Should process 10k events in <1s

        // Test concurrent monitoring
        let concurrent_monitor = Arc::new(Mutex::new(ThreatMonitor::new()));
        let mut handles = vec![];

        for thread_id in 0..5 {
            let monitor_clone = concurrent_monitor.clone();
            let handle = std::thread::spawn(move || {
                for i in 0..100 {
                    monitor_clone
                        .lock()
                        .unwrap()
                        .process_event(&ThreatEvent::new(
                            ThreatType::None,
                            ThreatSeverity::Info,
                            format!("thread_{thread_id}_{i}"),
                        ));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_stats = concurrent_monitor.lock().unwrap().get_statistics();
        assert_eq!(final_stats.total_events, 500); // 5 threads * 100 events
    }

    /// TEST 5: Threat Classification and Scoring
    ///
    /// Tests threat scoring and classification:
    /// - Severity calculation
    /// - Risk scoring algorithms
    /// - Threat categorization
    /// - Composite threat assessment
    #[test]
    fn test_threat_classification() {
        let classifier = ThreatClassifier::new();

        // Test basic severity levels
        let low_threat = Threat::new(ThreatType::SuspiciousActivity, ThreatSeverity::Low);
        near_f64(classifier.calculate_score(&low_threat), 25.0);

        let medium_threat = Threat::new(ThreatType::UnusualAccess, ThreatSeverity::Medium);
        near_f64(classifier.calculate_score(&medium_threat), 50.0);

        let high_threat = Threat::new(ThreatType::Malware, ThreatSeverity::High);
        near_f64(classifier.calculate_score(&high_threat), 75.0);

        let critical_threat = Threat::new(ThreatType::DataBreach, ThreatSeverity::Critical);
        near_f64(classifier.calculate_score(&critical_threat), 100.0);

        // Test threat categorization
        assert_eq!(
            classifier.categorize(&Threat::new(ThreatType::BruteForce, ThreatSeverity::High)),
            ThreatCategory::AuthenticationAttack
        );

        assert_eq!(
            classifier.categorize(&Threat::new(ThreatType::SQLInjection, ThreatSeverity::High)),
            ThreatCategory::InjectionAttack
        );

        assert_eq!(
            classifier.categorize(&Threat::new(ThreatType::Malware, ThreatSeverity::Critical)),
            ThreatCategory::MaliciousSoftware
        );

        // Test composite threat assessment
        let mut composite = CompositeThreat::new("multi_vector_attack");

        composite.add_indicator(Threat::new(ThreatType::BruteForce, ThreatSeverity::Medium));
        composite.add_indicator(Threat::new(
            ThreatType::UnusualAccess,
            ThreatSeverity::Medium,
        ));
        composite.add_indicator(Threat::new(
            ThreatType::DataExfiltration,
            ThreatSeverity::High,
        ));

        let composite_score = classifier.calculate_composite_score(&composite);
        assert!(composite_score > 50.0); // Multiple indicators should elevate score

        // Test threat prioritization
        let threats = vec![
            Threat::new(ThreatType::SuspiciousActivity, ThreatSeverity::Low),
            Threat::new(ThreatType::DataBreach, ThreatSeverity::Critical),
            Threat::new(ThreatType::Malware, ThreatSeverity::High),
            Threat::new(ThreatType::UnusualAccess, ThreatSeverity::Medium),
        ];

        let prioritized = classifier.prioritize(threats);
        assert_eq!(prioritized[0].severity(), ThreatSeverity::Critical);
        assert_eq!(prioritized[1].severity(), ThreatSeverity::High);
        assert_eq!(prioritized[2].severity(), ThreatSeverity::Medium);
        assert_eq!(prioritized[3].severity(), ThreatSeverity::Low);

        // Test confidence scoring
        let mut confidence_threat = Threat::new(ThreatType::Malware, ThreatSeverity::High);
        confidence_threat.set_confidence(0.95);

        near_f64(confidence_threat.confidence(), 0.95);

        let adjusted_score = classifier.calculate_score_with_confidence(&confidence_threat);
        assert!(adjusted_score > 70.0); // High confidence should maintain high score

        let low_confidence_threat = Threat::new(ThreatType::Malware, ThreatSeverity::High);
        let mut low_conf = low_confidence_threat;
        low_conf.set_confidence(0.3);

        let adjusted_low = classifier.calculate_score_with_confidence(&low_conf);
        assert!(adjusted_low < adjusted_score); // Low confidence should reduce score
    }

    /// TEST 6: False Positive Handling
    ///
    /// Tests false positive reduction and handling:
    /// - Allowlist management
    /// - False positive feedback
    /// - Adaptive threshold adjustment
    /// - Pattern refinement
    #[test]
    fn test_false_positive_handling() {
        let mut fp_handler = FalsePositiveHandler::new();

        // Test allowlist functionality (formerly whitelist)
        fp_handler.add_to_allowlist("trusted_ip", "192.168.1.100");
        fp_handler.add_to_allowlist("trusted_user", "admin@example.com");

        assert!(fp_handler.is_allowed("trusted_ip", "192.168.1.100"));
        assert!(fp_handler.is_allowed("trusted_user", "admin@example.com"));
        assert!(!fp_handler.is_allowed("trusted_ip", "10.0.0.1"));

        // Test threat filtering with allowlist
        let threat = Threat::new(ThreatType::UnusualAccess, ThreatSeverity::Medium)
            .with_metadata("source_ip", "192.168.1.100");

        assert!(fp_handler.should_suppress(&threat));

        let non_allowlisted = Threat::new(ThreatType::UnusualAccess, ThreatSeverity::Medium)
            .with_metadata("source_ip", "10.0.0.1");

        assert!(!fp_handler.should_suppress(&non_allowlisted));

        // Test false positive feedback
        let _detected_threat =
            Threat::new(ThreatType::SuspiciousActivity, ThreatSeverity::Low).with_id("threat_001");

        fp_handler.mark_as_false_positive("threat_001");

        assert!(fp_handler.is_known_false_positive("threat_001"));
        assert_eq!(fp_handler.false_positive_count(), 1);

        // Test adaptive threshold
        let mut adaptive_detector = AdaptiveDetector::new();
        adaptive_detector.set_threshold(0.5);

        // Simulate false positive feedback
        for _ in 0..10 {
            adaptive_detector.report_false_positive();
        }

        adaptive_detector.adjust_threshold();
        assert!(adaptive_detector.threshold() > 0.5); // Threshold should increase

        // Simulate true positive feedback
        for _ in 0..20 {
            adaptive_detector.report_true_positive();
        }

        adaptive_detector.adjust_threshold();
        // With more true positives, threshold might adjust differently

        // Test pattern refinement
        let mut pattern_refiner = PatternRefiner::new();

        let pattern = ThreatPattern::new("test_pattern", r"suspicious", ThreatSeverity::Medium);

        pattern_refiner.add_pattern(pattern);

        // Mark matches as false positives
        pattern_refiner.record_false_positive("test_pattern", "suspicious but benign context");
        pattern_refiner.record_false_positive("test_pattern", "another suspicious benign case");

        let fp_rate = pattern_refiner.false_positive_rate("test_pattern");
        assert!(fp_rate > 0.0);

        // Test automatic pattern disabling
        for _ in 0..20 {
            pattern_refiner.record_false_positive("test_pattern", "fp");
        }

        if pattern_refiner.false_positive_rate("test_pattern") > 0.5 {
            pattern_refiner.disable_pattern("test_pattern");
        }

        assert!(!pattern_refiner.is_pattern_enabled("test_pattern"));
    }
}
