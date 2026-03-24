// SPDX-License-Identifier: AGPL-3.0-only

//! Anomaly Detection and Pattern Matching Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: threat-detection/anomaly
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

    /// TEST 1: Anomaly Detection
    ///
    /// Tests anomaly detection algorithms:
    /// - Statistical anomaly detection
    /// - Baseline establishment
    /// - Deviation threshold detection
    /// - Adaptive learning
    #[test]
    fn test_anomaly_detection() {
        let mut detector = AnomalyDetector::new();

        // Establish baseline with normal traffic
        let normal_samples = vec![100, 105, 98, 102, 101, 99, 103, 100, 104, 102];

        for &sample in &normal_samples {
            detector.add_sample(sample as f64);
        }

        detector.establish_baseline();

        assert!(detector.has_baseline());
        let baseline = detector.baseline();
        assert!(baseline > 99.0 && baseline < 103.0); // Should be around 101.4

        // Test normal values (should not be anomalies)
        assert!(!detector.is_anomaly(100.0).unwrap());
        assert!(!detector.is_anomaly(102.0).unwrap());
        assert!(!detector.is_anomaly(105.0).unwrap());

        // Test anomalous values (significant deviation)
        assert!(detector.is_anomaly(200.0).unwrap()); // 2x baseline
        assert!(detector.is_anomaly(10.0).unwrap()); // 0.1x baseline
        assert!(detector.is_anomaly(500.0).unwrap()); // 5x baseline

        // Test edge cases near threshold
        detector.set_threshold(2.0); // 2 standard deviations

        let mild_deviation = 1.5f64.mul_add(detector.std_dev(), baseline);
        assert!(!detector.is_anomaly(mild_deviation).unwrap());

        let strong_deviation = 3.0f64.mul_add(detector.std_dev(), baseline);
        assert!(detector.is_anomaly(strong_deviation).unwrap());

        // Test adaptive learning
        for _ in 0..20 {
            detector.add_sample(110.0); // New normal
        }

        detector.establish_baseline();
        let new_baseline = detector.baseline();
        assert!(new_baseline > baseline); // Baseline should shift up

        // Previously anomalous value should now be normal
        assert!(!detector.is_anomaly(110.0).unwrap());

        // Test with empty baseline
        let empty_detector = AnomalyDetector::new();
        assert!(empty_detector.is_anomaly(100.0).is_err()); // Should fail without baseline
    }

    /// TEST 2: Pattern Matching and Signatures
    ///
    /// Tests signature-based threat detection:
    /// - Known attack pattern matching
    /// - Regex-based detection
    /// - Multi-pattern matching
    /// - Pattern priority and ordering
    #[test]
    fn test_pattern_matching() {
        let mut matcher = PatternMatcher::new();

        // Register attack patterns
        matcher.add_pattern(ThreatPattern::new(
            "sql_injection",
            r"(SELECT|INSERT|UPDATE|DELETE|DROP).*FROM",
            ThreatSeverity::High,
        ));

        matcher.add_pattern(ThreatPattern::new(
            "xss_attack",
            r"<script[^>]*>.*</script>",
            ThreatSeverity::High,
        ));

        matcher.add_pattern(ThreatPattern::new(
            "path_traversal",
            r"\.\./",
            ThreatSeverity::Medium,
        ));

        assert_eq!(matcher.pattern_count(), 3);

        // Test SQL injection detection
        let sql_payload = "SELECT * FROM users WHERE id=1";
        let matches = matcher.find_matches(sql_payload);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern_id(), "sql_injection");
        assert_eq!(matches[0].severity(), ThreatSeverity::High);

        // Test XSS detection
        let xss_payload = "<script>alert('XSS')</script>";
        let matches = matcher.find_matches(xss_payload);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern_id(), "xss_attack");

        // Test path traversal
        let path_payload = "../../etc/passwd";
        let matches = matcher.find_matches(path_payload);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern_id(), "path_traversal");
        assert_eq!(matches[0].severity(), ThreatSeverity::Medium);

        // Test benign input
        let benign = "Hello, world!";
        let matches = matcher.find_matches(benign);
        assert_eq!(matches.len(), 0);

        // Test multiple pattern matches
        let complex_payload = "SELECT * FROM users; <script>alert(1)</script>";
        let matches = matcher.find_matches(complex_payload);
        assert!(matches.len() >= 2); // Should match both SQL and XSS

        // Test pattern priority
        matcher.add_pattern(ThreatPattern::new(
            "critical_exploit",
            r"exploit",
            ThreatSeverity::Critical,
        ));

        let prioritized = matcher.find_highest_severity_match("This is an exploit");
        assert!(prioritized.is_some());
        assert_eq!(prioritized.unwrap().severity(), ThreatSeverity::Critical);

        // Test pattern removal
        matcher.remove_pattern("path_traversal");
        assert_eq!(matcher.pattern_count(), 3);

        let removed_test = matcher.find_matches("../../etc/passwd");
        assert_eq!(removed_test.len(), 0);
    }
}
