//! Tests for security module components

use super::*;

#[test]
fn test_security_tokens_new() {
    let tokens = SecurityTokens::new();
    assert!(tokens.get_token("any").is_none());
}

#[test]
fn test_security_tokens_default() {
    let tokens = SecurityTokens::default();
    assert!(tokens.get_token("any").is_none());
}

#[test]
fn test_security_tokens_add_and_get() {
    let mut tokens = SecurityTokens::new();
    tokens.add_token("api_key".to_string(), "secret123".to_string());
    assert_eq!(tokens.get_token("api_key"), Some(&"secret123".to_string()));
}

#[test]
fn test_security_tokens_multiple() {
    let mut tokens = SecurityTokens::new();
    tokens.add_token("token1".to_string(), "value1".to_string());
    tokens.add_token("token2".to_string(), "value2".to_string());
    tokens.add_token("token3".to_string(), "value3".to_string());

    assert_eq!(tokens.get_token("token1"), Some(&"value1".to_string()));
    assert_eq!(tokens.get_token("token2"), Some(&"value2".to_string()));
    assert_eq!(tokens.get_token("token3"), Some(&"value3".to_string()));
}

#[test]
fn test_security_tokens_overwrite() {
    let mut tokens = SecurityTokens::new();
    tokens.add_token("key".to_string(), "old_value".to_string());
    tokens.add_token("key".to_string(), "new_value".to_string());

    assert_eq!(tokens.get_token("key"), Some(&"new_value".to_string()));
}

#[test]
fn test_security_tokens_nonexistent() {
    let tokens = SecurityTokens::new();
    assert!(tokens.get_token("nonexistent").is_none());
}

#[test]
fn test_security_tokens_clone() {
    let mut tokens = SecurityTokens::new();
    tokens.add_token("key".to_string(), "value".to_string());

    let cloned = tokens.clone();
    assert_eq!(cloned.get_token("key"), Some(&"value".to_string()));
}

#[test]
fn test_security_entry_creation() {
    let entry = SecurityEntry {
        trust_level: crate::TrustLevel::Basic,
    };
    assert!(format!("{:?}", entry).contains("SecurityEntry"));
}

#[test]
fn test_security_entry_clone() {
    let entry = SecurityEntry {
        trust_level: crate::TrustLevel::Verified,
    };
    let cloned = entry.clone();
    assert_eq!(
        format!("{:?}", entry.trust_level),
        format!("{:?}", cloned.trust_level)
    );
}
