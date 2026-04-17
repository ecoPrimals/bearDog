// SPDX-License-Identifier: AGPL-3.0-or-later
//! Security, MFA, encryption, rate limiting, and alerting tests.

#![cfg(test)]

use std::sync::Arc;
use std::time::Duration;

use crate::canonical::config::security::{
    CanonicalAuthorizationConfig, CanonicalEncryptionConfig, CanonicalMfaConfig,
    CanonicalSecurityConfig, RateLimitingConfig,
};
use crate::canonical::monitoring::MonitoringConfigValidation;
use crate::canonical::monitoring::alerting::{
    AlertCondition, AlertRule, AlertSeverity, ComparisonOperator, NotificationChannelType,
    UnifiedAlertingConfig,
};

use super::common::assert_serde_json_roundtrip;

#[test]
fn canonical_mfa_config_and_nested_validate() {
    let m = CanonicalMfaConfig::default();
    assert_serde_json_roundtrip(&m);
    let json = serde_json::to_string(&m).expect("serialize mfa");
    let rt: CanonicalMfaConfig = serde_json::from_str(&json).expect("deserialize mfa");
    assert_eq!(m.enabled, rt.enabled);

    let prod = CanonicalMfaConfig::production();
    assert!(prod.required);

    m.validate().expect("default mfa validates");

    let mut bad = CanonicalMfaConfig::default();
    bad.totp.enabled = true;
    bad.totp.issuer = String::new();
    bad.validate().expect_err("empty totp issuer");

    let mut bad_sms = CanonicalMfaConfig::default();
    bad_sms.sms.enabled = true;
    bad_sms.sms.from_number = String::new();
    bad_sms.validate().expect_err("sms from number");
}

#[test]
fn canonical_security_config_roundtrip_validate_and_rate_limiting() {
    let mut c = CanonicalSecurityConfig::default();
    c.authentication.jwt_secret = Arc::from("01234567890123456789012345678901");
    c.authorization = CanonicalAuthorizationConfig::production();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize security");
    let rt: CanonicalSecurityConfig = serde_json::from_str(&json).expect("deserialize security");
    assert_eq!(c.enable_encryption, rt.enable_encryption);

    let p = CanonicalSecurityConfig::production();
    assert!(p.enable_hsm);

    c.validate()
        .expect("security validates with non-placeholder JWT secret");

    let rl = RateLimitingConfig::with_defaults();
    assert_eq!(
        rl.max_requests_per_minute,
        RateLimitingConfig::DEFAULT_MAX_REQUESTS_PER_MINUTE
    );

    let from_map = RateLimitingConfig::from_env_provider(|k| match k {
        "BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN" => Some("42".to_string()),
        _ => None,
    });
    assert_eq!(from_map.max_requests_per_minute, 42);

    let prod_rl = RateLimitingConfig::production();
    assert!(prod_rl.max_requests_per_minute > 0);
}

#[test]
fn canonical_encryption_validate_and_key_derivation_errors() {
    let e = CanonicalEncryptionConfig::default();
    e.validate().expect("default encryption ok");
    assert_serde_json_roundtrip(&e);
    let json = serde_json::to_string(&e).expect("serialize encryption");
    let rt: CanonicalEncryptionConfig =
        serde_json::from_str(&json).expect("deserialize encryption");
    assert_eq!(e.default_algorithm, rt.default_algorithm);

    let mut bad = CanonicalEncryptionConfig::default();
    bad.default_algorithm = String::new();
    bad.validate().expect_err("empty algorithm");

    let mut enc = CanonicalEncryptionConfig::default();
    enc.key_derivation.iterations = 100;
    enc.validate().expect_err("low iterations");

    let mut enc2 = CanonicalEncryptionConfig::default();
    enc2.key_derivation.salt_length = 4;
    enc2.validate().expect_err("salt too short");
}

#[test]
fn unified_alerting_validate_happy_and_error_paths() {
    let mut a = UnifiedAlertingConfig::default();
    MonitoringConfigValidation::validate(&a).expect("default ok");

    a.evaluation_interval = Duration::ZERO;
    MonitoringConfigValidation::validate(&a).expect_err("zero eval interval");

    let mut a2 = UnifiedAlertingConfig::default();
    a2.notification_timeout = Duration::ZERO;
    MonitoringConfigValidation::validate(&a2).expect_err("zero notification timeout");

    let mut a3 = UnifiedAlertingConfig::default();
    a3.rules.push(AlertRule {
        name: String::new(),
        condition: AlertCondition::MetricThreshold {
            metric: "m".to_string(),
            operator: ComparisonOperator::GreaterThan,
            threshold: 1.0,
        },
        severity: AlertSeverity::High,
        duration: Duration::from_secs(5),
        labels: std::collections::HashMap::new(),
        annotations: std::collections::HashMap::new(),
    });
    MonitoringConfigValidation::validate(&a3).expect_err("empty rule name");

    let mut a4 = UnifiedAlertingConfig::default();
    a4.rules.push(AlertRule {
        name: "r".to_string(),
        condition: AlertCondition::HealthCheckFailure {
            service: "s".to_string(),
            failure_count: 3,
        },
        severity: AlertSeverity::Low,
        duration: Duration::ZERO,
        labels: std::collections::HashMap::new(),
        annotations: std::collections::HashMap::new(),
    });
    MonitoringConfigValidation::validate(&a4).expect_err("zero duration");

    assert!(MonitoringConfigValidation::is_compatible_with(
        &UnifiedAlertingConfig::default(),
        0
    ));
}

#[test]
fn unified_alerting_enums_serde_roundtrip() {
    let cond = AlertCondition::ErrorRate {
        service: "svc".to_string(),
        rate_threshold: 0.05,
        time_window: Duration::from_secs(60),
    };
    assert_serde_json_roundtrip(&cond);

    let n = NotificationChannelType::Custom("x".to_string());
    assert_serde_json_roundtrip(&n);
}

#[test]
fn unified_alerting_config_default_serde_roundtrip() {
    let u = UnifiedAlertingConfig::default();
    assert_serde_json_roundtrip(&u);
}

#[test]
fn alert_severity_and_comparison_operator_roundtrip() {
    assert_serde_json_roundtrip(&AlertSeverity::Critical);
    assert_serde_json_roundtrip(&ComparisonOperator::GreaterThanOrEqual);
}

#[test]
fn canonical_encryption_production_sets_rotation() {
    let p = CanonicalEncryptionConfig::production();
    assert_eq!(p.key_rotation_days, 30);
}

#[test]
fn canonical_mfa_totp_and_backup_serde() {
    use crate::canonical::config::security::{BackupCodesConfig, TotpConfig};
    assert_serde_json_roundtrip(&TotpConfig::default());
    assert_serde_json_roundtrip(&BackupCodesConfig::default());
}

#[test]
fn hsm_encryption_config_default_serde() {
    use crate::canonical::config::security::HsmEncryptionConfig;
    assert_serde_json_roundtrip(&HsmEncryptionConfig::default());
}

#[test]
fn key_derivation_config_validate_ok_at_defaults() {
    use crate::canonical::config::security::KeyDerivationConfig;
    let k = KeyDerivationConfig::default();
    k.validate().expect("default key derivation");
}

#[test]
fn alerting_custom_condition_serde() {
    let mut params = std::collections::HashMap::new();
    params.insert("k".to_string(), serde_json::json!(1));
    let c = AlertCondition::Custom {
        expression: "1 > 0".to_string(),
        parameters: params,
    };
    assert_serde_json_roundtrip(&c);
}

#[test]
fn rate_limiting_config_default_equals_with_defaults() {
    assert_eq!(
        RateLimitingConfig::default().max_requests_per_minute,
        RateLimitingConfig::with_defaults().max_requests_per_minute
    );
}

#[test]
fn canonical_security_new_matches_default() {
    assert_eq!(
        CanonicalSecurityConfig::new().enable_encryption,
        CanonicalSecurityConfig::default().enable_encryption
    );
}
