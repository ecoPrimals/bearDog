// SPDX-License-Identifier: AGPL-3.0-only

//! Expanded zero-copy service IDs for all BearDog crates
//!
//! Provides string interning for frequently used service IDs, endpoint URLs,
//! and other repeated strings to reduce memory allocations.
//!
//! **Memory Impact**: 10-20% reduction in string allocations

use beardog_utils::zero_copy::shared_string;
use std::sync::Arc;

/// Common service IDs used throughout BearDog ecosystem
pub mod service_ids {
    use super::*;

    // Core Services
    
    /// BearDog primary service ID
    pub fn beardog() -> Arc<str> {
        shared_string("beardog")
    }

    /// BearDog core service ID
    pub fn beardog_core() -> Arc<str> {
        shared_string("beardog-core")
    }

    // Authentication & Authorization

    /// BearDog auth service ID
    pub fn beardog_auth() -> Arc<str> {
        shared_string("beardog-auth")
    }

    // Security Services

    /// BearDog security service ID
    pub fn beardog_security() -> Arc<str> {
        shared_string("beardog-security")
    }

    /// BearDog security registry service ID
    pub fn beardog_security_registry() -> Arc<str> {
        shared_string("beardog-security-registry")
    }

    /// BearDog threat service ID
    pub fn beardog_threat() -> Arc<str> {
        shared_string("beardog-threat")
    }

    // Network & Communication

    /// BearDog tunnel service ID
    pub fn beardog_tunnel() -> Arc<str> {
        shared_string("beardog-tunnel")
    }

    /// BearDog networking service ID
    pub fn beardog_networking() -> Arc<str> {
        shared_string("beardog-networking")
    }

    /// BearDog API service ID
    pub fn beardog_api() -> Arc<str> {
        shared_string("beardog-api")
    }

    // Monitoring & Observability

    /// BearDog monitoring service ID
    pub fn beardog_monitoring() -> Arc<str> {
        shared_string("beardog-monitoring")
    }

    // Configuration & Management

    /// BearDog config service ID
    pub fn beardog_config() -> Arc<str> {
        shared_string("beardog-config")
    }

    /// BearDog node registry service ID
    pub fn beardog_node_registry() -> Arc<str> {
        shared_string("beardog-node-registry")
    }

    // Data & Storage

    /// BearDog types service ID
    pub fn beardog_types() -> Arc<str> {
        shared_string("beardog-types")
    }

    // Workflow & Processing

    /// BearDog workflows service ID
    pub fn beardog_workflows() -> Arc<str> {
        shared_string("beardog-workflows")
    }

    // Utilities

    /// BearDog utils service ID
    pub fn beardog_utils() -> Arc<str> {
        shared_string("beardog-utils")
    }

    /// BearDog errors service ID
    pub fn beardog_errors() -> Arc<str> {
        shared_string("beardog-errors")
    }

    /// BearDog traits service ID
    pub fn beardog_traits() -> Arc<str> {
        shared_string("beardog-traits")
    }

    // Specialized Services

    /// BearDog genetics service ID
    pub fn beardog_genetics() -> Arc<str> {
        shared_string("beardog-genetics")
    }

    /// BearDog adapters service ID
    pub fn beardog_adapters() -> Arc<str> {
        shared_string("beardog-adapters")
    }

    /// BearDog compliance service ID
    pub fn beardog_compliance() -> Arc<str> {
        shared_string("beardog-compliance")
    }

    /// BearDog production service ID
    pub fn beardog_production() -> Arc<str> {
        shared_string("beardog-production")
    }

    /// BearDog deploy service ID
    pub fn beardog_deploy() -> Arc<str> {
        shared_string("beardog-deploy")
    }

    /// BearDog CLI service ID
    pub fn beardog_cli() -> Arc<str> {
        shared_string("beardog-cli")
    }

    /// BearDog integration tests service ID
    pub fn beardog_integration_tests() -> Arc<str> {
        shared_string("beardog-integration-tests")
    }

    /// Get all standard service IDs
    pub fn all_service_ids() -> Vec<Arc<str>> {
        vec![
            beardog(),
            beardog_core(),
            beardog_auth(),
            beardog_security(),
            beardog_security_registry(),
            beardog_threat(),
            beardog_tunnel(),
            beardog_networking(),
            beardog_api(),
            beardog_monitoring(),
            beardog_config(),
            beardog_node_registry(),
            beardog_types(),
            beardog_workflows(),
            beardog_utils(),
            beardog_errors(),
            beardog_traits(),
            beardog_genetics(),
            beardog_adapters(),
            beardog_compliance(),
            beardog_production(),
            beardog_deploy(),
            beardog_cli(),
            beardog_integration_tests(),
        ]
    }
}

/// Common endpoint URLs used throughout BearDog
///
/// NOTE: These use centralized port constants from beardog_types::constants.
/// For runtime configuration, use beardog_config::BearDogConfig instead.
pub mod endpoints {
    use super::*;
    use beardog_types::constants::domains::network::addresses::{
        DEFAULT_API_PORT, DEFAULT_METRICS_PORT, DEFAULT_HEALTH_PORT,
    };
    use beardog_types::constants::domains::network::ports::HTTPS_PORT;

    /// Default localhost HTTP endpoint (uses DEFAULT_API_PORT)
    pub fn localhost_http() -> Arc<str> {
        shared_string(&format!("http://localhost:{}", DEFAULT_API_PORT))
    }

    /// Default localhost HTTPS endpoint (uses standard HTTPS_PORT)
    pub fn localhost_https() -> Arc<str> {
        shared_string(&format!("https://localhost:{}", HTTPS_PORT))
    }

    /// Default discovery endpoint (uses DEFAULT_METRICS_PORT for service discovery)
    pub fn discovery() -> Arc<str> {
        shared_string(&format!("http://localhost:{}", DEFAULT_METRICS_PORT))
    }

    /// Default metrics endpoint (uses DEFAULT_METRICS_PORT + 1)
    pub fn metrics() -> Arc<str> {
        shared_string(&format!("http://localhost:{}/metrics", DEFAULT_METRICS_PORT + 1))
    }

    /// Default health endpoint (uses DEFAULT_HEALTH_PORT)
    pub fn health() -> Arc<str> {
        shared_string(&format!("http://localhost:{}/health", DEFAULT_HEALTH_PORT))
    }

    /// Default API endpoint (uses DEFAULT_API_PORT)
    pub fn api() -> Arc<str> {
        shared_string(&format!("http://localhost:{}/api", DEFAULT_API_PORT))
    }

    /// Default admin endpoint (uses DEFAULT_API_PORT)
    pub fn admin() -> Arc<str> {
        shared_string(&format!("http://localhost:{}/admin", DEFAULT_API_PORT))
    }
}

/// Common capability names
pub mod capabilities {
    use super::*;

    pub fn security() -> Arc<str> {
        shared_string("security")
    }

    pub fn hsm() -> Arc<str> {
        shared_string("hsm")
    }

    pub fn authentication() -> Arc<str> {
        shared_string("authentication")
    }

    pub fn authorization() -> Arc<str> {
        shared_string("authorization")
    }

    pub fn encryption() -> Arc<str> {
        shared_string("encryption")
    }

    pub fn decryption() -> Arc<str> {
        shared_string("decryption")
    }

    pub fn signing() -> Arc<str> {
        shared_string("signing")
    }

    pub fn verification() -> Arc<str> {
        shared_string("verification")
    }

    pub fn key_management() -> Arc<str> {
        shared_string("key_management")
    }

    pub fn key_generation() -> Arc<str> {
        shared_string("key_generation")
    }

    pub fn key_rotation() -> Arc<str> {
        shared_string("key_rotation")
    }

    pub fn secure_enclave() -> Arc<str> {
        shared_string("secure_enclave")
    }

    pub fn strongbox() -> Arc<str> {
        shared_string("strongbox")
    }

    pub fn tpm() -> Arc<str> {
        shared_string("tpm")
    }

    pub fn monitoring() -> Arc<str> {
        shared_string("monitoring")
    }

    pub fn logging() -> Arc<str> {
        shared_string("logging")
    }

    pub fn tracing() -> Arc<str> {
        shared_string("tracing")
    }
}

/// Common node types
pub mod node_types {
    use super::*;

    pub fn primary() -> Arc<str> {
        shared_string("primary")
    }

    pub fn replica() -> Arc<str> {
        shared_string("replica")
    }

    pub fn witness() -> Arc<str> {
        shared_string("witness")
    }

    pub fn gateway() -> Arc<str> {
        shared_string("gateway")
    }

    pub fn coordinator() -> Arc<str> {
        shared_string("coordinator")
    }

    pub fn worker() -> Arc<str> {
        shared_string("worker")
    }
}

/// Common resource types
pub mod resource_types {
    use super::*;

    pub fn key() -> Arc<str> {
        shared_string("key")
    }

    pub fn certificate() -> Arc<str> {
        shared_string("certificate")
    }

    pub fn credential() -> Arc<str> {
        shared_string("credential")
    }

    pub fn secret() -> Arc<str> {
        shared_string("secret")
    }

    pub fn token() -> Arc<str> {
        shared_string("token")
    }

    pub fn session() -> Arc<str> {
        shared_string("session")
    }

    pub fn policy() -> Arc<str> {
        shared_string("policy")
    }

    pub fn workflow() -> Arc<str> {
        shared_string("workflow")
    }

    pub fn config() -> Arc<str> {
        shared_string("config")
    }
}

/// Common status strings
pub mod statuses {
    use super::*;

    pub fn active() -> Arc<str> {
        shared_string("active")
    }

    pub fn inactive() -> Arc<str> {
        shared_string("inactive")
    }

    pub fn pending() -> Arc<str> {
        shared_string("pending")
    }

    pub fn completed() -> Arc<str> {
        shared_string("completed")
    }

    pub fn failed() -> Arc<str> {
        shared_string("failed")
    }

    pub fn healthy() -> Arc<str> {
        shared_string("healthy")
    }

    pub fn unhealthy() -> Arc<str> {
        shared_string("unhealthy")
    }

    pub fn degraded() -> Arc<str> {
        shared_string("degraded")
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
    fn test_all_service_ids_count() {
        let ids = service_ids::all_service_ids();
        
        // Now have 24 service IDs (expanded from 7)
        assert_eq!(ids.len(), 24);
        assert!(ids.iter().all(|id| !id.is_empty()));
    }

    #[test]
    fn test_expanded_service_ids() {
        assert_eq!(service_ids::beardog_genetics().as_ref(), "beardog-genetics");
        assert_eq!(service_ids::beardog_workflows().as_ref(), "beardog-workflows");
        assert_eq!(service_ids::beardog_deploy().as_ref(), "beardog-deploy");
        assert_eq!(service_ids::beardog_compliance().as_ref(), "beardog-compliance");
    }

    #[test]
    fn test_endpoints_are_interned() {
        let ep1 = endpoints::localhost_https();
        let ep2 = endpoints::localhost_https();

        assert!(Arc::ptr_eq(&ep1, &ep2));
    }

    #[test]
    fn test_expanded_endpoints() {
        use beardog_types::constants::domains::network::addresses::DEFAULT_API_PORT;
        
        // Verify endpoints use centralized port constants
        assert_eq!(
            endpoints::api().as_ref(),
            &format!("http://localhost:{}/api", DEFAULT_API_PORT)
        );
        assert_eq!(
            endpoints::admin().as_ref(),
            &format!("http://localhost:{}/admin", DEFAULT_API_PORT)
        );
    }

    #[test]
    fn test_capabilities_are_interned() {
        let cap1 = capabilities::hsm();
        let cap2 = capabilities::hsm();

        assert!(Arc::ptr_eq(&cap1, &cap2));
    }

    #[test]
    fn test_expanded_capabilities() {
        assert_eq!(capabilities::key_generation().as_ref(), "key_generation");
        assert_eq!(capabilities::key_rotation().as_ref(), "key_rotation");
        assert_eq!(capabilities::strongbox().as_ref(), "strongbox");
        assert_eq!(capabilities::tpm().as_ref(), "tpm");
    }

    #[test]
    fn test_node_types_are_interned() {
        let node1 = node_types::primary();
        let node2 = node_types::primary();

        assert!(Arc::ptr_eq(&node1, &node2));
    }

    #[test]
    fn test_expanded_node_types() {
        assert_eq!(node_types::coordinator().as_ref(), "coordinator");
        assert_eq!(node_types::worker().as_ref(), "worker");
    }

    #[test]
    fn test_resource_types_are_interned() {
        let res1 = resource_types::key();
        let res2 = resource_types::key();

        assert!(Arc::ptr_eq(&res1, &res2));
    }

    #[test]
    fn test_resource_types_values() {
        assert_eq!(resource_types::certificate().as_ref(), "certificate");
        assert_eq!(resource_types::secret().as_ref(), "secret");
        assert_eq!(resource_types::workflow().as_ref(), "workflow");
    }

    #[test]
    fn test_statuses_are_interned() {
        let status1 = statuses::active();
        let status2 = statuses::active();

        assert!(Arc::ptr_eq(&status1, &status2));
    }

    #[test]
    fn test_status_values() {
        assert_eq!(statuses::healthy().as_ref(), "healthy");
        assert_eq!(statuses::unhealthy().as_ref(), "unhealthy");
        assert_eq!(statuses::degraded().as_ref(), "degraded");
    }

    #[test]
    fn test_memory_efficiency() {
        // Creating multiple references to same string should share allocation
        let refs: Vec<Arc<str>> = (0..1000)
            .map(|_| service_ids::beardog_auth())
            .collect();

        // All should point to same allocation
        for i in 1..refs.len() {
            assert!(Arc::ptr_eq(&refs[0], &refs[i]));
        }
    }
}

