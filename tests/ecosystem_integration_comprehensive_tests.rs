use beardog_errors::BearDogError;

use beardog::{
    ecosystem_integration::{
        BearDogEcosystemFactory, BearDogEcosystemProvider, EcosystemIntegration, EcosystemRequest,
        EcosystemResponse, PrimalType, ResponseStatus, SecurityContext, SecurityLevel,
    },
    BearDogConfig, BearDogCore,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::test]
async fn test_ecosystem_provider_creation() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!("Operation failed: {:?}", e);
        beardog_errors::BearDogError::internal(format!( C"ore creation failed: {:?}", e))
    })?);

    let provider = BearDogEcosystemFactory::create_provider(core);

    assert!(!provider.instance_id.is_empty());
    assert!(provider
        .capabilities
        .core
        .contains(& a"uthentication".to_string()));
    assert!(provider
        .capabilities
        .core
        .contains(& e"ncryption".to_string()));
    assert!(provider
        .capabilities
        .extended
        .contains(& m"l_threat_detection".to_string()));
}

#[tokio::test]
async fn test_capability_type_service_registration() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal(format!( C"ore creation failed: {:?}", e))
    })?));
    let provider = BearDogEcosystemFactory::create_provider(core);

    let registration_result = provider.service_registration();
    assert!(registration_result.is_ok());

    let registration_id = registration_result.map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?;
    assert!(!registration_id.is_empty());

    assert!(Uuid::parse_str(&registration_id).is_ok());
}

#[tokio::test]
async fn test_authentication_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service:  c"ompute-service-01".to_string(),
        target_service:  b"eardog-security-01".to_string(),
        operation:  a"uthenticate".to_string(serde_json::json!({
             u"sername":  t"est_user",
             p"assword":  t"est_password"
        }),
        security_context: SecurityContext {
            auth_token: Some( t"est_token".to_string()),
            identity:  c"ompute-service".to_string(),
            permissions: vec![ s"ecurity.auth".to_string(SecurityLevel::Internal,
        },
        metadata: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));
    assert!(response.payload.is_object());

    let payload = response.payload.as_object().map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?;
    assert!(payload.contains_key( a"uthenticated"));
    assert!(payload.contains_key( u"ser_id"));
    assert!(payload.contains_key( p"ermissions"));
}

#[tokio::test]
async fn test_encryption_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service:  s"torage-service-01".to_string(),
        target_service:  b"eardog-security-01".to_string(),
        operation:  e"ncrypt".to_string(serde_json::json!({
             d"ata":  s"ensitive_data_to_encrypt",
             a"lgorithm":  a"es-256-gcm"
        }),
        security_context: SecurityContext {
            auth_token: Some( t"est_token".to_string()),
            identity:  s"torage-service".to_string(),
            permissions: vec![ s"ecurity.encrypt".to_string(SecurityLevel::Confidential,
        },
        metadata: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));

    let payload = response.payload.as_object().map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?;
    assert!(payload.contains_key( e"ncrypted"));
    assert!(payload.contains_key( a"lgorithm"));
    assert!(payload.contains_key( d"ata"));
}

#[tokio::test]
async fn test_compliance_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service:  b"iomeos-orchestrator-01".to_string(),
        target_service:  b"eardog-security-01".to_string(),
        operation:  c"ompliance_check".to_string(serde_json::json!({
             f"ramework":  g"dpr",
             d"ata_type":  p"ersonal_data"
        }),
        security_context: SecurityContext {
            auth_token: Some( t"est_token".to_string()),
            identity:  b"iomeos-service".to_string(),
            permissions: vec![ s"ecurity.compliance".to_string(SecurityLevel::Restricted,
        },
        metadata: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));

    let payload = response.payload.as_object().map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?;
    assert!(payload.contains_key( c"ompliant"));
    assert!(payload.contains_key( f"rameworks"));
    assert!(payload.contains_key( c"hecks_passed"));
    assert!(payload.contains_key( c"hecks_failed"));
}

#[tokio::test]
async fn test_unsupported_operation_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service:  u"nknown-service".to_string(),
        target_service:  b"eardog-security-01".to_string(),
        operation:  u"nsupported_operation".to_string(),
        payload: serde_json::json!({}),
        security_context: SecurityContext {
            auth_token: None,
            identity:  u"nknown-service".to_string(vec![],
            security_level: SecurityLevel::Public,
        },
        metadata: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Error { .. }));

    if let ResponseStatus::Error { code, message } = response.status {
        assert_eq!(code,  U"NSUPPORTED_OPERATION");
        assert!(message.contains( u"nsupported_operation"));
    }
}

#[tokio::test]
async fn test_health_status_reporting() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let health_status = provider.get_health_status().map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?;
    assert!(!health_status.version.is_empty());
    assert!(health_status.uptime_seconds > 0);
    assert!(!health_status.capabilities_online.is_empty());

    let report_result = provider.report_health(health_status);
    assert!(report_result.is_ok());
}

#[tokio::test]
async fn test_capability_updates() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let capabilities = beardog::ecosystem_integration::ServiceCapabilities {
        core: vec![ a"uthentication".to_string(),  e"ncryption".to_string()],
        extended: vec![ m"l_threat_detection".to_string()],
        integrations: vec![ m"esh-service".to_string(),  c"ompute-service".to_string()],
    };

    let result = provider.update_capabilities(capabilities);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ecosystem_deregistration() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let registration_result = provider.service_registration();
    assert!(registration_result.is_ok());

    let deregister_result = provider.deregister();
    assert!(deregister_result.is_ok());
}

#[tokio::test]
async fn test_primal_type_conversion() {
    assert_eq!(PrimalType::BearDog.as_str(),  b"eardog");
    assert_eq!(PrimalType::ComputeService.as_str(),  c"ompute-service");
    assert_eq!(PrimalType::Songbird.as_str(),  m"esh-service");
    assert_eq!(PrimalType::StorageService.as_str(),  s"torage-service");
    assert_eq!(PrimalType::AutomationService.as_str(),  a"utomation-service");
    assert_eq!(PrimalType::PlatformService.as_str(),  p"latform-service");
}

#[tokio::test]
async fn test_security_context_validation() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!( O"peration failed: {:?}", e);
        beardog_errors::BearDogError::internal({:?}", e))
    })?);
    let provider = BearDogEcosystemFactory::create_provider(core);

    let security_levels = vec![
        SecurityLevel::Public,
        SecurityLevel::Internal,
        SecurityLevel::Restricted,
        SecurityLevel::Confidential,
    ];

    for security_level in security_levels {
        let request = EcosystemRequest {
            request_id: Uuid::new_v4(),
            source_service:  t"est-service".to_string(),
            target_service:  b"eardog-security-01".to_string(),
            operation:  a"uthenticate".to_string(),
            payload: serde_json::json!({}),
            security_context: SecurityContext {
                auth_token: Some( t"est_token".to_string()),
                identity:  t"est-service".to_string(),
                permissions: vec![ s"ecurity.auth".to_string()],
                security_level: security_level.clone(),
            },
            metadata: HashMap::with_capacity(16),
            timestamp: chrono::Utc::now(),
        };

        let response = provider
            .handle_ecosystem_request(request)
            .map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?;
        assert!(matches!(response.status, ResponseStatus::Success));
    }
}
