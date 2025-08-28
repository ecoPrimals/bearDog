use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::*;
use crate::{{EcosystemError}};

#[derive(Debug)]
pub enum DiscoveryBackendType {
    Consul(ConsulBackend),
    Etcd(EtcdBackend),
    Kubernetes(KubernetesBackend),
    Static(StaticBackend),
}

#[derive(Debug)]
pub struct ServiceDiscoveryClient {

    config: DiscoveryConfig,

    cache: ServiceCache,

    backends: Vec<DiscoveryBackendType>,
}

#[derive(Debug, Clone)]

impl ServiceDiscoveryClient {

    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            cache: ServiceCache {
                by_capability: HashMap::with_capacity(16),
                by_service_id: HashMap::with_capacity(16),
            },
            backends: Vec::new(),
        }
    }

    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        self.backends.push(backend);

    pub async fn discover_by_capability(
        &mut self,
        capability: &str,
    ) -> Result<Vec<UniversalServiceRegistration, BearDogError>> {

        if let Some(cached) = self.cache.by_capability.get(capability) {
            if cached.expires_at > Instant::now() {
                return Ok(cached.data.clone());
            }

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

        results.sort_by_key(|s| s.service_id);
        results.dedup_by_key(|s| s.service_id);

        let cache_entry = CachedEntry {
            data: results.clone(),
            expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
        };
        self.cache
            .by_capability
            .insert(capability.to_string(), cache_entry);
        Ok(results)

    pub async fn get_service(
        service_id: uuid::Uuid,
    ) -> Result<Option<UniversalServiceRegistration, BearDogError>> {
        if let Some(cached) = self.cache.by_service_id.get(&service_id) {
                return Ok(Some(cached.data.clone()));
            if let Ok(Some(service)) = backend.get_service(service_id).await {

                let cache_entry = CachedEntry {
                    data: service.clone(),
                    expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
                };
                self.cache.by_service_id.insert(service_id, cache_entry);
                return Ok(Some(service));
        Ok(None)

    pub async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> Result<(), BearDogError> {
        let mut success_count = 0;
            match backend.register_service(registration).await {
                Ok(_) => success_count += 1,
                Err(e) => last_error = Some(e),
        if success_count == 0 {
                return Err(EcosystemError::RegistrationFailed {
                    reason: "No backends available".to_string(),
                });
        Ok(())

    pub fn clear_cache(&mut self) {
        self.cache.by_capability.clear();
        self.cache.by_service_id.clear();

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

pub struct CacheStats {

    pub capability_entries: usize,

    pub service_entries: usize,

    pub expired_entries: usize,

pub trait DiscoveryBackend: Send + Sync + std::fmt::Debug {

    async fn discover_by_capability(
    ) -> Result<Vec<UniversalServiceRegistration, BearDogError>>;

    async fn get_service(
    ) -> Result<Option<UniversalServiceRegistration, BearDogError>>;

    async fn register_service(
    ) -> Result<(), BearDogError>;

    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;

pub struct InMemoryDiscoveryBackend {

    registry: tokio::sync::RwLock<super::capability_registry::CapabilityRegistry>,}

impl InMemoryDiscoveryBackend {

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

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
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
