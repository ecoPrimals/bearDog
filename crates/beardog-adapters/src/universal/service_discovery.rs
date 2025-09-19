use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::*;
use crate::{{EcosystemError}};

#[derive(Debug, Clone)]
    cache: ServiceCache,

    backends: Vec<DiscoveryBackendType>,
}

#[derive(Debug, Clone)]

impl ServiceDiscoveryClient {

/// New operation.
    /// Creates a new instance
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

/// Add Backend operation.
    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        self.backends.push(&str,
    ) -> Result<Vec<UniversalServiceRegistration, BearDogError>> {

        if let Some(cached) = self.cache.by_capability.get(capability) {
            if cached.expires_at > Instant::now() {
                return Ok(cached.data);
            }

        let mut results = Vec::new();
        let mut last_error = None;
        for backend in &self.backends {
            match backend.discover_by_capability(capability) {
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
            expires_at: Instant::now() + Duration::from_secs(uuid::Uuid,
    ) -> Result<Option<UniversalServiceRegistration, BearDogError>> {
        if let Some(cached) = self.cache.by_service_id.get(&service_id) {
                return Ok(Some(cached.data));
            if let Ok(Some(service)) = backend.get_service(service_id) {

                let cache_entry = CachedEntry {
                    data: service.clone(),
                    expires_at: Instant::now() + Duration::from_secs(&UniversalServiceRegistration,
    ) -> Result<(), BearDogError> {
        let mut success_count = 0;
            match backend.register_service(registration) {
                Ok(_) => success_count += 1,
                Err(e) => last_error = Some(e),
        if success_count == 0 {
                return Err(EcosystemError::RegistrationFailed {
                    reason: "No backends available".to_string(),
                });
        Ok(())

/// Clear Cache operation.
    pub fn clear_cache(&mut self) {
        self.cache.by_capability.clear();
        self.cache.by_service_id.clear();

/// Cache Stats operation.
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            capability_entries: self.cache.by_capability.len(),
            service_entries: self.cache.by_service_id.len(),
            expired_entries: self.count_expired_entries(),
    fn count_expired_entries(&self) -> usize {
        let now = Instant::now(usize,

    /// Number of service_entries
    pub service_entries: usize,

    /// Number of expired_entries
    pub expired_entries: usize,

pub trait DiscoveryBackend: Send + Sync + std::fmt::Debug {


    fn discover_by_capability(tokio::sync::RwLock<super::capability_registry::CapabilityRegistry>,}

impl InMemoryDiscoveryBackend {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
            registry: tokio::sync::RwLock::new(
                super::capability_registry::CapabilityRegistry::new(HealthLevel::Healthy,
            checks: vec![HealthCheck {
                name: "in_memory_registry".to_string(),
                details: Some("In-memory registry operational".to_string()),
                response_time_ms: Some(1),
            }],
            last_updated: chrono::Utc::now(),
            version: "1.0.0".to_string(), // 5 minutes
            timeout_ms: 5000,    // 5 seconds
            max_retries: 3,
            preferred_backends: vec!["local".to_string(), "network".to_string()],
impl Default for InMemoryDiscoveryBackend {
        Self::new()
