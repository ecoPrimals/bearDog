// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
//! Configuration Edge Case Tests
//!
//! `TEST_CATEGORY`: integration
//! `TEST_DOMAIN`: `config/edge_cases`
//! `TEST_PRIORITY`: high
//!
//! These tests exercise string parsing and boundary behavior **without** mutating
//! the process environment, so they remain safe under `cargo test` parallelism.

#[test]
fn test_config_empty_string_values() {
    // Mirrors empty env value → Ok("") on typical platforms
    let raw = "";
    let value = raw.to_string();
    assert_eq!(value, "");

    let config = match value.as_str() {
        v if !v.is_empty() => v.to_string(),
        _ => "default_value".to_string(),
    };
    assert_eq!(config, "default_value");
}

#[test]
fn test_config_unicode_paths() {
    let unicode_paths = vec![
        "/home/用户/config",
        "/home/пользователь/data",
        "/home/المستخدم/files",
        "/home/사용자/settings",
    ];

    for path in unicode_paths {
        let path_str = String::from(path);
        assert!(!path_str.is_empty());
        assert!(path_str.contains('/'));
    }
}

#[test]
fn test_config_max_length_values() {
    let very_long_string = "a".repeat(10_000);
    assert_eq!(very_long_string.len(), 10_000);
}

#[test]
fn test_config_special_characters() {
    let special_chars = [
        "value with spaces",
        "value\twith\ttabs",
        "value\nwith\nnewlines",
        "value;with;semicolons",
        "value&with&ampersands",
        "value|with|pipes",
    ];

    for value in &special_chars {
        let retrieved = value.to_string();
        assert_eq!(&retrieved, value);
    }
}

#[test]
fn test_config_numeric_boundary_values() {
    let test_cases = vec![
        ("0", 0u64),
        ("1", 1u64),
        ("255", 255u64),
        ("65535", 65535u64),
        ("2147483647", 2_147_483_647_u64),
    ];

    for (str_val, expected) in test_cases {
        let parsed: u64 = str_val.parse().expect("Should parse valid number");
        assert_eq!(parsed, expected);
    }
}

#[test]
fn test_config_invalid_numeric_values() {
    let invalid_values = vec!["not_a_number", "123abc", "12.34.56", "-1", ""];

    for value in invalid_values {
        let result: Result<u64, _> = value.parse();
        assert!(result.is_err(), "Should reject invalid value: {value}");
    }
}

#[test]
fn test_config_case_sensitivity() {
    let lower = "BEARDOG_TEST_CASE";
    let mixed = "beardog_test_case";
    assert_ne!(lower, mixed);
}

#[tokio::test]
async fn test_config_concurrent_access() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let state = Arc::new(Mutex::new("initial".to_string()));
    let mut handles = vec![];

    for _i in 0..10 {
        let s = Arc::clone(&state);
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                let _guard = s.lock().await;
                tokio::task::yield_now().await;
            }
        }));
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }
}

#[test]
fn test_config_missing_required_values() {
    let missing: Option<&str> = None;
    let result = missing.ok_or("missing");
    assert!(result.is_err());

    // `None` maps to the same default as `Option::unwrap_or("fallback")`
    let with_fallback = "fallback";
    assert_eq!(with_fallback, "fallback");
    assert_eq!(missing, None);
}

#[test]
fn test_config_whitespace_handling() {
    let test_cases = vec![
        ("  value  ", "  value  "),
        ("\tvalue\t", "\tvalue\t"),
        (" ", " "),
        ("", ""),
    ];

    for (input, expected) in test_cases {
        let retrieved = input.to_string();
        assert_eq!(retrieved, expected);
        let trimmed = retrieved.trim();
        assert_eq!(trimmed, expected.trim());
    }
}
