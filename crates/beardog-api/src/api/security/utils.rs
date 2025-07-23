//! Security API utility functions
//!
//! This module contains utility functions and helpers used throughout
//! the security API implementation.

// Private IP ranges for security validation
const PRIVATE_IP_RANGES: &[&str] = &[
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "::1/128",
    "fc00::/7",
    "fe80::/10",
];

use std::net::IpAddr;

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
    let ip_addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    PRIVATE_IP_RANGES.iter().any(|range| {
        if let Ok(network) = range.parse::<ipnet::IpNet>() {
            network.contains(&ip_addr)
        } else {
            false
        }
    })
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

/// Secure threat detection using conservative security-first approach
/// TODO: Replace with real threat detection system integration
///
/// # Arguments
/// * `source_ip` - Source IP address of the event
/// * `event_type` - Type of security event
///
/// # Returns
/// Number of threats detected (conservative approach)
pub fn secure_threat_detection(source_ip: &str, event_type: &str) -> usize {
    // Security-first: Be more suspicious than permissive
    let mut threat_score = 0;

    // Check for suspicious IP patterns
    if !is_private_ip(source_ip) {
        threat_score += 1;
    }

    // Check for high-risk event types
    match event_type.to_lowercase().as_str() {
        "login_failure" | "brute_force" | "privilege_escalation" | "data_exfiltration" => {
            threat_score += 2
        }
        "suspicious_activity" | "anomaly_detected" => threat_score += 1,
        _ => {}
    }

    // Return conservative threat count
    if threat_score >= 2 {
        2
    } else {
        threat_score
    }
}

/// Secure model accuracy reporting - conservative estimates until real metrics available
/// TODO: Replace with real ML model performance metrics
///
/// # Arguments
/// * `model_id` - ID of the ML model
///
/// # Returns
/// Accuracy score between 0.0 and 1.0 (conservative estimates)
pub fn get_secure_model_accuracy(model_id: &str) -> f64 {
    // Return conservative accuracy estimates that don't overstate performance
    // In production, these should come from real model validation metrics
    match model_id {
        "login_anomaly_v1" => 0.75,      // Conservative estimate - was 0.92
        "behavioral_anomaly_v2" => 0.70, // Conservative estimate - was 0.88
        "data_exfiltration_v1" => 0.65,  // Conservative estimate - was 0.85
        _ => 0.60,                       // Conservative default - was 0.80
    }
}

/// Secure behavioral analysis with conservative risk assessment
/// TODO: Replace with real behavioral analysis system
///
/// # Arguments
/// * `user_id` - User ID to analyze  
/// * `time_window_hours` - Analysis time window in hours
///
/// # Returns
/// Anomaly score between 0.0 and 1.0 (security-first approach)
pub fn secure_behavioral_analysis(user_id: &str, time_window_hours: u32) -> f64 {
    // Conservative behavioral scoring - prefer higher risk scores when uncertain
    let mut risk_score = 0.0;

    // Base risk from user ID characteristics (detect common attack patterns)
    if user_id.starts_with("admin") || user_id.starts_with("test") || user_id.starts_with("guest") {
        risk_score += 0.3; // Higher risk for common account names
    }

    // Time-based risk adjustment
    if time_window_hours <= 1 {
        risk_score += 0.2; // Short analysis windows are more suspicious
    }

    // Add base uncertainty score for missing real analysis
    risk_score += 0.4; // Conservative baseline when we can't do real analysis

    // Ensure we don't exceed 1.0
    if risk_score > 1.0 {
        1.0
    } else {
        risk_score
    }
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
    fn test_secure_threat_detection() {
        assert_eq!(secure_threat_detection("192.168.1.1", "login"), 0);
        assert_eq!(secure_threat_detection("8.8.8.8", "login"), 1);
        assert_eq!(secure_threat_detection("8.8.8.8", "brute_force"), 2);
    }

    #[test]
    fn test_secure_model_accuracy() {
        assert_eq!(get_secure_model_accuracy("login_anomaly_v1"), 0.75);
        assert_eq!(get_secure_model_accuracy("behavioral_anomaly_v2"), 0.70);
        assert_eq!(get_secure_model_accuracy("unknown_model"), 0.60);
    }

    #[test]
    fn test_secure_behavioral_analysis() {
        // Test admin account gets high risk
        assert_eq!(secure_behavioral_analysis("admin", 24), 0.7); // 0.3 + 0.0 + 0.4 = 0.7
                                                                  // Test admin with short time window gets higher risk
        assert!(secure_behavioral_analysis("admin", 1) > 0.8); // 0.3 + 0.2 + 0.4 = 0.9
                                                               // Test normal user gets moderate risk
        assert_eq!(secure_behavioral_analysis("normal_user", 24), 0.4); // 0.0 + 0.0 + 0.4 = 0.4
        assert!(secure_behavioral_analysis("normal_user", 1) > 0.5); // 0.0 + 0.2 + 0.4 = 0.6
    }

    #[test]
    fn test_generate_ids() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // UUID length
    }
}
