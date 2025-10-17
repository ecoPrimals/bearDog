// Comprehensive Capability Registry Tests
//
// Extensive test coverage for capability discovery and management

#[cfg(test)]
mod comprehensive_tests {
    use super::super::*;
    use beardog_types::canonical::capabilities::{
        AuthConfig, CircuitBreakerConfig, EndpointConfig, HealthStatus as CapHealthStatus,
        PerformanceMetrics, ProviderInfo, SecurityLevel, UniversalCapability,
    };
    use beardog_types::canonical::providers_unified::core::ProviderType;
    use std::collections::HashMap;

    fn create_test_capability_with_metadata(
        cap_type: ServiceCapabilityType,
        metadata: HashMap<String, String>,
    ) -> UniversalCapability {
        UniversalCapability {
            capability_type: cap_type,
            provider: ProviderInfo {
                provider_id: "test-provider".to_string(),
                provider_name: "Test Provider".to_string(),
                provider_type: ProviderType::Custom("test-provider".to_string()),
                version: "1.0.0".to_string(),
                region: None,
            },
            endpoint: EndpointConfig {
                base_url: "http://test.local:8080".to_string(),
                api_version: Some("v1".to_string()),
                timeout_ms: 5000,
                max_retries: 3,
                circuit_breaker: CircuitBreakerConfig::default(),
            },
            auth_config: AuthConfig {
                auth_type: beardog_types::canonical::capabilities::AuthType::None,
                api_key: None,
                bearer_token: None,
                cert_path: None,
                custom_params: HashMap::new(),
            },
            health_status: CapHealthStatus::Unknown,
            performance: PerformanceMetrics::default(),
            security_level: SecurityLevel::Standard,
            metadata,
        }
    }

    #[tokio::test]
    async fn test_registry_concurrent_registrations() {
        let registry = CapabilityRegistry::new();
        
        let mut handles = vec![];
        for i in 0..10 {
            let reg = registry.clone();
            handles.push(tokio::spawn(async move {
                let mut metadata = HashMap::new();
                metadata.insert("index".to_string(), i.to_string());
                let cap = create_test_capability_with_metadata(
                    ServiceCapabilityType::Compute,
                    metadata,
                );
                reg.register(cap).await
            }));
        }
        
        let results: Vec<_> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        
        assert_eq!(results.len(), 10);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_registry_empty_discovery() {
        let registry = CapabilityRegistry::new();
        let capabilities = registry
            .discover_by_type(ServiceCapabilityType::Storage)
            .await
            .unwrap();
        
        assert_eq!(capabilities.len(), 0);
    }

    #[tokio::test]
    async fn test_registry_health_degradation() {
        let registry = CapabilityRegistry::new();
        let cap = create_test_capability_with_metadata(
            ServiceCapabilityType::Compute,
            HashMap::new(),
        );
        let id = registry.register(cap).await.unwrap();
        
        // Mark as degraded multiple times
        for _ in 0..2 {
            registry
                .update_health_status(&id, HealthStatus::Degraded)
                .await
                .unwrap();
        }
        
        let registered = registry.get(&id).await.unwrap().unwrap();
        assert_eq!(registered.consecutive_failures, 2);
    }

    #[tokio::test]
    async fn test_registry_cleanup_threshold() {
        let config = CapabilityRegistryConfig {
            max_consecutive_failures: 2,
            ..Default::default()
        };
        let registry = CapabilityRegistry::with_config(config);
        
        let cap = create_test_capability_with_metadata(
            ServiceCapabilityType::Compute,
            HashMap::new(),
        );
        let id = registry.register(cap).await.unwrap();
        
        // Exceed threshold
        for _ in 0..3 {
            registry
                .update_health_status(&id, HealthStatus::Unhealthy)
                .await
                .unwrap();
        }
        
        let removed = registry.cleanup_unhealthy().await.unwrap();
        assert_eq!(removed, 1);
    }

    #[tokio::test]
    async fn test_registry_statistics_with_mixed_health() {
        let registry = CapabilityRegistry::new();
        
        // Register healthy capability
        let healthy_cap = create_test_capability_with_metadata(
            ServiceCapabilityType::Compute,
            HashMap::new(),
        );
        let healthy_id = registry.register(healthy_cap).await.unwrap();
        
        // Register unhealthy capability
        let unhealthy_cap = create_test_capability_with_metadata(
            ServiceCapabilityType::Storage,
            HashMap::new(),
        );
        let unhealthy_id = registry.register(unhealthy_cap).await.unwrap();
        
        // Mark one as unhealthy
        for _ in 0..5 {
            registry
                .update_health_status(&unhealthy_id, HealthStatus::Unhealthy)
                .await
                .unwrap();
        }
        
        let stats = registry.statistics().await.unwrap();
        assert_eq!(stats.total_capabilities, 2);
        assert!(stats.by_health_status.contains_key(&HealthStatus::Healthy));
        assert!(stats.by_health_status.contains_key(&HealthStatus::Unhealthy));
    }

    #[tokio::test]
    async fn test_registry_remove_nonexistent() {
        let registry = CapabilityRegistry::new();
        let fake_id = CapabilityId::new();
        
        let result = registry.remove(&fake_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_update_health_nonexistent() {
        let registry = CapabilityRegistry::new();
        let fake_id = CapabilityId::new();
        
        let result = registry
            .update_health_status(&fake_id, HealthStatus::Healthy)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_list_all_empty() {
        let registry = CapabilityRegistry::new();
        let all = registry.list_all().await.unwrap();
        assert_eq!(all.len(), 0);
    }

    #[tokio::test]
    async fn test_registry_list_all_populated() {
        let registry = CapabilityRegistry::new();
        
        for i in 0..5 {
            let mut metadata = HashMap::new();
            metadata.insert("id".to_string(), i.to_string());
            let cap = create_test_capability_with_metadata(
                ServiceCapabilityType::Compute,
                metadata,
            );
            registry.register(cap).await.unwrap();
        }
        
        let all = registry.list_all().await.unwrap();
        assert_eq!(all.len(), 5);
    }

    #[tokio::test]
    async fn test_capability_id_uniqueness() {
        let id1 = CapabilityId::new();
        let id2 = CapabilityId::new();
        
        assert_ne!(id1.as_str(), id2.as_str());
    }
}

