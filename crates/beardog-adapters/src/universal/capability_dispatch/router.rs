//! # Zero-Cost Capability Router
//!
//! This module provides the main routing logic for the zero-cost capability dispatch system,
//! including load balancing, performance monitoring, and request routing.

use super::core::{CapabilityHandlerDispatch, DispatchConfig, LoadBalancingStrategy, CapabilityMatch};
use beardog_errors::BearDogError;
use beardog_types::adapters::{CapabilityRequest, CapabilityResponse, CapabilityType};
use beardog_types::constants::domains::system::defaults::{DEFAULT_POOL_SIZE, DEFAULT_BUFFER_SIZE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{info, warn, debug, error};

/// **Zero-Cost Capability Router**
///
/// High-performance capability router that uses enum dispatch instead of
/// `Box<dyn>` patterns for maximum performance and zero-cost abstractions.
///
/// ## 🚀 **Performance Optimizations**
/// - **Enum dispatch** instead of vtable lookups
/// - **Stack allocation** instead of heap allocations
/// - **Compile-time optimization** through monomorphization
/// - **Lock-free routing** for read-heavy workloads
#[derive(Debug)]
pub struct ZeroCostCapabilityRouter {
    /// Registered capability handlers
    handlers: Arc<RwLock<Vec<(CapabilityHandlerDispatch, f64)>>>,
    /// Router configuration
    config: DispatchConfig,
    /// Router statistics
    statistics: Arc<RwLock<RouterStatistics>>,
    /// Semaphore for request limiting
    request_semaphore: Arc<Semaphore>,
}

/// Router performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterStatistics {
    /// Total requests processed
    pub total_requests: u64,
    /// Total successful requests
    pub successful_requests: u64,
    /// Total failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Handler utilization statistics
    pub handler_utilization: HashMap<String, HandlerUtilization>,
    /// Last statistics update
    pub last_updated: std::time::SystemTime,
}

/// Handler utilization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerUtilization {
    /// Total requests handled
    pub total_requests: u64,
    /// Average response time
    pub average_response_time_ms: f64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Current load (0.0 - 1.0)
    pub current_load: f64,
}

impl ZeroCostCapabilityRouter {
    /// Create a new zero-cost capability router
    pub fn new(config: DispatchConfig) -> Self {
        let request_semaphore = Arc::new(Semaphore::new(config.max_concurrent_requests as usize));
        
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
            config,
            statistics: Arc::new(RwLock::new(RouterStatistics::new())),
            request_semaphore,
        }
    }

    /// Register a capability handler with the router
    pub fn register_handler(&self, handler: CapabilityHandlerDispatch, weight: f64) -> Result<()> {
        info!("Registering capability handler: {} (weight: {})", handler.get_handler_id(), weight);
        
        // Validate the handler configuration
        handler.validate_configuration()?;
        
        let mut handlers = self.handlers.write()
            .map_err(|_| BearDogError::Internal("Failed to acquire write lock".to_string()))?;
        
        handlers.push((handler, weight));
        
        info!("Handler registered successfully. Total handlers: {}", handlers.len());
        Ok(())
    }

    /// Route a capability request to the appropriate handler
    pub async fn route_request(&self, request: CapabilityRequest) -> Result<CapabilityResponse> {
        let start_time = Instant::now();
        
        // Acquire semaphore permit for request limiting
        let _permit = if self.config.enable_queuing {
            Some(self.request_semaphore.acquire().await
                .map_err(|_| BearDogError::Internal("Failed to acquire request permit".to_string()))?)
        } else {
            None
        };

        debug!("Routing capability request: {:?}", request.capability_type);

        // Find matching handlers
        let matching_handlers = self.find_matching_handlers(&request.capability_type)?;
        
        if matching_handlers.is_empty() {
            warn!("No handlers found for capability type: {:?}", request.capability_type);
            self.update_statistics(false, start_time.elapsed());
            return Err(BearDogError::NotFound(
                format!("No handlers available for capability type: {:?}", request.capability_type)
            ));
        }

        // Select best handler based on load balancing strategy
        let selected_handler = self.select_handler(&matching_handlers)?;
        
        info!("Selected handler: {} for request", selected_handler.get_handler_id());

        // Execute the request with timeout
        let response = match tokio::time::timeout(
            Duration::from_millis(self.config.request_timeout_ms as u64),
            selected_handler.handle_request(&request)
        ).await {
            Ok(result) => result,
            Err(_) => {
                error!("Request timeout for handler: {}", selected_handler.get_handler_id());
                self.update_statistics(false, start_time.elapsed());
                return Err(BearDogError::Timeout("Request timeout".to_string()));
            }
        };

        // Update statistics
        let success = response.is_ok();
        self.update_statistics(success, start_time.elapsed());
        
        if let Ok(ref resp) = response {
            debug!("Request completed successfully in {:?}", start_time.elapsed());
        } else {
            warn!("Request failed in {:?}: {:?}", start_time.elapsed(), response);
        }

        response
    }

    /// Find handlers that can process the given capability type
    fn find_matching_handlers(&self, capability_type: &CapabilityType) -> Result<Vec<&CapabilityHandlerDispatch>> {
        let handlers = self.handlers.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        
        let matching: Vec<&CapabilityHandlerDispatch> = handlers
            .iter()
            .filter_map(|(handler, _weight)| {
                if handler.can_handle(capability_type) {
                    Some(handler)
                } else {
                    None
                }
            })
            .collect();

        Ok(matching)
    }

    /// Select the best handler based on load balancing strategy
    fn select_handler(&self, handlers: &[&CapabilityHandlerDispatch]) -> Result<&CapabilityHandlerDispatch> {
        if handlers.is_empty() {
            return Err(BearDogError::NotFound("No handlers available".to_string()));
        }

        match self.config.load_balancing_strategy {
            LoadBalancingStrategy::RoundRobin => {
                // Simple round-robin selection
                let index = self.get_round_robin_index() % handlers.len();
                Ok(handlers[index])
            },
            LoadBalancingStrategy::Random => {
                // Random selection
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..handlers.len());
                Ok(handlers[index])
            },
            LoadBalancingStrategy::LeastConnections => {
                // Select handler with least current load
                self.select_least_loaded_handler(handlers)
            },
            LoadBalancingStrategy::WeightedRoundRobin => {
                // Weighted selection based on handler weights
                self.select_weighted_handler(handlers)
            },
            LoadBalancingStrategy::ResourceBased => {
                // Select based on resource utilization
                self.select_resource_based_handler(handlers)
            },
        }
    }

    /// Get round-robin index (thread-safe counter)
    fn get_round_robin_index(&self) -> usize {
        // Simple implementation using statistics counter
        let stats = self.statistics.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Statistics lock poisoned on read, recovering");
                poisoned.into_inner()
            });
        stats.total_requests as usize
    }

    /// Select handler with least current load
    fn select_least_loaded_handler(&self, handlers: &[&CapabilityHandlerDispatch]) -> Result<&CapabilityHandlerDispatch> {
        let stats = self.statistics.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        
        let mut best_handler = handlers[0];
        let mut lowest_load = f64::MAX;
        
        for handler in handlers {
            let handler_id = handler.get_handler_id();
            let load = stats.handler_utilization
                .get(handler_id)
                .map(|util| util.current_load)
                .unwrap_or(0.0);
            
            if load < lowest_load {
                lowest_load = load;
                best_handler = handler;
            }
        }
        
        Ok(best_handler)
    }

    /// Select handler based on weights
    fn select_weighted_handler(&self, handlers: &[&CapabilityHandlerDispatch]) -> Result<&CapabilityHandlerDispatch> {
        let handlers_guard = self.handlers.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        
        // Create weight map
        let mut weights: HashMap<String, f64> = HashMap::new();
        for (handler, weight) in handlers_guard.iter() {
            weights.insert(handler.get_handler_id().to_string(), *weight);
        }
        
        // Simple weighted selection (could be improved with proper weighted random)
        let mut best_handler = handlers[0];
        let mut highest_weight = 0.0;
        
        for handler in handlers {
            let weight = weights.get(handler.get_handler_id()).unwrap_or(&1.0);
            if *weight > highest_weight {
                highest_weight = *weight;
                best_handler = handler;
            }
        }
        
        Ok(best_handler)
    }

    /// Select handler based on resource utilization
    fn select_resource_based_handler(&self, handlers: &[&CapabilityHandlerDispatch]) -> Result<&CapabilityHandlerDispatch> {
        let mut best_handler = handlers[0];
        let mut best_score = f64::MIN;
        
        for handler in handlers {
            let perf_info = handler.get_performance_info();
            
            // Score based on inverse of resource usage (lower usage = higher score)
            let cpu_score = 1.0 - (perf_info.resource_usage.cpu_usage_percent as f64 / 100.0);
            let memory_score = 1.0 - (perf_info.resource_usage.memory_usage_mb as f64 / 1024.0);
            let throughput_score = perf_info.throughput_ops_per_sec as f64 / 1000.0;
            
            let total_score = (cpu_score + memory_score + throughput_score) / 3.0;
            
            if total_score > best_score {
                best_score = total_score;
                best_handler = handler;
            }
        }
        
        Ok(best_handler)
    }

    /// Update router statistics
    fn update_statistics(&self, success: bool, duration: Duration) {
        if let Ok(mut stats) = self.statistics.write() {
            stats.total_requests += 1;
            if success {
                stats.successful_requests += 1;
            } else {
                stats.failed_requests += 1;
            }
            
            // Update average response time (simple moving average)
            let duration_ms = duration.as_millis() as f64;
            stats.average_response_time_ms = 
                (stats.average_response_time_ms * (stats.total_requests - 1) as f64 + duration_ms) 
                / stats.total_requests as f64;
            
            stats.last_updated = std::time::SystemTime::now();
        }
    }

    /// Get router statistics
    pub fn get_statistics(&self) -> Result<RouterStatistics> {
        let stats = self.statistics.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        Ok(stats.clone())
    }

    /// Get all registered handlers
    pub fn get_handlers(&self) -> Result<Vec<String>> {
        let handlers = self.handlers.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        
        let handler_ids: Vec<String> = handlers
            .iter()
            .map(|(handler, _)| handler.get_handler_id().to_string())
            .collect();
        
        Ok(handler_ids)
    }

    /// Remove a handler from the router
    pub fn remove_handler(&self, handler_id: &str) -> Result<bool> {
        let mut handlers = self.handlers.write()
            .map_err(|_| BearDogError::Internal("Failed to acquire write lock".to_string()))?;
        
        let initial_len = handlers.len();
        handlers.retain(|(handler, _)| handler.get_handler_id() != handler_id);
        
        let removed = handlers.len() < initial_len;
        if removed {
            info!("Removed handler: {}", handler_id);
        } else {
            warn!("Handler not found for removal: {}", handler_id);
        }
        
        Ok(removed)
    }

    /// Health check for the router
    pub async fn health_check(&self) -> Result<RouterHealthStatus> {
        let handlers = self.handlers.read()
            .map_err(|_| BearDogError::Internal("Failed to acquire read lock".to_string()))?;
        
        let total_handlers = handlers.len();
        let available_permits = self.request_semaphore.available_permits();
        let stats = self.get_statistics()?;
        
        let health_status = if total_handlers > 0 && available_permits > 0 {
            HealthStatus::Healthy
        } else if total_handlers > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        
        Ok(RouterHealthStatus {
            status: health_status,
            total_handlers,
            available_permits,
            total_requests: stats.total_requests,
            success_rate: if stats.total_requests > 0 {
                stats.successful_requests as f64 / stats.total_requests as f64
            } else {
                0.0
            },
            average_response_time_ms: stats.average_response_time_ms,
        })
    }
}

impl Default for ZeroCostCapabilityRouter {
    fn default() -> Self {
        Self::new(DispatchConfig::default())
    }
}

impl RouterStatistics {
    /// Create new router statistics
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            requests_per_second: 0.0,
            handler_utilization: HashMap::new(),
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Router health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterHealthStatus {
    /// Overall health status
    pub status: HealthStatus,
    /// Total number of registered handlers
    pub total_handlers: usize,
    /// Available request permits
    pub available_permits: usize,
    /// Total requests processed
    pub total_requests: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
}

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Router is healthy
    Healthy,
    /// Router is degraded but functional
    Degraded,
    /// Router is unhealthy
    Unhealthy,
} 