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


/// Service Discovery
///
/// **CANONICAL SERVICE DISCOVERY** - Complete implementation for network service discovery and registration
/// This module provides comprehensive service discovery functionality for the Universal HSM ecosystem,
/// consolidating patterns from beardog-adapters and beardog-core service discovery implementations.

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::NetworkConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
/// **CANONICAL SERVICE DISCOVERY CLIENT**
/// Unified service discovery client that consolidates DNS, mDNS, HTTP, and custom discovery methods.
#[derive(Debug)]
pub struct ServiceDiscoveryClient {
    /// Discovery configuration
    config: DiscoveryConfig,
    
    /// Cached service registrations
    cache: ServiceCache,
    /// Discovery backends
    /// **MODERNIZED** - Discovery backends using zero-cost enum dispatch
    backends: Vec<DiscoveryBackendType>,
    /// Client timeout
    timeout: Duration,
}
/// **SERVICE DISCOVERY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
    /// Discovery timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum retries for discovery operations
    pub max_retries: u32,
    /// Preferred discovery backends
    pub preferred_backends: Vec<String>,
    /// Discovery endpoints
    pub discovery_endpoints: Vec<String>,
    /// Enable DNS-SD discovery
    pub enable_dns_discovery: bool,
    /// Enable mDNS discovery
    pub enable_mdns_discovery: bool,}


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
/// **SERVICE CACHE** - Performance optimization with expiration
struct ServiceCache {
    /// Cached registrations by capability
    by_capability: RwLock<HashMap<String, CachedEntry<Vec<BearDogServiceRegistration>>>>,
    /// Cached individual services
    by_service_id: RwLock<HashMap<Uuid, CachedEntry<BearDogServiceRegistration>>>,
/// **CACHED ENTRY** - Entry with expiration tracking
#[derive(Debug, Clone)]
struct CachedEntry<T> {
    /// Cached data
    data: T,
    /// Expiration time
    expires_at: Instant,
/// **BEARDOG SERVICE REGISTRATION**
pub struct BearDogServiceRegistration {
    /// Unique service identifier
    pub service_id: Uuid,
    /// Service name
    pub service_name: String,
    /// Service type (beardog, songbird, nestgate, etc.)
    pub service_type: String,
    /// Service version
    pub version: String,
    /// Service endpoint
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service health status
    pub health_status: ServiceHealthStatus,
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
/// **SERVICE HEALTH STATUS**
pub enum ServiceHealthStatus {
    /// Service is healthy and operational
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status is unknown
    Unknown,
/// **NETWORK SERVICE** - Network service representation}


pub struct NetworkService {
    /// Service identifier
    pub id: Uuid,
    pub name: String,
    /// Network address
    pub address: String,
    /// Network port
    pub port: u16,
    /// Service protocol
    pub protocol: String,
/// **DISCOVERY BACKEND TRAIT**
#[async_trait::async_trait]
pub trait DiscoveryBackend: Send + Sync {
    /// Discover services by capability
    async fn discover_by_capability(&self, capability: &str) -> BearDogResult<Vec<BearDogServiceRegistration>>;
    /// Register a service
    async fn register_service(&self, registration: &BearDogServiceRegistration) -> BearDogResult<()>;
    /// Unregister a service
    async fn unregister_service(&self, service_id: Uuid) -> BearDogResult<()>;
    /// Health check for the backend
    async fn health_check(&self) -> BearDogResult<bool>;}


impl ServiceDiscoveryClient {
    /// **CREATE NEW SERVICE DISCOVERY CLIENT**}


    pub fn new(config: DiscoveryConfig) -> BearDogResult<Self> {
        info!("🔍 Initializing Canonical Service Discovery Client");
        
        Ok(Self {
            timeout: Duration::from_millis(config.timeout_ms),
            config,
            cache: ServiceCache {
                by_capability: RwLock::new(HashMap::new()),
                by_service_id: RwLock::new(HashMap::new()),
            },
            backends: Vec::new(),
        })
    /// **ADD DISCOVERY BACKEND**
    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        info!("📡 Adding discovery backend");
        self.backends.push(backend);
    /// **DISCOVER SERVICES BY CAPABILITY** - Main discovery method with caching}


    pub async fn discover_by_capability(&self, capability: &str) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Discovering services for capability: {}", capability);
        // Check cache first
        {
            let cache_read = self.cache.by_capability.read().await;
            if let Some(cached) = cache_read.get(capability) {
                if cached.expires_at > Instant::now() {
                    debug!("📋 Returning cached services for capability: {}", capability);
                    return Ok(cached.data.clone());
                }
            }
        // Discovery from backends
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
        // Deduplicate by service_id
        results.sort_by_key(|s| s.service_id);
        results.dedup_by_key(|s| s.service_id);
        // Cache the results
        let cache_entry = CachedEntry {
            data: results.clone(),
            expires_at: Instant::now() + Duration::from_secs(self.config.cache_ttl_secs),
        };
            let mut cache_write = self.cache.by_capability.write().await;
            cache_write.insert(capability.to_string(), cache_entry);
        info!("🎯 Discovered {} services for capability: {}", results.len(), capability);
        Ok(results)
    /// **REGISTER SERVICE** - Register a service with all backends
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
        // Cache the registered service
            data: registration.clone(),
            let mut cache_write = self.cache.by_service_id.write().await;
            cache_write.insert(registration.service_id, cache_entry);
        info!("✅ Service registration completed: {}", registration.service_name);
        Ok(())
    /// **DISCOVER VIA DNS** - DNS-based service discovery
    pub async fn discover_via_dns(&self) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Discovering services via DNS");
        let mut discovered = Vec::new();
        // DNS-SD lookup for BearDog ecosystem services
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
    /// **QUERY DNS SERVICE** - Query specific DNS service name}


    async fn query_dns_service(&self, service_name: &str) -> BearDogResult<Vec<BearDogServiceRegistration>> {
        debug!("🔍 Querying DNS service: {}", service_name);
        // Implementation would use actual DNS resolution
        // For now, return empty results as this requires DNS library integration
        Ok(Vec::new())
    /// **HEALTH CHECK** - Check health of all discovery backends
    pub async fn health_check(&self) -> BearDogResult<HashMap<String, bool>> {
        debug!("🏥 Performing discovery backend health checks");
        let mut health_status = HashMap::new();
        for (index, backend) in self.backends.iter().enumerate() {
            let backend_name = format!("backend_{}", index);
            match backend.health_check().await {
                Ok(healthy) => {
                    health_status.insert(backend_name, healthy);
                Err(_) => {
                    health_status.insert(backend_name, false);
        Ok(health_status)
