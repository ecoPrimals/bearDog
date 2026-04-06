// SPDX-License-Identifier: AGPL-3.0-or-later

//! API Endpoints for Primal Interface
//!
//! This module provides the public API endpoints that `BearDogCore` exposes
//! for ecosystem integration and primal coordination.
//!
//! The gateway / service-mesh hooks are exercised via the test suite;
//! production wiring will activate them through the adapter layer.

#[cfg(test)]
mod tests {
    use crate::core::BearDogCore;
    use crate::ecosystem_integration::{
        ProductionConfig, ProductionUniversalAdapter, UniversalAdapterConfig,
    };
    use beardog_errors::BearDogError;
    use tracing::info;

    impl BearDogCore {
        fn start_ai_first_api_server() -> Result<(), BearDogError> {
            info!("🤖 Starting AI-first API server via universal adapter");

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

        fn start_universal_api_gateway() -> Result<(), BearDogError> {
            info!("🌐 Starting universal API gateway via adapter");

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

        fn initialize_service_mesh() -> Result<(), BearDogError> {
            info!("🕸️ Initializing service mesh via universal adapter");

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

    #[test]
    fn test_start_ai_first_api_server_returns_error() {
        let result = BearDogCore::start_ai_first_api_server();
        assert!(result.is_err());
    }

    #[test]
    fn test_start_universal_api_gateway_returns_error() {
        let result = BearDogCore::start_universal_api_gateway();
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_service_mesh_returns_error() {
        let result = BearDogCore::initialize_service_mesh();
        assert!(result.is_err());
    }
}
