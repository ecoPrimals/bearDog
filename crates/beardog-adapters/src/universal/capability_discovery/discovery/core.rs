//! # Core Capability Discovery Engine
//!
//! This module contains the main discovery engine and core trait definitions.

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{
    CapabilityType, UniversalCapability, CapabilityDiscoveryRequest, CapabilityDiscoveryResponse,
    DiscoveryMetadata, ProviderInfo, ProviderType, EndpointConfig, AuthConfig, AuthType,
    HealthStatus, PerformanceMetrics, SecurityLevel, CircuitBreakerConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, debug, error};

use super::config::DiscoveryConfig;
use super::metrics::DiscoveryMetrics;
use super::strategies::*;

/// Universal capability discovery engine that replaces all hardcoded integrations
pub struct UniversalCapabilityDiscovery {
    /// Registry of discovered capabilities
    capability_registry: Arc<RwLock<HashMap<CapabilityType, Vec<UniversalCapability>>>>,
    discovery_strategies: Vec<Box<dyn DiscoveryStrategy + Send + Sync>>,
    config: DiscoveryConfig,
    metrics: DiscoveryMetrics,
}

/// Discovery strategy trait for implementing different discovery mechanisms
pub trait DiscoveryStrategy: Send + Sync + std::fmt::Debug {
    /// Discover capabilities of specified types
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError>;
    
    /// Get strategy name
    fn strategy_name(&self) -> &'static str;
    
    /// Check if strategy is available
    fn is_available(&self) -> bool;
}

impl UniversalCapabilityDiscovery {
    /// Create new universal capability discovery system
    pub async fn new() -> Result<Self, BearDogError> {
        let config = DiscoveryConfig::default();
        let mut discovery = Self {
            capability_registry: Arc::new(RwLock::new(HashMap::new())),
            discovery_strategies: Vec::new(),
            config,
            metrics: DiscoveryMetrics::default(),
        };
        
        // Initialize discovery strategies
        discovery.initialize_strategies()?;
        
        info!("🌌 Universal Capability Discovery initialized with {} strategies", 
              discovery.discovery_strategies.len());
        
        Ok(discovery)
    }
    
    /// Initialize all discovery strategies
    fn initialize_strategies(&mut self) -> Result<(), BearDogError> {
        // Environment-based discovery (replaces hardcoded endpoints)
        self.discovery_strategies.push(Box::new(EnvironmentDiscoveryStrategy::new()));
        
        // Universal service mesh discovery
        self.discovery_strategies.push(Box::new(ServiceMeshDiscoveryStrategy::new()));
        
        // Cloud vendor discovery (replaces hardcoded cloud providers)
        self.discovery_strategies.push(Box::new(CloudVendorDiscoveryStrategy::new()));
        
        // Container orchestration discovery (replaces hardcoded biomeOS)
        self.discovery_strategies.push(Box::new(ContainerDiscoveryStrategy::new()));
        
        // Network discovery (replaces hardcoded primal connections)
        self.discovery_strategies.push(Box::new(NetworkDiscoveryStrategy::new()));
        
        Ok(())
    }
    
    /// Discover capabilities by type (main public interface)
    pub async fn discover_capabilities(
        &self,
        request: CapabilityDiscoveryRequest,
    ) -> Result<CapabilityDiscoveryResponse, BearDogError> {
        let start_time = std::time::Instant::now();
        
        info!("🔍 Discovering capabilities: {:?}", request.capability_types);
        
        let mut all_capabilities = Vec::new();
        let mut providers_queried = 0;
        
        // Try each discovery strategy
        for strategy in &self.discovery_strategies {
            if !strategy.is_available() {
                debug!("Strategy {} not available, skipping", strategy.strategy_name());
                continue;
            }
            
            match strategy.discover_capabilities(&request.capability_types).await {
                Ok(mut capabilities) => {
                    providers_queried += 1;
                    
                    // Filter capabilities based on requirements
                    capabilities = self.filter_capabilities(capabilities, &request);
                    
                    all_capabilities.extend(capabilities);
                    
                    debug!("Strategy {} discovered {} capabilities", 
                           strategy.strategy_name(), all_capabilities.len());
                }
                Err(e) => {
                    warn!("Discovery strategy {} failed: {}", strategy.strategy_name(), e);
                }
            }
        }
        
        // Deduplicate and rank capabilities
        let final_capabilities = self.deduplicate_and_rank(all_capabilities);
        
        // Update cache
        self.update_cache(&final_capabilities);
        
        let discovery_duration = start_time.elapsed().as_millis() as u64;
        
        let response = CapabilityDiscoveryResponse {
            capabilities: final_capabilities.clone(),
            metadata: DiscoveryMetadata {
                timestamp: chrono::Utc::now(),
                discovery_duration_ms: discovery_duration,
                providers_queried,
                capabilities_found: final_capabilities.len() as u32,
            },
        };
        
        info!("✅ Discovery completed: {} capabilities found in {}ms", 
              final_capabilities.len(), discovery_duration);
        
        Ok(response)
    }
    
    /// Discover specific capability type (convenience method)
    pub async fn discover_capability(
        &self,
        capability_type: CapabilityType,
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let request = CapabilityDiscoveryRequest {
            capability_types: vec![capability_type],
            min_security_level: None,
            max_response_time_ms: None,
            min_success_rate: None,
            preferred_regions: Vec::new(),
            required_compliance: Vec::new(),
        };
        
        let response = self.discover_capabilities(request).await?;
        Ok(response.capabilities)
    }
    
    /// Get cached capability if available
    pub async fn get_cached_capability(
        &self,
        capability_type: &CapabilityType,
    ) -> Option<Vec<UniversalCapability>> {
        let registry = self.capability_registry.read().await;
        registry.get(capability_type).cloned()
    }
    
    /// Filter capabilities based on requirements
    fn filter_capabilities(
        &self,
        capabilities: Vec<UniversalCapability>,
        request: &CapabilityDiscoveryRequest,
    ) -> Vec<UniversalCapability> {
        capabilities
            .into_iter()
            .filter(|cap| {
                // Security level filter
                if let Some(min_security) = &request.min_security_level {
                    if cap.security_level < *min_security {
                        return false;
                    }
                }
                
                // Response time filter
                if let Some(max_response_time) = request.max_response_time_ms {
                    if cap.performance.avg_response_time_ms > max_response_time as f64 {
                        return false;
                    }
                }
                
                // Success rate filter
                if let Some(min_success_rate) = request.min_success_rate {
                    if cap.performance.success_rate < min_success_rate {
                        return false;
                    }
                }
                
                // Region filter
                if !request.preferred_regions.is_empty() {
                    if let Some(region) = &cap.provider.region {
                        if !request.preferred_regions.contains(region) {
                            return false;
                        }
                    }
                }
                
                true
            })
            .collect()
    }
    
    fn deduplicate_and_rank(&self, capabilities: Vec<UniversalCapability>) -> Vec<UniversalCapability> {
        let mut capability_map: HashMap<String, Vec<UniversalCapability>> = HashMap::new();
        
        // Group by capability type and provider
        for capability in capabilities {
            let key = format!("{}:{}", 
                             capability.capability_type.name(), 
                             capability.provider.provider_id);
            capability_map.entry(key).or_default().push(capability);
        }
        
        // Select best capability for each type/provider combination
        let mut ranked_capabilities = Vec::new();
        for (_, mut caps) in capability_map {
            // Sort by health, then performance, then security level
            caps.sort_by(|a, b| {
                use std::cmp::Ordering;
                
                // Health status priority
                match (&a.health_status, &b.health_status) {
                    (HealthStatus::Healthy, HealthStatus::Healthy) => {},
                    (HealthStatus::Healthy, _) => return Ordering::Less,
                    (_, HealthStatus::Healthy) => return Ordering::Greater,
                    _ => {},
                }
                
                // Performance comparison
                let a_score = a.performance.success_rate - (a.performance.avg_response_time_ms / 1000.0);
                let b_score = b.performance.success_rate - (b.performance.avg_response_time_ms / 1000.0);
                b_score.partial_cmp(&a_score).unwrap_or(Ordering::Equal)
            });
            
            if let Some(best_capability) = caps.into_iter().next() {
                ranked_capabilities.push(best_capability);
            }
        }
        
        ranked_capabilities
    }
    
    /// Update capability cache
    async fn update_cache(&self, capabilities: &[UniversalCapability]) {
        let mut registry = self.capability_registry.write().await;
        
        for capability in capabilities {
            registry
                .entry(capability.capability_type.clone())
                .or_default()
                .push(capability.clone());
        }
        
        debug!("Updated capability cache with {} capabilities", capabilities.len());
    }
} 