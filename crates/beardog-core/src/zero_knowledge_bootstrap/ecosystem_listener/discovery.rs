// SPDX-License-Identifier: AGPL-3.0-or-later

//! Discovery protocol helpers: mDNS, HTTP polling, environment, service mesh.

use super::env::EcosystemListenerEnvInputs;
use super::types::{PrimalAnnouncement, UNKNOWN_ENDPOINT_URL};
use crate::ecosystem::primal_types::{
    DiscoveredPrimal, PrimalMetadata, PrimalMetrics, UniversalEndpoint,
};
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{
    CapabilityType, ServiceCapabilityType, UniversalCapability,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub(super) async fn listen_mdns_announcements(
    env: &EcosystemListenerEnvInputs,
) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
    debug!("Listening for mDNS primal announcements");

    let announcements = Vec::new();

    // Check if mDNS discovery is enabled via environment
    if env.mdns_discovery_enabled {
        // In a real implementation, this would use mdns-sd or similar
        // For now, we simulate by checking for known service patterns
        debug!("mDNS discovery enabled, scanning for services...");

        // Check for local services advertising BearDog capabilities
        if let Ok(response) = tokio::process::Command::new("avahi-browse")
            .args(["-t", "_beardog._tcp"])
            .output()
            .await
            && response.status.success()
        {
            let output = String::from_utf8_lossy(&response.stdout);
            for line in output.lines() {
                if line.contains("beardog ") {
                    debug!("Found potential BearDog service via mDNS: {}", line);
                }
            }
        }
    }

    Ok(announcements)
}

/// Poll HTTP discovery endpoints
pub(super) async fn poll_http_discovery(
    env: &EcosystemListenerEnvInputs,
) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
    debug!("Polling HTTP discovery endpoints");

    let mut announcements = Vec::new();

    let discovery_endpoints = match env.discovery_endpoints() {
        Ok(endpoints) => endpoints,
        Err(e) => {
            debug!("HTTP discovery skipped: {e}");
            return Ok(announcements);
        }
    };

    for endpoint in discovery_endpoints {
        debug!(%endpoint, "Checking discovery endpoint");

        // Attempt HTTP discovery request with timeout
        match tokio::time::timeout(
            std::time::Duration::from_secs(env.http_discovery_timeout_secs),
            std::future::ready(make_discovery_request(&endpoint)),
        )
        .await
        {
            Ok(Ok(discovered)) => {
                announcements.extend(discovered);
            }
            Ok(Err(e)) => {
                debug!("Discovery endpoint {} failed: {}", endpoint, e);
            }
            Err(_) => {
                debug!("Discovery endpoint {} timed out", endpoint);
            }
        }
    }

    Ok(announcements)
}

pub(super) fn check_environment_announcements() -> Result<Vec<PrimalAnnouncement>, BearDogError> {
    check_environment_announcements_with_lookup(|key| std::env::var(key))
}

/// Tests and injected maps: lookup function instead of reading global environment.
pub(super) fn check_environment_announcements_with_lookup<G>(
    mut get_var: G,
) -> Result<Vec<PrimalAnnouncement>, BearDogError>
where
    G: FnMut(&str) -> Result<String, std::env::VarError>,
{
    debug!("Checking environment for primal announcements");

    let mut announcements = Vec::new();

    // Check for primal endpoint environment variables
    let env_vars = [
        "BEARDOG_COMPUTE_ENDPOINT",
        "BEARDOG_MESH_ENDPOINT",
        "BEARDOG_AI_ENDPOINT",
        "BEARDOG_STORAGE_ENDPOINT",
    ];

    for var in &env_vars {
        if let Ok(endpoint) = get_var(var) {
            debug!(var, %endpoint, "Found primal endpoint in environment");

            // Create announcement from environment variable
            let capability = match *var {
                "BEARDOG_COMPUTE_ENDPOINT" => ServiceCapabilityType::ComputeIntelligence,
                "BEARDOG_MESH_ENDPOINT" => ServiceCapabilityType::ServiceMesh,
                "BEARDOG_AI_ENDPOINT" => ServiceCapabilityType::DistributedIntelligence,
                "BEARDOG_STORAGE_ENDPOINT" => ServiceCapabilityType::DataStorage,
                _ => continue,
            };

            // Create announcement from environment-discovered service
            let announcement = PrimalAnnouncement {
                primal_id: format!("env-discovered-{}", var.to_lowercase()),
                capabilities: vec![capability],
                endpoints: vec![UniversalEndpoint {
                    url: endpoint,
                    protocols: vec!["HTTP".to_string()],
                    auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
                    security_config:
                        crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
                }],
                metadata: PrimalMetadata {
                    display_name: Some(format!("Environment-Discovered-{var}")),
                    version: "unknown".to_string(),
                    protocol_versions: vec!["1.0".to_string()],
                    security_attestations: vec![],
                    custom_fields: HashMap::new(),
                    capabilities: vec![],
                    dependencies: vec![],
                    supported_protocols: vec!["http".to_string()],
                    health_check_endpoint: "/health".to_string(),
                    metrics_endpoint: "/metrics".to_string(),
                },
                announcement_timestamp: std::time::SystemTime::now(),
                source_protocol: "environment ".to_string(),
            };

            announcements.push(announcement);
        }
    }

    Ok(announcements)
}

#[cfg(test)]
pub(super) fn check_environment_announcements_for_test(
    vars: &HashMap<String, String>,
) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
    check_environment_announcements_with_lookup(|k| {
        vars.get(k).cloned().ok_or(std::env::VarError::NotPresent)
    })
}

/// Discover primals via service mesh
pub(super) fn discover_service_mesh_primals() -> Vec<PrimalAnnouncement> {
    debug!("Discovering primals via service mesh");

    // Minimal implementation - production deployments should integrate with service mesh
    // like Istio, Linkerd, or Consul Connect for automatic service discovery
    Vec::new() // No service mesh integration yet - returns empty
}

/// Process primal announcement
pub(super) async fn process_primal_announcement(
    announcement: PrimalAnnouncement,
    discovered_primals: &Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    discovered_capabilities: &Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
) -> Result<(), BearDogError> {
    info!(
        primal_id = %announcement.primal_id,
        "Processing primal announcement"
    );

    // Validate announcement
    if announcement.primal_id.is_empty() {
        warn!("Invalid announcement: empty primal ID");
        return Ok(());
    }

    // Get endpoint or create placeholder for announcement-only primals
    let endpoint = announcement.endpoints.first().cloned().unwrap_or_else(|| {
        warn!(
            primal_id = %announcement.primal_id,
            "No endpoints provided for primal, using placeholder (announcement-only mode)"
        );
        UniversalEndpoint {
            url: UNKNOWN_ENDPOINT_URL.to_string(),
            protocols: vec![],
            auth_requirements: crate::ecosystem::primal_types::AuthRequirements::default(),
            security_config: crate::ecosystem::primal_types::EndpointSecurityConfig::default(),
        }
    });

    let discovered_primal = DiscoveredPrimal {
        primal_id: announcement.primal_id.clone(),
        capabilities: announcement.capabilities.clone(),
        endpoint,
        metadata: announcement.metadata.clone(),
        discovered_at: announcement.announcement_timestamp,
        metrics: PrimalMetrics {
            response_times: crate::ecosystem::primal_types::ResponseTimeMetrics::default(),
            availability: 1.0, // Assume available until proven otherwise
            load_metrics: crate::ecosystem::primal_types::LoadMetrics::default(),
            error_rates: crate::ecosystem::primal_types::ErrorRateMetrics::default(),
        },
    };

    // Check for sovereignty violations (hardcoded primal references)
    let primal_id_lower = discovered_primal.primal_id.to_lowercase();
    if primal_id_lower.contains("hardcoded")
        || primal_id_lower.contains("legacy")
        || primal_id_lower.contains("deprecated")
    {
        warn!(
            primal_id = %discovered_primal.primal_id,
            "Potential sovereignty violation detected in primal ID"
        );
        warn!(
            "   Each primal should only know itself and discover others through universal adapter"
        );
    }

    // Store discovered primal with capability-based identification
    {
        let mut primals = discovered_primals.write().await;
        primals.insert(
            discovered_primal.primal_id.clone(),
            discovered_primal.clone(),
        );
    }

    // Store discovered capabilities
    {
        let mut capabilities = discovered_capabilities.write().await;
        for capability_type in &announcement.capabilities {
            let universal_capability = UniversalCapability {
                capability_type: CapabilityType::Custom(capability_type.to_string()),
                provider: beardog_types::canonical::capabilities::ProviderInfo {
                    provider_id: announcement.primal_id.clone(),
                    provider_name: format!(
                        "Primal-{}",
                        &announcement.primal_id[..8.min(announcement.primal_id.len())]
                    ),
                    provider_type:
                        beardog_types::canonical::providers_unified::core::ProviderType::Custom(
                            "primal".to_string(),
                        ),
                    version: "1.0".to_string(),
                    region: None,
                },
                endpoint: beardog_types::canonical::capabilities::EndpointConfig {
                    base_url: announcement
                        .endpoints
                        .first()
                        .map_or_else(|| "unknown".to_string(), |e| e.url.clone()),
                    api_version: Some("1.0".to_string()),
                    timeout_ms: 30000,
                    max_retries: 3,
                    circuit_breaker:
                        beardog_types::canonical::capabilities::CircuitBreakerConfig::default(),
                },
                auth_config: beardog_types::canonical::capabilities::AuthConfig {
                    auth_type: beardog_types::canonical::capabilities::AuthType::None,
                    api_key: None,
                    bearer_token: None,
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: beardog_types::canonical::capabilities::HealthStatus::Healthy,
                performance: beardog_types::canonical::capabilities::PerformanceMetrics::default(),
                security_level: beardog_types::canonical::capabilities::SecurityLevel::Standard,
                metadata: HashMap::new(),
            };

            capabilities
                .entry(capability_type.clone())
                .or_insert_with(Vec::new)
                .push(universal_capability);
        }
    }

    info!(
        primal_id = %announcement.primal_id,
        capability_count = announcement.capabilities.len(),
        "Primal announcement processed"
    );

    Ok(())
}

/// Make HTTP discovery request to endpoint
pub(super) fn make_discovery_request(
    endpoint: &str,
) -> Result<Vec<PrimalAnnouncement>, BearDogError> {
    debug!("Making discovery request to: {}", endpoint);

    // Use tokio's HTTP client implementation instead of external dependency
    // This provides a basic HTTP client without adding dependencies

    // Parse the URL
    let url = endpoint
        .parse::<http::Uri>()
        .map_err(|e| BearDogError::network(format!("Invalid discovery endpoint URL: {e}")))?;

    // For HTTP discovery, we expect a JSON response with primal announcements
    // If the endpoint is not accessible, we return empty results rather than failing
    let announcements = attempt_http_request(&url);
    debug!(
        "Successfully discovered {} primals from {}",
        announcements.len(),
        endpoint
    );
    Ok(announcements)
}

/// Attempt HTTP request with basic implementation
const fn attempt_http_request(_uri: &http::Uri) -> Vec<PrimalAnnouncement> {
    // Basic HTTP implementation - in production this would make actual HTTP requests
    // For now, return empty to avoid external dependencies
    // This could be enhanced with tokio's native HTTP capabilities
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_discovery_request_invalid_url_errors() {
        let err = make_discovery_request(":::not-a-uri").expect_err("bad url");
        assert!(err.to_string().contains("Invalid") || err.to_string().contains("discovery"));
    }

    #[test]
    fn discover_service_mesh_returns_empty() {
        assert!(discover_service_mesh_primals().is_empty());
    }
}
