// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_authentication_config_default() {
    let config = CanonicalAuthenticationConfig::default();
    assert_eq!(config.jwt_expiration_seconds, 3600);
    assert_eq!(config.api_key_header.as_ref(), "X-API-Key");
    assert_eq!(config.password_min_length, 8);
    assert_eq!(config.max_auth_attempts, 3);
}

#[test]
fn test_authentication_config_production() {
    let config = CanonicalAuthenticationConfig::production();
    assert_eq!(config.jwt_expiration_seconds, 1800);
    assert_eq!(config.api_key_min_length, 64);
    assert_eq!(config.password_min_length, 12);
    assert!(config.password_require_symbols);
}

#[test]
fn test_validation_fails_with_default_jwt_secret() {
    let config = CanonicalAuthenticationConfig::default();
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_fails_with_short_jwt_secret() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("too_short");
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_succeeds_with_valid_jwt_secret() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validation_fails_with_zero_expiration() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.jwt_expiration_seconds = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_fails_oauth_missing_client_id() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.enable_oauth = true;
    config.oauth_client_id = Arc::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_fails_oauth_missing_client_secret() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.enable_oauth = true;
    config.oauth_client_id = Arc::from("valid-id");
    config.oauth_client_secret = Arc::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_succeeds_oauth_disabled() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.enable_oauth = false;
    assert!(config.validate().is_ok());
}

#[test]
fn test_validation_fails_short_api_key() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.api_key_min_length = 8;
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_fails_short_password() {
    let mut config = CanonicalAuthenticationConfig::default();
    config.jwt_secret = Arc::from("a".repeat(32).as_str());
    config.password_min_length = 4;
    assert!(config.validate().is_err());
}

#[test]
fn test_identity_provider_config_default() {
    let idp = IdentityProviderConfig::default();
    assert!(idp.provider_type.is_empty());
    assert!(!idp.enabled);
    assert!(idp.scopes.is_empty());
}

#[test]
fn test_password_complexity_flags() {
    let config = CanonicalAuthenticationConfig::default();
    assert!(config.password_require_uppercase);
    assert!(config.password_require_lowercase);
    assert!(config.password_require_numbers);
    assert!(!config.password_require_symbols);
}

#[test]
fn test_lockout_configuration() {
    let config = CanonicalAuthenticationConfig::default();
    assert_eq!(config.max_auth_attempts, 3);
    assert_eq!(config.lockout_duration_seconds, 900);

    let prod_config = CanonicalAuthenticationConfig::production();
    assert_eq!(prod_config.max_auth_attempts, 5);
    assert_eq!(prod_config.lockout_duration_seconds, 1800);
}

#[test]
fn test_jwt_refresh_configuration() {
    let config = CanonicalAuthenticationConfig::default();
    assert!(config.enable_jwt_refresh);
    assert_eq!(config.jwt_refresh_expiration_seconds, 86400);

    let prod_config = CanonicalAuthenticationConfig::production();
    assert_eq!(prod_config.jwt_refresh_expiration_seconds, 604_800);
}
