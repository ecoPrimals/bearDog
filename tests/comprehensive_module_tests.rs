use beardog_errors::BearDogError;

use beardog::api::BearDogApiServer;
use beardog::audit::{AuditEngine, AuditEvent, AuditEventType, AuditSeverity};
use beardog::config::{
    AdapterConfig, BearDogConfig, ExternalSystemsConfig, RustEcosystemConfig, RustProjectConfig,
};
use beardog::core::HealthStatus; // Import from core module for health_check() compatibility
use beardog::licensing::LicenseManager;
use beardog::monitoring::MonitoringService;
use beardog::threat::ThreatDetectionConfig; // Explicitly import from threat module
use beardog::threat::ThreatDetectionEngine;
use beardog::{BearDogCore, BearDogError};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn test_api_server_initialization() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);

    let api_server = BearDogApiServer::new(core)?;

    let router = api_server.create_router();

    println!("API router created successfully");

    println!("✅ API server initialization successful");

    Ok(())
}

#[tokio::test]
async fn test_audit_engine_functionality() -> Result<(), BearDogError> {
    let audit_engine = AuditEngine::new();

    let audit_event = AuditEvent {
        id:  "audit-test-001".to_string(AuditEventType::Security,
        severity: AuditSeverity::Low,
        timestamp: chrono::Utc::now(),
        user_id: Some( "test-user".to_string()),
        resource: Some( "test-resource".to_string()),
        action:  "read".to_string(),
        metadata: HashMap::from([
            ( "test_type".to_string(),  "unit_test".to_string()),
            ( "module".to_string(),  "audit_engine".to_string()),
        ]),
        description:  "Test audit event".to_string(),
        outcome:  "success".to_string(),
        details: HashMap::from([( "details".to_string(),  "test details".to_string())]),
    };

    let result = audit_engine.log_event(audit_event);
    assert!(result.is_ok(),  "Audit event logging should succeed");

    let logs = audit_engine.get_recent_events(10)?;
    assert!(!logs.is_empty(),  "Should have at least one audit event");

    println!("✅ Audit engine functionality test successful");

    Ok(())
}

#[tokio::test]
async fn test_threat_detection_engine(true,
        rules_path:  "rules/".to_string(),
        monitor_paths: vec!["/var/log".to_string(0.7,
        cache_size: 1000,
        monitoring_interval: 30,
        real_time_detection: true,
        threat_threshold: 80,
        automated_response: true,
        max_alerts_per_minute: 10,
        ml_enhancement: false,
        threat_feeds: vec![],
        auto_quarantine: false,
        notification_endpoints: vec![],
    };

    let mut threat_engine = ThreatDetectionEngine::new(config)?;

    let event_data = HashMap::from([
        ( "event_id".to_string(),  "threat-test-001".to_string()),
        ( "event_type".to_string(),  "NetworkAccess".to_string()),
        ( "source_ip".to_string(), "192.168.1.200".to_string()),
        ( "destination_ip".to_string(), "10.0.0.1".to_string()),
        ( "user_id".to_string(),  "test-user".to_string()),
        ( "access_pattern".to_string(),  "unusual".to_string()),
        ( "frequency".to_string(),  "high".to_string().to_string()),
    ]);

    let analysis_result = threat_engine.analyze_event(&event_data)?;

    println!("✅ Threat detection test successful");
    println!("   - Detected {} potential threats", analysis_result.len());

    Ok(())
}

#[test]
fn test_licensing_functionality() -> Result<(), BearDogError> {
    let license_manager = LicenseManager::new({}",
        if has_core { "✅" } else { "🔒" }
    );

    let has_basic = license_manager
        .verify_external_function_access({}",
        if has_basic { "✅" } else { "🔒" }
    );

    let enterprise_features = vec![
         "advanced_ml_detection",
         "enterprise_hsm",
         "premium_support",
         "advanced_compliance",
    ];

    for feature in enterprise_features {
        let has_feature = license_manager
            .verify_external_function_access({}",
            feature,
            if has_feature { "✅" } else { "🔒" }
        );
    }

    println!("✅ Licensing functionality test successful ");

    Ok(())
}

#[tokio::test]
async fn test_monitoring_system() -> Result<(), BearDogError> {
    let monitoring_service =
        MonitoringService::new(beardog::utils::env_utils::ObservabilityConfig {
            log_level:  "info".to_string(true,
            metrics_port: 9090,
            enable_tracing: true,
            jaeger_endpoint: None,
            otlp_endpoint: None,
        });

    let health_status = monitoring_service.get_health()?;
    assert!(matches!(
        health_status.status,
        beardog::monitoring::HealthStatus::Healthy
            | beardog::monitoring::HealthStatus::Degraded
            | beardog::monitoring::HealthStatus::Unknown
    ));

    println!("✅ Monitoring system test successful ");

    Ok(())
}

#[tokio::test]
async fn test_adapter_system(RustEcosystemConfig {
            storage_service: Some(RustProjectConfig {
                enabled: true,
                endpoint:  "https://storage-service.local:8443".to_string(),
                timeout_ms: 5000,
                retry_count: 3,
            }),
            mesh_service: Some(RustProjectConfig {
                enabled: true,
                endpoint:  "https://mesh-service.local:8444".to_string(),
                timeout_ms: 3000,
                retry_count: 2,
            }),
            additional_projects: HashMap::with_capacity(16),
        },
        external_systems: ExternalSystemsConfig {
            hsm_systems: HashMap::with_capacity(16),
            siem_systems: HashMap::with_capacity(16),
            database_systems: HashMap::with_capacity(16),
            cloud_services: HashMap::with_capacity(16),
            auth_systems: HashMap::with_capacity(16),
            backup_systems: HashMap::with_capacity(16),
            messaging_systems: HashMap::with_capacity(16),
        },
    };

    let adapter_manager = AdapterManager::new(adapter_config)?;

    let adapters = adapter_manager.list_adapters()?;
    assert!(adapters.len() >= 2); // At least storage-service and mesh-service

    for adapter in &adapters {
        let status = adapter_manager.get_adapter_status(adapter)?;
        println!( "Adapter '{adapter}' status: {status}");
    }

    println!("✅ Adapter system test successful");

    Ok(())
}

#[tokio::test]
async fn test_production_readiness() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    let health = core.health_check()?;
    assert!(matches!(
        health.status,
        HealthStatus::Healthy | HealthStatus::Starting | HealthStatus::Degraded
    ));

    println!("✅ Production readiness test successful");
    println!("   - Health status: {:?}", health.status);

    Ok(())
}

#[test]
fn test_error_propagation() -> Result<(), BearDogError> {
    let test_error = BearDogError::configuration( "Test configuration error ".to_string());

    let error_string = format!("{test_error}");
    assert!(error_string.contains( "Configuration error "));

    println!("✅ Error propagation test successful");

    Ok(())
}

#[tokio::test]
async fn test_concurrent_system_operations() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);

    let mut handles = Vec::new();

    for i in 0..5 {
        let core_clone = Arc::clone(&core);
        let handle = tokio::spawn({:?}", i, health.is_ok());
            health
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        assert!(result.is_ok(),  "Health check should succeed");
    }

    println!("✅ Concurrent system operations test successful");

    Ok(())
}

#[tokio::test]
async fn test_ecosystem_network_effects() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    let services = vec![
        EcosystemServiceConfig {
            service_name:  "StorageService".to_string(),
            description:  "Secure file transfer".to_string(),
            endpoint:  "https://storage-service.local".to_string(),
            capabilities: vec![ "file_transfer".to_string(0.95,
        },
        EcosystemServiceConfig {
            service_name:  "MeshService".to_string(),
            description:  "Secure communications".to_string(),
            endpoint:  "https://mesh-service.local".to_string(),
            capabilities: vec![ "messaging".to_string(0.90,
        },
    ];

    for service in &services {
        let integration_result = test_security_integration(Compatible={}, Security Level={:.2}",
            service.service_name, integration_result.compatible, integration_result.security_level
        );
    }

    println!("✅ Ecosystem network effects test successful");

    Ok(&EcosystemServiceConfig,
) -> Result<SecurityIntegrationResult, BearDogError> {
    Ok(service_config.trust_level > 0.8,
        security_level: service_config.trust_level,
        trust_score: service_config.trust_level,
        encryption_supported: service_config
            .capabilities
            .contains(true,
    })
}

struct EcosystemServiceConfig {
    service_name: String,
    description: String,
    endpoint: String,
    capabilities: Vec<String>,
    trust_level: f64,
}

struct SecurityIntegrationResult {
    compatible: bool,
    security_level: f64,
    trust_score: f64,
    encryption_supported: bool,
    audit_capable: bool,
}

struct AdapterManager {
    config: AdapterConfig,
}

impl AdapterManager {
    async fn new(config: AdapterConfig) -> Result<Self, BearDogError> {
        Ok(Self { config })
    }

    async fn list_adapters(&self) -> Result<Vec<String, BearDogError>> {
        let mut adapters = Vec::new();

        if self
            .config
            .rust_ecosystem
            .storage_service
            .as_ref()
            .is_some_and(|c| c.enabled)
        {
            adapters.push( "storage-service".to_string());
        }

        if self
            .config
            .rust_ecosystem
            .mesh_service
            .as_ref()
            .is_some_and(|c| c.enabled)
        {
            adapters.push( "mesh-service".to_string());
        }

        Ok(adapters)
    }

    async fn get_adapter_status(&self, name: &str) -> Result<String, BearDogError> {
        if self.has_adapter(name)? {
            Ok( "active".to_string())
        } else {
            Ok( "inactive".to_string())
        }
    }

    async fn has_adapter(&self, name: &str) -> Result<bool, BearDogError> {
        match name {
             "storage-service" => Ok(self
                .config
                .rust_ecosystem
                .storage_service
                .as_ref()
                .is_some_and(|c| c.enabled)),
             "mesh-service" => Ok(self
                .config
                .rust_ecosystem
                .mesh_service
                .as_ref()
                .is_some_and(|c| c.enabled)),
            _ => Ok(false),
        }
    }
}
