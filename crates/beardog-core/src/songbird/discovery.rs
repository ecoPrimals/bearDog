

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

pub struct ServiceMeshDiscoveryClient {

    client: HttpClient,

    timeout: Duration,

    discovery_endpoints: Vec<String>,
}
impl ServiceMeshDiscoveryClient {

    pub fn new() -> BearDogResult<Self> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("`BearDog`-Discovery/1.0")
            .build()
            .map_err(|e| {
                BearDogError::internal(format!("Failed to create discovery client: {e}"))
            })?;

        let discovery_endpoints = Self::load_discovery_endpoints();
        Ok(Self {
            client,
            timeout: Duration::from_secs(10),
            discovery_endpoints,
        })
    }

    fn load_discovery_endpoints() -> Vec<String> {

        if let Ok(endpoints_str) = std::env::var("BEARDOG_DISCOVERY_ENDPOINTS") {
            return endpoints_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        vec![
            "https://service-mesh.ecosystem.internal:8443".to_string(),
            "https://songbird.ecosystem.internal:8443".to_string(),
            "https://localhost:8443".to_string(),
        ]

    async fn discover_via_dns(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Discovering service meshes via DNS");
        let mut discovered = Vec::new();

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

    async fn query_dns_service(&self, service_name: &str) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Querying DNS for service: {}", service_name);
        
        let mut discovered_services = Vec::new();

        let dns_patterns = vec![
            format_args!("{}.local", service_name).to_string(),
            format_args!("_{}._tcp.local", service_name).to_string(),
            format_args!("{}.mesh", service_name).to_string(),
            format_args!("{}.service.consul", service_name).to_string(),
            format_args!("{}.default.svc.cluster.local", service_name).to_string(), // Kubernetes
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

    async fn resolve_dns_pattern(&self, pattern: &str) -> BearDogResult<Option<ServiceMeshInfo>> {
        use std::net::{IpAddr, Ipv4Addr};

        match tokio::net::lookup_host(pattern).await {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    let service_info = ServiceMeshInfo {
                        service_id: uuid::Uuid::new_v4().to_string(),
                        service_name: self.extract_service_name(pattern),
                        endpoint: format_args!("http://{}:8080", addr.ip().to_string()), // Common service mesh port
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

                Ok(None)
            }
        }
    }

    fn extract_service_name(&self, pattern: &str) -> String {

        if let Some(name) = pattern.split('.').next() {
            name.trim_start_matches('_').to_string()
        } else {
            "unknown".to_string()
        }
    }

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

    async fn discover_via_multicast(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        debug!("🔍 Discovering service meshes via multicast");

        warn!("Multicast discovery not yet implemented");

#[allow(async_fn_in_trait)]
impl ServiceMeshDiscovery for ServiceMeshDiscoveryClient {

    async fn discover(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        info!("🔍 Starting comprehensive service mesh discovery");
        let mut all_discovered = Vec::new();

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

        all_discovered.dedup_by(|a, b| a.endpoint == b.endpoint);
        info!(
            "✅ Discovery complete. Found {} service mesh(es)",
            all_discovered.len()
        );
        Ok(all_discovered)

    async fn health_check(&self, mesh: &ServiceMeshInfo) -> BearDogResult<ServiceHealth> {
        let health_url = format_args!("{}/health", mesh.endpoint).to_string();
        match self
            .get(&health_url)
            .timeout(Duration::from_secs(5))
        {
            Ok(response) => {
                if response.status().is_success() {

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

    async fn rank_meshes(
        &self,
        mut meshes: Vec<ServiceMeshInfo>,
    ) -> Result<Vec<ServiceMeshInfo, SecurityError>> {
        debug!("📊 Ranking {} service meshes by preference", meshes.len());

        for mesh in &mut meshes {
            mesh.health = self.health_check(mesh).await.unwrap_or(ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Discovery failed".to_string()),
            });
            mesh.last_health_check = Some(chrono::Utc::now());

        meshes.sort_by(|a, b| {

            let health_cmp =             // Compare by health status (healthy services come first)
            match (a.health.is_healthy, b.health.is_healthy) {
                (true, true) => {

                    a.health.response_time_ms.partial_cmp(&b.health.response_time_ms)
                        .unwrap_or(std::cmp::Ordering::Equal)
                (true, false) => std::cmp::Ordering::Less,    // healthy comes first
                (false, true) => std::cmp::Ordering::Greater, // unhealthy comes last
                (false, false) => {

            };
            if health_cmp != std::cmp::Ordering::Equal {
                return health_cmp;

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
