

use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub url: String,
    pub health_check_path: String,
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub capabilities: Vec<String>,
    pub priority: u32, // Lower number = higher priority
}

#[derive(Debug, Clone)]
pub struct DiscoveredService {
    pub endpoint: ServiceEndpoint,
    pub health_status: ServiceHealthStatus,
    pub last_health_check: Instant,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,

pub struct UniversalAdapterConfig {
    pub service_endpoints: HashMap<String, ServiceEndpoint>,
    pub default_timeout_ms: u64,}

impl Default for UniversalAdapterConfig {}

    fn default() -> Self {
        Self {
            service_endpoints: ahash::HashMap::default(),
            default_timeout_ms: 30000,
        }
    }

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
            discovered_services: ahash::HashMap::default(),
            service_discovery_cache_ttl: Duration::from_secs(300), // 5 minute cache

    async fn discover_services(
        &mut self,
        capability: &str,
    ) -> BearDogResult<Vec<DiscoveredService>> {
        let mut discovered = Vec::new();

        for (service_name, endpoint) in &self.config.service_endpoints {
            if endpoint.capabilities.contains(&capability.to_string()) {

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

                self.discovered_services
                    .insert(service_name.clone(), discovered_service.clone());

                if matches!(
                    health_status,
                    ServiceHealthStatus::Healthy | ServiceHealthStatus::Degraded
                ) {
                    discovered.push(discovered_service);
                }
            }

        discovered.sort_by(|a, b| match (&a.health_status, &b.health_status) {
            (ServiceHealthStatus::Healthy, ServiceHealthStatus::Degraded) => {
                std::cmp::Ordering::Less
            (ServiceHealthStatus::Degraded, ServiceHealthStatus::Healthy) => {
                std::cmp::Ordering::Greater
            _ => a.endpoint.priority.cmp(&b.endpoint.priority),
        });
        Ok(discovered)

    async fn check_service_health(&self, endpoint: &ServiceEndpoint) -> ServiceHealthStatus {
        let health_url = format_args!("{}{}", endpoint.url, endpoint.health_check_path).to_string();
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

        for service in services {
            let url = format_args!("{}{}", service.endpoint.url, request_path).to_string();
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

                if attempt < service.endpoint.retry_count - 1 {
                    tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
        Err(BearDogError::ServiceUnavailable {
            service: "universal_adapter".to_string(),
            message: "All service attempts failed".to_string(),
        })

