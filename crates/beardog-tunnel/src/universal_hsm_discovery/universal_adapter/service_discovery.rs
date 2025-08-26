

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::NetworkConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug)]
pub struct ServiceDiscoveryClient {

    config: DiscoveryConfig,

    cache: ServiceCache,

    backends: Vec<DiscoveryBackendType>,

    timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

impl Default for DiscoveryConfig {}

    fn default() -> Self {
        Self {
            cache_ttl_secs: 300, // 5 minutes
            timeout_ms: 10000,   // 10 seconds
            max_retries: 3,
            preferred_backends: vec!["dns".to_string(), "mdns".to_string()],
            discovery_endpoints: vec![
                "https://service-mesh.ecosystem.internal:8443".to_string(),
                "https://songbird.ecosystem.internal:8443".to_string(),
                "https://localhost:8443".to_string(),
            ],
            enable_dns_discovery: true,
            enable_mdns_discovery: true,
        }
    }

struct ServiceCache {

    by_capability: RwLock<HashMap<String, CachedEntry<Vec<BearDogServiceRegistration>>>>,

    by_service_id: RwLock<HashMap<Uuid, CachedEntry<BearDogServiceRegistration>>>,

#[derive(Debug, Clone)]
struct CachedEntry<T> {

    data: T,

    expires_at: Instant,

pub struct BearDogServiceRegistration {

    pub service_id: Uuid,

    pub service_name: String,

    pub service_type: String,

    pub version: String,

    pub endpoint: String,

    pub capabilities: Vec<String>,

    pub metadata: HashMap<String, String>,

    pub health_status: ServiceHealthStatus,

    pub registered_at: chrono::DateTime<chrono::Utc>,

    pub last_seen: chrono::DateTime<chrono::Utc>,

pub enum ServiceHealthStatus {

    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

pub struct NetworkService {

    pub id: Uuid,
    pub name: String,

    pub address: String,

    pub port: u16,

    pub protocol: String,

pub trait DiscoveryBackend: Send + Sync {

    async fn discover_by_capability(&self, capability: &str) -> BearDogResult<Vec<BearDogServiceRegistration>>;

    async fn register_service(&self, registration: &BearDogServiceRegistration) -> BearDogResult<()>;

    async fn unregister_service(&self, service_id: Uuid) -> BearDogResult<()>;

    async fn health_check(&self) -> BearDogResult<bool>;}

impl ServiceDiscoveryClient {

    pub fn new(config: DiscoveryConfig) -> BearDogResult<Self> {
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

    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        info!("📡 Adding discovery backend");
        self.backends.push(backend);

    pub async fn discover_by_capability(&self, capability: &str) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Discovering services for capability: {}", capability);

        {
            let cache_read = self.cache.by_capability.read().await;
            if let Some(cached) = cache_read.get(capability) {
                if cached.expires_at > Instant::now() {
                    debug!("📋 Returning cached services for capability: {}", capability);
                    return Ok(cached.data.clone());
                }
            }

        let mut results = Vec::new();
        let mut last_error = None;
        for backend in &self.backends {
            match backend.discover_by_capability(capability).await {
                Ok(mut services) => {
                    debug!("✅ Backend discovered {} services", services.len());
                    results.append(&mut services);
                Err(e) => {
                    warn!("⚠️ Backend discovery failed: {}", e);
                    last_error = Some(e);
        if results.is_empty() {
            if let Some(error) = last_error {
                return Err(error);
            } else {
                debug!("🔍 No services found for capability: {}", capability);
                return Ok(Vec::new());

        results.sort_by_key(|s| s.service_id);
        results.dedup_by_key(|s| s.service_id);

        let cache_entry = CachedEntry {
            data: results.clone(),
            expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
        };
            let mut cache_write = self.cache.by_capability.write().await;
            cache_write.insert(capability.to_string(), cache_entry);
        info!("🎯 Discovered {} services for capability: {}", results.len(), capability);
        Ok(results)

    pub async fn register_service(&self, registration: &BearDogServiceRegistration) -> BearDogResult<()> {
        info!("📝 Registering service: {}", registration.service_name);
        let mut success_count = 0;
            match backend.register_service(registration).await {
                Ok(()) => {
                    success_count += 1;
                    debug!("✅ Service registered with backend");
                    warn!("⚠️ Failed to register with backend: {}", e);
        if success_count == 0 {
                return Err(BearDogError::internal("No backends available for registration"));

            data: registration.clone(),
            let mut cache_write = self.cache.by_service_id.write().await;
            cache_write.insert(registration.service_id, cache_entry);
        info!("✅ Service registration completed: {}", registration.service_name);
        Ok(())

    pub async fn discover_via_dns(&self) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Discovering services via DNS");
        let mut discovered = Vec::new();

        let dns_names = vec![
            "_beardog._tcp.local",
            "_songbird._tcp.local", 
            "_nestgate._tcp.local",
            "_service-mesh._tcp.local",
            "_hsm._tcp.local",
        ];
        for name in dns_names {
            if let Ok(services) = self.query_dns_service(name).await {
                discovered.extend(services);
        Ok(discovered)

    async fn query_dns_service(&self, service_name: &str) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Querying DNS service: {}", service_name);

        Ok(Vec::new())

    pub async fn health_check(&self) -> BearDogResult<HashMap<String, bool>> {
        debug!("🏥 Performing discovery backend health checks");
        let mut health_status = HashMap::with_capacity(16);
        for (index, backend) in self.backends.iter().enumerate() {
            let backend_name = format_args!("backend_{}", index).to_string();
            match backend.health_check().await {
                Ok(healthy) => {
                    health_status.insert(backend_name, healthy);
                Err(_) => {
                    health_status.insert(backend_name, false);
        Ok(health_status)
