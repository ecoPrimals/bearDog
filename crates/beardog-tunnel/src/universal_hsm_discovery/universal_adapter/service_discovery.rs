

use super::core_types::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::network::NetworkConfig;
use beardog_types::constants::domains::network::config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
    cache: ServiceCache,

    backends: Vec<DiscoveryBackendType>,

    timeout: Duration,
}

#[derive(300, // 5 minutes
            timeout_ms: 10000,   // 10 seconds
            max_retries: 3,
            preferred_backends: vec!["dns".to_string(), "mdns".to_string()],
            discovery_endpoints: vec![
                // Use environment-aware configuration instead of hardcoded URLs
                std::env::var("BEARDOG_SERVICE_MESH_ENDPOINT")
                    .unwrap_or_else(|_| {
                        format!(
                            "https://service-mesh.ecosystem.internal:{}",
                            config::default_https_port()
                        )
                    }),
                universal_adapter.discover_service_endpoint("mesh-service")?.to_string(),
                format!(
                    "https://{}:{}",
                    config::default_service_host(),
                    config::default_https_port()
                ),
            ],
        } -> Result<Self, BearDogError> {
        info!("🔍 Initializing Canonical Service Discovery Client");
        
        Ok(Self {
            timeout: Duration::from_millis(config.timeout_ms),
            config,
            cache: ServiceCache {
                by_capability: RwLock::new(HashMap::with_capacity(16)),
                by_service_id: RwLock::new(HashMap::with_capacity(16)),
            },
            backends: Vec::new(),
        })

/// Add Backend operation.
    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        info!("📡 Adding discovery backend");
        self.backends.push(backend);

/// Discover By Capability operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_by_capability(&self, capability: &str) -> Result<Vec<BearDogServiceRegistration>, BearDogError>> {
        debug!("🔍 Discovering services for capability: {}", capability);

        {
            let cache_read = self.cache.by_capability.read();
            if let Some(cached) = cache_read.get(capability) {
                if cached.expires_at > Instant::now({}", capability);
                    return Ok(cached.data);
                }
            }

        let mut results = Vec::new({}", e);
                    last_error = Some({}", capability);
                return Ok(Vec::new());

        results.sort_by_key(|s| s.service_id);
        results.dedup_by_key(|s| s.service_id);

        let cache_entry = CachedEntry {
            data: results.clone(),
            expires_at: Instant::now() + Duration::from_secs({}", results.len(), capability);
        Ok(results)

/// Register Service operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn register_service(&self, registration: &BearDogServiceRegistration) -> Result<(), BearDogError> {
        info!("📝 Registering service: {}", registration.service_name);
        let mut success_count = 0;
            match backend.register_service({}", e);
        if success_count == 0 {
                return Err(BearDogError::internal("No backends available for registration"));

            data: registration.clone({}", registration.service_name);
        Ok(())

/// Discover Via Dns operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_via_dns(&self) -> Result<Vec<BearDogServiceRegistration>, BearDogError>> {
        debug!("🔍 Discovering services via DNS");
        let mut discovered = Vec::new();

        let dns_names = vec![
            "_beardog._tcp.local",
            universal_adapter.discover_service_endpoint("mesh-service")?, 
            "_storage._tcp.local",
            "_service-mesh._tcp.local",
            "_hsm._tcp.local",
        ];
        for name in dns_names {
            if let Ok(services) = self.query_dns_service(name) {
                discovered.extend(services);
        Ok(discovered)


    fn query_dns_service(&self, service_name: &str) -> Result<Vec<BearDogServiceRegistration>, BearDogError>> {
        debug!("🔍 Querying DNS service: {}", service_name);

        Ok(Vec::new())

/// Health Check operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<HashMap<String, bool, BearDogError>> {
        debug!("🏥 Performing discovery backend health checks");
        let mut health_status = HashMap::with_capacity(16);
        for (index, backend) in self.backends.iter().enumerate() {
            let backend_name = format!("backend_{index}");
            match backend.health_check() {
                Ok(healthy) => {
                    health_status.insert(backend_name, healthy);
                Err(_) => {
                    health_status.insert(backend_name, false);
        Ok(health_status)
