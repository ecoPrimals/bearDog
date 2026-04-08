// SPDX-License-Identifier: AGPL-3.0-or-later

//! Factory for the default discovery stack and Phase-2 detection stubs.

use std::sync::Arc;

use super::core::{DiscoveryError, ServiceDiscoveryCapability};
use super::kubernetes::KubernetesDiscovery;
use super::providers::DnsHttpDiscovery;

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
}
