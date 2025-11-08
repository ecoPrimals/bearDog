// Implementation methods for UniversalCapabilityAdapter
//
// This module contains the core implementation logic for the universal adapter,
// separated to keep the main file focused and under the 1000-line limit.

use super::capability_based_adapter::UniversalCapabilityAdapter;
use super::capability_helpers::*;
use super::types::*;
use crate::ecosystem::primal_types::{DiscoveredPrimal, UniversalEndpoint};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

impl UniversalCapabilityAdapter {
    /// Create new universal capability adapter
    /// Creates a new instance
    pub async fn new() -> BearDogResult<Self> {
        Self::with_config(AdapterConfig::default())
    }

    /// Create universal adapter with custom configuration
    /// Creates instance with config
    pub fn with_config(config: AdapterConfig) -> BearDogResult<Self> {
        info!("🔌 Initializing Universal Capability Adapter");
        info!("🎯 Mission: Replace ALL hardcoded integrations with dynamic discovery");
        info!("📋 Configuration:");
        info!(
            "   📊 Max providers per capability: {}",
            config.max_providers_per_capability
        );
        info!(
            "   ❤️  Health check interval: {}s",
            config.health_check_interval_secs
        );
        info!(
            "   ⏱️  Connection timeout: {}ms",
            config.connection_timeout_ms
        );
        info!("   🔄 Failover enabled: {}", config.enable_failover);
        info!("   ⚖️  Load balancing: {:?}", config.load_balancing);

        Ok(Self {
            capabilities: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            primals: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
            metrics: AdapterMetrics::default(),
            connections: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        })
    }

    /// Discover capabilities dynamically (replaces ALL hardcoded integrations)
    pub fn discover_capability(
        &self,
        request: CapabilityDiscoveryRequest,
    ) -> BearDogResult<CapabilityDiscoveryResult> {
        let start_time = std::time::Instant::now();

        info!("🔍 Discovering capability: {:?}", request.capability_type);
        debug!(
            "📋 Discovery request details: capability={:?}, requirements={:?}",
            request.capability_type, request.requirements
        );

        // Phase 1: Find all providers for this capability type
        let providers = self
            .find_capability_providers(&request.capability_type)
            ?;
        info!("📊 Found {} potential providers", providers.len());

        // Phase 2: Filter providers based on requirements
        let filtered_providers =
            ProviderUtils::filter_providers(&providers, &request.requirements)?;
        info!(
            "✅ {} providers meet requirements",
            filtered_providers.len()
        );

        // Phase 3: Rank providers based on preferences and performance
        let ranked_providers =
            ProviderUtils::rank_providers(&filtered_providers, &request.preferences)?;
        info!("📈 Providers ranked by suitability");

        // Phase 4: Validate top providers
        let validated_providers = ProviderUtils::validate_providers(&ranked_providers)?;
        info!(
            "✅ {} providers validated and ready",
            validated_providers.len()
        );

        let discovery_duration = start_time.elapsed().as_millis() as u64;

        let result = CapabilityDiscoveryResult {
            request_id: request.request_id,
            providers: validated_providers,
            selection_criteria: SelectionCriteria {
                performance_weight: 0.3,
                security_weight: 0.25,
                availability_weight: 0.25,
                cost_weight: 0.2,
            },
        };

        info!(
            "🎯 Discovery completed in {}ms - {} providers ready",
            discovery_duration,
            result.providers.len()
        );

        Ok(result)
    }

    /// Connect to a specific capability provider
    pub fn connect_to_capability(
        &self,
        provider: &RankedCapabilityProvider,
    ) -> BearDogResult<String> {
        info!("🔗 Connecting to provider: {}", provider.provider_id);

        // Create connection using helper utilities
        let connection = ConnectionUtils::create_connection(
            provider.provider_id.clone(),
            provider.capability_type.clone(),
            UniversalEndpoint {
                url: format!("http://provider-{}.local", provider.provider_id),
                protocol: "http".to_string(),
                metadata: HashMap::new(),
            },
        )
        ?;

        let connection_id = connection.connection_id.clone();

        // Store the connection
        {
            let mut connections = self.connections.write();
            connections.insert(connection_id.clone(), connection);
        }

        info!("✅ Connected to provider: {}", provider.provider_id);
        Ok(connection_id)
    }

    /// Get current adapter metrics
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> &AdapterMetrics {
        &self.metrics
    }

    /// Get health status of all connections
    /// Gets connection_health
    /// Gets connection_health
    pub fn get_connection_health(&self) -> BearDogResult<HashMap<String, String>> {
        let connections = self.connections.read();
        let mut health_status = HashMap::new();

        for (connection_id, connection) in connections.iter() {
            let status = match connection.health_status {
                beardog_types::canonical::capabilities::HealthStatus::Healthy => "healthy",
                beardog_types::canonical::capabilities::HealthStatus::Degraded => "degraded",
                beardog_types::canonical::capabilities::HealthStatus::Unhealthy => "unhealthy",
                _ => "unknown",
            };
            health_status.insert(connection_id.clone(), status.to_string());
        }

        Ok(health_status)
    }

    pub fn perform_health_checks(&mut self) -> BearDogResult<()> {
        let mut connections = self.connections.write();

        for connection in connections.values_mut() {
            ConnectionUtils::update_connection_health(connection)?;
        }

        info!(
            "💓 Health checks completed for {} connections",
            connections.len()
        );
        Ok(())
    }

    // Private helper methods
    fn find_capability_providers(
        &self,
        capability_type: &ServiceCapabilityType,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        debug!("🔍 Finding providers for capability: {:?}", capability_type);

        // Read from capability cache
        let capabilities = self.capabilities.read();
        if let Some(providers) = capabilities.get(capability_type) {
            return Ok(providers.clone());
        }

        // If not in cache, perform discovery
        let discovered_providers = self.perform_capability_discovery(capability_type)?;

        // Cache the results
        drop(capabilities);
        let mut capabilities = self.capabilities.write();
        capabilities.insert(capability_type.clone(), discovered_providers.clone());

        Ok(discovered_providers)
    }


    fn perform_capability_discovery(
        &self,
        capability_type: &ServiceCapabilityType,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        debug!("🕵️ Performing discovery for: {:?}", capability_type);

        // Real capability discovery implementation
        // Priority order: Environment variables -> Service mesh -> mDNS -> Defaults

        let mut discovered_providers = Vec::new();

        // 1. Check environment variables first (highest priority)
        if let Some(endpoint) = self.discover_from_environment(capability_type)? {
            discovered_providers.push(UniversalCapability {
                provider_id: format!("env-{}-1", capability_type.as_str()),
                capability_type: capability_type.clone(),
                endpoint_url: endpoint,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("discovery_method".to_string(), "environment".to_string());
                    meta.insert("priority".to_string(), "high".to_string());
                    meta
                },
                health_status: beardog_types::canonical::capabilities::HealthStatus::Healthy,
                performance_metrics:
                    beardog_types::canonical::capabilities::PerformanceMetrics::default(),
            });
        }

        // 2. Service mesh discovery (if available)
        if let Ok(mesh_providers) = self.discover_from_service_mesh(capability_type) {
            discovered_providers.extend(mesh_providers);
        }

        // 3. Fallback to local discovery patterns
        if discovered_providers.is_empty() {
            discovered_providers = self.discover_local_fallbacks(capability_type)?;
        }

        info!(
            "🎯 Discovered {} providers for {:?}",
            discovered_providers.len(),
            capability_type
        );
        Ok(discovered_providers)
    }


    fn discover_from_environment(
        &self,
        capability_type: &ServiceCapabilityType,
    ) -> BearDogResult<Option<String>> {
        let env_var = match capability_type {
            ServiceCapabilityType::ComputeIntelligence => "COMPUTE_SERVICE_ENDPOINT",
            ServiceCapabilityType::ServiceMesh => "MESH_SERVICE_ENDPOINT",
            ServiceCapabilityType::DataStorage => "STORAGE_SERVICE_ENDPOINT",
            ServiceCapabilityType::DistributedIntelligence => "AI_SERVICE_ENDPOINT",
            _ => return Ok(None),
        };

        Ok(std::env::var(env_var).ok())
    }


    fn discover_from_service_mesh(
        &self,
        capability_type: &ServiceCapabilityType,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        // Service mesh discovery would integrate with Consul, etcd, Kubernetes, etc.
        // For now, return empty to indicate no mesh discovery available
        let _ = capability_type;
        Ok(Vec::new())
    }


    fn discover_local_fallbacks(
        &self,
        capability_type: &ServiceCapabilityType,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        use beardog_types::constants::domains::network::config;
        
        // Local fallback discovery using default ports and localhost
        let default_provider = UniversalCapability {
            provider_id: format!("local-{}-fallback", capability_type.as_str()),
            capability_type: capability_type.clone(),
            endpoint_url: format!(
                "http://{}:{}",
                config::default_service_host(),
                self.get_default_port(capability_type)
            ),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("discovery_method".to_string(), "local_fallback".to_string());
                meta.insert("priority".to_string(), "low".to_string());
                meta
            },
            health_status: beardog_types::canonical::capabilities::HealthStatus::Unknown,
            performance_metrics:
                beardog_types::canonical::capabilities::PerformanceMetrics::default(),
        };

        Ok(vec![default_provider])
    }

    /// Gets default_port
    fn get_default_port(&self, capability_type: &ServiceCapabilityType) -> u16 {
        match capability_type {
            ServiceCapabilityType::ComputeIntelligence => 8081,
            ServiceCapabilityType::ServiceMesh => 8082,
            ServiceCapabilityType::DataStorage => 8083,
            ServiceCapabilityType::DistributedIntelligence => 8084,
            _ => 8080,
        }
    }
}

// Extension trait for ServiceCapabilityType to get string representation
trait ServiceCapabilityTypeExt {
    /// Returns as str
    fn as_str(&self) -> &str;
}

impl ServiceCapabilityTypeExt for ServiceCapabilityType {
    /// Returns as str
    fn as_str(&self) -> &str {
        match self {
            ServiceCapabilityType::ComputeIntelligence => "compute-intelligence",
            ServiceCapabilityType::KeyManagement => "key-management",
            ServiceCapabilityType::ServiceMesh => "service-mesh",
            ServiceCapabilityType::DataStorage => "data-storage",
            ServiceCapabilityType::SecretsManagement => "secrets-management",
            ServiceCapabilityType::ServiceDiscovery => "service-discovery",
            ServiceCapabilityType::ContainerOrchestration => "container-orchestration",
            ServiceCapabilityType::CloudProvider => "cloud-provider",
            ServiceCapabilityType::DistributedStorage => "distributed-storage",
            ServiceCapabilityType::DistributedIntelligence => "distributed-intelligence",
        }
    }
}
