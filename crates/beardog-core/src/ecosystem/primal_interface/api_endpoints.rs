use crate::core::BearDogCore;
use crate::ecosystem_integration::{
    ProductionConfig, ProductionUniversalAdapter, UniversalAdapterConfig,
};
use beardog_errors::BearDogError;
use serde_json::json;
use tracing::info;

impl BearDogCore {
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
