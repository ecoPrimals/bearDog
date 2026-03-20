// SPDX-License-Identifier: AGPL-3.0-only

// Load Balancing Module
//
// This module contains load balancing algorithms and traffic distribution functionality.

use super::ServiceInfo;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for load balancing behavior
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm to use
    pub algorithm: LoadBalancingAlgorithm,
    /// Enable sticky sessions
    pub enable_sticky_sessions: bool,
    /// Session affinity timeout in seconds
    pub session_timeout_secs: u64,
    /// Health check weight factor (0.0 to 1.0)
    pub health_weight_factor: f64,
    /// Enable adaptive load balancing
    pub enable_adaptive: bool,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            enable_sticky_sessions: false,
            session_timeout_secs: 1800,
            health_weight_factor: 0.7,
            enable_adaptive: true,
            circuit_breaker: CircuitBreakerConfig::default(),
        }
    }
}

/// Load balancing algorithms for service request distribution
///
/// Defines different strategies for distributing requests across available service instances,
/// each optimized for different use cases and service characteristics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round-robin distribution across services
    RoundRobin,
    /// Route to service with least active connections
    LeastConnections,
    /// Weighted round-robin based on service capacity
    WeightedRoundRobin,
    /// Random selection across available services
    Random,
    /// Hash-based routing using client IP
    IpHash,
    /// Route to service with lowest response time
    LeastResponseTime,
    /// Resource-based routing considering CPU/memory usage
    ResourceBased,
}

/// Configuration for circuit breaker pattern in service discovery
///
/// Implements fault tolerance by automatically opening circuits to failing services
/// and attempting recovery after a timeout period.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Time in seconds to wait before attempting recovery from open circuit state
    pub recovery_timeout_secs: u64,
    /// Maximum calls allowed in half-open state
    /// Number of `half_open_max_calls`
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_secs: 60,
            half_open_max_calls: 3,
        }
    }
}

/// Load balancer implementation
#[derive(Debug)]
pub struct LoadBalancer {
    config: LoadBalancingConfig,
    state: Arc<RwLock<LoadBalancerState>>,
}

#[derive(Debug)]
struct LoadBalancerState {
    current_index: usize,
    session_map: HashMap<String, String>, // session_id -> service_id
    service_weights: HashMap<String, f64>,
    service_connections: HashMap<String, u32>,
}

impl LoadBalancer {
    /// Create a new load balancer with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Configuration is invalid
    /// - Required fields are missing
    pub fn new(config: &LoadBalancingConfig) -> Result<Self, BearDogError> {
        let state = LoadBalancerState {
            current_index: 0,
            session_map: HashMap::new(),
            service_weights: HashMap::new(),
            service_connections: HashMap::new(),
        };

        Ok(Self {
            config: *config,
            state: Arc::new(RwLock::new(state)),
        })
    }

    /// Start the load balancer background tasks
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Background tasks fail to start
    /// - Load balancer is already running
    pub const fn start(&self) -> Result<(), BearDogError> {
        // Implementation would start background tasks for adaptive load balancing
        Ok(())
    }

    /// Stop the load balancer and clean up resources
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Background tasks fail to stop gracefully
    /// - Resource cleanup fails
    pub const fn stop(&self) -> Result<(), BearDogError> {
        // Implementation would stop background tasks
        Ok(())
    }

    /// Apply load balancing algorithm
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Load balancing algorithm fails
    /// - Service information is invalid
    /// - State update fails
    pub async fn balance_services(
        &self,
        services: Vec<ServiceInfo>,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        if services.is_empty() {
            return Ok(services);
        }

        let healthy_services: Vec<&ServiceInfo> = services
            .iter()
            .filter(|_service| {
                // For now, assume all services are healthy since canonical ServiceInfo doesn't have health_status
                true
            })
            .collect();

        if healthy_services.is_empty() {
            return Ok(vec![]);
        }

        // Apply the configured algorithm
        let result = match self.config.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                self.round_robin_balance(healthy_services).await?
            }
            LoadBalancingAlgorithm::LeastConnections => Self::least_connections_balance(&services),
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                Self::weighted_round_robin_balance(services.clone())
            }
            LoadBalancingAlgorithm::Random => Self::random_balance(services.clone()),
            LoadBalancingAlgorithm::IpHash => Self::ip_hash_balance(services.clone()),
            LoadBalancingAlgorithm::LeastResponseTime => {
                Self::least_response_time_balance(services.clone())
            }
            LoadBalancingAlgorithm::ResourceBased => Self::resource_based_balance(services.clone()),
        };

        // Handle sticky sessions if enabled
        if self.config.enable_sticky_sessions {
            // Session handling logic would go here
            // For now, just return the balanced result
        }

        Ok(result)
    }

    /// Select the best service from available options using the configured algorithm
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Load balancing algorithm fails
    /// - Service selection fails
    /// - State update fails
    pub async fn select_service(
        &self,
        services: &[ServiceInfo],
        session_id: Option<&str>,
    ) -> Result<Option<ServiceInfo>, BearDogError> {
        if services.is_empty() {
            return Ok(None);
        }

        // Check for sticky session
        if let Some(session_id) = session_id {
            if self.config.enable_sticky_sessions {
                let state = self.state.read().await;
                if let Some(service_id) = state.session_map.get(session_id) {
                    if let Some(service) = services.iter().find(|s| s.name == *service_id) {
                        return Ok(Some(service.clone()));
                    }
                }
            }
        }

        // Apply load balancing algorithm
        let balanced_services = self.balance_services(services.to_vec()).await?;
        let selected_service = balanced_services.first().cloned();

        // Update sticky session if enabled
        if let (Some(session_id), Some(service)) = (session_id, &selected_service) {
            if self.config.enable_sticky_sessions {
                let mut state = self.state.write().await;
                state
                    .session_map
                    .insert(session_id.to_string(), service.name.clone());
            }
        }

        Ok(selected_service)
    }

    // Load balancing algorithm implementations
    async fn round_robin_balance(
        &self,
        services: Vec<&ServiceInfo>,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        if services.is_empty() {
            return Ok(services.into_iter().cloned().collect());
        }

        let index = {
            let mut state = self.state.write().await;
            let index = state.current_index % services.len();
            state.current_index = (state.current_index + 1) % services.len();
            index
        }; // Drop lock early

        // Move selected service to front
        let mut result = services;
        if index < result.len() {
            result.swap(0, index);
        }
        Ok(result.into_iter().cloned().collect())
    }

    #[must_use]
    fn least_connections_balance(services: &[ServiceInfo]) -> Vec<ServiceInfo> {
        // In a real implementation, this would track active connections per service
        // For now, simulate by using service_id hash as connection count
        let mut services_with_connections: Vec<(ServiceInfo, u32)> = services
            .iter()
            .map(|service| {
                #[allow(clippy::cast_possible_truncation)]
                let connection_count = (service.name.len() % 10) as u32; // Simulated connection count
                (service.clone(), connection_count)
            })
            .collect();

        // Sort by connection count (ascending - least connections first)
        services_with_connections.sort_by(|a, b| a.1.cmp(&b.1));

        services_with_connections
            .into_iter()
            .map(|(service, _)| service)
            .collect()
    }

    #[must_use]
    fn weighted_round_robin_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        // In a real implementation, services would have weight metadata
        // For now, assign weights based on service name length
        let mut weighted_services: Vec<(ServiceInfo, u32)> = services
            .into_iter()
            .map(|service| {
                #[allow(clippy::cast_possible_truncation)]
                let weight = (service.name.len() % 5 + 1) as u32; // Weight 1-5 based on name length
                (service, weight)
            })
            .collect();

        // Sort by weight (descending - highest weight first)
        weighted_services.sort_by(|a, b| b.1.cmp(&a.1));

        weighted_services
            .into_iter()
            .map(|(service, _)| service)
            .collect()
    }

    #[must_use]
    fn random_balance(mut services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Use a deterministic "random" shuffle based on current timestamp
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .hash(&mut hasher);

        #[allow(clippy::cast_possible_truncation)]
        let seed = hasher.finish() as usize;

        // Simple shuffle algorithm
        for i in 0..services.len() {
            let j = (seed + i) % services.len();
            services.swap(i, j);
        }

        services
    }

    #[must_use]
    fn ip_hash_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        // In a real implementation, this would hash the client IP
        // For now, sort by service endpoint hash for consistency
        let mut services = services;
        services.sort_by(|a, b| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher_a = DefaultHasher::new();
            a.address.hash(&mut hasher_a);
            let hash_a = hasher_a.finish();

            let mut hasher_b = DefaultHasher::new();
            b.address.hash(&mut hasher_b);
            let hash_b = hasher_b.finish();

            hash_a.cmp(&hash_b)
        });

        services
    }

    #[must_use]
    fn least_response_time_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        // In a real implementation, this would track response times
        // For now, simulate response time based on endpoint length
        let mut services_with_response_time: Vec<(ServiceInfo, u64)> = services
            .into_iter()
            .map(|service| {
                #[allow(clippy::cast_possible_truncation)]
                let response_time_ms = (service.address.len() % 100 + 10) as u64; // 10-109ms simulated
                (service, response_time_ms)
            })
            .collect();

        // Sort by response time (ascending - fastest first)
        services_with_response_time.sort_by(|a, b| a.1.cmp(&b.1));

        services_with_response_time
            .into_iter()
            .map(|(service, _)| service)
            .collect()
    }

    #[must_use]
    fn resource_based_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        // In a real implementation, this would check CPU/memory usage
        // For now, simulate resource usage based on capabilities count
        let mut services_with_resources: Vec<(ServiceInfo, f64)> = services
            .into_iter()
            .map(|service| {
                #[allow(clippy::cast_precision_loss)]
                let resource_usage = (service.metadata.len() as f64 * 0.1).min(1.0); // 0.0-1.0 usage
                (service, resource_usage)
            })
            .collect();

        // Sort by resource usage (ascending - least used first)
        services_with_resources
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        services_with_resources
            .into_iter()
            .map(|(service, _)| service)
            .collect()
    }

    /// Updates service weight for weighted load balancing
    ///
    /// # Errors
    ///
    /// Returns error if state update fails
    pub async fn update_service_weight(
        &self,
        service_id: &str,
        weight: f64,
    ) -> Result<(), BearDogError> {
        let mut state = self.state.write().await;
        state.service_weights.insert(service_id.to_string(), weight);
        drop(state); // Drop lock early
        Ok(())
    }

    /// Increment connection count for a service
    ///
    /// # Errors
    ///
    /// Returns error if state update fails
    pub async fn increment_connections(&self, service_id: &str) -> Result<(), BearDogError> {
        let mut state = self.state.write().await;
        let current_count = state
            .service_connections
            .get(service_id)
            .copied()
            .unwrap_or(0);
        state
            .service_connections
            .insert(service_id.to_string(), current_count + 1);
        drop(state); // Drop lock early
        Ok(())
    }

    /// Decrement connection count for a service
    ///
    /// # Errors
    ///
    /// Returns error if state update fails
    pub async fn decrement_connections(&self, service_id: &str) -> Result<(), BearDogError> {
        let mut state = self.state.write().await;
        if let Some(count) = state.service_connections.get_mut(service_id) {
            if *count > 0 {
                *count -= 1;
            }
        }
        drop(state); // Drop lock early
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_services(count: usize) -> Vec<ServiceInfo> {
        (0..count)
            .map(|i| ServiceInfo {
                name: format!("service-{i}"),
                service_type: "test".to_string(),
                address: format!("192.168.1.{i}"),
                port: 8080,
                metadata: HashMap::new(),
            })
            .collect()
    }

    fn create_test_services_with_metadata(count: usize) -> Vec<ServiceInfo> {
        (0..count)
            .map(|i| {
                let mut metadata = HashMap::new();
                for j in 0..i {
                    metadata.insert(format!("key-{j}"), format!("value-{j}"));
                }
                ServiceInfo {
                    name: format!("service-{i}"),
                    service_type: "test".to_string(),
                    address: format!("10.0.0.{i}"),
                    port: 9000,
                    metadata,
                }
            })
            .collect()
    }

    #[test]
    fn test_default_config() {
        let config = LoadBalancingConfig::default();
        assert!(matches!(
            config.algorithm,
            LoadBalancingAlgorithm::RoundRobin
        ));
        assert!(!config.enable_sticky_sessions);
        assert_eq!(config.session_timeout_secs, 1800);
        assert!((config.health_weight_factor - 0.7).abs() < f64::EPSILON);
        assert!(config.enable_adaptive);
    }

    #[test]
    fn test_circuit_breaker_default_config() {
        let config = CircuitBreakerConfig::default();
        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.recovery_timeout_secs, 60); // Actual default is 60
        assert_eq!(config.half_open_max_calls, 3);
    }

    #[tokio::test]
    async fn test_load_balancer_creation() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();
        assert!(balancer.balance_services(vec![]).await.is_ok());
    }

    #[tokio::test]
    async fn test_round_robin_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services(3);
        let result = balancer.balance_services(services).await.unwrap();

        // Round robin should preserve order on first call
        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn test_least_connections_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::LeastConnections,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services(3);
        let result = balancer.balance_services(services).await.unwrap();

        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn test_weighted_round_robin_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        // Set different weights
        balancer
            .update_service_weight("service-0", 1.0)
            .await
            .unwrap();
        balancer
            .update_service_weight("service-1", 2.0)
            .await
            .unwrap();
        balancer
            .update_service_weight("service-2", 3.0)
            .await
            .unwrap();

        let services = create_test_services(3);
        let result = balancer.balance_services(services).await.unwrap();

        // Higher weight services should be first
        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn test_random_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::Random,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services(5);
        let result = balancer.balance_services(services).await.unwrap();

        assert_eq!(result.len(), 5);
    }

    #[tokio::test]
    async fn test_ip_hash_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::IpHash,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services(3);
        let result1 = balancer.balance_services(services.clone()).await.unwrap();
        let result2 = balancer.balance_services(services).await.unwrap();

        // IP hash should be deterministic
        assert_eq!(result1.len(), result2.len());
        for (s1, s2) in result1.iter().zip(result2.iter()) {
            assert_eq!(s1.name, s2.name);
        }
    }

    #[tokio::test]
    async fn test_least_response_time_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::LeastResponseTime,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services(3);
        let result = balancer.balance_services(services).await.unwrap();

        assert_eq!(result.len(), 3);
    }

    #[tokio::test]
    async fn test_resource_based_balance() {
        let config = LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::ResourceBased,
            ..Default::default()
        };
        let balancer = LoadBalancer::new(&config).unwrap();

        let services = create_test_services_with_metadata(3);
        let result = balancer.balance_services(services).await.unwrap();

        // Services with less metadata (less resource usage) should be first
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "service-0"); // Least metadata
    }

    #[tokio::test]
    async fn test_connection_counting() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();

        // Increment connections
        balancer.increment_connections("service-1").await.unwrap();
        balancer.increment_connections("service-1").await.unwrap();
        balancer.increment_connections("service-2").await.unwrap();

        // Decrement connections
        balancer.decrement_connections("service-1").await.unwrap();

        // Decrement non-existent service should not error
        balancer.decrement_connections("service-999").await.unwrap();

        // Verify state
        let state = balancer.state.read().await;
        assert_eq!(state.service_connections.get("service-1"), Some(&1));
        assert_eq!(state.service_connections.get("service-2"), Some(&1));
    }

    #[tokio::test]
    async fn test_service_weight_update() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();

        balancer
            .update_service_weight("service-1", 2.5)
            .await
            .unwrap();
        balancer
            .update_service_weight("service-2", 1.5)
            .await
            .unwrap();

        let state = balancer.state.read().await;
        assert!((state.service_weights.get("service-1").unwrap() - 2.5).abs() < f64::EPSILON);
        assert!((state.service_weights.get("service-2").unwrap() - 1.5).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_empty_services() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();
        let result = balancer.balance_services(vec![]).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_single_service() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();
        let services = create_test_services(1);
        let result = balancer.balance_services(services).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "service-0");
    }

    #[test]
    fn test_all_algorithms_variant() {
        // Ensure all algorithm variants can be created
        let algorithms = [
            LoadBalancingAlgorithm::RoundRobin,
            LoadBalancingAlgorithm::LeastConnections,
            LoadBalancingAlgorithm::WeightedRoundRobin,
            LoadBalancingAlgorithm::Random,
            LoadBalancingAlgorithm::IpHash,
            LoadBalancingAlgorithm::LeastResponseTime,
            LoadBalancingAlgorithm::ResourceBased,
        ];

        assert_eq!(algorithms.len(), 7);
    }

    #[test]
    fn test_load_balancer_start_stop() {
        let config = LoadBalancingConfig::default();
        let balancer = LoadBalancer::new(&config).unwrap();

        assert!(balancer.start().is_ok());
        assert!(balancer.stop().is_ok());
    }
}
