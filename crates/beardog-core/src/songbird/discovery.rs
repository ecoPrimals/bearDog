

use beardog_errors::BearDogError;
use reqwest::Client as HttpClient;
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
    pub fn new() -> Result<Self, BearDogError> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("BearDog-Discovery/1.0")
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
    }

    async fn discover_via_dns(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
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
        }

        Ok(discovered)
    }

    async fn query_dns_service(&self, service_name: &str) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
        debug!("🔍 Querying DNS service: {}", service_name);
        
        // Mock implementation - in real use this would use DNS queries
        warn!("DNS service discovery not fully implemented - using mock data");
        
        Ok(vec![])
    }

    async fn discover_via_endpoints(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
        debug!("🔍 Discovering service meshes via endpoints");
        let mut discovered = Vec::new();

        for endpoint in &self.discovery_endpoints {
            if let Ok(mesh_info) = self.probe_endpoint(endpoint).await {
                discovered.push(mesh_info);
            }
        }

        Ok(discovered)
    }

    async fn probe_endpoint(&self, endpoint: &str) -> Result<ServiceMeshInfo, BearDogError> {
        debug!("🔍 Probing endpoint: {}", endpoint);
        
        let response = self
            .client
            .get(endpoint)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| {
                BearDogError::internal(format!("Failed to probe endpoint {}: {e}", endpoint))
            })?;

        if response.status().is_success() {
            let info: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse endpoint response: {e}"))
            })?;

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
                .and_then(|v| v.as_str())
                .unwrap_or("v1")
                .to_string();

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
                metadata: std::collections::HashMap::default(),
                priority: 5,
            })
        } else {
            Err(BearDogError::internal(format!(
                "Endpoint probe failed with status: {}",
                response.status()
            )))
        }
    }

    async fn discover_via_multicast(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
        debug!("🔍 Discovering service meshes via multicast");
        warn!("Multicast discovery not yet implemented");
        Ok(Vec::new())
    }
}

#[allow(async_fn_in_trait)]
impl ServiceMeshDiscovery for ServiceMeshDiscoveryClient {
    async fn discover(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
        info!("🔍 Starting comprehensive service mesh discovery");
        let mut all_discovered = Vec::new();

        // Try DNS discovery
        if let Ok(dns_meshes) = self.discover_via_dns().await {
            all_discovered.extend(dns_meshes);
        }

        // Try endpoint discovery
        if let Ok(endpoint_meshes) = self.discover_via_endpoints().await {
            all_discovered.extend(endpoint_meshes);
        }

        // Try multicast discovery
        if let Ok(multicast_meshes) = self.discover_via_multicast().await {
            all_discovered.extend(multicast_meshes);
        }

        // Remove duplicates based on endpoint
        all_discovered.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));
        all_discovered.dedup_by(|a, b| a.endpoint == b.endpoint);

        info!("✅ Discovered {} unique service meshes", all_discovered.len());
        Ok(all_discovered)
    }

    async fn health_check(&self, mesh: &ServiceMeshInfo) -> Result<ServiceHealth, BearDogError> {
        debug!("🏥 Health checking mesh: {}", mesh.name);
        
        let start_time = std::time::Instant::now();
        let health_endpoint = format!("{}/health", mesh.endpoint);
        
        match self.client.get(&health_endpoint).timeout(self.timeout).send().await {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as f64;
                
                if response.status().is_success() {
                    Ok(ServiceHealth {
                        is_healthy: true,
                        last_check: chrono::Utc::now(),
                        response_time_ms: response_time,
                        error_message: None,
                    })
                } else {
                    Ok(ServiceHealth {
                        is_healthy: false,
                        last_check: chrono::Utc::now(),
                        response_time_ms: response_time,
                        error_message: Some(format!("HTTP {}", response.status())),
                    })
                }
            }
            Err(e) => {
                warn!("Service mesh {} health check failed: {}", mesh.name, e);
                Ok(ServiceHealth {
                    is_healthy: false,
                    last_check: chrono::Utc::now(),
                    response_time_ms: self.timeout.as_millis() as f64,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }

    async fn rank_meshes(
        &self,
        mut meshes: Vec<ServiceMeshInfo>,
    ) -> Result<Vec<ServiceMeshInfo>, BearDogError> {
        debug!("📊 Ranking {} service meshes by preference", meshes.len());

        // Update health status for all meshes
        for mesh in &mut meshes {
            mesh.health = self.health_check(mesh).await.unwrap_or(ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Discovery failed".to_string()),
            });
            mesh.last_health_check = Some(chrono::Utc::now());
        }

        // Sort by health status and priority
        meshes.sort_by(|a, b| {
            let health_cmp = match (a.health.is_healthy, b.health.is_healthy) {
                (true, true) => {
                    a.health.response_time_ms.partial_cmp(&b.health.response_time_ms)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => {
                    a.health.response_time_ms.partial_cmp(&b.health.response_time_ms)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
            };
            
            if health_cmp != std::cmp::Ordering::Equal {
                return health_cmp;
            }
            
            b.priority.cmp(&a.priority)
        });

        debug!("✅ Ranked {} service meshes", meshes.len());
        Ok(meshes)
    }
}

impl Default for ServiceMeshDiscoveryClient {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!("Failed to create discovery client: {:?}", e);
            Self {
                discovery_endpoints: vec!["http://localhost:8080".to_string()],
                client: reqwest::Client::new(),
                timeout: Duration::from_secs(10),
            }
        })
    }
}
