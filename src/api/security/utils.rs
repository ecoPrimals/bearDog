//! Security API utility functions
//!
//! This module contains utility functions and helpers used throughout
//! the security API implementation.

use crate::config::constants::network::PRIVATE_IP_RANGES;

/// Check if an IP address is in a private range
///
/// # Arguments
/// * `ip` - The IP address string to check
///
/// # Returns
/// `true` if the IP is in a private range, `false` otherwise
///
/// # Example
/// ```rust
/// use beardog::api::security::utils::is_private_ip;
///
/// assert!(is_private_ip("192.168.1.1"));
/// assert!(is_private_ip("10.0.0.1"));
/// assert!(!is_private_ip("8.8.8.8"));
/// ```
pub fn is_private_ip(ip: &str) -> bool {
    PRIVATE_IP_RANGES.iter().any(|range| ip.starts_with(range))
}

/// Generate a unique request ID for API tracking
///
/// # Returns
/// A UUID string for request tracking
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a unique batch ID for batch operations
///
/// # Returns
/// A UUID string for batch operation tracking
pub fn generate_batch_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a unique event ID for security events
///
/// # Returns
/// A UUID string for event tracking
pub fn generate_event_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a unique incident ID for security incidents
///
/// # Returns
/// A UUID string for incident tracking
pub fn generate_incident_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a unique rule ID for detection rules
///
/// # Returns
/// A UUID string for rule tracking
pub fn generate_rule_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate a unique feed ID for threat intelligence feeds
///
/// # Returns
/// A UUID string for feed tracking
pub fn generate_feed_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Calculate risk level based on threat count and severity
///
/// # Arguments
/// * `threat_count` - Number of threats detected
/// * `high_severity_count` - Number of high severity threats
///
/// # Returns
/// Risk level string (LOW, MEDIUM, HIGH, CRITICAL)
pub fn calculate_risk_level(threat_count: usize, high_severity_count: usize) -> String {
    match (threat_count, high_severity_count) {
        (0, _) => "LOW".to_string(),
        (1..=2, 0) => "MEDIUM".to_string(),
        (1..=2, _) => "HIGH".to_string(),
        (_, _) => "CRITICAL".to_string(),
    }
}

/// Determine if an incident should be created based on threat analysis
///
/// # Arguments
/// * `threat_count` - Number of threats detected
/// * `risk_level` - Risk level assessment
///
/// # Returns
/// `true` if an incident should be created, `false` otherwise
pub fn should_create_incident(threat_count: usize, risk_level: &str) -> bool {
    threat_count > 0 && matches!(risk_level, "HIGH" | "CRITICAL")
}

/// Mock function to simulate threat detection logic
///
/// # Arguments
/// * `source_ip` - Source IP address of the event
/// * `event_type` - Type of security event
///
/// # Returns
/// Number of threats detected (simplified mock logic)
pub fn mock_threat_detection(source_ip: &str, _event_type: &str) -> usize {
    if is_private_ip(source_ip) {
        0
    } else {
        1
    }
}

/// Mock function to simulate ML model accuracy
///
/// # Arguments
/// * `model_id` - ID of the ML model
///
/// # Returns
/// Accuracy score between 0.0 and 1.0
pub fn mock_model_accuracy(model_id: &str) -> f64 {
    match model_id {
        "login_anomaly_v1" => 0.92,
        "behavioral_anomaly_v2" => 0.88,
        "data_exfiltration_v1" => 0.85,
        _ => 0.80,
    }
}

/// Mock function to simulate behavioral anomaly scoring
///
/// # Arguments
/// * `user_id` - User ID to analyze
///
/// # Returns
/// Anomaly score between 0.0 and 1.0
pub fn mock_behavioral_score(user_id: &str) -> f64 {
    // Simple hash-based mock scoring
    let hash = user_id.chars().map(|c| c as u32).sum::<u32>();
    (hash % 100) as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_ip() {
        assert!(is_private_ip("192.168.1.1"));
        assert!(is_private_ip("10.0.0.1"));
        assert!(is_private_ip("172.16.0.1"));
        assert!(!is_private_ip("8.8.8.8"));
        assert!(!is_private_ip("1.1.1.1"));
    }

    #[test]
    fn test_calculate_risk_level() {
        assert_eq!(calculate_risk_level(0, 0), "LOW");
        assert_eq!(calculate_risk_level(1, 0), "MEDIUM");
        assert_eq!(calculate_risk_level(1, 1), "HIGH");
        assert_eq!(calculate_risk_level(5, 2), "CRITICAL");
    }

    #[test]
    fn test_should_create_incident() {
        assert!(!should_create_incident(0, "LOW"));
        assert!(!should_create_incident(1, "MEDIUM"));
        assert!(should_create_incident(1, "HIGH"));
        assert!(should_create_incident(1, "CRITICAL"));
    }

    #[test]
    fn test_mock_threat_detection() {
        assert_eq!(mock_threat_detection("192.168.1.1", "login"), 0);
        assert_eq!(mock_threat_detection("8.8.8.8", "login"), 1);
    }

    #[test]
    fn test_mock_model_accuracy() {
        assert_eq!(mock_model_accuracy("login_anomaly_v1"), 0.92);
        assert_eq!(mock_model_accuracy("behavioral_anomaly_v2"), 0.88);
        assert_eq!(mock_model_accuracy("unknown_model"), 0.80);
    }

    #[test]
    fn test_generate_ids() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // UUID length
    }
} 