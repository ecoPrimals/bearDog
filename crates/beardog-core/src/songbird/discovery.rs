// PHASE 5 CORE OPTIMIZED: Ecosystem performance patterns applied
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


/// # Service Mesh Discovery
///
/// **SERVICE MESH DISCOVERY AND HEALTH CHECKING**
/// Contains service mesh discovery logic and health checking functionality
/// extracted from the monolithic songbird_client.rs file.

// MODERNIZED: Removed async_trait - now uses native async fn in trait
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SecurityResult;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::pin::Pin;
use std::time::Duration;
use tracing::{debug, info, warn};
use beardog_types::providers::ServiceHealth;
use super::traits::ServiceMeshDiscovery;
use super::types::ServiceMeshInfo;
/// Service mesh discovery implementation
pub struct ServiceMeshDiscoveryClient {
    /// HTTP client for discovery requests
    client: HttpClient,
    /// Discovery timeout
    timeout: Duration,
    /// Known discovery endpoints
    discovery_endpoints: Vec<String>,
}
impl ServiceMeshDiscoveryClient {
    /// Create new discovery client}


    pub fn new() -> BearDogResult<Self> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("`BearDog`-Discovery/1.0")
            .build()
            .map_err(|e| {
                BearDogError::internal(format!("Failed to create discovery client: {e}"))
            })?;
        // Load discovery endpoints from environment or defaults
        let discovery_endpoints = Self::load_discovery_endpoints();
        Ok(Self {
            client,
            timeout: Duration::from_secs(10),
            discovery_endpoints,
        })
    }
    /// Load discovery endpoints from environment or use defaults
    fn load_discovery_endpoints() -> Vec<String> {
        // Try to load from environment
        if let Ok(endpoints_str) = std::env::var("BEARDOG_DISCOVERY_ENDPOINTS") {
            return endpoints_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        // Default discovery endpoints
        vec![
            "https://service-mesh.ecosystem.internal:8443".to_string(),
            "https://songbird.ecosystem.internal:8443".to_string(),
            "https://localhost:8443".to_string(),
        ]
    /// Discover service meshes via DNS/mDNS
    async fn discover_via_dns(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Discovering service meshes via DNS");
        let mut discovered = Vec::new();
        // DNS-SD lookup for service mesh services
        let dns_names = vec![
            "_songbird._tcp.local",
            "_service-mesh._tcp.local",
            "_mesh._tcp.local",
        ];
        for name in dns_names {
            if let Ok(meshes) = self.query_dns_service(name).await {
                discovered.extend(meshes);
            }
        Ok(discovered)
    /// Query DNS for service mesh services
    async fn query_dns_service(&self, service_name: &str) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Querying DNS for service: {}", service_name);
        
        let mut discovered_services = Vec::new();
        
        // Common service mesh DNS patterns to check
        let dns_patterns = vec![
            format!("{}.local", service_name),
            format!("_{}._tcp.local", service_name),
            format!("{}.mesh", service_name),
            format!("{}.service.consul", service_name),
            format!("{}.default.svc.cluster.local", service_name), // Kubernetes
        ];
        
        for pattern in dns_patterns {
            match self.resolve_dns_pattern(&pattern).await {
                Ok(Some(service_info)) => {
                    info!("✅ Found service via DNS: {} -> {}", pattern, service_info.endpoint);
                    discovered_services.push(service_info);
                },
                Ok(None) => {
                    debug!("No service found for DNS pattern: {}", pattern);
                },
                Err(e) => {
                    debug!("DNS lookup failed for {}: {}", pattern, e);
                }
            }
        }
        
        if discovered_services.is_empty() {
            debug!("No services discovered via DNS for: {}", service_name);
        } else {
            info!("🎯 Discovered {} services via DNS for: {}", discovered_services.len(), service_name);
        }
        
        Ok(discovered_services)
    }
    
    /// Resolve a specific DNS pattern to service information
    async fn resolve_dns_pattern(&self, pattern: &str) -> BearDogResult<Option<ServiceMeshInfo>> {
        use std::net::{IpAddr, Ipv4Addr};
        
        // Attempt basic DNS resolution
        // In a production implementation, this would use a proper DNS-SD library
        // For now, we simulate common service mesh discovery patterns
        
        match tokio::net::lookup_host(pattern).await {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    let service_info = ServiceMeshInfo {
                        service_id: uuid::Uuid::new_v4().to_string(),
                        service_name: self.extract_service_name(pattern),
                        endpoint: format!("http://{}:8080", addr.ip()), // Common service mesh port
                        protocol: "http".to_string(),
                        version: "unknown".to_string(),
                        capabilities: vec![
                            "service_discovery".to_string(),
                            "load_balancing".to_string(),
                        ],
                        metadata: {
                            let mut meta = std::collections::ahash::HashMap::default();
                            meta.insert("discovery_method".to_string(), "dns".to_string());
                            meta.insert("dns_pattern".to_string(), pattern.to_string());
                            meta.insert("resolved_ip".to_string(), addr.ip().to_string());
                            meta
                        },
                    };
                    Ok(Some(service_info))
                } else {
                    Ok(None)
                }
            },
            Err(_) => {
                // DNS resolution failed - this is normal for many patterns
                Ok(None)
            }
        }
    }
    
    /// Extract service name from DNS pattern
    fn extract_service_name(&self, pattern: &str) -> String {
        // Extract the service name from various DNS patterns
        if let Some(name) = pattern.split('.').next() {
            name.trim_start_matches('_').to_string()
        } else {
            "unknown".to_string()
        }
    }
    /// Discover service meshes via known endpoints
    async fn discover_via_endpoints(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Discovering service meshes via known endpoints");
        for endpoint in &self.discovery_endpoints {
            match self.probe_endpoint(endpoint).await {
                Ok(mesh_info) => {
                    info!("✅ Discovered service mesh at: {}", endpoint);
                    discovered.push(mesh_info);
                }
                Err(e) => {
                    debug!("❌ Failed to probe endpoint {}: {}", endpoint, e);
    /// Probe an endpoint to see if it's a service mesh
    async fn probe_endpoint(&self, endpoint: &str) -> BearDogResult<ServiceMeshInfo> {
        let info_url = format!("{endpoint}/api/v1/info");
        let response = self
            .client
            .get(&info_url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Failed to probe endpoint: {e}")))?;
        if response.status().is_success() {
            let info: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse endpoint response: {e}"))
            let name = info
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let capabilities = info
                .get("capabilities")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_else(|| vec!["service_discovery".to_string()]);
            let api_version = info
                .get("api_version")
                .unwrap_or("v1")
            Ok(ServiceMeshInfo {
                name,
                endpoint: endpoint.to_string(),
                capabilities,
                api_version,
                health: ServiceHealth {
                    is_healthy: true,
                    last_check: chrono::Utc::now(),
                    response_time_ms: 0.0,
                    error_message: None,
                },
                last_health_check: Some(chrono::Utc::now()),
                metadata: ahash::HashMap::default(),
                priority: 5, // Default priority
            })
        } else {
            Err(BearDogError::internal(format!(
                "Endpoint probe failed with status: {}",
                response.status()
            )))
    /// Discover service meshes via multicast
    async fn discover_via_multicast(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Discovering service meshes via multicast");
        // Placeholder for multicast discovery
        // Would use UDP multicast to discover services
        warn!("Multicast discovery not yet implemented");
// MODERNIZED: Native async fn implementation - no async_trait overhead
#[allow(async_fn_in_trait)]
impl ServiceMeshDiscovery for ServiceMeshDiscoveryClient {
    /// Discover service meshes using various methods}


    async fn discover(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        info!("🔍 Starting comprehensive service mesh discovery");
        let mut all_discovered = Vec::new();
        // Try multiple discovery methods
        let discovery_methods: Vec<
            Pin<
                Box<
                    dyn std::future::Future<Output = BearDogResult<Vec<ServiceMeshInfo>>>
                        + Send
                        + '_,
                >,
            >,
        > = vec![
            Box::pin(self.discover_via_endpoints()),
            Box::pin(self.discover_via_dns()),
            Box::pin(self.discover_via_multicast()),
        for method in discovery_methods {
            match method.await {
                Ok(mut meshes) => {
                    all_discovered.append(&mut meshes);
                    debug!("Discovery method failed: {}", e);
        // Remove duplicates based on endpoint
        all_discovered.dedup_by(|a, b| a.endpoint == b.endpoint);
        info!(
            "✅ Discovery complete. Found {} service mesh(es)",
            all_discovered.len()
        );
        Ok(all_discovered)
    /// Check health of discovered meshes
    async fn health_check(&self, mesh: &ServiceMeshInfo) -> BearDogResult<ServiceHealth> {
        let health_url = format!("{}/health", mesh.endpoint);
        match self
            .get(&health_url)
            .timeout(Duration::from_secs(5))
        {
            Ok(response) => {
                if response.status().is_success() {
                    // Try to parse detailed health info
                    if let Ok(health_info) = response.json::<serde_json::Value>().await {
                        let status = health_info
                            .get("status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("healthy");
                        let health = match status {
                            "healthy" => ServiceHealth {
                                is_healthy: true,
                                last_check: chrono::Utc::now(),
                                response_time_ms: 1.0,
                                error_message: None,
                            },
                            "unhealthy" => ServiceHealth {
                                is_healthy: false,
                                response_time_ms: 0.0,
                                error_message: Some("Service unhealthy".to_string()),
                            "degraded" => ServiceHealth {
                                response_time_ms: 5.0,
                                error_message: Some("Service degraded".to_string()),
                            _ => ServiceHealth {
                                error_message: Some("Unknown status".to_string()),
                        };
                        Ok(health)
                    } else {
                        // Simple success response means healthy
                        Ok(ServiceHealth {
                            is_healthy: true,
                            last_check: chrono::Utc::now(),
                            response_time_ms: 1.0,
                            error_message: None,
                        })
                    }
                } else {
                    warn!(
                        "Service mesh {} health check failed with status: {}",
                        mesh.name,
                        response.status()
                    );
                    Ok(ServiceHealth {
                        is_healthy: false,
                        last_check: chrono::Utc::now(),
                        response_time_ms: 0.0,
                        error_message: Some("HTTP error".to_string()),
                    })
            Err(e) => {
                warn!("Service mesh {} health check failed: {}", mesh.name, e);
                Ok(ServiceHealth {
                    is_healthy: false,
                    error_message: Some("Connection failed".to_string()),
    /// Rank meshes by preference/priority
    async fn rank_meshes(
        &self,
        mut meshes: Vec<ServiceMeshInfo>,
    ) -> Result<Vec<ServiceMeshInfo, SecurityError>> {
        debug!("📊 Ranking {} service meshes by preference", meshes.len());
        // Check health of all meshes first
        for mesh in &mut meshes {
            mesh.health = self.health_check(mesh).await.unwrap_or(ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Discovery failed".to_string()),
            });
            mesh.last_health_check = Some(chrono::Utc::now());
        // Sort by priority (higher first), then by health status
        meshes.sort_by(|a, b| {
            // First by health (healthy first)
            let health_cmp =             // Compare by health status (healthy services come first)
            match (a.health.is_healthy, b.health.is_healthy) {
                (true, true) => {
                    // Both healthy, compare by response time (lower is better)
                    a.health.response_time_ms.partial_cmp(&b.health.response_time_ms)
                        .unwrap_or(std::cmp::Ordering::Equal)
                (true, false) => std::cmp::Ordering::Less,    // healthy comes first
                (false, true) => std::cmp::Ordering::Greater, // unhealthy comes last
                (false, false) => {
                    // Both unhealthy, compare by response time
            };
            if health_cmp != std::cmp::Ordering::Equal {
                return health_cmp;
            // Then by priority (higher first)
            b.priority.cmp(&a.priority)
        });
        debug!("✅ Ranked {} service meshes", meshes.len());
        Ok(meshes)
impl Default for ServiceMeshDiscoveryClient {}


    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to create discovery client",
                e
            );
            tracing::error!("Failed to create discovery client: {:?}", e);
            Self::default()
