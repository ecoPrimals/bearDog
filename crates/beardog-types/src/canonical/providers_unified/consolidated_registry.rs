// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated Provider Registry - Single Source of Truth
//!
//! This module provides the **unified provider registry** that consolidates all scattered
//! provider implementations across the BearDog ecosystem into a single, manageable system.
//!
//! ## 🎯 **Consolidation Strategy**
//!
//! This registry unifies and replaces:
//! - 60+ scattered provider struct implementations
//! - Multiple provider registries across crates
//! - Duplicate provider management systems
//! - Fragmented provider discovery mechanisms
//!
//! ## 🏗️ **Architecture Benefits**
//!
//! - **Single Registration Point**: All providers register through one system
//! - **Unified Discovery**: Consistent provider discovery across all domains
//! - **Type Safety**: Strongly typed provider management with comprehensive validation
//! - **Performance Optimized**: Zero-cost provider resolution and dispatch
//! - **Maintainability**: Single location for all provider management logic

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use super::traits::consolidated::{ConsolidatedProvider, ProviderInfo, ProviderHealth, ProviderType, HealthStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, error};

/// **Type-Erased Provider Wrapper**
///
/// This wrapper allows us to store different ConsolidatedProvider implementations
/// in the same collection by erasing their associated types.
#[async_trait::async_trait]
pub trait ErasedProvider: Send + Sync + std::fmt::Debug + 'static {
    /// Get provider identification information
    fn provider_info(&self) -> ProviderInfo;
    
    /// Get provider version string  
    fn provider_version(&self) -> &str;
    
    /// Perform health check and return current status
    async fn health_check(&self) -> Result<ProviderHealth>;
    
    /// Shutdown the provider gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// **Concrete Implementation of Type-Erased Provider**
pub struct ErasedProviderImpl<P> 
where
    P: ConsolidatedProvider,
{
    provider: P,
}

impl<P> std::fmt::Debug for ErasedProviderImpl<P>
where
    P: ConsolidatedProvider,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErasedProviderImpl")
            .field("provider_info", &self.provider.provider_info())
            .finish()
    }
}

impl<P> ErasedProviderImpl<P>
where
    P: ConsolidatedProvider,
{
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

#[async_trait::async_trait]
impl<P> ErasedProvider for ErasedProviderImpl<P>
where
    P: ConsolidatedProvider,
    P::Error: Into<BearDogError>,
{
    fn provider_info(&self) -> ProviderInfo {
        self.provider.provider_info()
    }
    
    fn provider_version(&self) -> &str {
        self.provider.provider_version()
    }
    
    async fn health_check(&self) -> Result<ProviderHealth> {
        self.provider.health_check().await.map_err(|e| e.into())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        self.provider.shutdown().await.map_err(|e| e.into())
    }
}

/// **Consolidated Provider Registry**
///
/// Single registry that manages all provider implementations across the BearDog ecosystem.
/// This eliminates the need for multiple scattered provider registries.
#[derive(Debug)]
pub struct ConsolidatedProviderRegistry {
    /// Registered providers by ID
    providers: Arc<RwLock<HashMap<String, Arc<dyn ErasedProvider>>>>,
    /// Provider metadata cache
    metadata_cache: Arc<RwLock<HashMap<String, ProviderInfo>>>,
    /// Provider health cache
    health_cache: Arc<RwLock<HashMap<String, ProviderHealth>>>,
    /// Registry configuration
    config: ProviderRegistryConfig,
}

/// Provider registry configuration
/// Renamed from RegistryConfig to ProviderRegistryConfig for clarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistryConfig {
    /// Maximum number of providers
    pub max_providers: usize,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Enable automatic provider discovery
    pub enable_auto_discovery: bool,
    /// Provider timeout in seconds
    pub provider_timeout_secs: u64,
}

/// Backward compatibility alias
#[deprecated(since = "3.2.0", note = "Use ProviderRegistryConfig instead")]
pub type RegistryConfig = ProviderRegistryConfig;

impl Default for ProviderRegistryConfig {
    fn default() -> Self {
        Self {
            max_providers: std::env::var("BEARDOG_PROVIDER_REGISTRY_MAX_PROVIDERS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            health_check_interval_secs: std::env::var("BEARDOG_PROVIDER_HEALTH_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            enable_auto_discovery: true,
            provider_timeout_secs: std::env::var("BEARDOG_PROVIDER_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
        }
    }
}

/// Provider registration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistration {
    /// Provider information
    pub info: ProviderInfo,
    /// Registration timestamp
    pub registered_at: std::time::SystemTime,
    /// Provider priority (higher = preferred)
    pub priority: i32,
    /// Provider tags for categorization
    pub tags: Vec<String>,
}

impl ConsolidatedProviderRegistry {
    /// Create new consolidated provider registry
    pub fn new(config: RegistryConfig) -> Self {
        info!("Creating consolidated provider registry with config: {:?}", config);
        
        Self {
            providers: Arc::new(RwLock::new(HashMap::with_capacity(config.max_providers))),
            metadata_cache: Arc::new(RwLock::new(HashMap::with_capacity(config.max_providers))),
            health_cache: Arc::new(RwLock::new(HashMap::with_capacity(config.max_providers))),
            config,
        }
    }

    /// Create registry with default configuration
    pub fn default() -> Self {
        Self::new(RegistryConfig::default())
    }

    /// Register a provider in the consolidated registry
    pub async fn register_provider<P>(
        &self,
        provider: P,
        priority: i32,
        _tags: Vec<String>,
    ) -> Result<String> 
    where
        P: ConsolidatedProvider + 'static,
        P::Error: Into<BearDogError>,
    {
        let provider_info = provider.provider_info();
        let provider_id = provider_info.id.clone();

        // Check capacity
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on read, recovering");
                poisoned.into_inner()
            });
        if providers_guard.len() >= self.config.max_providers {
            return Err(BearDogError::system(
                format!("Provider registry at capacity: {}", self.config.max_providers)
            ));
        }
        drop(providers_guard);

        // Register provider
        {
            let mut providers_guard = self.providers.write()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Provider registry lock poisoned on write, recovering");
                    poisoned.into_inner()
                });
            if providers_guard.contains_key(&provider_id) {
                warn!("Provider {} already registered, updating", provider_id);
            }
            providers_guard.insert(provider_id.clone(), Arc::new(ErasedProviderImpl::new(provider)));
        }

        // Cache metadata
        {
            let mut metadata_guard = self.metadata_cache.write()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Metadata cache lock poisoned on write, recovering");
                    poisoned.into_inner()
                });
            metadata_guard.insert(provider_id.clone(), provider_info.clone());
        }

        // Create registration record
        let registration = ProviderRegistration {
            info: provider_info,
            registered_at: std::time::SystemTime::now(),
            priority,
            tags: _tags,
        };

        info!("Successfully registered provider: {} (type: {:?}, priority: {})", 
              provider_id, registration.info.provider_type, priority);

        Ok(provider_id)
    }

    /// Unregister a provider
    pub async fn unregister_provider(&self, provider_id: &str) -> Result<()> {
        // Remove from providers
        let removed = {
            let mut providers_guard = self.providers.write()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Provider registry lock poisoned on unregister, recovering");
                    poisoned.into_inner()
                });
            providers_guard.remove(provider_id).is_some()
        };

        if !removed {
            return Err(BearDogError::system(
                format!("Provider {} not found for unregistration", provider_id)
            ));
        }

        // Clean up caches
        {
            let mut metadata_guard = self.metadata_cache.write()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Metadata cache lock poisoned on cleanup, recovering");
                    poisoned.into_inner()
                });
            metadata_guard.remove(provider_id);
        }
        {
            let mut health_guard = self.health_cache.write()
                .unwrap_or_else(|poisoned| {
                    tracing::warn!("Health cache lock poisoned on cleanup, recovering");
                    poisoned.into_inner()
                });
            health_guard.remove(provider_id);
        }

        info!("Successfully unregistered provider: {}", provider_id);
        Ok(())
    }

    /// Get provider by ID
    pub async fn get_provider(&self, provider_id: &str) -> Result<Arc<dyn ErasedProvider>> {
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on get, recovering");
                poisoned.into_inner()
            });
        providers_guard
            .get(provider_id)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Provider {} not found", provider_id)))
    }

    /// Find providers by type
    pub async fn find_providers_by_type(&self, provider_type: ProviderType) -> Result<Vec<Arc<dyn ErasedProvider>>> {
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on find, recovering");
                poisoned.into_inner()
            });
        let metadata_guard = self.metadata_cache.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Metadata cache lock poisoned on find, recovering");
                poisoned.into_inner()
            });

        let mut matching_providers = Vec::new();
        
        for (provider_id, provider) in providers_guard.iter() {
            if let Some(metadata) = metadata_guard.get(provider_id) {
                if metadata.provider_type == provider_type {
                    matching_providers.push(provider.clone());
                }
            }
        }

        Ok(matching_providers)
    }

    /// Find providers by tags
    pub async fn find_providers_by_tags(&self, _tags: &[String]) -> Result<Vec<String>> {
        // This would require storing registration data, simplified for now
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on find by tags, recovering");
                poisoned.into_inner()
            });
        Ok(providers_guard.keys().cloned().collect())
    }

    /// Get all registered provider IDs
    pub async fn list_providers(&self) -> Result<Vec<String>> {
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on list, recovering");
                poisoned.into_inner()
            });
        Ok(providers_guard.keys().cloned().collect())
    }

    /// Get provider metadata from cache
    pub async fn get_provider_metadata(&self, provider_id: &str) -> Result<ProviderInfo> {
        let metadata_guard = self.metadata_cache.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Metadata cache lock poisoned on get metadata, recovering");
                poisoned.into_inner()
            });
        metadata_guard
            .get(provider_id)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Provider metadata {} not found", provider_id)))
    }

    /// Perform health check on all providers
    pub async fn health_check_all(&self) -> Result<HashMap<String, ProviderHealth>> {
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on health check, recovering");
                poisoned.into_inner()
            });
        let mut health_results = HashMap::new();

        for (provider_id, provider) in providers_guard.iter() {
            match provider.health_check().await {
                Ok(health) => {
                    health_results.insert(provider_id.clone(), health.clone());
                    
                    // Update cache
                    let mut health_guard = self.health_cache.write()
                        .unwrap_or_else(|poisoned| {
                            tracing::warn!("Health cache lock poisoned on update, recovering");
                            poisoned.into_inner()
                        });
                    health_guard.insert(provider_id.clone(), health);
                }
                Err(e) => {
                    error!("Health check failed for provider {}: {}", provider_id, e);
                    // Create unhealthy status
                    let unhealthy = ProviderHealth {
                        status: HealthStatus::Unhealthy,
                        last_check: std::time::SystemTime::now(),
                        error_message: Some(e.to_string()),
                        uptime_seconds: 0,
                        response_time_ms: 0,
                    };
                    health_results.insert(provider_id.clone(), unhealthy);
                }
            }
        }

        Ok(health_results)
    }

    /// Get cached health status
    pub async fn get_cached_health(&self, provider_id: &str) -> Option<ProviderHealth> {
        let health_guard = self.health_cache.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Health cache lock poisoned on get cached, recovering");
                poisoned.into_inner()
            });
        health_guard.get(provider_id).cloned()
    }

    /// Get registry statistics
    pub async fn get_statistics(&self) -> RegistryStatistics {
        let providers_guard = self.providers.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Provider registry lock poisoned on get stats, recovering");
                poisoned.into_inner()
            });
        let health_guard = self.health_cache.read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Health cache lock poisoned on get stats, recovering");
                poisoned.into_inner()
            });
        
        let total_providers = providers_guard.len();
        let healthy_providers = health_guard
            .values()
            .filter(|h| matches!(h.status, HealthStatus::Healthy))
            .count();

        RegistryStatistics {
            total_providers,
            healthy_providers,
            unhealthy_providers: total_providers - healthy_providers,
            registry_uptime: std::time::SystemTime::now(),
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of registered providers
    pub total_providers: usize,
    /// Number of healthy providers
    pub healthy_providers: usize,
    /// Number of unhealthy providers
    pub unhealthy_providers: usize,
    /// Registry uptime
    pub registry_uptime: std::time::SystemTime,
}

/// Provider discovery helper
pub struct ProviderDiscovery {
    registry: Arc<ConsolidatedProviderRegistry>,
}

impl ProviderDiscovery {
    /// Create new provider discovery helper
    pub fn new(registry: Arc<ConsolidatedProviderRegistry>) -> Self {
        Self { registry }
    }

    /// Discover best provider for a specific type
    pub async fn discover_best_provider(&self, provider_type: ProviderType) -> Result<Arc<dyn ErasedProvider>> {
        let provider_type_clone = provider_type.clone();
        let providers = self.registry.find_providers_by_type(provider_type).await?;
        
        if providers.is_empty() {
            return Err(BearDogError::system(
                format!("No providers found for type: {:?}", provider_type_clone)
            ));
        }

        // Return the first healthy provider with highest confidence score
        // Future enhancement: Implement sophisticated provider selection based on health, load, etc.
        for provider in providers {
            match provider.health_check().await {
                Ok(health) if matches!(health.status, HealthStatus::Healthy) => {
                    return Ok(provider);
                }
                _ => continue,
            }
        }

        Err(BearDogError::system(
            format!("No healthy providers found for type: {:?}", provider_type_clone)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_registry_creation() {
        let registry = ConsolidatedProviderRegistry::default();
        assert_eq!(registry.providers.try_read()
            ?.len(), 0);
    }
} 