// Universal Infant Discovery System
//
// This system starts with absolutely zero knowledge - no vendor names, no primal names,
// no hardcoded endpoints. Like a newborn infant, it discovers the world through
// universal patterns and learns what capabilities are available through interaction.

use crate::BearDogError;
use beardog_types::canonical::discovery::{
    AuthenticationMethod, ComputeAbility, NetworkFunction, OrchestrationFeature,
    PerformanceProfile, PerformanceRequirements, SecurityRequirements, SecurityService,
    ServiceEndpoint, StorageCharacteristic, UniversalCapabilityType, UniversalDiscoveryRequest,
    UniversalDiscoveryResponse, UniversalServiceDescriptor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Universal Infant Discovery System - starts with zero hardcoded knowledge
pub struct UniversalInfantDiscovery {
    /// Discovered services (learned through exploration)
    discovered_services: Arc<RwLock<HashMap<String, UniversalServiceDescriptor>>>,
    learning_strategies: Vec<Box<dyn LearningStrategy + Send + Sync>>,
    /// Discovery state and metrics
    discovery_state: Arc<RwLock<DiscoveryState>>,
    config: InfantDiscoveryConfig,
}

#[derive(Debug, Clone)]
pub struct InfantDiscoveryConfig {
    /// How long to spend on initial discovery (ms)
    pub initial_discovery_timeout_ms: u64,
    /// How often to rediscover services (ms)
    /// Number of rediscovery_interval_ms
    pub rediscovery_interval_ms: u64,
    /// Maximum number of concurrent discovery attempts
    /// Number of max_concurrent_discoveries
    pub max_concurrent_discoveries: usize,
    /// The trust threshold value
    pub trust_threshold: f64,
    pub default_performance_requirements: PerformanceRequirements,
    /// The default security requirements value
    pub default_security_requirements: SecurityRequirements,
}

impl Default for InfantDiscoveryConfig {
    fn default() -> Self {
        Self {
            initial_discovery_timeout_ms: 30_000, // 30 seconds to learn about the world
            rediscovery_interval_ms: 300_000,     // 5 minutes between rediscovery
            max_concurrent_discoveries: 5,
            trust_threshold: 0.7,
            default_performance_requirements: PerformanceRequirements::default(),
            default_security_requirements: SecurityRequirements::default(),
        }
    }
}

/// Current state of discovery
#[derive(Debug, Clone, Default)]
pub struct DiscoveryState {
    /// When discovery started
    pub discovery_start_time: Option<u64>,
    /// Total services discovered
    /// Number of total_services_discovered
    pub total_services_discovered: usize,
    /// Services currently available
    /// Number of available_services
    pub available_services: usize,
    /// Services that failed health checks
    /// Number of failed_services
    pub failed_services: usize,
    /// Last discovery timestamp
    pub last_discovery_timestamp: u64,
    /// Discovery statistics
    /// The discovery stats value
    pub discovery_stats: DiscoveryStatistics,
}

/// Statistics about discovery process
#[derive(Debug, Clone, Default)]
pub struct DiscoveryStatistics {
    /// Total discovery attempts
    /// Number of discovery_attempts
    pub discovery_attempts: u64,
    /// Successful discoveries
    /// Number of successful_discoveries
    pub successful_discoveries: u64,
    /// Failed discoveries
    /// Number of failed_discoveries
    pub failed_discoveries: u64,
    /// Average discovery time (ms)
    pub avg_discovery_time_ms: f64,
    /// Services by capability type
    /// Mapping of services by capability
    pub services_by_capability: HashMap<String, usize>,
}

#[async_trait::async_trait]
pub trait LearningStrategy: Send + Sync + std::fmt::Debug {
    /// Learn about available services
    fn discover_services(&self) -> Result<Vec<UniversalServiceDescriptor>, BearDogError>;

    /// Get strategy name
    fn strategy_name(&self) -> &'static str;

    /// Check if this strategy is available in current environment
    /// Checks if available
    fn is_available(&self) -> bool;

    /// Get priority of this strategy (higher = tried first)
    fn priority(&self) -> u32;
}

impl UniversalInfantDiscovery {
    /// Create new infant discovery system with zero initial knowledge
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("👶 Initializing Universal Infant Discovery - Starting with ZERO knowledge");

        let config = InfantDiscoveryConfig::default();
        let mut learning_strategies: Vec<Box<dyn LearningStrategy + Send + Sync>> = Vec::new();

        // Add learning strategies in priority order
        learning_strategies.push(Box::new(EnvironmentLearningStrategy::new()));
        learning_strategies.push(Box::new(NetworkLearningStrategy::new()));
        learning_strategies.push(Box::new(ProcessLearningStrategy::new()));
        learning_strategies.push(Box::new(FileSystemLearningStrategy::new()));

        info!(
            "👶 Configured {} learning strategies",
            learning_strategies.len()
        );

        Ok(Self {
            discovered_services: Arc::new(RwLock::new(HashMap::new())),
            learning_strategies,
            discovery_state: Arc::new(RwLock::new(DiscoveryState::default())),
            config,
        })
    }

    /// Start the infant discovery process - learn about the world
    /// Starts discovery
    /// Starts discovery
    pub fn start_discovery(&self) -> Result<(), BearDogError> {
        info!("👶 Starting infant discovery - learning about the world...");

        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update discovery state
        {
            let mut state = self.discovery_state.write();
            state.discovery_start_time = Some(start_time);
            state.last_discovery_timestamp = start_time;
        }

        // Try each learning strategy
        for strategy in &self.learning_strategies {
            if strategy.is_available() {
                info!("👶 Trying learning strategy: {}", strategy.strategy_name());

                match strategy.discover_services() {
                    Ok(services) => {
                        info!(
                            "👶 Learned about {} services from {}",
                            services.len(),
                            strategy.strategy_name()
                        );

                        // Add discovered services
                        let mut discovered = self.discovered_services.write();
                        for service in services {
                            discovered.insert(service.service_id.clone(), service);
                        }
                    }
                    Err(e) => {
                        debug!(
                            "👶 Learning strategy {} failed: {}",
                            strategy.strategy_name(),
                            e
                        );
                    }
                }
            } else {
                debug!(
                    "👶 Learning strategy {} not available in this environment",
                    strategy.strategy_name()
                );
            }
        }

        // Update statistics
        self.update_discovery_statistics();

        let discovered_count = self.discovered_services.read().len();
        info!(
            "👶 Initial discovery complete - learned about {} services",
            discovered_count
        );

        Ok(())
    }

    /// Discover services with specific capabilities
    pub fn discover_capabilities(
        &self,
        request: UniversalDiscoveryRequest,
    ) -> Result<UniversalDiscoveryResponse, BearDogError> {
        info!(
            "👶 Looking for capabilities: {:?}",
            request.required_capabilities
        );

        let discovered = self.discovered_services.read();
        let mut matching_services = Vec::new();
        let mut partial_matches = Vec::new();

        for service in discovered.values() {
            let match_score = self.calculate_capability_match(service, &request);

            if match_score >= 1.0 {
                // Perfect match - has all required capabilities
                matching_services.push(service.clone());
            } else if match_score >= 0.5 {
                // Partial match - has some required capabilities
                partial_matches.push(service.clone());
            }
        }

        info!(
            "👶 Found {} perfect matches, {} partial matches",
            matching_services.len(),
            partial_matches.len()
        );

        Ok(UniversalDiscoveryResponse {
            matching_services,
            partial_matches,
            metadata: beardog_types::canonical::discovery::DiscoveryMetadata {
                discovery_timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                discovery_duration_ms: 0, // Will be calculated by caller
                services_discovered: discovered.len(),
                strategy_used: "infant_discovery".to_string(),
                warnings: Vec::new(),
            },
        })
    }

    /// Get all discovered services
    /// Gets discovered_services
    /// Gets discovered_services
    pub fn get_discovered_services(&self) -> HashMap<String, UniversalServiceDescriptor> {
        self.discovered_services.read().clone()
    }

    /// Get discovery statistics
    /// Gets discovery_state
    /// Gets discovery_state
    pub fn get_discovery_state(&self) -> DiscoveryState {
        self.discovery_state.read().clone()
    }

    /// Calculate how well a service matches capability requirements
    fn calculate_capability_match(
        &self,
        service: &UniversalServiceDescriptor,
        request: &UniversalDiscoveryRequest,
    ) -> f64 {
        let mut required_matches = 0;
        let total_required = request.required_capabilities.len();

        if total_required == 0 {
            return 1.0; // No requirements means perfect match
        }

        for required_cap in &request.required_capabilities {
            if service.capabilities.contains(required_cap) {
                required_matches += 1;
            }
        }

        required_matches as f64 / total_required as f64
    }

    /// Update discovery statistics
    /// Updates discovery_statistics
    fn update_discovery_statistics(&self) {
        let mut state = self.discovery_state.write();
        let discovered = self.discovered_services.read();

        state.total_services_discovered = discovered.len();
        state.available_services = discovered
            .values()
            .filter(|s| s.trust_score >= self.config.trust_threshold)
            .count();
        state.failed_services = discovered.len() - state.available_services;

        // Count services by capability
        state.discovery_stats.services_by_capability.clear();
        for service in discovered.values() {
            for capability in &service.capabilities {
                let cap_name = format!("{:?}", capability);
                *state
                    .discovery_stats
                    .services_by_capability
                    .entry(cap_name)
                    .or_insert(0) += 1;
            }
        }
    }
}

/// Learning strategy that discovers services through environment variables
#[derive(Debug)]
pub struct EnvironmentLearningStrategy;

impl EnvironmentLearningStrategy {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LearningStrategy for EnvironmentLearningStrategy {
    fn discover_services(&self) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("👶 Learning from environment variables...");

        let mut services = Vec::new();

        // Collect environment variables first to avoid Send issues
        let env_vars: Vec<(String, String)> = std::env::vars()
            .filter(|(key, _)| {
                key.ends_with("_ENDPOINT") || key.ends_with("_URL") || key.ends_with("_SERVICE")
            })
            .collect();

        // Look for service endpoints in environment
        for (key, value) in env_vars {
            if let Ok(service) = self.parse_service_from_env(&key, &value) {
                services.push(service);
            }
        }

        debug!(
            "👶 Learned about {} services from environment",
            services.len()
        );
        Ok(services)
    }


    fn strategy_name(&self) -> &'static str {
        "environment_learning"
    }

    /// Checks if available
    fn is_available(&self) -> bool {
        true // Environment variables are always available
    }


    fn priority(&self) -> u32 {
        100 // High priority - environment is explicit configuration
    }
}

impl EnvironmentLearningStrategy {
    /// Parses service_from_env
    fn parse_service_from_env(
        &self,
        key: &str,
        value: &str,
    ) -> Result<UniversalServiceDescriptor, BearDogError> {
        // Try to infer service capabilities from environment variable names
        let capabilities = self.infer_capabilities_from_name(key);

        // Parse endpoint
        let endpoint = if value.starts_with("http") {
            // Simple URL parsing without external dependency
            let parts: Vec<&str> = value.split("://").collect();
            if parts.len() == 2 {
                let protocol = parts[0];
                let host_port: Vec<&str> = parts[1].split(':').collect();
                let host = host_port[0];
                let port = if host_port.len() > 1 {
                    host_port[1].parse().unwrap_or(80)
                } else {
                    if protocol == "https" {
                        443
                    } else {
                        80
                    }
                };
                ServiceEndpoint {
                    protocol: protocol.to_string(),
                    host: host.to_string(),
                    port,
                    path: Some("/".to_string()),
                    parameters: HashMap::new(),
                }
            } else {
                ServiceEndpoint {
                    protocol: "unknown".to_string(),
                    host: value.to_string(),
                    port: 80,
                    path: None,
                    parameters: HashMap::new(),
                }
            }
        } else if let Ok(url_parsed) = value.parse::<std::net::SocketAddr>() {
            ServiceEndpoint {
                protocol: "tcp".to_string(),
                host: url_parsed.ip().to_string(),
                port: url_parsed.port(),
                path: None,
                parameters: HashMap::new(),
            }
        } else {
            // Fallback for non-URL values
            ServiceEndpoint {
                protocol: "unknown".to_string(),
                host: value.to_string(),
                port: 80,
                path: None,
                parameters: HashMap::new(),
            }
        };

        Ok(UniversalServiceDescriptor {
            service_id: format!("env_{}", key.to_lowercase()),
            capabilities,
            endpoint,
            auth_method: AuthenticationMethod::None, // Will be discovered later
            performance_profile: PerformanceProfile::default(),
            trust_score: 0.8, // Environment variables have high initial trust
        })
    }


    fn infer_capabilities_from_name(&self, name: &str) -> Vec<UniversalCapabilityType> {
        let mut capabilities = Vec::new();
        let name_lower = name.to_lowercase();

        // Infer compute capabilities
        if name_lower.contains("ai") || name_lower.contains("ml") || name_lower.contains("compute")
        {
            capabilities.push(UniversalCapabilityType::Compute {
                abilities: vec![
                    ComputeAbility::MachineLearning,
                    ComputeAbility::DataAnalysis,
                ],
            });
        }

        // Infer storage capabilities
        if name_lower.contains("storage")
            || name_lower.contains("database")
            || name_lower.contains("cache")
        {
            capabilities.push(UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::Persistent],
            });
        }

        // Infer network capabilities
        if name_lower.contains("mesh")
            || name_lower.contains("proxy")
            || name_lower.contains("gateway")
        {
            capabilities.push(UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::ServiceMesh, NetworkFunction::LoadBalancing],
            });
        }

        // Infer security capabilities
        if name_lower.contains("auth")
            || name_lower.contains("security")
            || name_lower.contains("vault")
        {
            capabilities.push(UniversalCapabilityType::Security {
                services: vec![
                    SecurityService::Authentication,
                    SecurityService::KeyManagement,
                ],
            });
        }

        // Infer orchestration capabilities
        if name_lower.contains("orchestr")
            || name_lower.contains("deploy")
            || name_lower.contains("container")
        {
            capabilities.push(UniversalCapabilityType::Orchestration {
                features: vec![
                    OrchestrationFeature::ServiceDeployment,
                    OrchestrationFeature::ContainerManagement,
                ],
            });
        }

        capabilities
    }
}

/// Learning strategy that discovers services through network scanning
#[derive(Debug)]
pub struct NetworkLearningStrategy;

impl NetworkLearningStrategy {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LearningStrategy for NetworkLearningStrategy {
    fn discover_services(&self) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("👶 Learning from network discovery...");

        // This would implement actual network discovery
        // For now, return empty to avoid network scanning in all environments
        Ok(Vec::new())
    }


    fn strategy_name(&self) -> &'static str {
        "network_learning"
    }

    /// Checks if available
    fn is_available(&self) -> bool {
        // Only available if explicitly enabled
        std::env::var("BEARDOG_ENABLE_NETWORK_DISCOVERY").is_ok()
    }


    fn priority(&self) -> u32 {
        50 // Medium priority - network discovery is less reliable
    }
}

/// Learning strategy that discovers services through process inspection
#[derive(Debug)]
pub struct ProcessLearningStrategy;

impl ProcessLearningStrategy {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LearningStrategy for ProcessLearningStrategy {
    fn discover_services(&self) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("👶 Learning from running processes...");

        // This would inspect running processes to find services
        // For now, return empty to avoid process scanning
        Ok(Vec::new())
    }


    fn strategy_name(&self) -> &'static str {
        "process_learning"
    }

    /// Checks if available
    fn is_available(&self) -> bool {
        // Only available on Unix-like systems
        cfg!(unix)
    }


    fn priority(&self) -> u32 {
        30 // Lower priority - process inspection is intrusive
    }
}

/// Learning strategy that discovers services through filesystem inspection
#[derive(Debug)]
pub struct FileSystemLearningStrategy;

impl FileSystemLearningStrategy {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LearningStrategy for FileSystemLearningStrategy {
    fn discover_services(&self) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("👶 Learning from filesystem...");

        // This would look for service configuration files, sockets, etc.
        // For now, return empty to avoid filesystem scanning
        Ok(Vec::new())
    }


    fn strategy_name(&self) -> &'static str {
        "filesystem_learning"
    }

    /// Checks if available
    fn is_available(&self) -> bool {
        true // Filesystem is always available
    }


    fn priority(&self) -> u32 {
        20 // Lowest priority - filesystem scanning is slow
    }
}
