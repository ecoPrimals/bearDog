// SPDX-License-Identifier: AGPL-3.0-or-later

//! Consul, etcd, and DNS/HTTP fallback discovery providers.

use crate::constants::domains::network::addresses::LOCALHOST_IPV4;
use crate::constants::domains::network::config::LOCALHOST_NAME;
use crate::constants::localhost::LOCALHOST_V4;
use beardog_config::domains::network_ports::DEFAULT_API_PORT_STR;
use beardog_config::env_keys;
use std::collections::HashMap;

use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::types::ids::{RegistrationId, ServiceInstanceId};

use super::core::{
    DiscoveryCapabilities, DiscoveryError, DiscoveryHealthStatus, ServiceDescriptor,
    ServiceDiscoveryCapability, ServiceHealth, ServiceProtocol,
};

/// Consul service discovery (to be implemented)
#[derive(Debug)]
pub struct ConsulDiscovery {
    // Will contain Consul HTTP client
}

impl ConsulDiscovery {
    /// Attempts to construct a Consul-backed discovery client. **Current default:** returns
    /// [`DiscoveryError::BackendUnavailable`] until the client is implemented.
    ///
    /// # Errors
    ///
    /// Always returns [`DiscoveryError::BackendUnavailable`] until Consul support is implemented.
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // PHASE-2(Discovery): Implement Consul client creation
        Err(DiscoveryError::BackendUnavailable {
            provider: "consul".to_string(),
            reason: "Not implemented yet".to_string(),
        })
    }
}

/// etcd service discovery (to be implemented)
#[derive(Debug)]
pub struct EtcdDiscovery {
    // Will contain etcd client
}

impl EtcdDiscovery {
    /// Attempts to construct an etcd-backed discovery client. **Current default:** returns
    /// [`DiscoveryError::BackendUnavailable`] until the client is implemented.
    ///
    /// # Errors
    ///
    /// Always returns [`DiscoveryError::BackendUnavailable`] until etcd support is implemented.
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // PHASE-2(Discovery): Implement etcd client creation
        Err(DiscoveryError::BackendUnavailable {
            provider: "etcd".to_string(),
            reason: "Not implemented yet".to_string(),
        })
    }
}

/// DNS + HTTP fallback discovery
///
/// Uses DNS SRV records and A/AAAA records for service discovery.
/// This is the universal fallback that works in any environment with DNS.
#[derive(Debug, Clone)]
pub struct DnsHttpDiscovery {
    /// Search domains for SRV queries
    search_domains: Vec<String>,
    /// DNS timeout in seconds
    _timeout_secs: u64,
}

impl DnsHttpDiscovery {
    /// Create a new DNS/HTTP discovery with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create DNS/HTTP discovery with custom search domains
    pub const fn with_domains(domains: Vec<String>) -> Self {
        Self {
            search_domains: domains,
            _timeout_secs: 5,
        }
    }

    /// Get search domains for DNS queries
    fn get_search_domains(&self) -> Vec<String> {
        if self.search_domains.is_empty() {
            // Default search domains
            vec![
                "local".to_string(),
                "cluster.local".to_string(),
                "service.consul".to_string(),
            ]
        } else {
            self.search_domains.clone()
        }
    }

    /// Query DNS SRV records for a service.
    ///
    /// DNS resolution lives in the runtime layer (`beardog-core` / `beardog-discovery`)
    /// rather than in this types crate.  This default returns an empty set so callers
    /// degrade gracefully until a resolver is wired in.
    async fn query_dns_srv(&self, service: &str) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::debug!(
            service,
            "DNS SRV query — no resolver available in types crate"
        );
        Ok(Vec::new())
    }

    /// Resolve service name to IP addresses
    async fn resolve_service_name(&self, name: &str) -> Result<Vec<String>, DiscoveryError> {
        // Real implementation would use DNS A/AAAA lookups
        // For now, check if it's already an IP or localhost
        if name == LOCALHOST_NAME || name == LOCALHOST_IPV4 {
            // Use canonical network configuration instead of hardcoding
            let default_port = std::env::var(env_keys::ENV_DEFAULT_SERVICE_PORT)
                .or_else(|_| std::env::var(env_keys::ENV_API_PORT))
                .unwrap_or_else(|_| DEFAULT_API_PORT_STR.to_string());
            let host = std::env::var(env_keys::ENV_API_HOST)
                .or_else(|_| std::env::var(env_keys::ENV_LOCALHOST))
                .unwrap_or_else(|_| LOCALHOST_V4.to_string());
            return Ok(vec![format!("http://{}:{}", host, default_port)]);
        }

        Ok(Vec::new())
    }
}

impl Default for DnsHttpDiscovery {
    fn default() -> Self {
        Self {
            search_domains: vec![
                "local".to_string(),
                "cluster.local".to_string(),
                "service.consul".to_string(),
            ],
            _timeout_secs: 5,
        }
    }
}

// Real DNS/HTTP Discovery implementation
impl ServiceDiscoveryCapability for DnsHttpDiscovery {
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!(
            "DNS/HTTP: Discovering services by capability: {:?}",
            capability
        );

        // Use DNS SRV records for service discovery
        // Format: _capability._tcp.domain
        let service_name = format!("_{capability:?}._tcp").to_lowercase();

        // Check common domains
        let domains = self.get_search_domains();
        let mut discovered_services = Vec::new();

        for domain in domains {
            let full_service = format!("{service_name}.{domain}");

            if let Ok(services) = self.query_dns_srv(&full_service).await {
                discovered_services.extend(services);
            }
        }

        tracing::info!(
            "DNS/HTTP: Discovered {} services",
            discovered_services.len()
        );
        Ok(discovered_services)
    }

    async fn discover_by_name(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("DNS/HTTP: Discovering service by name: {}", service_name);

        // Try DNS A/AAAA records first
        if let Ok(endpoints) = self.resolve_service_name(service_name).await {
            let descriptors: Vec<ServiceDescriptor> = endpoints
                .into_iter()
                .map(|endpoint| ServiceDescriptor {
                    instance_id: ServiceInstanceId::new(format!(
                        "dns-http://{service_name}:{endpoint}"
                    )),
                    endpoint,
                    capabilities: vec![], // Empty for generic discovery
                    metadata: HashMap::new(),
                    health: ServiceHealth::Healthy,
                    priority: 100,
                    protocol: ServiceProtocol::Http,
                })
                .collect();

            tracing::info!("DNS/HTTP: Discovered {} endpoints", descriptors.len());
            return Ok(descriptors);
        }

        tracing::warn!("DNS/HTTP: Service {} not found", service_name);
        Ok(Vec::new())
    }

    async fn register_service(
        &self,
        _descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError> {
        // DNS is read-only for discovery
        // Registration would require dynamic DNS updates (RFC 2136)
        // For now, return unsupported
        Err(DiscoveryError::Other {
            message: "Service registration not supported in DNS fallback mode. Use Kubernetes, Consul, or etcd for dynamic registration.".to_string(),
        })
    }

    async fn unregister_service(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        Ok(())
    }

    async fn renew_registration(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError> {
        Ok(DiscoveryHealthStatus {
            is_healthy: true,
            details: HashMap::new(),
            response_time_ms: 0,
        })
    }

    fn provider_name(&self) -> &'static str {
        "dns-http-fallback"
    }

    fn capabilities(&self) -> DiscoveryCapabilities {
        DiscoveryCapabilities {
            supports_capability_query: false,
            supports_registration: false,
            supports_health_checks: false,
            supports_metadata: false,
            max_results: None,
            registration_ttl_seconds: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::capabilities::ServiceCapabilityType;
    use crate::constants::domains::network::config::LOCALHOST_NAME;

    #[tokio::test]
    async fn consul_try_create_returns_backend_unavailable() {
        let e = ConsulDiscovery::try_create()
            .await
            .expect_err("consul backend should be unavailable until implemented");
        match e {
            DiscoveryError::BackendUnavailable { provider, .. } => {
                assert_eq!(provider, "consul");
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[tokio::test]
    async fn etcd_try_create_returns_backend_unavailable() {
        let e = EtcdDiscovery::try_create()
            .await
            .expect_err("etcd backend should be unavailable until implemented");
        match e {
            DiscoveryError::BackendUnavailable { provider, .. } => {
                assert_eq!(provider, "etcd");
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[test]
    fn dns_http_discovery_default_and_new_match() {
        let a = DnsHttpDiscovery::new();
        let b = DnsHttpDiscovery::default();
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    #[test]
    fn dns_http_discovery_with_domains_clone() {
        let d = DnsHttpDiscovery::with_domains(vec!["custom".to_string()]);
        let c = d.clone();
        assert_eq!(format!("{d:?}"), format!("{c:?}"));
    }

    #[tokio::test]
    async fn dns_http_discover_by_capability_runs() {
        let d = DnsHttpDiscovery::default();
        let out = d
            .discover_by_capability(ServiceCapabilityType::ServiceMesh)
            .await
            .expect("stub discovery should return Ok");
        assert!(out.is_empty());
    }

    #[tokio::test]
    async fn dns_http_discover_localhost_yields_endpoint() {
        let d = DnsHttpDiscovery::with_domains(vec![]);
        let out = d
            .discover_by_name(LOCALHOST_NAME)
            .await
            .expect("localhost resolution should succeed");
        assert!(!out.is_empty(), "expected at least one localhost endpoint");
    }

    #[tokio::test]
    async fn dns_http_register_service_errors() {
        let d = DnsHttpDiscovery::default();
        let desc = ServiceDescriptor {
            instance_id: ServiceInstanceId::new("i1"),
            endpoint: "http://x".to_string(),
            capabilities: vec![],
            metadata: HashMap::new(),
            health: ServiceHealth::Healthy,
            priority: 1,
            protocol: ServiceProtocol::Http,
        };
        let e = d
            .register_service(desc)
            .await
            .expect_err("DNS mode must reject registration");
        match e {
            DiscoveryError::Other { message } => {
                assert!(message.contains("not supported"), "{message}");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[tokio::test]
    async fn dns_http_unregister_and_renew_noop() {
        let d = DnsHttpDiscovery::default();
        let id = RegistrationId::new("r1");
        d.unregister_service(&id).await.expect("unregister noop");
        d.renew_registration(&id).await.expect("renew noop");
    }

    #[tokio::test]
    async fn dns_http_health_check_ok() {
        let d = DnsHttpDiscovery::default();
        let h = d.health_check().await.expect("health check");
        assert!(h.is_healthy);
    }

    #[test]
    fn dns_http_provider_name_and_capabilities() {
        let d = DnsHttpDiscovery::default();
        assert_eq!(d.provider_name(), "dns-http-fallback");
        let c = d.capabilities();
        assert!(!c.supports_registration);
    }
}
