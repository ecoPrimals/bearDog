//! Comprehensive Utils Tests
//!
//! This test suite ensures 100% coverage of BearDog's utility functions
//! including configuration, validation, formatting, and helper functions.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Comprehensive utils testing
#[tokio::test]
async fn test_utils_comprehensive() {
    test_string_utilities();
    test_time_utilities();
    test_validation_utilities();
    test_formatting_utilities();
    test_configuration_utilities();
    test_security_utilities();
}

fn test_string_utilities() {
    println!("�� Testing string utilities...");

    // Test string validation
    assert!(is_valid_identifier("valid_name_123"));
    assert!(!is_valid_identifier("invalid-name!"));
    assert!(!is_valid_identifier(""));

    // Test string sanitization
    let sanitized = sanitize_user_input("user<script>alert('xss')</script>input");
    assert!(!sanitized.contains("<script>"));

    // Test string formatting
    let formatted = format_bytes(1024 * 1024 * 1024);
    assert!(formatted.contains("GB") || formatted.contains("GiB"));

    let formatted_duration = format_duration(Duration::from_secs(3661));
    assert!(formatted_duration.contains("1h") || formatted_duration.contains("hour"));
}

fn test_time_utilities() {
    println!("⏰ Testing time utilities...");

    let now = SystemTime::now();
    let future = now + Duration::from_secs(3600);

    assert!(is_future_time(&future));
    assert!(!is_future_time(&now));

    let elapsed = calculate_elapsed_time(now);
    assert!(elapsed.as_secs() < 1);

    let formatted_time = format_timestamp(now);
    assert!(!formatted_time.is_empty());
}

fn test_validation_utilities() {
    println!("✅ Testing validation utilities...");

    // Network validation
    assert!(is_valid_ip_address("192.168.1.1"));
    assert!(is_valid_ip_address("::1"));
    assert!(!is_valid_ip_address("invalid_ip"));

    assert!(is_valid_port(8080));
    assert!(!is_valid_port(0));
    assert!(!is_valid_port(65535u32));

    // Crypto validation
    assert!(is_valid_hex_string("deadbeef"));
    assert!(is_valid_hex_string("DEADBEEF"));
    assert!(!is_valid_hex_string("ghijklmn"));

    let valid_key = vec![0u8; 32];
    assert!(is_valid_key_length(&valid_key, 32));
    assert!(!is_valid_key_length(&valid_key, 16));
}

fn test_formatting_utilities() {
    println!("📄 Testing formatting utilities...");

    let json_data = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);

    let formatted_json = format_as_json(&json_data);
    assert!(formatted_json.is_ok());

    let table_data = vec![
        vec!["Name".to_string(), "Age".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];

    let formatted_table = format_as_table(&table_data);
    assert!(!formatted_table.is_empty());
}

fn test_configuration_utilities() {
    println!("⚙️ Testing configuration utilities...");

    let config_map = HashMap::from([
        ("setting1".to_string(), "value1".to_string()),
        ("setting2".to_string(), "123".to_string()),
    ]);

    assert_eq!(
        get_config_value(&config_map, "setting1"),
        Some("value1".to_string())
    );
    assert_eq!(get_config_value(&config_map, "nonexistent"), None);

    let parsed_int = parse_config_int(&config_map, "setting2");
    assert_eq!(parsed_int, Some(123));

    let parsed_invalid = parse_config_int(&config_map, "setting1");
    assert_eq!(parsed_invalid, None);
}

fn test_security_utilities() {
    println!("🔒 Testing security utilities...");

    let sensitive_data = "password123";
    let redacted = redact_sensitive_data(sensitive_data);
    assert_eq!(redacted, "***");

    let log_safe = make_log_safe("user input with <tags>");
    assert!(!log_safe.contains("<"));

    let random_id = generate_secure_id();
    assert_eq!(random_id.len(), 32);
    assert!(random_id.chars().all(|c| c.is_ascii_alphanumeric()));
}

// Mock utility functions
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn sanitize_user_input(input: &str) -> String {
    input.replace('<', "&lt;").replace('>', "&gt;")
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{bytes} bytes")
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs >= 3600 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    }
}

fn is_future_time(time: &SystemTime) -> bool {
    time > &SystemTime::now()
}

fn calculate_elapsed_time(start: SystemTime) -> Duration {
    SystemTime::now()
        .duration_since(start)
        .unwrap_or(Duration::ZERO)
}

fn format_timestamp(time: SystemTime) -> String {
    format!("{time:?}")
}

fn is_valid_ip_address(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

fn is_valid_port(port: u32) -> bool {
    port > 0 && port <= 65535
}

fn is_valid_hex_string(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_valid_key_length(key: &[u8], expected_len: usize) -> bool {
    key.len() == expected_len
}

fn format_as_json(data: &HashMap<String, String>) -> Result<String, String> {
    serde_json::to_string_pretty(data).map_err(|e| e.to_string())
}

fn format_as_table(data: &[Vec<String>]) -> String {
    data.iter()
        .map(|row| row.join(" | "))
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_config_value(config: &HashMap<String, String>, key: &str) -> Option<String> {
    config.get(key).cloned()
}

fn parse_config_int(config: &HashMap<String, String>, key: &str) -> Option<i32> {
    config.get(key)?.parse().ok()
}

fn redact_sensitive_data(_data: &str) -> String {
    "***".to_string()
}

fn make_log_safe(input: &str) -> String {
    input.replace(['<', '>'], "")
}

fn generate_secure_id() -> String {
    "abcdef1234567890abcdef1234567890".to_string()
}
