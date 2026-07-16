// SPDX-License-Identifier: AGPL-3.0-or-later

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

        let state = self.state.read().await;

        let result = match self.config.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                drop(state);
                self.round_robin_balance(healthy_services).await?
            }
            LoadBalancingAlgorithm::LeastConnections => {
                Self::least_connections_balance(&services, &state.service_connections)
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                Self::weighted_round_robin_balance(services, &state.service_weights)
            }
            LoadBalancingAlgorithm::Random => Self::random_balance(services),
            LoadBalancingAlgorithm::IpHash => Self::ip_hash_balance(services),
            LoadBalancingAlgorithm::LeastResponseTime => {
                Self::least_response_time_balance(services)
            }
            LoadBalancingAlgorithm::ResourceBased => Self::resource_based_balance(services),
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
        if let Some(session_id) = session_id
            && self.config.enable_sticky_sessions
        {
            let state = self.state.read().await;
            if let Some(service_id) = state.session_map.get(session_id)
                && let Some(service) = services.iter().find(|s| s.name == *service_id)
            {
                return Ok(Some(service.clone()));
            }
        }

        // Apply load balancing algorithm
        let balanced_services = self.balance_services(services.to_vec()).await?;
        let selected_service = balanced_services.first().cloned();

        // Update sticky session if enabled
        if let (Some(session_id), Some(service)) = (session_id, &selected_service)
            && self.config.enable_sticky_sessions
        {
            let mut state = self.state.write().await;
            state
                .session_map
                .insert(session_id.to_string(), service.name.clone());
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

    fn least_connections_balance(
        services: &[ServiceInfo],
        tracked: &HashMap<String, u32>,
    ) -> Vec<ServiceInfo> {
        let mut services_with_connections: Vec<(ServiceInfo, u32)> = services
            .iter()
            .map(|service| {
                let count = tracked.get(&service.name).copied().unwrap_or(0);
                (service.clone(), count)
            })
            .collect();

        services_with_connections.sort_by(|a, b| a.1.cmp(&b.1));

        services_with_connections
            .into_iter()
            .map(|(service, _)| service)
            .collect()
    }

    fn weighted_round_robin_balance(
        services: Vec<ServiceInfo>,
        weights: &HashMap<String, f64>,
    ) -> Vec<ServiceInfo> {
        let mut weighted: Vec<(ServiceInfo, f64)> = services
            .into_iter()
            .map(|service| {
                let w = weights.get(&service.name).copied().unwrap_or(1.0);
                (service, w)
            })
            .collect();

        weighted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        weighted.into_iter().map(|(service, _)| service).collect()
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

        #[expect(
            clippy::cast_possible_truncation,
            reason = "Hash seed for deterministic shuffle"
        )]
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

    fn least_response_time_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        let mut with_rt: Vec<(ServiceInfo, u64)> = services
            .into_iter()
            .map(|service| {
                let rt = service
                    .metadata
                    .get("avg_response_ms")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(u64::MAX);
                (service, rt)
            })
            .collect();

        with_rt.sort_by(|a, b| a.1.cmp(&b.1));

        with_rt.into_iter().map(|(service, _)| service).collect()
    }

    fn resource_based_balance(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        let mut with_usage: Vec<(ServiceInfo, f64)> = services
            .into_iter()
            .map(|service| {
                let usage = service
                    .metadata
                    .get("resource_usage")
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or(0.5);
                (service, usage)
            })
            .collect();

        with_usage.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        with_usage.into_iter().map(|(service, _)| service).collect()
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
        if let Some(count) = state.service_connections.get_mut(service_id)
            && *count > 0
        {
            *count -= 1;
        }
        drop(state); // Drop lock early
        Ok(())
    }
}

#[cfg(test)]
#[path = "load_balancing_tests.rs"]
mod tests;
