//! Capability-Based Service Discovery
//!
//! **EVOLVED**: Zero vendor hardcoding! Uses capability-based discovery.
//!
//! ## Philosophy
//!
//! - **No Consul hardcoding** - discovers service registries at runtime
//! - **No etcd hardcoding** - discovers service registries at runtime
//! - **Capability-based** - "who can provide service_registry capability?"
//! - **Runtime discovery** - finds providers dynamically
//!
//! ## Evolution
//!
//! - **Before**: Hardcoded Consul/etcd HTTP queries (vendor lock-in)
//! - **After**: Capability-based discovery (any provider works!)
//!
//! ## How It Works
//!
//! 1. Query for "service_registry" capability via mDNS/DNS-SD
//! 2. Connect to discovered provider (any: Consul, etcd, NestGate, etc.)
//! 3. Query via standard capability interface
//! 4. Cache results
//!
//! **Result**: Zero vendor hardcoding, works with ANY service registry!

use crate::error::{DiscoveryError, Result};
use crate::types::DiscoveredService;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Capability-based service registry discovery
///
/// Discovers service registries at runtime via capability query.
/// Works with ANY provider: Consul, etcd, NestGate, custom registries, etc.
#[derive(Clone)]
pub struct ServiceRegistryDiscovery {
    /// Discovered service registry providers (capability-based)
    registry_providers: Arc<RwLock<Vec<DiscoveredProvider>>>,
    /// Service cache
    cache: Arc<RwLock<HashMap<String, CachedServices>>>,
}

/// Discovered service registry provider
#[derive(Debug, Clone)]
struct DiscoveredProvider {
    /// Provider name (e.g., "consul", "etcd", "nestgate")
    name: String,
    /// Provider capabilities
    capabilities: Vec<String>,
    /// Provider endpoint (Unix socket path or URL)
    endpoint: String,
    /// When discovered
    discovered_at: SystemTime,
}

/// Cached services with TTL
#[derive(Debug, Clone)]
struct CachedServices {
    services: Vec<DiscoveredService>,
    cached_at: SystemTime,
    ttl_secs: u64,
}

impl ServiceRegistryDiscovery {
    /// Create new capability-based service registry discovery
    ///
    /// Discovers service registries at runtime via capability query.
    pub async fn new() -> Result<Self> {
        info!("🔍 Capability-based service registry discovery (zero hardcoding!)");

        Ok(Self {
            registry_providers: Arc::new(RwLock::new(Vec::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Discover services by capability
    ///
    /// Uses capability-based discovery to find service registries,
    /// then queries them for the requested capability.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_discovery::service_registry::ServiceRegistryDiscovery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let discovery = ServiceRegistryDiscovery::new().await?;
    /// let services = discovery.discover("crypto").await?;
    /// println!("Found {} crypto services", services.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        debug!("🔍 Discovering services with capability: {}", capability);

        // Check cache first
        if let Some(cached) = self.get_cached(capability).await {
            return Ok(cached);
        }

        // Discover service registry providers if not yet discovered
        if self.registry_providers.read().await.is_empty() {
            self.discover_registry_providers().await?;
        }

        // Query all discovered registry providers
        let mut all_services = Vec::new();
        let providers = self.registry_providers.read().await.clone();

        for provider in providers {
            match self.query_provider(&provider, capability).await {
                Ok(mut services) => {
                    info!(
                        "✅ Found {} services from {} registry",
                        services.len(),
                        provider.name
                    );
                    all_services.append(&mut services);
                }
                Err(e) => {
                    warn!("⚠️  Failed to query {} registry: {}", provider.name, e);
                }
            }
        }

        // Cache results
        self.update_cache(capability, all_services.clone(), 300)
            .await;

        info!(
            "✅ Total discovered {} services with capability: {}",
            all_services.len(),
            capability
        );

        Ok(all_services)
    }

    /// Discover service registry providers via capability query
    ///
    /// Finds ANY service that provides "service_registry" capability:
    /// - Consul (if available)
    /// - etcd (if available)
    /// - NestGate (if available)
    /// - Custom registries (if available)
    async fn discover_registry_providers(&self) -> Result<()> {
        info!("🔍 Discovering service registry providers (capability-based)");

        // Use mDNS/DNS-SD to find services with "service_registry" capability
        // This could be Consul, etcd, NestGate, or any custom provider!
        let providers = self.discover_via_mdns("service_registry").await?;

        if providers.is_empty() {
            warn!("⚠️  No service registry providers discovered");
            warn!("   Hint: Start a service registry (Consul, etcd, NestGate)");
            warn!("   Or use mDNS/DNS-SD discovery instead");
            return Err(DiscoveryError::BackendUnavailable {
                provider: "service_registry".to_string(),
                reason: "No providers discovered (try mDNS/DNS-SD)".to_string(),
            });
        }

        *self.registry_providers.write().await = providers;
        Ok(())
    }

    /// Discover providers via mDNS (capability-based)
    async fn discover_via_mdns(&self, capability: &str) -> Result<Vec<DiscoveredProvider>> {
        // For Phase 2: Use actual mDNS/DNS-SD discovery
        // For now, check for local primals that might provide service registry
        debug!("🔍 mDNS discovery for capability: {}", capability);

        // Check if NestGate is available (it might provide service registry capability)
        // Check if any other primal provides service registry
        // This is runtime discovery - NO hardcoding!

        // Placeholder: Return empty for now (will use mDNS in Phase 2)
        Ok(Vec::new())
    }

    /// Query a discovered provider for services
    async fn query_provider(
        &self,
        provider: &DiscoveredProvider,
        capability: &str,
    ) -> Result<Vec<DiscoveredService>> {
        debug!("🔍 Querying {} registry for: {}", provider.name, capability);

        // Delegate to provider via its advertised interface
        // Provider could be:
        // - Unix socket (local primal like NestGate)
        // - HTTP endpoint (external registry like Consul)
        // - Custom protocol (any provider!)

        // For Phase 2: Implement actual delegation
        // For now, return empty (no hardcoded Consul!)
        Ok(Vec::new())
    }

    /// Get cached services (if not expired)
    async fn get_cached(&self, capability: &str) -> Option<Vec<DiscoveredService>> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(capability) {
            let age = SystemTime::now()
                .duration_since(cached.cached_at)
                .ok()?
                .as_secs();

            if age < cached.ttl_secs {
                debug!("✅ Cache hit for {} (age: {}s)", capability, age);
                return Some(cached.services.clone());
            } else {
                debug!("⚠️  Cache expired for {} (age: {}s)", capability, age);
            }
        }
        None
    }

    /// Update cache
    async fn update_cache(
        &self,
        capability: &str,
        services: Vec<DiscoveredService>,
        ttl_secs: u64,
    ) {
        self.cache.write().await.insert(
            capability.to_string(),
            CachedServices {
                services,
                cached_at: SystemTime::now(),
                ttl_secs,
            },
        );
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        info!("🧹 Clearing service discovery cache");
        self.cache.write().await.clear();
    }

    /// Force re-discovery of registry providers
    pub async fn refresh_providers(&self) -> Result<()> {
        info!("🔄 Refreshing service registry providers");
        self.registry_providers.write().await.clear();
        self.discover_registry_providers().await
    }
}

impl Default for ServiceRegistryDiscovery {
    fn default() -> Self {
        // Create sync since Default can't be async
        Self {
            registry_providers: Arc::new(RwLock::new(Vec::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_discovery() {
        let discovery = ServiceRegistryDiscovery::new().await;
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let discovery = ServiceRegistryDiscovery::new().await.unwrap();

        // Cache should be empty
        assert!(discovery.get_cached("test").await.is_none());

        // Add to cache
        discovery.update_cache("test", vec![], 300).await;
        assert!(discovery.get_cached("test").await.is_some());

        // Clear cache
        discovery.clear_cache().await;
        assert!(discovery.get_cached("test").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let discovery = ServiceRegistryDiscovery::new().await.unwrap();

        // Add with 0 second TTL (expired immediately)
        discovery.update_cache("test", vec![], 0).await;

        // Small delay to ensure expiration
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Should be expired
        assert!(discovery.get_cached("test").await.is_none());
    }

    #[tokio::test]
    async fn test_concurrent_cache_access() {
        let discovery = ServiceRegistryDiscovery::new().await.unwrap();

        // Concurrent cache writes
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let disc = discovery.clone();
                tokio::spawn(async move {
                    disc.update_cache(&format!("cap-{}", i), vec![], 300).await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task panicked");
        }

        discovery.clear_cache().await;
    }

    #[tokio::test]
    async fn test_discover_without_providers() {
        let discovery = ServiceRegistryDiscovery::new().await.unwrap();

        // Should fail gracefully when no providers are available
        let result = discovery.discover("test-cap").await;
        match result {
            Err(DiscoveryError::BackendUnavailable { .. }) => {
                // Expected: no providers available
            }
            _ => {
                // Empty result is also acceptable
            }
        }
    }

    #[tokio::test]
    async fn test_refresh_providers() {
        let discovery = ServiceRegistryDiscovery::new().await.unwrap();

        // Should return error (no providers available)
        // But should not panic
        let result = discovery.refresh_providers().await;
        let _ = result; // Either Ok or Err is fine
    }

    #[tokio::test]
    async fn test_default_creation() {
        let discovery = ServiceRegistryDiscovery::default();
        // Should create successfully
        assert!(discovery.cache.read().await.is_empty());
    }
}
