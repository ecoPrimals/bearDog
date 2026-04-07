// SPDX-License-Identifier: AGPL-3.0-or-later

//! Factory for the default discovery stack and Phase-2 detection stubs.

use std::sync::Arc;

use super::core::{DiscoveryError, ServiceDiscoveryCapability};
use super::kubernetes::KubernetesDiscovery;
use super::providers::{ConsulDiscovery, DnsHttpDiscovery, EtcdDiscovery};

/// Auto-detect and create the best available service discovery implementation
///
/// Tries providers in order of sophistication:
/// 1. Kubernetes (if in K8s cluster)
/// 2. Consul (if Consul available)
/// 3. etcd (if etcd available)
/// 4. DNS + HTTP (always available fallback)
///
/// # Returns
///
/// * `Ok(Arc<dyn ServiceDiscoveryCapability>)` — Kubernetes discovery when the cluster is
///   detected, otherwise the DNS/HTTP fallback (this function does not return `Err` today).
///
/// # Examples
///
/// ```rust,no_run
/// # use beardog_types::canonical::discovery::service_discovery_capability::*;
/// # async fn example() -> Result<(), DiscoveryError> {
/// let discovery = create_service_discovery().await?;
/// println!("Using discovery provider: {}", discovery.provider_name());
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This factory does not currently return `Err`: it tries Kubernetes first, and if that is
/// unavailable it falls back to DNS/HTTP. Consul/etcd are not selected here (see module comments).
pub async fn create_service_discovery()
-> Result<Arc<dyn ServiceDiscoveryCapability>, DiscoveryError> {
    // Try Kubernetes first (environment detection implemented)
    if let Ok(discovery) = KubernetesDiscovery::try_create().await {
        tracing::info!("Using Kubernetes service discovery");
        return Ok(Arc::new(discovery));
    }

    // ✅ ARCHITECTURAL DECISION: Service discovery delegated via capability discovery
    // BearDog follows "Discover, Don't Implement" principle:
    // - Consul, etcd, mDNS, DNS-SD → Handled by primals with ServiceDiscovery capability
    // - BearDog receives discovered services via UniversalAdapter
    // - See: crates/beardog-adapters/src/universal/primal_capability_adapter.rs
    //
    // For network service discovery, use UniversalPrimalAdapter::discover_network_primals()
    // instead of direct protocol implementation (capability-based, primal-agnostic)

    // Fallback to DNS + HTTP (always available)
    tracing::info!("Using DNS/HTTP fallback discovery");
    Ok(Arc::new(DnsHttpDiscovery::new()))
}

// ============================================================================
// Detection functions - Phase 2 Implementation Stubs
// ============================================================================
// These functions will auto-detect available discovery backends.
// Currently stubbed out; implementation tracking:
// - Kubernetes: Check KUBERNETES_SERVICE_HOST, service account token
// - Consul: Check CONSUL_HTTP_ADDR, local agent at 127.0.0.1:8500
// - etcd: Check ETCD_ENDPOINTS, standard locations
// ============================================================================

/// Auto-detect Kubernetes availability
///
/// # Phase 2 Implementation
///
/// Will check for:
/// - `KUBERNETES_SERVICE_HOST` environment variable
/// - Service account token at /var/run/secrets/kubernetes.io/
/// - Accessible API server endpoint
#[deprecated(
    since = "0.1.0",
    note = "Phase 2 stub: Implement Kubernetes auto-detection. Use KubernetesDiscovery::try_create() instead."
)]
async fn _detect_kubernetes() -> Result<KubernetesDiscovery, DiscoveryError> {
    Err(DiscoveryError::BackendUnavailable {
        provider: "kubernetes".to_string(),
        reason: "Kubernetes auto-detection not yet implemented (Phase 2)".to_string(),
    })
}

/// Auto-detect Consul availability
///
/// # Phase 2 Implementation
///
/// Will check for:
/// - `CONSUL_HTTP_ADDR` environment variable
/// - Local agent at 127.0.0.1:8500
/// - DNS-based agent discovery
#[deprecated(
    since = "0.1.0",
    note = "Phase 2 stub: Implement Consul auto-detection. Implement ConsulDiscovery::try_create() first."
)]
async fn _detect_consul() -> Result<ConsulDiscovery, DiscoveryError> {
    Err(DiscoveryError::BackendUnavailable {
        provider: "consul".to_string(),
        reason: "Consul auto-detection not yet implemented (Phase 2)".to_string(),
    })
}

/// Auto-detect etcd availability
///
/// # Phase 2 Implementation
///
/// Will check for:
/// - `ETCD_ENDPOINTS` environment variable
/// - Standard etcd ports (2379, 4001)
/// - Cluster member discovery
#[deprecated(
    since = "0.1.0",
    note = "Phase 2 stub: Implement etcd auto-detection. Implement EtcdDiscovery::try_create() first."
)]
async fn _detect_etcd() -> Result<EtcdDiscovery, DiscoveryError> {
    Err(DiscoveryError::BackendUnavailable {
        provider: "etcd".to_string(),
        reason: "etcd auto-detection not yet implemented (Phase 2)".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_service_discovery_yields_working_provider() {
        let discovery = create_service_discovery()
            .await
            .expect("factory should return a discovery implementation");
        let name = discovery.provider_name();
        assert!(
            name.contains("kubernetes") || name.contains("dns"),
            "unexpected provider: {name}"
        );
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn phase2_detect_stubs_return_backend_unavailable() {
        let e = _detect_kubernetes()
            .await
            .expect_err("kubernetes stub should err");
        match e {
            DiscoveryError::BackendUnavailable { provider, .. } => {
                assert_eq!(provider, "kubernetes");
            }
            other => panic!("unexpected error: {other:?}"),
        }
        let e = _detect_consul().await.expect_err("consul stub");
        match e {
            DiscoveryError::BackendUnavailable { provider, .. } => {
                assert_eq!(provider, "consul");
            }
            other => panic!("unexpected error: {other:?}"),
        }
        let e = _detect_etcd().await.expect_err("etcd stub");
        match e {
            DiscoveryError::BackendUnavailable { provider, .. } => {
                assert_eq!(provider, "etcd");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
