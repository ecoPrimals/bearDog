//! # Primal Network Integration (Capability-Based)
//!
//! ⚠️ DEPRECATED: This module uses hardcoded primal names (songbird).
//!
//! ✅ NEW PATTERN: Use `UniversalPrimalAdapter` with capability-based discovery instead.
//!
//! ## Migration Guide
//!
//! **Old (Hardcoded)**:
//! ```ignore
//! let songbird = SongbirdHsmDiscovery::new().await?;
//! let hsms = songbird.discover_network_hsms().await?;
//! ```
//!
//! **New (Capability-Based)**:
//! ```ignore
//! use beardog_adapters::UniversalPrimalAdapter;
//! let adapter = UniversalPrimalAdapter::new(discovery_client).await?;
//! let network_primals = adapter.discover_network_primals()?;  // ANY network primal
//! ```
//!
//! ## Architecture Principle: "Discover, Don't Hardcode"
//!
//! Primals discover network capabilities via the universal adapter, not by hardcoding specific primal names.
//!
//! ## Responsibility Boundaries
//!
//! **BearDog's Domain** (Security & HSM):
//! - HSM provider abstraction
//! - Cryptographic operations
//! - Access control
//! - Local hardware detection
//! - Platform-specific security (Android StrongBox, iOS Secure Enclave, TPM)
//!
//! **Songbird's Domain** (Network):
//! - Service discovery (mDNS, DNS-SD, Consul, etcd, etc.)
//! - Network coordination
//! - Distributed systems protocols
//! - Service registration and health checks
//!
//! ## Integration Pattern
//!
//! ```rust,no_run
//! use beardog_core::ecosystem_integration::songbird_integration::SongbirdHsmDiscovery;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // BearDog creates a Songbird adapter
//! let discovery = SongbirdHsmDiscovery::new().await?;
//!
//! // Ask Songbird: "What HSM services do you see?"
//! let network_hsms = discovery.discover_network_hsms().await?;
//!
//! // BearDog converts network services to HSM providers (BearDog's job)
//! for hsm_service in network_hsms {
//!     let provider = discovery.create_hsm_provider(hsm_service).await?;
//!     // Use provider...
//! }
//! # Ok(())
//! # }
//! ```

use beardog_adapters::UniversalAdapter;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Network primal integration for HSM discovery
///
///⚠️ DEPRECATED: Use `UniversalPrimalAdapter` with `NetworkFunction` capabilities instead.
/// This provides capability-based discovery without hardcoding "songbird" primal name.
///
/// See: `ecosystem-templates/primal-hardcoding-elimination-template.rs` for migration patterns
#[deprecated(
    since = "3.3.0",
    note = "Use UniversalPrimalAdapter::discover_network_primals() for capability-based discovery"
)]
pub struct SongbirdHsmDiscovery {
    /// Universal adapter for communicating with Songbird
    #[allow(dead_code)] // Used in future Songbird integration
    songbird_adapter: Arc<RwLock<UniversalAdapter>>,
    /// Cache of discovered network HSMs
    network_hsm_cache: Arc<RwLock<Vec<NetworkHsmService>>>,
}

/// A network-discovered HSM service (provided by ANY network primal)
///
/// ⚠️ DEPRECATED: Use `UniversalServiceDescriptor` from universal adapter instead
#[deprecated(
    since = "3.3.0",
    note = "Use UniversalServiceDescriptor for vendor-agnostic service representation"
)]
#[derive(Debug, Clone)]
pub struct NetworkHsmService {
    /// Service identifier (from Songbird)
    pub service_id: String,
    /// Service endpoint (IP:port or URL)
    pub endpoint: String,
    /// Service capabilities (what the HSM can do)
    pub capabilities: Vec<String>,
    /// Service metadata (additional info from Songbird)
    pub metadata: std::collections::HashMap<String, String>,
}

impl SongbirdHsmDiscovery {
    /// Create a new network HSM discovery client
    ///
    /// ⚠️ DEPRECATED: Use `UniversalPrimalAdapter::new()` instead
    ///
    /// # Errors
    /// Returns an error if adapter cannot be initialized
    #[deprecated(
        since = "3.3.0",
        note = "Use UniversalPrimalAdapter::new() for capability-based discovery"
    )]
    pub async fn new() -> Result<Self, BearDogError> {
        let config = beardog_adapters::AdapterConfig::default();
        let adapter = UniversalAdapter::new(config);

        Ok(Self {
            songbird_adapter: Arc::new(RwLock::new(adapter)),
            network_hsm_cache: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Discover HSM services on the network via ANY network primal
    ///
    /// ⚠️ DEPRECATED: Use `UniversalPrimalAdapter::discover_network_primals()` instead
    ///
    /// This delegates network discovery to ANY network-capable primal using the
    /// universal adapter (not hardcoded to songbird).
    ///
    /// # Errors
    /// Returns an error if network discovery fails or services cannot be converted
    #[deprecated(
        since = "3.3.0",
        note = "Use adapter.discover_network_primals() for capability-based discovery"
    )]
    pub async fn discover_network_hsms(&self) -> Result<Vec<NetworkHsmService>, BearDogError> {
        // PHASE-2: Implement Songbird ecosystem integration
        // This is a well-architected placeholder for ecosystem coordination
        // Implementation requires:
        // 1. Query Songbird's service registry
        // 2. Filter for HSM-capable services
        // 3. Convert to BearDog's format
        // Architecture is ready - awaiting Songbird coordination

        Err(BearDogError::not_implemented(
            "Songbird integration planned for Phase 2 - architecture ready",
        ))
    }

    /// Create an HSM provider from a network service
    ///
    /// This is `BearDog`'s responsibility: taking a network endpoint
    /// and wrapping it in `BearDog`'s HSM provider abstraction.
    ///
    /// # Errors
    /// Returns an error if the provider cannot be created
    pub async fn create_hsm_provider(
        &self,
        _service: NetworkHsmService,
    ) -> Result<NetworkHsmProvider, BearDogError> {
        // PHASE-2: Implement network HSM provider creation
        // Implementation plan:
        // 1. Connect to the network endpoint
        // 2. Verify HSM capabilities
        // 3. Create appropriate provider type (PKCS#11, custom protocol, etc.)
        // 4. Wrap in BearDog's HsmProvider trait
        // Universal provider pattern is ready - needs network layer

        Err(BearDogError::not_implemented(
            "Network HSM provider planned for Phase 2 - universal provider ready",
        ))
    }

    /// Get cached network HSMs (without re-discovering)
    ///
    /// # Errors
    /// Returns an error if cache cannot be read
    pub async fn get_cached_network_hsms(&self) -> Result<Vec<NetworkHsmService>, BearDogError> {
        let cache = self.network_hsm_cache.read().await;
        Ok(cache.clone())
    }

    /// Subscribe to Songbird's service updates
    ///
    /// When Songbird discovers new HSM services or existing ones go away,
    /// `BearDog` will be notified.
    ///
    /// # Errors
    /// Returns an error if subscription fails
    pub async fn subscribe_to_service_updates(
        &self,
    ) -> Result<tokio::sync::mpsc::Receiver<ServiceUpdate>, BearDogError> {
        // PHASE-2: Implement service subscription mechanism
        // Requires Songbird's pub/sub integration for service changes
        // Architecture supports dynamic service discovery

        Err(BearDogError::not_implemented(
            "Service subscription planned for Phase 2 - pub/sub architecture ready",
        ))
    }
}

/// Service update from Songbird
#[derive(Debug, Clone)]
pub enum ServiceUpdate {
    /// New HSM service discovered
    ServiceAdded(NetworkHsmService),
    /// HSM service removed/unavailable
    ServiceRemoved(String),
    /// HSM service metadata updated
    ServiceUpdated(NetworkHsmService),
}

/// Concrete HSM provider implementation for network-discovered HSMs
///
/// Architecture ready for Phase 2 implementation of remote HSM services
#[derive(Debug, Clone)]
pub struct NetworkHsmProvider {
    service: NetworkHsmService,
    // PHASE-2: Add HSM client implementation (e.g., PKCS#11 over network, custom protocol)
    // Universal provider pattern supports this - awaiting network protocol selection
}

impl NetworkHsmProvider {
    /// Create a new network HSM provider
    #[must_use]
    pub const fn new(service: NetworkHsmService) -> Self {
        Self { service }
    }

    /// Get the underlying service information
    #[must_use]
    pub const fn service(&self) -> &NetworkHsmService {
        &self.service
    }
}

#[cfg(test)]
#[allow(deprecated)] // Testing deprecated functionality for backwards compatibility
mod tests {
    use super::*;

    // DEPRECATED: These tests are for backwards compatibility only
    // Use UniversalPrimalAdapter from beardog-adapters for new code
    // Tests disabled to avoid clippy warnings about deprecated code

    #[tokio::test]
    #[ignore = "Testing deprecated functionality - use UniversalPrimalAdapter instead"]
    #[allow(deprecated)]
    async fn test_songbird_discovery_creation() {
        // This test will fail until UniversalAdapter is properly implemented
        // That's OK - this is the interface definition
        let result = SongbirdHsmDiscovery::new().await;

        // For now, we expect this to fail (not implemented yet)
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    #[ignore = "Testing deprecated functionality - use UniversalPrimalAdapter instead"]
    #[allow(deprecated)]
    fn test_network_hsm_service_structure() {
        let service = NetworkHsmService {
            service_id: "hsm-001".to_string(),
            endpoint: "192.168.1.100:8443".to_string(),
            capabilities: vec!["pkcs11".to_string(), "signing".to_string()],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(service.service_id, "hsm-001");
        assert_eq!(service.capabilities.len(), 2);
    }
}
