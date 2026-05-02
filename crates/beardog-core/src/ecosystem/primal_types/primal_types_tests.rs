// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_types::canonical::capabilities::ServiceCapabilityType;

#[test]
fn test_service_dependency_required() {
    let dep = ServiceDependency::Required {
        capability: ServiceCapabilityType::Security,
        min_version: "1.0.0".to_string(),
        reason: "Required for auth".to_string(),
    };
    assert!(matches!(dep, ServiceDependency::Required { .. }));
}

#[test]
fn test_service_dependency_optional() {
    let dep = ServiceDependency::Optional {
        capability: ServiceCapabilityType::Compute,
        min_version: "1.0.0".to_string(),
        reason: "Optional enhancement".to_string(),
    };
    assert!(matches!(dep, ServiceDependency::Optional { .. }));
}

#[test]
fn test_capability_integration_config_default() {
    let config = CapabilityIntegrationConfig::default();
    assert!(config.has_security_capability());
    assert!(config.has_storage_capability());
    assert!(config.has_networking_capability());
    assert!(!config.has_compute_capability());
    assert!(!config.has_ai_capability());
}

#[test]
fn test_capability_type_serialization() {
    let types = [
        CapabilityType::Security,
        CapabilityType::Storage,
        CapabilityType::Compute,
        CapabilityType::Networking,
        CapabilityType::AI,
    ];
    for ct in types {
        let json = serde_json::to_string(&ct).unwrap();
        let decoded: CapabilityType = serde_json::from_str(&json).unwrap();
        assert_eq!(ct, decoded);
    }
}

#[test]
fn test_primal_health_default() {
    let health = PrimalHealth::default();
    assert_eq!(health.status, HealthStatus::Healthy);
    assert!(health.checks.is_empty());
}

#[test]
fn test_response_time_metrics_default() {
    let metrics = ResponseTimeMetrics::default();
    assert_eq!(metrics.average, 0.0);
    assert_eq!(metrics.p95, 0.0);
    assert_eq!(metrics.p99, 0.0);
}

#[test]
fn test_primal_error_new() {
    let err = PrimalError::new("CODE", "message");
    assert_eq!(err.code, "CODE");
    assert_eq!(err.message, "message");
    assert!(err.details.is_empty());
}

#[test]
fn test_primal_error_initialization_failed() {
    let err = PrimalError::initialization_failed("failed");
    assert_eq!(err.code, "InitializationFailed");
}

#[test]
fn test_primal_error_health_check_failed() {
    let err = PrimalError::health_check_failed("failed");
    assert_eq!(err.code, "HealthCheckFailed");
}

#[test]
fn test_primal_error_unsupported_operation() {
    let err = PrimalError::unsupported_operation("op");
    assert_eq!(err.code, "UnsupportedOperation");
}

#[test]
fn test_primal_request_default() {
    let req = PrimalRequest::default();
    assert!(!req.id.is_empty());
    assert!(!req.request_id.is_empty());
    assert_eq!(req.operation_type, "default");
}

#[test]
fn test_primal_response_default() {
    let resp = PrimalResponse::default();
    assert!(!resp.id.is_empty());
    assert!(!resp.request_id.is_empty());
    assert_eq!(resp.status, "pending");
    assert!(!resp.success);
}

#[test]
fn test_universal_integration_config_default() {
    let config = UniversalIntegrationConfig::default();
    assert!(config.enable_capability_discovery);
    assert!(config.enable_environment_discovery);
    assert!(!config.required_capabilities.is_empty());
}

#[test]
fn test_primal_capability_variants() {
    let _ = PrimalCapability::Security;
    let _ = PrimalCapability::Custom("test".to_string());
}

#[test]
fn test_auth_requirements_default() {
    let auth = AuthRequirements::default();
    assert!(auth.auth_type.is_empty());
    assert!(auth.required_scopes.is_empty());
}

#[test]
fn test_endpoint_security_config_default() {
    let config = EndpointSecurityConfig::default();
    assert!(!config.tls_required);
    assert!(!config.cert_validation);
}

#[test]
fn test_load_metrics_default() {
    let metrics = LoadMetrics::default();
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.memory_usage, 0.0);
}

#[test]
fn test_error_rate_metrics_default() {
    let metrics = ErrorRateMetrics::default();
    assert_eq!(metrics.error_rate, 0.0);
    assert_eq!(metrics.timeout_rate, 0.0);
}
