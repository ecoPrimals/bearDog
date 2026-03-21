// SPDX-License-Identifier: AGPL-3.0-only

//! Zero-copy service ID manager
//!
//! Provides string interning for frequently used service IDs, endpoint URLs,
//! and other repeated strings to reduce memory allocations.

use beardog_utils::zero_copy::shared_string;
use std::sync::Arc;

/// Common service IDs used throughout BearDog
pub mod service_ids {
    use super::*;

    /// BearDog primary service ID
    pub fn beardog() -> Arc<str> {
        shared_string("beardog")
    }

    /// BearDog auth service ID
    pub fn beardog_auth() -> Arc<str> {
        shared_string("beardog-auth")
    }

    /// BearDog API service ID
    pub fn beardog_api() -> Arc<str> {
        shared_string("beardog-api")
    }

    /// BearDog tunnel service ID
    pub fn beardog_tunnel() -> Arc<str> {
        shared_string("beardog-tunnel")
    }

    /// BearDog monitoring service ID
    pub fn beardog_monitoring() -> Arc<str> {
        shared_string("beardog-monitoring")
    }

    /// BearDog security service ID
    pub fn beardog_security() -> Arc<str> {
        shared_string("beardog-security")
    }

    /// BearDog core service ID
    pub fn beardog_core() -> Arc<str> {
        shared_string("beardog-core")
    }

    /// Get all standard service IDs
    pub fn all_service_ids() -> Vec<Arc<str>> {
        vec![
            beardog(),
            beardog_auth(),
            beardog_api(),
            beardog_tunnel(),
            beardog_monitoring(),
            beardog_security(),
            beardog_core(),
        ]
    }
}

/// Common endpoint URLs used throughout BearDog
///
/// NOTE: Host comes from [`DEFAULT_EXTERNAL_HOST`](beardog_config::domains::network_addresses::DEFAULT_EXTERNAL_HOST)
/// (overridable via `BEARDOG_EXTERNAL_HOST`); ports use [`beardog_config::domains::network_ports`] fallbacks.
/// For full runtime configuration, use `beardog_config::BearDogConfig`.
pub mod endpoints {
    use super::*;
    use beardog_config::domains::network_addresses::DEFAULT_EXTERNAL_HOST;
    use beardog_config::domains::network_ports::{
        DEFAULT_API_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT,
    };
    use beardog_types::constants::domains::network::ports::HTTPS_PORT;

    /// Default localhost HTTP endpoint (uses DEFAULT_API_PORT)
    pub fn localhost_http() -> Arc<str> {
        shared_string(&format!(
            "http://{}:{}",
            DEFAULT_EXTERNAL_HOST, DEFAULT_API_PORT
        ))
    }

    /// Default localhost HTTPS endpoint (uses standard HTTPS_PORT)
    pub fn localhost_https() -> Arc<str> {
        shared_string(&format!(
            "https://{}:{}",
            DEFAULT_EXTERNAL_HOST, HTTPS_PORT
        ))
    }

    /// Default discovery endpoint (uses DEFAULT_METRICS_PORT for service discovery)
    pub fn discovery() -> Arc<str> {
        shared_string(&format!(
            "http://{}:{}",
            DEFAULT_EXTERNAL_HOST, DEFAULT_METRICS_PORT
        ))
    }

    /// Default metrics endpoint (uses DEFAULT_METRICS_PORT + 1)
    pub fn metrics() -> Arc<str> {
        shared_string(&format!(
            "http://{}:{}/metrics",
            DEFAULT_EXTERNAL_HOST,
            DEFAULT_METRICS_PORT + 1
        ))
    }

    /// Default health endpoint (uses DEFAULT_HEALTH_PORT)
    pub fn health() -> Arc<str> {
        shared_string(&format!(
            "http://{}:{}/health",
            DEFAULT_EXTERNAL_HOST, DEFAULT_HEALTH_PORT
        ))
    }
}

/// Common capability names
pub mod capabilities {
    use super::*;

    /// Interned capability id for the security domain.
    pub fn security() -> Arc<str> {
        shared_string("security")
    }

    /// Interned capability id for hardware security modules.
    pub fn hsm() -> Arc<str> {
        shared_string("hsm")
    }

    /// Interned capability id for authentication flows.
    pub fn authentication() -> Arc<str> {
        shared_string("authentication")
    }

    /// Interned capability id for authorization decisions.
    pub fn authorization() -> Arc<str> {
        shared_string("authorization")
    }

    /// Interned capability id for encryption services.
    pub fn encryption() -> Arc<str> {
        shared_string("encryption")
    }

    /// Interned capability id for key lifecycle operations.
    pub fn key_management() -> Arc<str> {
        shared_string("key_management")
    }

    /// Interned capability id for secure enclave / StrongBox style hardware.
    pub fn secure_enclave() -> Arc<str> {
        shared_string("secure_enclave")
    }
}

/// Common node types
pub mod node_types {
    use super::*;

    /// Interned node role label for the primary replica.
    pub fn primary() -> Arc<str> {
        shared_string("primary")
    }

    /// Interned node role label for read replicas.
    pub fn replica() -> Arc<str> {
        shared_string("replica")
    }

    /// Interned node role label for witness / observer nodes.
    pub fn witness() -> Arc<str> {
        shared_string("witness")
    }

    /// Interned node role label for edge gateways.
    pub fn gateway() -> Arc<str> {
        shared_string("gateway")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_ids_are_interned() {
        let id1 = service_ids::beardog_auth();
        let id2 = service_ids::beardog_auth();

        // Same allocation (interned)
        assert!(Arc::ptr_eq(&id1, &id2));
    }

    #[test]
    fn test_all_service_ids() {
        let ids = service_ids::all_service_ids();
        
        assert_eq!(ids.len(), 7);
        assert!(ids.iter().all(|id| !id.is_empty()));
    }

    #[test]
    fn test_endpoints_are_interned() {
        let ep1 = endpoints::localhost_https();
        let ep2 = endpoints::localhost_https();

        assert!(Arc::ptr_eq(&ep1, &ep2));
    }

    #[test]
    fn test_capabilities_are_interned() {
        let cap1 = capabilities::hsm();
        let cap2 = capabilities::hsm();

        assert!(Arc::ptr_eq(&cap1, &cap2));
    }

    #[test]
    fn test_node_types_are_interned() {
        let node1 = node_types::primary();
        let node2 = node_types::primary();

        assert!(Arc::ptr_eq(&node1, &node2));
    }

    #[test]
    fn test_service_id_values() {
        assert_eq!(service_ids::beardog().as_ref(), "beardog");
        assert_eq!(service_ids::beardog_auth().as_ref(), "beardog-auth");
        assert_eq!(service_ids::beardog_api().as_ref(), "beardog-api");
    }

    #[test]
    fn test_endpoint_values() {
        use beardog_config::domains::network_addresses::DEFAULT_EXTERNAL_HOST;
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;
        use beardog_types::constants::domains::network::ports::HTTPS_PORT;
        
        // Verify endpoints use centralized port constants
        assert_eq!(
            endpoints::localhost_http().as_ref(),
            &format!("http://{DEFAULT_EXTERNAL_HOST}:{DEFAULT_API_PORT}")
        );
        assert_eq!(
            endpoints::localhost_https().as_ref(),
            &format!("https://{DEFAULT_EXTERNAL_HOST}:{HTTPS_PORT}")
        );
    }

    #[test]
    fn test_capability_values() {
        assert_eq!(capabilities::security().as_ref(), "security");
        assert_eq!(capabilities::hsm().as_ref(), "hsm");
    }
}

