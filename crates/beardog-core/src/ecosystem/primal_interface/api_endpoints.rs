//! API Endpoints for Primal Interface
//!
//! This module provides the public API endpoints that `BearDogCore` exposes
//! for ecosystem integration and primal coordination.

use crate::core::BearDogCore;
use crate::ecosystem_integration::{
    ProductionConfig, ProductionUniversalAdapter, UniversalAdapterConfig,
};
use beardog_errors::BearDogError;
// json! macro no longer used - using explicit JSON construction
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
    pub(crate) fn start_ai_first_api_server() -> Result<(), BearDogError> {
        info!("🤖 Starting AI-first API server via universal adapter");

        let _payload = {
            use serde_json::{Map, Value};
            let mut payload = Map::new();
            payload.insert("type".to_string(), Value::String("ai_first".to_string()));
            payload.insert(
                "capabilities".to_string(),
                Value::Array(vec![
                    Value::String("routing".to_string()),
                    Value::String("load_balancing".to_string()),
                    Value::String("versioning".to_string()),
                ]),
            );
            Value::Object(payload)
        };

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system(
            "api_gateway",
            "start_server",
            serde_json::Value::Object(serde_json::Map::new()),
        )?;
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
    pub(crate) fn start_universal_api_gateway() -> Result<(), BearDogError> {
        info!("🌐 Starting universal API gateway via adapter");

        let _payload = {
            use serde_json::{Map, Value};
            let mut payload = Map::new();
            payload.insert(
                "features".to_string(),
                Value::Array(vec![
                    Value::String("universal_routing".to_string()),
                    Value::String("service_mesh".to_string()),
                    Value::String("versioning".to_string()),
                ]),
            );
            Value::Object(payload)
        };

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system(
            "service_mesh",
            "initialize_gateway",
            serde_json::Value::Object(serde_json::Map::new()),
        )?;
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
    pub(crate) fn initialize_service_mesh() -> Result<(), BearDogError> {
        info!("🕸️ Initializing service mesh via universal adapter");

        let _payload = {
            use serde_json::{Map, Value};
            let mut payload = Map::new();
            payload.insert(
                "capabilities".to_string(),
                Value::Array(vec![
                    Value::String("discovery".to_string()),
                    Value::String("routing".to_string()),
                    Value::String("load_balancing".to_string()),
                ]),
            );
            Value::Object(payload)
        };

        // Create proper config for ProductionUniversalAdapter
        let config = UniversalAdapterConfig::default();
        let production_config = ProductionConfig::default();
        let adapter = ProductionUniversalAdapter::new(config, production_config);

        adapter.execute_on_system(
            "service_mesh",
            "initialize",
            serde_json::Value::Object(serde_json::Map::new()),
        )?;
        Ok(())
    }
}
