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


/// Universal adapter for ecosystem integration

// Use universal capabilities from canonical types
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{debug, info, warn};
/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub url: String,
    pub health_check_path: String,
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub capabilities: Vec<String>,
    pub priority: u32, // Lower number = higher priority
}
/// Service discovery result
#[derive(Debug, Clone)]
pub struct DiscoveredService {
    pub endpoint: ServiceEndpoint,
    pub health_status: ServiceHealthStatus,
    pub last_health_check: Instant,
/// Service health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
/// Universal adapter configuration}


pub struct UniversalAdapterConfig {
    pub service_endpoints: HashMap<String, ServiceEndpoint>,
    pub default_timeout_ms: u64,}


impl Default for UniversalAdapterConfig {}


    fn default() -> Self {
        Self {
            service_endpoints: HashMap::new(),
            default_timeout_ms: 30000,
        }
    }
/// Production universal adapter implementation
#[allow(dead_code)]
pub struct ProductionUniversalAdapter {
    config: UniversalAdapterConfig,
    http_client: reqwest::Client,
    discovered_services: HashMap<String, DiscoveredService>,
    service_discovery_cache_ttl: Duration,}


impl ProductionUniversalAdapter {}


    #[must_use]
    pub fn new(config: UniversalAdapterConfig) -> Self {
            config,
            http_client: reqwest::Client::new(),
            discovered_services: HashMap::new(),
            service_discovery_cache_ttl: Duration::from_secs(300), // 5 minute cache
    /// Discover available services for a given capability}


    async fn discover_services(
        &mut self,
        capability: &str,
    ) -> BearDogResult<Vec<DiscoveredService>> {
        let mut discovered = Vec::new();
        // Check configured service endpoints
        for (service_name, endpoint) in &self.config.service_endpoints {
            if endpoint.capabilities.contains(&capability.to_string()) {
                // Check if we have a cached health status
                let needs_health_check = match self.discovered_services.get(service_name) {
                    Some(cached) => {
                        cached.last_health_check.elapsed() > self.service_discovery_cache_ttl
                    }
                    None => true,
                };
                let health_status = if needs_health_check {
                    self.check_service_health(endpoint).await
                } else {
                    self.discovered_services
                        .get(service_name)
                        .map_or(ServiceHealthStatus::Unknown, |s| s.health_status.clone())
                let discovered_service = DiscoveredService {
                    endpoint: endpoint.clone(),
                    health_status: health_status.clone(),
                    last_health_check: Instant::now(),
                // Update cache
                self.discovered_services
                    .insert(service_name.clone(), discovered_service.clone());
                // Only include healthy or degraded services
                if matches!(
                    health_status,
                    ServiceHealthStatus::Healthy | ServiceHealthStatus::Degraded
                ) {
                    discovered.push(discovered_service);
                }
            }
        // Sort by priority (lower number = higher priority) and health status
        discovered.sort_by(|a, b| match (&a.health_status, &b.health_status) {
            (ServiceHealthStatus::Healthy, ServiceHealthStatus::Degraded) => {
                std::cmp::Ordering::Less
            (ServiceHealthStatus::Degraded, ServiceHealthStatus::Healthy) => {
                std::cmp::Ordering::Greater
            _ => a.endpoint.priority.cmp(&b.endpoint.priority),
        });
        Ok(discovered)
    /// Check health of a specific service
    async fn check_service_health(&self, endpoint: &ServiceEndpoint) -> ServiceHealthStatus {
        let health_url = format!("{}{}", endpoint.url, endpoint.health_check_path);
        let request_timeout = Duration::from_millis(endpoint.timeout_ms);
        match timeout(request_timeout, self.http_client.get(&health_url).send()).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    ServiceHealthStatus::Healthy
                } else if response.status().is_server_error() {
                    ServiceHealthStatus::Degraded
                    ServiceHealthStatus::Unhealthy
            Ok(Err(_)) => ServiceHealthStatus::Unhealthy,
            Err(_) => ServiceHealthStatus::Unhealthy, // Timeout
    /// Route request to the best available service
    async fn route_request(
        &self,
        services: &[DiscoveredService],
        request_path: &str,
        request_body: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        if services.is_empty() {
            return Err(BearDogError::ServiceUnavailable {
                service: "universal_adapter".to_string(),
                message: "No healthy services available for this capability".to_string(),
            });
        // Try services in priority order
        for service in services {
            let url = format!("{}{}", service.endpoint.url, request_path);
            let request_timeout = Duration::from_millis(service.endpoint.timeout_ms);
            for attempt in 0..service.endpoint.retry_count {
                debug!("Attempting request to {} (attempt {})", url, attempt + 1);
                match timeout(
                    request_timeout,
                    self.http_client.post(&url).json(&request_body).send(),
                )
                .await
                {
                    Ok(Ok(response)) => {
                        if response.status().is_success() {
                            match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    info!("Successfully routed request to {}", url);
                                    return Ok(json);
                                }
                                Err(e) => {
                                    warn!("Failed to parse response from {}: {}", url, e);
                                    continue;
                            }
                        } else {
                            warn!("Service {} returned status: {}", url, response.status());
                        }
                    Ok(Err(e)) => {
                        warn!("Request to {} failed: {}", url, e);
                    Err(_) => {
                        warn!("Request to {} timed out", url);
                // Wait before retry (exponential backoff)
                if attempt < service.endpoint.retry_count - 1 {
                    tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
        Err(BearDogError::ServiceUnavailable {
            service: "universal_adapter".to_string(),
            message: "All service attempts failed".to_string(),
        })
// AI trait implementation disabled during canonical modernization
// impl UniversalAdapter for ProductionUniversalAdapter {
//     async fn handle_capability_request(
//         &self,
//         request: CapabilityRequest,
//     ) -> BearDogResult<CapabilityResponse> {
//         // Implementation disabled
//         Ok(CapabilityResponse::default())
//     }
// }
    // AI service routing disabled during canonical modernization
    // async fn route_to_ai_service(&self, ai_request: ExternalAIRequest) -> BearDogResult<serde_json::Value> {
    //     // Implementation disabled
    //     Ok(serde_json::Value::Null)
    // }
    // async fn route_to_compute_service(&self, compute_request: ComputeRequest) -> Result<serde_json::Value, SystemError> {
