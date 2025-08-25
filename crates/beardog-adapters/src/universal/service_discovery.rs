// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Service Discovery for Universal Ecosystem Integration
///
/// This module handles dynamic service discovery in the ecosystem following
/// the Universal Primal Architecture Standard

// MODERNIZED: Removed async_trait - now uses native async fn in trait
use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::*;
use crate::{EcosystemError, EcosystemResult};
// CANONICAL IMPORT: use beardog_types::config::UnifiedDiscoveryConfig;
/// **MODERNIZED** - Discovery backend types using enum dispatch
#[derive(Debug)]
pub enum DiscoveryBackendType {
    Consul(ConsulBackend),
    Etcd(EtcdBackend),
    Kubernetes(KubernetesBackend),
    Static(StaticBackend),
}

/// **ZERO-COST** - Universal service discovery client with enum dispatch
#[derive(Debug)]
pub struct ServiceDiscoveryClient {
    /// Discovery configuration
    config: DiscoveryConfig,
    /// Cached service registrations
    cache: ServiceCache,
    /// Discovery backends - zero-cost enum dispatch
    backends: Vec<DiscoveryBackendType>,
}
/// Service discovery configuration
#[derive(Debug, Clone)]
// MIGRATED: DiscoveryConfig -> use beardog_types::config::UnifiedDiscoveryConfig;


impl ServiceDiscoveryClient {
    /// Create new service discovery client}


    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            cache: ServiceCache {
                by_capability: HashMap::new(),
                by_service_id: HashMap::new(),
            },
            backends: Vec::new(),
        }
    }
    /// Add a discovery backend
    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        self.backends.push(backend);
    /// Discover services by capability with caching}


    pub async fn discover_by_capability(
        &mut self,
        capability: &str,
    ) -> EcosystemResult<Vec<UniversalServiceRegistration>> {
        // Check cache first
        if let Some(cached) = self.cache.by_capability.get(capability) {
            if cached.expires_at > Instant::now() {
                return Ok(cached.data.clone());
            }
        // Discovery from backends
        let mut results = Vec::new();
        let mut last_error = None;
        for backend in &self.backends {
            match backend.discover_by_capability(capability).await {
                Ok(mut services) => {
                    results.append(&mut services);
                }
                Err(e) => {
                    last_error = Some(e);
        if results.is_empty() {
            if let Some(error) = last_error {
                return Err(error);
            } else {
                return Ok(Vec::new());
        // Deduplicate by service_id
        results.sort_by_key(|s| s.service_id);
        results.dedup_by_key(|s| s.service_id);
        // Cache the results
        let cache_entry = CachedEntry {
            data: results.clone(),
            expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
        };
        self.cache
            .by_capability
            .insert(capability.to_string(), cache_entry);
        Ok(results)
    /// Get service by ID with caching
    pub async fn get_service(
        service_id: uuid::Uuid,
    ) -> EcosystemResult<Option<UniversalServiceRegistration>> {
        if let Some(cached) = self.cache.by_service_id.get(&service_id) {
                return Ok(Some(cached.data.clone()));
            if let Ok(Some(service)) = backend.get_service(service_id).await {
                // Cache the result
                let cache_entry = CachedEntry {
                    data: service.clone(),
                    expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
                };
                self.cache.by_service_id.insert(service_id, cache_entry);
                return Ok(Some(service));
        Ok(None)
    /// Register a service with all backends
    pub async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> EcosystemResult<()> {
        let mut success_count = 0;
            match backend.register_service(registration).await {
                Ok(_) => success_count += 1,
                Err(e) => last_error = Some(e),
        if success_count == 0 {
                return Err(EcosystemError::RegistrationFailed {
                    reason: "No backends available".to_string(),
                });
        Ok(())
    /// Clear cache entries
    pub fn clear_cache(&mut self) {
        self.cache.by_capability.clear();
        self.cache.by_service_id.clear();
    /// Get cache statistics}


    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            capability_entries: self.cache.by_capability.len(),
            service_entries: self.cache.by_service_id.len(),
            expired_entries: self.count_expired_entries(),
    fn count_expired_entries(&self) -> usize {
        let now = Instant::now();
        let capability_expired = self
            .cache
            .values()
            .filter(|entry| entry.expires_at <= now)
            .count();
        let service_expired = self
            .by_service_id
        capability_expired + service_expired
/// Cache statistics
pub struct CacheStats {
    /// Number of cached capability entries
    pub capability_entries: usize,
    /// Number of cached service entries
    pub service_entries: usize,
    /// Number of expired entries
    pub expired_entries: usize,
/// Discovery backend trait

pub trait DiscoveryBackend: Send + Sync + std::fmt::Debug {
    /// Discover services by capability}


    async fn discover_by_capability(
    ) -> EcosystemResult<Vec<UniversalServiceRegistration>>;
    /// Get specific service by ID
    async fn get_service(
    ) -> EcosystemResult<Option<UniversalServiceRegistration>>;
    /// Register a service
    async fn register_service(
    ) -> EcosystemResult<()>;
    /// Health check for the backend
    async fn health_check(&self) -> EcosystemResult<HealthStatus>;
/// In-memory discovery backend for testing
pub struct InMemoryDiscoveryBackend {
    /// Local service registry
    registry: tokio::sync::RwLock<super::capability_registry::CapabilityRegistry>,}


impl InMemoryDiscoveryBackend {
    /// Create new in-memory backend}


    pub fn new() -> Self {
            registry: tokio::sync::RwLock::new(
                super::capability_registry::CapabilityRegistry::new(),
            ),
impl DiscoveryBackend for InMemoryDiscoveryBackend {
        let registry = self.registry.read().await;
        registry.find_by_capability(capability).await
        let all_services = registry.get_all_services().await?;
        Ok(all_services
            .into_iter()
            .find(|s| s.service_id == service_id))
        let mut registry = self.registry.write().await;
        registry.register_service(registration.clone()).await}


    async fn health_check(&self) -> EcosystemResult<HealthStatus> {
        Ok(HealthStatus {
            status: HealthLevel::Healthy,
            checks: vec![HealthCheck {
                name: "in_memory_registry".to_string(),
                status: HealthLevel::Healthy,
                details: Some("In-memory registry operational".to_string()),
                response_time_ms: Some(1),
            }],
            last_updated: chrono::Utc::now(),
            version: "1.0.0".to_string(),
        })
impl Default for DiscoveryConfig {}


    fn default() -> Self {
            cache_ttl_secs: 300, // 5 minutes
            timeout_ms: 5000,    // 5 seconds
            max_retries: 3,
            preferred_backends: vec!["local".to_string(), "network".to_string()],
impl Default for InMemoryDiscoveryBackend {
        Self::new()
