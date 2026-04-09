// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::canonical::monitoring::MonitoringConfigValidation;
use std::collections::HashMap;
use std::time::Duration;

fn sample_valid_config() -> UnifiedHealthConfig {
    let mut c = UnifiedHealthConfig::default();
    c.check_interval = Duration::from_secs(1);
    c.global_timeout = Duration::from_secs(2);
    c.failure_threshold = 1;
    c.success_threshold = 1;
    c.http_checks.endpoints.push(HttpEndpoint {
        name: "api".to_string(),
        url: "https://example/health".to_string(),
        method: HttpMethod::GET,
        headers: std::collections::HashMap::from([("x-test".to_string(), "1".to_string())]),
        expected_status: vec![200],
        expected_body: None,
        timeout: Some(Duration::from_secs(3)),
    });
    c.tcp_checks.endpoints.push(TcpEndpoint {
        name: "db".to_string(),
        host: "127.0.0.1".to_string(),
        port: 5432,
        timeout: None,
    });
    c.database_checks.connections.push(DatabaseConnection {
        name: "main".to_string(),
        connection_string: "postgres://localhost/db".to_string(),
        database_type: DatabaseType::PostgreSQL,
        timeout: None,
        custom_query: None,
    });
    c
}

#[test]
fn unified_health_default_clone_debug() {
    let a = UnifiedHealthConfig::default();
    let b = a.clone();
    assert_eq!(a.enabled, b.enabled);
    let _ = format!("{a:?}");
}

#[test]
fn http_tcp_db_defaults_and_user_agent() {
    let http = HttpHealthCheckConfig::default();
    assert!(http.enabled);
    assert!(http.follow_redirects);
    assert!(http.verify_ssl);
    assert!(http.user_agent.contains("BearDog"));

    let tcp = TcpHealthCheckConfig::default();
    assert!(tcp.enabled);
    assert!(!tcp.connection_reuse);

    let db = DatabaseHealthCheckConfig::default();
    assert_eq!(db.test_query, "SELECT 1");
}

#[test]
fn http_method_database_service_recovery_serde_roundtrip() {
    for m in [
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
    ] {
        let v = serde_json::to_value(&m).expect("serialize HttpMethod");
        let back: HttpMethod = serde_json::from_value(v).expect("deserialize HttpMethod");
        assert_eq!(std::mem::discriminant(&m), std::mem::discriminant(&back));
    }

    for dt in [
        DatabaseType::PostgreSQL,
        DatabaseType::MySQL,
        DatabaseType::Custom("x".into()),
    ] {
        let v = serde_json::to_value(&dt).expect("serialize DatabaseType");
        let back: DatabaseType = serde_json::from_value(v).expect("deserialize DatabaseType");
        assert_eq!(format!("{dt:?}"), format!("{back:?}"));
    }

    for st in [
        ServiceType::HTTP,
        ServiceType::GRpc,
        ServiceType::Custom("c".into()),
    ] {
        let v = serde_json::to_value(&st).expect("serialize ServiceType");
        let back: ServiceType = serde_json::from_value(v).expect("deserialize ServiceType");
        assert_eq!(format!("{st:?}"), format!("{back:?}"));
    }

    for ra in [
        RecoveryActionType::RestartService,
        RecoveryActionType::Custom("script".into()),
    ] {
        let v = serde_json::to_value(&ra).expect("serialize RecoveryActionType");
        let back: RecoveryActionType =
            serde_json::from_value(v).expect("deserialize RecoveryActionType");
        assert_eq!(format!("{ra:?}"), format!("{back:?}"));
    }
}

#[test]
fn escalation_condition_serde_roundtrip() {
    let cases = vec![
        EscalationCondition::FailureCount(3),
        EscalationCondition::FailureDuration(Duration::from_secs(10)),
        EscalationCondition::FailurePercentage(12.5),
        EscalationCondition::Custom {
            condition: "x".into(),
            parameters: HashMap::from([("k".into(), serde_json::json!(1))]),
        },
    ];
    for c in cases {
        let v = serde_json::to_value(&c).expect("serialize EscalationCondition");
        let back: EscalationCondition =
            serde_json::from_value(v).expect("deserialize EscalationCondition");
        assert_eq!(format!("{c:?}"), format!("{back:?}"));
    }
}

#[test]
fn custom_check_and_full_config_json_roundtrip() {
    let mut custom = HashMap::new();
    custom.insert(
        "k".into(),
        CustomHealthCheckConfig {
            name: "c".into(),
            check_type: "exec".into(),
            config: HashMap::from([("p".into(), serde_json::json!(true))]),
            timeout: Duration::from_secs(1),
            interval: Duration::from_secs(2),
        },
    );
    let cfg = UnifiedHealthConfig {
        custom_checks: custom,
        ..sample_valid_config()
    };
    let v = serde_json::to_value(&cfg).expect("serialize UnifiedHealthConfig");
    let back: UnifiedHealthConfig =
        serde_json::from_value(v).expect("deserialize UnifiedHealthConfig");
    assert_eq!(back.custom_checks.len(), 1);
    assert!(MonitoringConfigValidation::validate(&back).is_ok());
}

#[test]
fn validate_errors_cover_branches() {
    let mut c = sample_valid_config();
    c.check_interval = Duration::ZERO;
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.global_timeout = Duration::ZERO;
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.failure_threshold = 0;
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.success_threshold = 0;
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.http_checks.endpoints[0].name.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.http_checks.endpoints[0].url.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.http_checks.endpoints[0].expected_status.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.tcp_checks.endpoints[0].name.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.tcp_checks.endpoints[0].host.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.tcp_checks.endpoints[0].port = 0;
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.database_checks.connections[0].name.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());

    c = sample_valid_config();
    c.database_checks.connections[0].connection_string.clear();
    assert!(MonitoringConfigValidation::validate(&c).is_err());
}

#[test]
fn is_compatible_with_always_true() {
    let c = UnifiedHealthConfig::default();
    assert!(MonitoringConfigValidation::is_compatible_with(&c, 99));
}
