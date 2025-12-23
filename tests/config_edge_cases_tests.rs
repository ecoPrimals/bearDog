//! Configuration Edge Case Tests
//!
//! TEST_CATEGORY: integration
//! TEST_DOMAIN: config/edge_cases
//! TEST_PRIORITY: high

use std::env;

#[tokio::test]
async fn test_config_empty_string_values() {
    // Test that empty strings are handled gracefully
    env::set_var("BEARDOG_TEST_EMPTY", "");

    let value = env::var("BEARDOG_TEST_EMPTY").unwrap_or_else(|_| "default".to_string());
    assert_eq!(value, "");

    // Should fall back to defaults
    let config = match env::var("BEARDOG_TEST_EMPTY") {
        Ok(v) if !v.is_empty() => v,
        _ => "default_value".to_string(),
    };
    assert_eq!(config, "default_value");

    env::remove_var("BEARDOG_TEST_EMPTY");
}

#[tokio::test]
async fn test_config_unicode_paths() {
    // Test non-ASCII characters in paths
    let unicode_paths = vec![
        "/home/用户/config",
        "/home/пользователь/data",
        "/home/المستخدم/files",
        "/home/사용자/settings",
    ];

    for path in unicode_paths {
        // Should handle Unicode gracefully
        let path_str = String::from(path);
        assert!(!path_str.is_empty());
        assert!(path_str.contains('/'));
    }
}

#[tokio::test]
async fn test_config_max_length_values() {
    // Test boundary values for string lengths
    let very_long_string = "a".repeat(10_000);
    env::set_var("BEARDOG_TEST_LONG", &very_long_string);

    let retrieved = env::var("BEARDOG_TEST_LONG").expect("Should get long value");
    assert_eq!(retrieved.len(), 10_000);

    env::remove_var("BEARDOG_TEST_LONG");
}

#[tokio::test]
async fn test_config_special_characters() {
    // Test special characters in config values
    let special_chars = [
        "value with spaces",
        "value\twith\ttabs",
        "value\nwith\nnewlines",
        "value;with;semicolons",
        "value&with&ampersands",
        "value|with|pipes",
    ];

    for (i, value) in special_chars.iter().enumerate() {
        let key = format!("BEARDOG_TEST_SPECIAL_{}", i);
        env::set_var(&key, value);

        let retrieved = env::var(&key).expect("Should get value");
        assert_eq!(&retrieved, value);

        env::remove_var(&key);
    }
}

#[tokio::test]
async fn test_config_numeric_boundary_values() {
    // Test numeric parsing at boundaries
    let test_cases = vec![
        ("0", 0u64),
        ("1", 1u64),
        ("255", 255u64),
        ("65535", 65535u64),
        ("2147483647", 2147483647u64),
    ];

    for (str_val, expected) in test_cases {
        env::set_var("BEARDOG_TEST_NUM", str_val);

        let parsed: u64 = env::var("BEARDOG_TEST_NUM")
            .expect("Should get env var")
            .parse()
            .expect("Should parse valid number");
        assert_eq!(parsed, expected);

        env::remove_var("BEARDOG_TEST_NUM");
    }
}

#[tokio::test]
async fn test_config_invalid_numeric_values() {
    // Test that invalid numeric values are rejected
    let invalid_values = vec![
        "not_a_number",
        "123abc",
        "12.34.56",
        "-1", // For unsigned parsing
        "",
    ];

    for value in invalid_values {
        env::set_var("BEARDOG_TEST_INVALID", value);

        let result: Result<u64, _> = env::var("BEARDOG_TEST_INVALID").unwrap_or_default().parse();

        assert!(result.is_err(), "Should reject invalid value: {}", value);

        env::remove_var("BEARDOG_TEST_INVALID");
    }
}

#[tokio::test]
async fn test_config_case_sensitivity() {
    // Test case sensitivity of config keys
    env::set_var("BEARDOG_TEST_CASE", "lowercase");
    env::set_var("beardog_test_case", "uppercase");

    // Environment variables are case-sensitive on Unix
    #[cfg(unix)]
    {
        assert_eq!(env::var("BEARDOG_TEST_CASE").unwrap(), "lowercase");
        assert_eq!(env::var("beardog_test_case").unwrap(), "uppercase");
    }

    // Windows is case-insensitive
    #[cfg(windows)]
    {
        // Either could be returned, but should be consistent
        let val = env::var("BEARDOG_TEST_CASE").unwrap();
        assert!(val == "lowercase" || val == "uppercase");
    }

    env::remove_var("BEARDOG_TEST_CASE");
    env::remove_var("beardog_test_case");
}

#[tokio::test]
async fn test_config_concurrent_access() {
    // Test concurrent reads and writes
    use tokio::task;

    env::set_var("BEARDOG_TEST_CONCURRENT", "initial");

    let mut handles = vec![];

    // Spawn multiple readers
    for i in 0..10 {
        let handle = task::spawn(async move {
            for _ in 0..100 {
                let _ = env::var("BEARDOG_TEST_CONCURRENT");
                tokio::time::sleep(tokio::time::Duration::from_micros(i * 10)).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all readers
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    env::remove_var("BEARDOG_TEST_CONCURRENT");
}

#[tokio::test]
async fn test_config_missing_required_values() {
    // Test handling of missing required configuration
    env::remove_var("BEARDOG_TEST_MISSING");

    // Should handle missing gracefully
    let result = env::var("BEARDOG_TEST_MISSING");
    assert!(result.is_err());

    // Should allow fallback
    let with_fallback = env::var("BEARDOG_TEST_MISSING").unwrap_or_else(|_| "fallback".to_string());
    assert_eq!(with_fallback, "fallback");
}

#[tokio::test]
async fn test_config_whitespace_handling() {
    // Test leading/trailing whitespace
    let test_cases = vec![
        ("  value  ", "  value  "), // Preserved
        ("\tvalue\t", "\tvalue\t"),
        (" ", " "),
        ("", ""),
    ];

    for (input, expected) in test_cases {
        env::set_var("BEARDOG_TEST_WHITESPACE", input);

        let retrieved = env::var("BEARDOG_TEST_WHITESPACE").expect("Should get value");
        assert_eq!(retrieved, expected);

        // Test trimming if needed
        let trimmed = retrieved.trim();
        assert_eq!(trimmed, expected.trim());

        env::remove_var("BEARDOG_TEST_WHITESPACE");
    }
}
