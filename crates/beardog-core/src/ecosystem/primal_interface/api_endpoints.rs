//! API Endpoints for Primal Interface
//!
//! This module provides the public API endpoints that `BearDogCore` exposes
//! for ecosystem integration and primal coordination.

use crate::core::BearDogCore;
use crate::ecosystem_integration::{
    ProductionConfig, ProductionUniversalAdapter, UniversalAdapterConfig,
};
use beardog_errors::BearDogError;
use serde_json::json;
use tracing::info;

impl BearDogCore {
    /// Start the AI-first API server
    ///
    /// Initializes an API server optimized for AI-driven routing, load balancing,
    /// and intelligent request handling.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the server starts successfully.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if the server fails to start or initialize.
    #[allow(dead_code)]
    pub(crate) fn start_ai_first_api_server(&self) -> Result<(), BearDogError> {
        info!("🤖 Starting AI-first API server via universal adapter");

        let _payload = json!({
            "type": "ai_first",
            "capabilities": ["routing", "load_balancing", "versioning"]
        });

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system("api_gateway", "start_server", json!({}))?;
        Ok(())
    }

    /// Start the universal API gateway
    ///
    /// Initializes the universal API gateway that provides routing to ecosystem
    /// services via the service mesh pattern.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the gateway starts successfully.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if the gateway fails to initialize.
    #[allow(dead_code)]
    pub(crate) fn start_universal_api_gateway(&self) -> Result<(), BearDogError> {
        info!("🌐 Starting universal API gateway via adapter");

        let _payload = json!({
            "features": ["universal_routing", "service_mesh", "versioning"]
        });

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system("service_mesh", "initialize_gateway", json!({}))?;
        Ok(())
    }

    /// Initialize the service mesh
    ///
    /// Sets up the service mesh infrastructure for primal-to-primal communication
    /// and capability-based service discovery.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if initialization succeeds.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if service mesh initialization fails.
    #[allow(dead_code)]
    pub(crate) fn initialize_service_mesh(&self) -> Result<(), BearDogError> {
        info!("🕸️ Initializing service mesh via universal adapter");

        let _payload = json!({
            "capabilities": ["discovery", "routing", "load_balancing"]
        });

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system("service_mesh", "initialize", json!({}))?;
        Ok(())
    }
}
