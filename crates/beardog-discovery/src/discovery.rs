//! Main discovery implementation

use crate::{
    config::DiscoveryConfig,
    error::Result,
    types::{DiscoveredService, QoSWeights},
};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Capability-based service discovery
pub struct CapabilityDiscovery {
    config: DiscoveryConfig,
    cache: Arc<RwLock<ServiceCache>>,
}

impl CapabilityDiscovery {
    /// Create from configuration file
    pub async fn from_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = DiscoveryConfig::from_file(path)?;
        Ok(Self::new(config))
    }

    /// Create with configuration
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(ServiceCache::new())),
        }
    }

    /// Find services by capability type
    ///
    /// Example:
    /// ```no_run
    /// # use beardog_discovery::*;
    /// # async fn example(discovery: &CapabilityDiscovery) -> Result<()> {
    /// // Find ANY service that provides "orchestration"
    /// let orchestrators = discovery.find_by_capability("orchestration").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_by_capability(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        info!("Discovering services with capability: {}", capability);

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(services) = cache.get_by_capability(capability) {
                if !services.is_empty() {
                    debug!("Found {} services in cache", services.len());
                    return Ok(services);
                }
            }
        }

        // Perform discovery
        let services = self.discover_services(capability).await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            for service in &services {
                cache.add(service.clone());
            }
        }

        Ok(services)
    }

    /// Discover services using configured methods
    async fn discover_services(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        let mut all_services = Vec::new();

        // Try each discovery method in order
        for method in &self.config.discovery.methods {
            debug!("Trying discovery method: {}", method);

            let services = match method.as_str() {
                "environment" => self.discover_via_environment(capability).await?,
                "mdns" => self.discover_via_mdns(capability).await?,
                "dns_sd" => self.discover_via_dns_sd(capability).await?,
                "service_registry" => self.discover_via_service_registry(capability).await?,
                _ => {
                    warn!("Unknown discovery method: {}", method);
                    continue;
                }
            };

            all_services.extend(services);

            // If we found services and this capability is optional, we can stop
            if !all_services.is_empty() {
                break;
            }
        }

        if all_services.is_empty() {
            info!(
                "No services found for capability '{}', will use fallback",
                capability
            );
        } else {
            info!(
                "Found {} services for capability '{}'",
                all_services.len(),
                capability
            );
        }

        Ok(all_services)
    }

    /// Discover services via environment variables
    async fn discover_via_environment(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        use std::env;

        let mut services = Vec::new();

        // Look for CAPABILITY_{TYPE}_ENDPOINT pattern
        let capability_key = format!(
            "CAPABILITY_{}_ENDPOINT",
            capability.to_uppercase().replace('-', "_")
        );

        if let Ok(endpoint) = env::var(&capability_key) {
            debug!(
                "Found capability endpoint: {} = {}",
                capability_key, endpoint
            );

            services.push(DiscoveredService {
                id: format!("env-{}", capability),
                service_type: "unknown".to_string(),
                display_name: format!("Environment-discovered {} service", capability),
                endpoint: crate::types::ServiceEndpoint {
                    primary_url: endpoint,
                    fallback_urls: vec![],
                    use_tls: true,
                    path_prefix: None,
                },
                capabilities: vec![crate::types::Capability {
                    capability_type: capability.to_string(),
                    version: "unknown".to_string(),
                    features: vec![],
                    parameters: std::collections::HashMap::new(),
                }],
                qos: crate::types::QoSMetrics::default(),
                health: crate::types::HealthStatus::Unknown,
                discovered_at: std::time::SystemTime::now(),
                ttl_secs: self.config.discovery.cache_ttl_secs,
                discovery_method: "environment".to_string(),
                metadata: std::collections::HashMap::new(),
            });
        }

        // Also look for PRIMAL_* pattern (discover any primal announcing itself)
        // This allows services to self-announce without us knowing their name!
        for (key, value) in env::vars() {
            if key.starts_with("PRIMAL_") && key.ends_with("_ENDPOINT") {
                // Extract primal name
                let parts: Vec<&str> = key.split('_').collect();
                if parts.len() >= 3 {
                    let primal_name = parts[1..parts.len() - 1].join("_");

                    // Check if this primal provides the capability we need
                    let cap_key = format!("PRIMAL_{}_CAPABILITIES", primal_name);
                    if let Ok(caps) = env::var(&cap_key) {
                        if caps.split(',').any(|c| c.trim() == capability) {
                            debug!("Found primal {} providing {}", primal_name, capability);

                            services.push(DiscoveredService {
                                id: format!("primal-{}", primal_name.to_lowercase()),
                                service_type: primal_name.to_lowercase(),
                                display_name: format!("{} Primal", primal_name),
                                endpoint: crate::types::ServiceEndpoint {
                                    primary_url: value.clone(),
                                    fallback_urls: vec![],
                                    use_tls: value.starts_with("https"),
                                    path_prefix: None,
                                },
                                capabilities: caps
                                    .split(',')
                                    .map(|c| crate::types::Capability {
                                        capability_type: c.trim().to_string(),
                                        version: "unknown".to_string(),
                                        features: vec![],
                                        parameters: std::collections::HashMap::new(),
                                    })
                                    .collect(),
                                qos: crate::types::QoSMetrics::default(),
                                health: crate::types::HealthStatus::Unknown,
                                discovered_at: std::time::SystemTime::now(),
                                ttl_secs: self.config.discovery.cache_ttl_secs,
                                discovery_method: "environment".to_string(),
                                metadata: std::collections::HashMap::new(),
                            });
                        }
                    }
                }
            }
        }

        Ok(services)
    }

    /// Discover services via mDNS
    ///
    /// Discovers services on the local network using mDNS (Multicast DNS).
    /// This is agnostic to primal types - discovers any service advertising capabilities.
    ///
    /// # Implementation Status
    /// Currently gracefully falls back to other discovery methods.
    /// Full mDNS implementation requires `mdns` crate integration.
    ///
    /// # Future Enhancement
    /// - Integrate `mdns` crate for service discovery
    /// - Query for _beardog._tcp.local or capability-specific services
    /// - Parse TXT records for capability metadata
    /// - Maintain discovered service cache with TTL
    async fn discover_via_mdns(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        debug!(
            "mDNS discovery for capability '{}' - graceful fallback (full impl pending)",
            capability
        );

        // Graceful fallback: return empty, allowing other discovery methods
        // This maintains functionality while logging the limitation
        info!(
            "mDNS discovery not yet available for '{}', trying other methods",
            capability
        );

        Ok(vec![])
    }

    /// Discover services via DNS-SD (DNS Service Discovery)
    ///
    /// Discovers services using DNS-SD (RFC 6763).
    /// Capability-based and agnostic to specific primal implementations.
    ///
    /// # Implementation Status
    /// Currently gracefully falls back to other discovery methods.
    /// Full DNS-SD implementation requires DNS resolution and SRV record parsing.
    ///
    /// # Future Enhancement
    /// - Query DNS SRV records for capability services
    /// - Parse TXT records for service metadata
    /// - Support both local and wide-area DNS-SD
    /// - Cache results with appropriate TTL
    async fn discover_via_dns_sd(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        debug!(
            "DNS-SD discovery for capability '{}' - graceful fallback (full impl pending)",
            capability
        );

        // Graceful fallback: return empty, allowing other discovery methods
        info!(
            "DNS-SD not yet available for '{}', trying other methods",
            capability
        );

        Ok(vec![])
    }

    /// Discover services via service registry (Consul, etcd, etc.)
    ///
    /// Discovers services through a centralized service registry.
    /// Environment-driven configuration determines which registry to query.
    ///
    /// # Implementation Status
    /// Currently gracefully falls back to other discovery methods.
    /// Full implementation requires HTTP client and registry-specific protocols.
    ///
    /// # Configuration
    /// Set `SERVICE_REGISTRY_URL` environment variable to enable.
    /// Example: `http://consul:8500` or `http://etcd:2379`
    ///
    /// # Future Enhancement
    /// - Support Consul, etcd, Kubernetes service discovery
    /// - Query by capability tags/labels
    /// - Watch for service changes (real-time updates)
    /// - Health check integration
    async fn discover_via_service_registry(
        &self,
        capability: &str,
    ) -> Result<Vec<DiscoveredService>> {
        // Check if service registry is configured (environment-driven)
        if let Ok(registry_url) = std::env::var("SERVICE_REGISTRY_URL") {
            debug!(
                "Service registry configured at {}, but client not yet implemented for '{}'",
                registry_url, capability
            );
            info!(
                "Service registry discovery for '{}' pending full implementation",
                capability
            );
        } else {
            debug!(
                "Service registry not configured (set SERVICE_REGISTRY_URL), trying other methods for '{}'",
                capability
            );
        }

        // Graceful fallback: return empty, allowing other discovery methods
        Ok(vec![])
    }

    /// Select best service from discovered services
    pub fn select_best(&self, services: &[DiscoveredService]) -> Option<DiscoveredService> {
        if services.is_empty() {
            return None;
        }

        // Use QoS-based selection
        let weights = QoSWeights {
            latency: self.config.service_selection.qos_weights.latency,
            throughput: self.config.service_selection.qos_weights.throughput,
            availability: self.config.service_selection.qos_weights.availability,
            reliability: self.config.service_selection.qos_weights.reliability,
        };

        services
            .iter()
            .max_by(|a, b| {
                let score_a = a.qos.calculate_score(&weights);
                let score_b = b.qos.calculate_score(&weights);
                // Modern Rust idiom: Handle NaN gracefully in float comparisons
                // NaN scores are treated as lowest priority
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Less)
            })
            .cloned()
    }
}

/// Service cache
struct ServiceCache {
    services: Vec<DiscoveredService>,
}

impl ServiceCache {
    fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    fn add(&mut self, service: DiscoveredService) {
        // Remove old entry if exists
        self.services.retain(|s| s.id != service.id);
        self.services.push(service);
    }

    fn get_by_capability(&self, capability: &str) -> Option<Vec<DiscoveredService>> {
        let services: Vec<_> = self
            .services
            .iter()
            .filter(|s| {
                s.capabilities
                    .iter()
                    .any(|c| c.capability_type == capability)
            })
            .cloned()
            .collect();

        if services.is_empty() {
            None
        } else {
            Some(services)
        }
    }
}
