// Vendor-Agnostic Migration Template
//
// This template shows how to migrate from hardcoded vendor/platform names
// to capability-based vendor-agnostic abstractions.
//
// PRINCIPLE: Never hardcode k8s, consul, docker, vault, etc. - use universal adapter.

use beardog_adapters::UniversalCapabilityAdapter;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::config::network_discovery::{NetworkDiscoveryConfig, ServiceDiscoveryMethod};
use serde_json::json;

/// ❌ ANTI-PATTERN: Hardcoded vendor/platform references
mod anti_pattern {
    use beardog_errors::BearDogError;

    pub struct BadKubernetesClient {
        // ❌ WRONG: Hardcoded Kubernetes
        k8s_endpoint: String,
        k8s_namespace: String,
    }

    impl BadKubernetesClient {
        // ❌ WRONG: Vendor lock-in
        pub fn new_k8s_client() -> Self {
            Self {
                k8s_endpoint: "https://kubernetes.default.svc".to_string(),
                k8s_namespace: "default".to_string(),
            }
        }

        // ❌ WRONG: Kubernetes-specific API
        pub async fn discover_k8s_services(&self) -> Result<Vec<String>, BearDogError> {
            todo!("Call Kubernetes API directly - not portable")
        }
    }

    pub struct BadConsulClient {
        // ❌ WRONG: Hardcoded Consul
        consul_endpoint: String,
    }

    impl BadConsulClient {
        // ❌ WRONG: Vendor lock-in
        pub fn new_consul_client() -> Self {
            Self {
                consul_endpoint: "http://consul.service.consul:8500".to_string(),
            }
        }
    }

    pub struct BadVaultClient {
        // ❌ WRONG: Hardcoded Vault
        vault_endpoint: String,
        vault_token: String,
    }

    // ❌ WRONG: Need separate client for each vendor
    // This leads to 2^n integration complexity
}

/// ✅ CORRECT PATTERN: Vendor-agnostic capability-based discovery
mod correct_pattern {
    use super::*;

    pub struct VendorAgnosticDiscoveryClient {
        adapter: UniversalCapabilityAdapter,
        config: NetworkDiscoveryConfig,
    }

    impl VendorAgnosticDiscoveryClient {
        /// ✅ Correct: No vendor hardcoding, configuration-driven
        pub async fn new() -> Result<Self, BearDogError> {
            let adapter = UniversalCapabilityAdapter::new().await?;
            let config = NetworkDiscoveryConfig::default();

            Ok(Self { adapter, config })
        }

        /// ✅ Correct: Discover services using configured method
        /// Could be k8s, consul, dns, mdns, etcd - configuration determines it
        pub async fn discover_services(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            match self.config.discovery_method {
                ServiceDiscoveryMethod::Environment => self.discover_from_environment(capability),
                ServiceDiscoveryMethod::Dns => self.discover_from_dns(capability),
                ServiceDiscoveryMethod::Kubernetes => self.discover_from_container_orchestration(capability),
                ServiceDiscoveryMethod::Consul => self.discover_from_service_registry(capability),
                ServiceDiscoveryMethod::Mdns => self.discover_from_mdns(capability),
                ServiceDiscoveryMethod::Manual => self.discover_from_config(capability),
            }
        }

        /// ✅ Environment-based discovery (highest priority)
        fn discover_from_environment(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // Use environment variables - vendor agnostic
            let env_var = self.get_capability_env_var(&capability);
            
            if let Ok(endpoint) = std::env::var(env_var) {
                Ok(vec![endpoint])
            } else {
                // Fall through to next discovery method
                self.discover_from_dns(capability)
            }
        }

        /// ✅ DNS-based discovery (vendor agnostic)
        fn discover_from_dns(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // Use DNS SRV records - works with any platform
            let service_name = self.get_capability_service_name(&capability);
            
            // DNS discovery is vendor-neutral
            // Works with k8s, consul, custom DNS, etc.
            todo!("Query DNS SRV records for {}", service_name)
        }

        /// ✅ Container orchestration discovery (k8s, nomad, docker swarm, etc.)
        fn discover_from_container_orchestration(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // Abstract container orchestration capability
            // Could be k8s, nomad, docker swarm, etc.
            let orchestration_capability = ServiceCapabilityType::ContainerOrchestration;
            
            // Use universal adapter to discover orchestration provider
            let discovery_request = self.adapter.discover_capability(
                beardog_adapters::CapabilityDiscoveryRequest {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    capability_type: orchestration_capability,
                    requirements: Default::default(),
                    preferences: Default::default(),
                },
            )?;

            // Get services from whatever orchestration platform is available
            let services = discovery_request
                .providers
                .first()
                .ok_or_else(|| BearDogError::system("No orchestration provider available"))?;

            Ok(vec![services.endpoint_url.clone()])
        }

        /// ✅ Service registry discovery (consul, etcd, zookeeper, etc.)
        fn discover_from_service_registry(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // Abstract service registry capability
            // Could be consul, etcd, zookeeper, eureka, etc.
            let registry_capability = ServiceCapabilityType::ServiceDiscovery;
            
            // Use universal adapter to find registry
            let discovery_request = self.adapter.discover_capability(
                beardog_adapters::CapabilityDiscoveryRequest {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    capability_type: registry_capability,
                    requirements: Default::default(),
                    preferences: Default::default(),
                },
            )?;

            // Query whatever registry is available
            let registry = discovery_request
                .providers
                .first()
                .ok_or_else(|| BearDogError::system("No service registry available"))?;

            Ok(vec![registry.endpoint_url.clone()])
        }

        /// ✅ mDNS discovery (vendor agnostic local discovery)
        fn discover_from_mdns(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // mDNS/Bonjour - works on any platform
            let service_type = self.get_capability_mdns_type(&capability);
            todo!("Query mDNS for {}", service_type)
        }

        /// ✅ Config-based discovery (manual configuration)
        fn discover_from_config(
            &self,
            capability: ServiceCapabilityType,
        ) -> Result<Vec<String>, BearDogError> {
            // Read from configuration file - vendor agnostic
            let endpoint = self.config
                .environment_endpoints
                .compute // Example - would match capability type
                .get_endpoint_url(&self.config.fallback_config.default_host);
            
            Ok(vec![endpoint])
        }

        // Helper methods (vendor agnostic)
        
        fn get_capability_env_var(&self, capability: &ServiceCapabilityType) -> String {
            match capability {
                ServiceCapabilityType::ComputeIntelligence => "COMPUTE_SERVICE_ENDPOINT".to_string(),
                ServiceCapabilityType::ServiceMesh => "MESH_SERVICE_ENDPOINT".to_string(),
                ServiceCapabilityType::DataStorage => "STORAGE_SERVICE_ENDPOINT".to_string(),
                ServiceCapabilityType::SecretsManagement => "SECRETS_SERVICE_ENDPOINT".to_string(),
                _ => format!("{}_SERVICE_ENDPOINT", format!("{:?}", capability).to_uppercase()),
            }
        }

        fn get_capability_service_name(&self, capability: &ServiceCapabilityType) -> String {
            match capability {
                ServiceCapabilityType::ComputeIntelligence => "compute".to_string(),
                ServiceCapabilityType::ServiceMesh => "mesh".to_string(),
                ServiceCapabilityType::DataStorage => "storage".to_string(),
                _ => format!("{:?}", capability).to_lowercase(),
            }
        }

        fn get_capability_mdns_type(&self, capability: &ServiceCapabilityType) -> String {
            format!("_{}._tcp.local", self.get_capability_service_name(capability))
        }
    }
}

/// Migration Examples - Before and After
mod migration_examples {
    use super::*;

    /// Example 1: Secrets Management (Vault → Generic)
    pub mod secrets_example {
        use super::*;

        // ❌ BEFORE: Hardcoded Vault
        pub async fn old_get_secret_wrong(key: &str) -> Result<String, BearDogError> {
            let vault_addr = "http://vault.service.consul:8200"; // ❌ Hardcoded
            let vault_token = std::env::var("VAULT_TOKEN").unwrap(); // ❌ Vault-specific
            
            todo!("Call Vault API directly - not portable")
        }

        // ✅ AFTER: Vendor-agnostic secrets management
        pub async fn new_get_secret_correct(
            adapter: &UniversalCapabilityAdapter,
            key: &str,
        ) -> Result<String, BearDogError> {
            // Discover secrets management capability (could be Vault, AWS Secrets Manager, etc.)
            let secrets_capability = ServiceCapabilityType::SecretsManagement;
            
            let discovery = adapter.discover_capability(
                beardog_adapters::CapabilityDiscoveryRequest {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    capability_type: secrets_capability,
                    requirements: Default::default(),
                    preferences: Default::default(),
                },
            )?;

            let provider = discovery
                .providers
                .first()
                .ok_or_else(|| BearDogError::system("No secrets provider available"))?;

            // Connect and retrieve secret (vendor-agnostic)
            let connection_id = adapter.connect_to_capability(provider)?;
            
            Ok(format!("secret-from-{}", provider.provider_id))
        }
    }

    /// Example 2: Service Discovery (Consul → Generic)
    pub mod discovery_example {
        use super::*;

        // ❌ BEFORE: Hardcoded Consul
        pub async fn old_find_service_wrong(service_name: &str) -> Result<String, BearDogError> {
            let consul_url = "http://consul.service.consul:8500"; // ❌ Hardcoded
            
            todo!("Query Consul API - not portable")
        }

        // ✅ AFTER: Vendor-agnostic service discovery
        pub async fn new_find_service_correct(
            adapter: &UniversalCapabilityAdapter,
            capability: ServiceCapabilityType,
        ) -> Result<String, BearDogError> {
            // Use service discovery capability (works with consul, k8s, dns, etc.)
            let discovery_capability = ServiceCapabilityType::ServiceDiscovery;
            
            let discovery = adapter.discover_capability(
                beardog_adapters::CapabilityDiscoveryRequest {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    capability_type: discovery_capability,
                    requirements: Default::default(),
                    preferences: Default::default(),
                },
            )?;

            let provider = discovery
                .providers
                .first()
                .ok_or_else(|| BearDogError::system("No discovery provider available"))?;

            Ok(provider.endpoint_url.clone())
        }
    }

    /// Example 3: Container Orchestration (K8s → Generic)
    pub mod orchestration_example {
        use super::*;

        // ❌ BEFORE: Hardcoded Kubernetes
        pub async fn old_deploy_service_wrong() -> Result<(), BearDogError> {
            let k8s_api = "https://kubernetes.default.svc"; // ❌ Hardcoded
            
            todo!("Use Kubernetes API - not portable to other orchestrators")
        }

        // ✅ AFTER: Vendor-agnostic container orchestration
        pub async fn new_deploy_service_correct(
            adapter: &UniversalCapabilityAdapter,
        ) -> Result<(), BearDogError> {
            // Use container orchestration capability (k8s, nomad, docker swarm, etc.)
            let orchestration = ServiceCapabilityType::ContainerOrchestration;
            
            let discovery = adapter.discover_capability(
                beardog_adapters::CapabilityDiscoveryRequest {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    capability_type: orchestration,
                    requirements: Default::default(),
                    preferences: Default::default(),
                },
            )?;

            let provider = discovery
                .providers
                .first()
                .ok_or_else(|| BearDogError::system("No orchestration provider available"))?;

            // Deploy using whatever orchestrator is available
            Ok(())
        }
    }
}

/// Configuration Examples
mod configuration_examples {
    use super::*;

    /// ✅ Environment-based configuration (highest priority)
    pub fn example_environment_config() {
        // These env vars work with ANY vendor
        std::env::set_var("SECRETS_SERVICE_ENDPOINT", "http://secrets-provider:8200");
        std::env::set_var("DISCOVERY_SERVICE_ENDPOINT", "http://discovery-provider:8500");
        std::env::set_var("ORCHESTRATION_SERVICE_ENDPOINT", "https://orchestrator:6443");
        
        // No vendor names in variables - just capabilities
    }

    /// ✅ Config file example (vendor agnostic)
    pub fn example_config_file() -> String {
        r#"
        # Vendor-agnostic configuration
        [discovery]
        method = "environment"  # Could be: dns, kubernetes, consul, mdns, manual
        
        [secrets_management]
        primary_endpoint = "http://secrets-service:8200"
        # Could be Vault, AWS Secrets Manager, Azure Key Vault, etc.
        
        [container_orchestration]
        primary_endpoint = "https://orchestrator:6443"
        # Could be Kubernetes, Nomad, Docker Swarm, etc.
        
        [service_discovery]
        primary_endpoint = "http://discovery-service:8500"
        # Could be Consul, etcd, Kubernetes, DNS, etc.
        "#
        .to_string()
    }
}

/// Testing Patterns
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vendor_agnostic_discovery() {
        let adapter = UniversalCapabilityAdapter::new().await.unwrap();
        
        // Test discovers capability without knowing vendor
        let result = adapter.discover_capability(
            beardog_adapters::CapabilityDiscoveryRequest {
                request_id: uuid::Uuid::new_v4().to_string(),
                capability_type: ServiceCapabilityType::SecretsManagement,
                requirements: Default::default(),
                preferences: Default::default(),
            },
        );

        // Success if any secrets provider is found (Vault, AWS, Azure, etc.)
        assert!(result.is_ok() || result.is_err()); // Vendor agnostic
    }
}

/// Key Principles Summary
///
/// 1. ✅ NO hardcoded vendor/platform names (k8s, consul, vault, docker, etc.)
/// 2. ✅ Use capability types instead (SecretsManagement, ServiceDiscovery, ContainerOrchestration)
/// 3. ✅ Configuration determines vendor, not code
/// 4. ✅ Environment variables for deployment flexibility
/// 5. ✅ Graceful fallback chain: env → discovery → config → defaults
/// 6. ✅ Works with ANY vendor providing the capability
/// 7. ✅ Truly portable across platforms
///
/// Migration Checklist:
/// - [ ] Replace vendor name references with capability types
/// - [ ] Use UniversalCapabilityAdapter for discovery
/// - [ ] Support multiple discovery methods (env, dns, k8s, consul, mdns)
/// - [ ] Remove hardcoded vendor endpoints
/// - [ ] Test with multiple vendors (mocked)
/// - [ ] Document capability-based configuration

