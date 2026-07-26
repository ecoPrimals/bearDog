// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// Injected environment-based discovery (same rules as [`crate::capability_env`]).
pub type EnvironmentDiscoveryFn = dyn Fn(&str, u64) -> Vec<DiscoveredService> + Send + Sync;

/// Capability-based service discovery
pub struct CapabilityDiscovery {
    config: DiscoveryConfig,
    cache: Arc<RwLock<ServiceCache>>,
    service_registry_url: Option<String>,
    discover_env: Arc<EnvironmentDiscoveryFn>,
}

impl CapabilityDiscovery {
    /// Create from configuration file
    ///
    /// # Errors
    ///
    /// Propagates errors from [`DiscoveryConfig::from_file`].
    pub fn from_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = DiscoveryConfig::from_file(path)?;
        Ok(Self::new(config))
    }

    /// Create with configuration
    #[must_use]
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(ServiceCache::new())),
            service_registry_url: None,
            discover_env: Arc::new(|cap, ttl| {
                crate::capability_env::discovered_services_from_environment_from_env(cap, ttl)
            }),
        }
    }

    /// Override the optional service registry URL used by the `service_registry` discovery method.
    #[must_use]
    pub fn with_service_registry_url(mut self, url: Option<String>) -> Self {
        self.service_registry_url = url;
        self
    }

    /// Replace environment-based discovery (for tests and custom injection).
    #[must_use]
    pub fn with_environment_discovery<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, u64) -> Vec<DiscoveredService> + Send + Sync + 'static,
    {
        self.discover_env = Arc::new(f);
        self
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
    ///
    /// # Errors
    ///
    /// Returns an error when underlying discovery transports fail.
    pub async fn find_by_capability(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        info!("Discovering services with capability: {}", capability);

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(services) = cache.get_by_capability(capability)
                && !services.is_empty()
            {
                debug!("Found {} services in cache", services.len());
                return Ok(services);
            }
        }

        // Perform discovery
        let services = self.discover_services(capability);

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
    fn discover_services(&self, capability: &str) -> Vec<DiscoveredService> {
        let mut all_services = Vec::new();

        // Try each discovery method in order
        for method in &self.config.discovery.methods {
            debug!("Trying discovery method: {}", method);

            let services = match method.as_str() {
                "environment" => self.discover_via_environment(capability),
                "service_registry" => self.discover_via_service_registry(capability),
                "mdns" | "dns_sd" => {
                    debug!("Skipping {method} — orchestrator layer owns network discovery");
                    continue;
                }
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

        all_services
    }

    /// Discover services via environment variables
    fn discover_via_environment(&self, capability: &str) -> Vec<DiscoveredService> {
        (self.discover_env)(capability, self.config.discovery.cache_ttl_secs)
    }

    /// Discover services via service registry (Consul, etcd, etc.)
    ///
    /// Discovers services through a centralized service registry.
    /// The registry URL is supplied via [`CapabilityDiscovery::with_service_registry_url`].
    ///
    /// # Implementation Status
    /// Currently gracefully falls back to other discovery methods.
    /// Full implementation requires HTTP client and registry-specific protocols.
    ///
    /// # Configuration
    /// Set [`CapabilityDiscovery::with_service_registry_url`] from the caller (e.g. after reading
    /// `SERVICE_REGISTRY_URL`). Example: `http://consul:8500` or `http://etcd:2379`
    ///
    /// # Future Enhancement
    /// - Support Consul, etcd, Kubernetes service discovery
    /// - Query by capability tags/labels
    /// - Watch for service changes (real-time updates)
    /// - Health check integration
    fn discover_via_service_registry(&self, capability: &str) -> Vec<DiscoveredService> {
        if let Some(registry_url) = self.service_registry_url.as_ref() {
            warn!(
                registry_url = %registry_url,
                capability,
                query_path = format!("/v1/catalog/service/{capability}"),
                "service registry URL configured but HTTP registry client is not wired — \
                 would query {registry_url}/v1/catalog/service/{capability} (Consul/etcd-style); \
                 falling back to mDNS and environment discovery"
            );
        } else {
            debug!(
                capability,
                "no service registry URL configured (set via with_service_registry_url); \
                 using mDNS and environment discovery only"
            );
        }

        vec![]
    }

    /// Select best service from discovered services
    #[must_use]
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
    const fn new() -> Self {
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

#[cfg(test)]
mod discovery_tests {
    use super::CapabilityDiscovery;
    use crate::capability_env::discovered_services_from_environment_with;
    use crate::config::DiscoveryConfig;
    use crate::types::{Capability, DiscoveredService, HealthStatus, QoSMetrics, ServiceEndpoint};
    use std::collections::HashMap;
    use std::env::VarError;
    use std::sync::Arc;
    use std::time::SystemTime;

    fn minimal_config_toml(methods: &str) -> String {
        format!(
            r#"
[primal_self]
primal_id = "t"
primal_type = "test"
version = "1"
display_name = "T"
self_capabilities = ["x"]

[primal_self.endpoint]
host = "127.0.0.1"
port = 8443
scheme = "https"
path_prefix = "/api"

[primal_self.announcement]
enabled = false
methods = []
announcement_interval_secs = 60
ttl_secs = 300

[required_capabilities._placeholder]
required = false
preferred = false
features = []
fallback = "standalone"

[discovery]
methods = [{methods}]
discovery_timeout_secs = 10
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.25
throughput = 0.25
availability = 0.25
reliability = 0.25
"#
        )
    }

    #[tokio::test]
    async fn from_config_creates_discovery() {
        let dir = tempfile::tempdir().expect("tempdir for discovery config test");
        let path = dir.path().join("d.toml");
        std::fs::write(&path, minimal_config_toml("\"environment\""))
            .expect("write minimal discovery config");
        let d = CapabilityDiscovery::from_config(&path).expect("from_config with minimal TOML");
        assert_eq!(d.config.primal_self.primal_id, "t");
    }

    #[tokio::test]
    async fn find_by_capability_environment_and_cache() {
        let dir = tempfile::tempdir().expect("tempdir for env discovery test");
        let path = dir.path().join("d.toml");
        let toml = minimal_config_toml("\"environment\"");
        std::fs::write(&path, toml).expect("write discovery TOML");

        let env = Arc::new({
            let mut m = HashMap::new();
            m.insert(
                "CAPABILITY_ORCH_ENDPOINT".to_string(),
                "http://127.0.0.1:9".to_string(),
            );
            m
        });
        let env_for_closure = env.clone();
        let d = CapabilityDiscovery::from_config(&path)
            .expect("from_config for env hook test")
            .with_environment_discovery(move |cap, ttl| {
                let e = env_for_closure.clone();
                discovered_services_from_environment_with(
                    cap,
                    ttl,
                    |k| e.get(k).cloned().ok_or(VarError::NotPresent),
                    e.iter().map(|(a, b)| (a.clone(), b.clone())),
                )
            });
        let first = d.find_by_capability("orch").await.expect("find orch first");
        assert_eq!(first.len(), 1);
        let second = d
            .find_by_capability("orch")
            .await
            .expect("find orch second");
        assert_eq!(second.len(), 1);
    }

    #[tokio::test]
    async fn discover_services_unknown_method_is_skipped() {
        let toml = minimal_config_toml("\"not-a-real-method\"");
        let config: DiscoveryConfig = toml::from_str(&toml).expect("parse discovery config TOML");
        let d = CapabilityDiscovery::new(config);
        let out = d
            .find_by_capability("anything")
            .await
            .expect("find with unknown method");
        assert!(out.is_empty());
    }

    #[tokio::test]
    async fn discover_methods_mdns_dns_sd_registry_branches() {
        for methods in [
            "\"mdns\"",
            "\"dns_sd\"",
            "\"service_registry\"",
            "\"mdns\", \"dns_sd\"",
        ] {
            let toml = minimal_config_toml(methods);
            let config: DiscoveryConfig =
                toml::from_str(&toml).expect("parse discovery config for methods branch");
            let d = CapabilityDiscovery::new(config);
            let out = d
                .find_by_capability("c")
                .await
                .expect("find by capability c");
            assert!(out.is_empty(), "expected empty for methods={methods}");
        }
    }

    #[tokio::test]
    async fn service_registry_branch_with_env_url() {
        let toml = minimal_config_toml("\"service_registry\"");
        let config: DiscoveryConfig =
            toml::from_str(&toml).expect("parse service_registry branch config");
        let d = CapabilityDiscovery::new(config)
            .with_service_registry_url(Some("http://127.0.0.1:8500".to_string()));
        let out = d
            .find_by_capability("x")
            .await
            .expect("find with registry URL set");
        assert!(out.is_empty());
    }

    #[test]
    fn select_best_empty_and_ranking() {
        let toml = minimal_config_toml("\"environment\"");
        let config: DiscoveryConfig =
            toml::from_str(&toml).expect("parse config for select_best test");
        let d = CapabilityDiscovery::new(config);

        assert!(d.select_best(&[]).is_none());

        let low = DiscoveredService {
            id: "a".to_string(),
            service_type: "t".to_string(),
            display_name: "a".to_string(),
            endpoint: ServiceEndpoint {
                primary_url: "http://a".to_string(),
                fallback_urls: vec![],
                use_tls: false,
                path_prefix: None,
            },
            capabilities: vec![Capability {
                capability_type: "c".to_string(),
                version: "1".to_string(),
                features: vec![],
                parameters: HashMap::new(),
            }],
            qos: QoSMetrics {
                latency_ms: 90.0,
                throughput_ops_sec: 1000.0,
                availability: 0.5,
                reliability: 0.5,
                updated_at: SystemTime::now(),
            },
            health: HealthStatus::Unknown,
            discovered_at: SystemTime::now(),
            ttl_secs: 60,
            discovery_method: "test".to_string(),
            metadata: HashMap::new(),
        };
        let high = DiscoveredService {
            id: "b".to_string(),
            qos: QoSMetrics {
                latency_ms: 5.0,
                throughput_ops_sec: 9000.0,
                availability: 1.0,
                reliability: 1.0,
                updated_at: SystemTime::now(),
            },
            ..low.clone()
        };

        let best = d
            .select_best(&[low, high])
            .expect("select_best with two candidates");
        assert_eq!(best.id, "b");
    }

    #[tokio::test]
    async fn builder_with_service_registry_url_none_still_runs() {
        let toml = minimal_config_toml("\"environment\"");
        let config: DiscoveryConfig = toml::from_str(&toml).expect("parse config for noop find");
        let d = CapabilityDiscovery::new(config).with_service_registry_url(None);
        let out = d
            .find_by_capability("noop")
            .await
            .expect("find noop with registry URL none");
        assert!(out.is_empty());
    }

    #[tokio::test]
    async fn from_config_missing_file_errors() {
        let res = CapabilityDiscovery::from_config("/nonexistent/discovery-config-xyz.toml");
        assert!(
            res.is_err(),
            "missing file should not load discovery config"
        );
    }

    #[test]
    fn select_best_prefers_finite_over_nan_score() {
        let toml = minimal_config_toml("\"environment\"");
        let config: DiscoveryConfig = toml::from_str(&toml).expect("parse config for nan test");
        let d = CapabilityDiscovery::new(config);

        let base = DiscoveredService {
            id: "a".to_string(),
            service_type: "t".to_string(),
            display_name: "a".to_string(),
            endpoint: ServiceEndpoint {
                primary_url: "http://a".to_string(),
                fallback_urls: vec![],
                use_tls: false,
                path_prefix: None,
            },
            capabilities: vec![Capability {
                capability_type: "c".to_string(),
                version: "1".to_string(),
                features: vec![],
                parameters: HashMap::new(),
            }],
            qos: QoSMetrics {
                latency_ms: f64::NAN,
                throughput_ops_sec: 0.0,
                availability: 0.0,
                reliability: 0.0,
                updated_at: SystemTime::now(),
            },
            health: HealthStatus::Unknown,
            discovered_at: SystemTime::now(),
            ttl_secs: 60,
            discovery_method: "test".to_string(),
            metadata: HashMap::new(),
        };
        let mut good = base.clone();
        good.id = "b".to_string();
        good.qos = QoSMetrics {
            latency_ms: 10.0,
            throughput_ops_sec: 100.0,
            availability: 1.0,
            reliability: 1.0,
            updated_at: SystemTime::now(),
        };

        let best = d
            .select_best(&[base, good])
            .expect("finite score wins over NaN");
        assert_eq!(best.id, "b");
    }

    #[tokio::test]
    async fn find_by_capability_skips_empty_cache_entry() {
        let dir = tempfile::tempdir().expect("tempdir for cache skip test");
        let path = dir.path().join("d.toml");
        let toml = minimal_config_toml("\"environment\"");
        std::fs::write(&path, toml).expect("write discovery TOML");
        let d = CapabilityDiscovery::from_config(&path)
            .expect("from_config")
            .with_environment_discovery(|_cap, _ttl| vec![]);
        let out = d
            .find_by_capability("orphan-cap")
            .await
            .expect("find orphan");
        assert!(out.is_empty());
    }
}
