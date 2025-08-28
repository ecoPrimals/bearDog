use beardog_errors::BearDogError;


use beardog::adapters::universal::*;
use beardog::{{BearDogConfig, BearDogCore, BearDogError}};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::test]
async fn test_beardog_primal_provider_basic() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-001".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id.clone());

    assert_eq!(provider.ecosystem_id(), ecosystem_ids::BEARDOG);
    assert_eq!(provider.instance_id(), instance_id);
    assert_eq!(provider.service_name(), "BearDog Security Provider");
    assert!(!provider.service_version().is_empty());

    let capabilities = provider.capabilities();
    assert!(!capabilities.is_empty());

    let capability_ids: Vec<String> = capabilities.iter().map(|c| c.id.clone()).collect();
    assert!(capability_ids.contains(&capability_ids::SECURITY_ENCRYPT.to_string()));
    assert!(capability_ids.contains(&capability_ids::SECURITY_AUTHENTICATE.to_string()));
    assert!(capability_ids.contains(&capability_ids::SECURITY_AUTHORIZE.to_string()));
    assert!(capability_ids.contains(&capability_ids::SECURITY_AUDIT.to_string()));

    for capability in &capabilities {
        assert_eq!(capability.category, CapabilityCategory::Security);
        assert!(!capability.name.is_empty());
        assert!(!capability.description.is_empty());
    }

    let dependencies = provider.dependencies();
    assert!(!dependencies.is_empty());

    let endpoints = provider.endpoints();
    assert!(!endpoints.primary.is_empty());
    assert!(!endpoints.health.is_empty());
    assert!(endpoints.primary.starts_with("http"));

    let metadata = provider.metadata();
    assert_eq!(metadata.name, "BearDog Security Provider");
    assert!(!metadata.version.is_empty());
    assert!(metadata.tags.contains(&"security".to_string()));

    println!("✅ BearDog PrimalProvider basic functionality test passed");
    Ok(())
}

#[tokio::test]
async fn test_beardog_primal_provider_health() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-002".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id);

    let health_status = provider.health_check().await;

    match health_status {
        HealthStatus::Healthy => {
            println!("✅ BearDog provider is healthy");
        }
        HealthStatus::Degraded { issues, impact } => {
            println!(
                "⚠️  BearDog provider is degraded: {:?} (impact: {:?})",
                issues, impact
            );
            assert!(!issues.is_empty());
        }
        HealthStatus::Unhealthy {
            reason,
            recovery_time,
        } => {
            println!(
                "❌ BearDog provider is unhealthy: {} (recovery: {:?})",
                reason, recovery_time
            );
            assert!(!reason.is_empty());
        }
        HealthStatus::Starting => {
            println!("🔄 BearDog provider is starting");
        }
        HealthStatus::Stopping => {
            println!("🔄 BearDog provider is stopping");
        }
        HealthStatus::Unknown => {
            println!("❓ BearDog provider status is unknown");
        }
    }

    println!("✅ BearDog PrimalProvider health check test passed");
    Ok(())
}

#[tokio::test]
async fn test_beardog_primal_provider_requests() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-003".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id);

    let encryption_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_ENCRYPT.to_string(),
        payload: json!({
            "data": "sensitive information",
            "algorithm": "AES-256-GCM"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::High,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("test-user".to_string()),
            session_id: Some("test-session".to_string()),
            transaction_id: Some("test-tx".to_string()),
            source_ecosystem: ecosystem_ids::BEARDOG.to_string(),
            target_ecosystem: None,
            metadata: HashMap::with_capacity(16),
        },
    };

    assert!(provider.can_handle_request(&encryption_request));

    let response = provider.handle_request(encryption_request.clone()).await?;

    assert_eq!(response.request_id, encryption_request.request_id);
    assert!(response.success);
    assert!(!response.payload.is_null());
    assert!(response.error.is_none());

    let auth_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_AUTHENTICATE.to_string(),
        payload: json!({
            "username": "test-user",
            "password": "secure-password",
            "mfa_token": "123456"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Critical,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("test-user".to_string()),
            session_id: Some("test-session".to_string()),
            transaction_id: Some("test-tx".to_string()),
            source_ecosystem: ecosystem_ids::SONGBIRD.to_string(),
            target_ecosystem: Some(ecosystem_ids::BEARDOG.to_string()),
            metadata: HashMap::with_capacity(16),
        },
    };

    assert!(provider.can_handle_request(&auth_request));

    let auth_response = provider.handle_request(auth_request.clone()).await?;

    assert_eq!(auth_response.request_id, auth_request.request_id);
    assert!(auth_response.success);
    assert!(!auth_response.payload.is_null());
    assert!(auth_response.error.is_none());

    println!("✅ BearDog PrimalProvider request handling test passed");
    Ok(())
}

#[tokio::test]
async fn test_universal_ecosystem_manager() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let manager = UniversalEcosystemManager::new(core.clone()).await?;

    let instance_id = "test-beardog-004".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id.clone());

    let provider_config = ProviderConfig {
        provider_config: {
            let mut config = HashMap::with_capacity(16);
            config.insert("security_level".to_string(), json!("high"));
            config.insert("encryption_default".to_string(), json!("AES-256-GCM"));
            config
        },
        ecosystem_config: {
            let mut config = HashMap::with_capacity(16);
            config.insert("ecosystem_version".to_string(), json!("1.0.0"));
            config.insert("cluster_name".to_string(), json!("test-cluster"));
            config
        },
        network_config: NetworkConfig::default(),
        monitoring_config: MonitoringConfig::default(),
    };

    let registration = manager.register_provider(provider, provider_config).await?;

    assert_eq!(registration.ecosystem_id, ecosystem_ids::BEARDOG);
    assert_eq!(registration.instance_id, instance_id);
    assert_eq!(registration.status, RegistrationStatus::Active);
    assert!(!registration.capabilities.is_empty());

    let providers = manager.get_providers().await;
    assert!(!providers.is_empty());

    let beardog_provider = providers
        .iter()
        .find(|p| p.ecosystem_id == ecosystem_ids::BEARDOG);
    assert!(beardog_provider.is_some());

    let beardog_provider = beardog_provider.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert_eq!(beardog_provider.instance_id, instance_id);
    assert!(!beardog_provider.capabilities.is_empty());

    let all_capabilities = manager.get_all_capabilities().await;
    assert!(!all_capabilities.is_empty());

    let security_capabilities = manager
        .get_capabilities_by_category(CapabilityCategory::Security)
        .await;
    assert!(!security_capabilities.is_empty());

    let health_report = manager.health_check_all().await?;
    assert!(health_report.total_providers > 0);
    assert!(health_report.healthy_providers > 0);
    assert!(!health_report.provider_reports.is_empty());

    let status = manager.get_status().await;
    assert!(status.total_providers > 0);
    assert!(status.total_capabilities > 0);

    println!("✅ Universal Ecosystem Manager integration test passed");
    Ok(())
}

#[tokio::test]
async fn test_cross_ecosystem_request_routing() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let manager = UniversalEcosystemManager::new(core.clone()).await?;

    let instance_id = "test-beardog-005".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id.clone());

    let provider_config = ProviderConfig {
        provider_config: HashMap::with_capacity(16),
        ecosystem_config: HashMap::with_capacity(16),
        network_config: NetworkConfig::default(),
        monitoring_config: MonitoringConfig::default(),
    };

    manager.register_provider(provider, provider_config).await?;

    let cross_ecosystem_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_ENCRYPT.to_string(),
        payload: json!({
            "data": "cross-ecosystem sensitive data",
            "algorithm": "AES-256-GCM",
            "source_ecosystem": "songbird"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::High,
        metadata: {
            let mut metadata = HashMap::with_capacity(16);
            metadata.insert("cross_ecosystem".to_string(), "true".to_string());
            metadata.insert(
                "requesting_service".to_string(),
                "songbird-discovery".to_string(),
            );
            metadata
        },
        context: RequestContext {
            user_id: Some("songbird-user".to_string()),
            session_id: Some("songbird-session".to_string()),
            transaction_id: Some("cross-tx-001".to_string()),
            source_ecosystem: ecosystem_ids::SONGBIRD.to_string(),
            target_ecosystem: Some(ecosystem_ids::BEARDOG.to_string()),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("route_type".to_string(), "cross_ecosystem".to_string());
                metadata
            },
        },
    };

    let response = manager
        .route_request(cross_ecosystem_request.clone())
        .await?;

    assert_eq!(response.request_id, cross_ecosystem_request.request_id);
    assert!(response.success);
    assert!(!response.payload.is_null());
    assert!(response.error.is_none());

    if let Some(encrypted_data) = response.payload.get("encrypted_data") {
        assert!(!encrypted_data.is_null());
        assert!(!encrypted_data.as_str().unwrap_or("").is_empty());
    }

    println!("✅ Cross-ecosystem request routing test passed");
    Ok(())
}

#[tokio::test]
async fn test_primal_provider_registration_discovery() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-006".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id.clone());

    let registration = provider.register_with_ecosystem().await?;

    assert_eq!(registration.ecosystem_id, ecosystem_ids::BEARDOG);
    assert_eq!(registration.instance_id, instance_id);
    assert!(!registration.capabilities.is_empty());
    assert!(!registration.endpoints.primary.is_empty());

    match registration.status {
        RegistrationStatus::Active => {
            println!("✅ BearDog provider registered as active");
        }
        RegistrationStatus::Pending => {
            println!("⏳ BearDog provider registration is pending");
        }
        other => {
            panic!("Unexpected registration status: {:?}", other);
        }
    }

    for capability in &registration.capabilities {
        assert!(!capability.id.is_empty());
        assert!(!capability.name.is_empty());
        assert!(!capability.description.is_empty());
        assert_eq!(capability.category, CapabilityCategory::Security);
        assert!(!capability.attributes.is_empty());
    }

    println!("✅ PrimalProvider registration and discovery test passed");
    Ok(())
}

#[tokio::test]
async fn test_primal_provider_lifecycle() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-007".to_string();
    let mut provider = BearDogPrimalProvider::new(core.clone(), instance_id.clone());

    let provider_config = ProviderConfig {
        provider_config: {
            let mut config = HashMap::with_capacity(16);
            config.insert("test_mode".to_string(), json!(true));
            config.insert("security_level".to_string(), json!("maximum"));
            config
        },
        ecosystem_config: {
            let mut config = HashMap::with_capacity(16);
            config.insert("ecosystem_version".to_string(), json!("1.0.0"));
            config
        },
        network_config: NetworkConfig {
            listen_address: "127.0.0.1".to_string(),
            port: 8443,
            tls_enabled: true,
            timeout_seconds: 30,
            connection_pool: ConnectionPoolConfig {
                max_connections: 100,
                connection_timeout_seconds: 10,
                idle_timeout_seconds: 60,
            },
        },
        monitoring_config: MonitoringConfig {
            metrics_enabled: true,
            log_level: "info".to_string(),
            health_check_interval_seconds: 30,
        },
    };

    provider.initialize(provider_config).await?;

    let health_status = provider.health_check().await;
    match health_status {
        HealthStatus::Healthy | HealthStatus::Starting => {
            println!("✅ Provider is healthy after initialization");
        }
        other => {
            println!("⚠️  Provider status after initialization: {:?}", other);
        }
    }

    let test_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::HEALTH_CHECK.to_string(),
        payload: json!({}),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Normal,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("test-user".to_string()),
            session_id: None,
            transaction_id: None,
            source_ecosystem: ecosystem_ids::BEARDOG.to_string(),
            target_ecosystem: None,
            metadata: HashMap::with_capacity(16),
        },
    };

    let response = provider.handle_request(test_request.clone()).await?;
    assert_eq!(response.request_id, test_request.request_id);
    assert!(response.success);

    provider.shutdown().await?;

    println!("✅ PrimalProvider lifecycle test passed");
    Ok(())
}

#[tokio::test]
async fn test_capability_attributes() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-008".to_string();
    let provider = BearDogPrimalProvider::new(core.clone(), instance_id);

    let capabilities = provider.capabilities();

    for capability in &capabilities {
        assert!(
            !capability.attributes.is_empty(),
            "Capability {} has no attributes",
            capability.id
        );

        for (attr_name, attr_value) in &capability.attributes {
            assert!(
                !attr_name.is_empty(),
                "Empty attribute name in capability {}",
                capability.id
            );
            assert!(
                !attr_value.value.is_empty(),
                "Empty attribute value for {} in capability {}",
                attr_name,
                capability.id
            );

            match attr_value.data_type {
                AttributeDataType::String => {

                    assert!(!attr_value.value.is_empty());
                }
                AttributeDataType::Boolean => {

                    assert!(attr_value.value == "true" || attr_value.value == "false");
                }
                AttributeDataType::Integer => {

                    assert!(attr_value.value.parse::<i64>().is_ok());
                }
                AttributeDataType::Float => {

                    assert!(attr_value.value.parse::<f64>().is_ok());
                }
                AttributeDataType::Array => {

                    assert!(
                        serde_json::from_str::<Vec<serde_json::Value>>(&attr_value.value).is_ok()
                    );
                }
                AttributeDataType::Object => {

                    assert!(serde_json::from_str::<serde_json::Value>(&attr_value.value).is_ok());
                }
                AttributeDataType::Duration => {

                    assert!(attr_value.value.parse::<u64>().is_ok());
                }
                AttributeDataType::Bytes => {

                    assert!(attr_value.value.parse::<u64>().is_ok());
                }
            }
        }

        assert!(capability.qos.avg_response_time_ms > 0);
        assert!(capability.qos.availability_percent > 0.0);
        assert!(capability.qos.availability_percent <= 100.0);
    }

    println!("✅ Capability attributes validation test passed");
    Ok(())
}

mod test_helpers {
    use super::*;

    pub struct MockToadStoolProvider {
        instance_id: String,
    }

    impl MockToadStoolProvider {
        pub fn new(instance_id: &str) -> Self {
            Self { instance_id }
        }
    }

    #[allow(async_fn_in_trait)]
    impl PrimalProvider for MockToadStoolProvider {
        fn ecosystem_id(&self) -> &str {
            ecosystem_ids::TOADSTOOL
        }

        fn instance_id(&self) -> &str {
            &self.instance_id
        }

        fn service_name(&self) -> &str {
            "Mock ToadStool Compute Provider"
        }

        fn service_version(&self) -> &str {
            "1.0.0"
        }

        fn capabilities(&self) -> Vec<Capability> {
            vec![Capability {
                id: capability_ids::COMPUTE_EXECUTE.to_string(),
                name: "Code Execution".to_string(),
                description: "Execute code on various compute platforms".to_string(),
                category: CapabilityCategory::Compute,
                attributes: HashMap::with_capacity(16),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }]
        }

        fn dependencies(&self) -> Vec<Dependency> {
            vec![Dependency {
                id: "beardog-security".to_string(),
                required_capability: capability_ids::SECURITY_AUTHENTICATE.to_string(),
                min_version: Some("1.0.0".to_string()),
                optional: false,
                attributes: HashMap::with_capacity(16),
            }]
        }

        fn endpoints(&self) -> ServiceEndpoints {
            ServiceEndpoints::default()
        }

        async fn health_check(&self) -> HealthStatus {
            HealthStatus::Healthy
        }

        async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, BearDogError> {
            Ok(ServiceResponse {
                request_id: request.request_id,
                success: true,
                payload: json!({"result": "mock computation completed"}),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::with_capacity(16),
                error: None,
            })
        }

        async fn register_with_ecosystem(&self) -> Result<EcosystemRegistration, BearDogError> {
            Ok(EcosystemRegistration {
                registration_id: Uuid::new_v4(),
                ecosystem_id: self.ecosystem_id().to_string(),
                instance_id: self.instance_id().to_string(),
                endpoints: self.endpoints(),
                capabilities: self.capabilities(),
                registration_time: chrono::Utc::now(),
                status: RegistrationStatus::Active,
            })
        }

        async fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn shutdown(&mut self) -> Result<(), BearDogError> {
            Ok(())
        }

        fn can_handle_request(&self, request: &ServiceRequest) -> bool {
            request.request_type.starts_with("compute.")
        }

        fn metadata(&self) -> ProviderMetadata {
            ProviderMetadata {
                name: "Mock ToadStool Provider".to_string(),
                version: "1.0.0".to_string(),
                description: "Mock compute provider for testing".to_string(),
                author: "Test Suite".to_string(),
                website: None,
                license: "MIT".to_string(),
                tags: vec!["compute".to_string(), "mock".to_string()],
                custom: HashMap::with_capacity(16),
            }
        }
    }
}

#[tokio::test]
async fn test_multi_provider_ecosystem() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let manager = UniversalEcosystemManager::new(core.clone()).await?;

    let beardog_provider = BearDogPrimalProvider::new(core.clone(), "beardog-001".to_string());
    manager
        .register_provider(
            beardog_provider,
            ProviderConfig {
                provider_config: HashMap::with_capacity(16),
                ecosystem_config: HashMap::with_capacity(16),
                network_config: NetworkConfig::default(),
                monitoring_config: MonitoringConfig::default(),
            },
        )
        .await?;

    let toadstool_provider = test_helpers::MockToadStoolProvider::new("toadstool-001".to_string());
    manager
        .register_provider(
            toadstool_provider,
            ProviderConfig {
                provider_config: HashMap::with_capacity(16),
                ecosystem_config: HashMap::with_capacity(16),
                network_config: NetworkConfig::default(),
                monitoring_config: MonitoringConfig::default(),
            },
        )
        .await?;

    let status = manager.get_status().await;
    assert_eq!(status.total_providers, 2);
    assert!(status.healthy_providers >= 1);
    assert!(status.total_capabilities > 0);

    let providers = manager.get_providers().await;
    assert_eq!(providers.len(), 2);

    let beardog_provider = providers
        .iter()
        .find(|p| p.ecosystem_id == ecosystem_ids::BEARDOG);
    let toadstool_provider = providers
        .iter()
        .find(|p| p.ecosystem_id == ecosystem_ids::TOADSTOOL);

    assert!(beardog_provider.is_some());
    assert!(toadstool_provider.is_some());

    let security_capabilities = manager
        .get_capabilities_by_category(CapabilityCategory::Security)
        .await;
    let compute_capabilities = manager
        .get_capabilities_by_category(CapabilityCategory::Compute)
        .await;

    assert!(!security_capabilities.is_empty());
    assert!(!compute_capabilities.is_empty());

    let security_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_ENCRYPT.to_string(),
        payload: json!({"data": "test data"}),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Normal,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("test-user".to_string()),
            session_id: None,
            transaction_id: None,
            source_ecosystem: ecosystem_ids::TOADSTOOL.to_string(),
            target_ecosystem: Some(ecosystem_ids::BEARDOG.to_string()),
            metadata: HashMap::with_capacity(16),
        },
    };

    let security_response = manager.route_request(security_request).await?;
    assert!(security_response.success);

    let compute_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::COMPUTE_EXECUTE.to_string(),
        payload: json!({"code": "print('hello world')"}),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Normal,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("test-user".to_string()),
            session_id: None,
            transaction_id: None,
            source_ecosystem: ecosystem_ids::BEARDOG.to_string(),
            target_ecosystem: Some(ecosystem_ids::TOADSTOOL.to_string()),
            metadata: HashMap::with_capacity(16),
        },
    };

    let compute_response = manager.route_request(compute_request).await?;
    assert!(compute_response.success);

    println!("✅ Multi-provider ecosystem integration test passed");
    Ok(())
}

#[tokio::test]
async fn test_primal_provider_performance() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let instance_id = "test-beardog-perf".to_string();
    let provider = Arc::new(BearDogPrimalProvider::new(core.clone(), instance_id));

    let mut handles = Vec::new();
    let num_requests = 10;

    for i in 0..num_requests {
        let provider_clone = provider.clone();
        let handle = tokio::spawn(async move {
            let request = ServiceRequest {
                request_id: Uuid::new_v4(),
                request_type: request_types::SECURITY_ENCRYPT.to_string(),
                payload: json!({
                    "data": format_args!("concurrent test data {}", i).to_string(),
                    "algorithm": "AES-256-GCM"
                }),
                timestamp: chrono::Utc::now(),
                priority: RequestPriority::Normal,
                metadata: HashMap::with_capacity(16),
                context: RequestContext {
                    user_id: Some(format_args!("user-{}", i).to_string()),
                    session_id: Some(format_args!("session-{}", i).to_string()),
                    transaction_id: Some(format_args!("tx-{}", i).to_string()),
                    source_ecosystem: ecosystem_ids::BEARDOG.to_string(),
                    target_ecosystem: None,
                    metadata: HashMap::with_capacity(16),
                },
            };

            let start = std::time::Instant::now();
            let response = provider_clone.handle_request(request).await?;
            let duration = start.elapsed();

            Ok::<(ServiceResponse, std::time::Duration), BearDogError>((response, duration))
        });

        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    let mut total_duration = std::time::Duration::from_millis(0);
    for result in results {
        let (response, duration) = result??;
        assert!(response.success);
        assert!(response.error.is_none());
        total_duration += duration;
    }

    let avg_duration = total_duration / num_requests as u32;
    println!(
        "✅ Concurrent requests completed. Average duration: {:?}",
        avg_duration
    );

    assert!(
        avg_duration < std::time::Duration::from_millis(100),
        "Average response time {} exceeded expected QoS",
        avg_duration.as_millis()
    );

    println!("✅ PrimalProvider performance test passed");
    Ok(())
}
