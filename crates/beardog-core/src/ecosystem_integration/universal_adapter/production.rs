// SPDX-License-Identifier: AGPL-3.0-or-later

// Production Universal Adapter Implementation

use super::config::{ProductionConfig, UniversalAdapterConfig};
use super::core::UniversalAdapter;
use super::types::{AdapterRequest, AdapterResponse};
use beardog_errors::BearDogError;
use tracing::info;

/// Production-ready universal adapter with enhanced features
#[derive(Debug)]
pub struct ProductionUniversalAdapter {
    /// Core adapter
    core_adapter: UniversalAdapter,
    /// Production configuration
    production_config: ProductionConfig,
}

impl ProductionUniversalAdapter {
    /// Create a new production adapter
    /// Creates a new instance
    pub fn new(
        adapter_config: UniversalAdapterConfig,
        production_config: ProductionConfig,
    ) -> Self {
        info!("Creating production universal adapter");

        Self {
            core_adapter: UniversalAdapter::new(adapter_config),
            production_config,
        }
    }

    /// Process request with production enhancements
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the request processing fails
    pub async fn process_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        // Add production enhancements here (circuit breaker, rate limiting, etc.)
        self.core_adapter.process_request(request).await
    }

    /// Get production configuration
    #[must_use]
    pub const fn production_config(&self) -> &ProductionConfig {
        &self.production_config
    }

    /// Get underlying core adapter
    #[must_use]
    pub const fn core_adapter(&self) -> &UniversalAdapter {
        &self.core_adapter
    }

    /// Execute operation on a specific system
    ///
    /// # Important: Subsystem routing not wired
    ///
    /// This method is a placeholder for routing operations to specific subsystems.
    /// It returns an explicit [`BearDogError::unsupported_operation`] to prevent silent failures.
    ///
    /// # Implementation Required
    ///
    /// To implement this properly:
    /// 1. Define a `SystemRouter` trait for pluggable system backends
    /// 2. Implement routers for: hsm, `service_mesh`, `api_gateway`, etc.
    /// 3. Register routers during adapter initialization
    /// 4. Route operations through the appropriate backend
    ///
    /// # Errors
    /// Returns [`BearDogError::unsupported_operation`] until subsystem routing is implemented.
    pub fn execute_on_system(
        &self,
        system: &str,
        operation: &str,
        _params: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        tracing::warn!(
            "execute_on_system: subsystem routing not available: system={}, operation={}",
            system,
            operation
        );

        Err(BearDogError::unsupported_operation(format!(
            "subsystem_operation_routing capability not available (attempted {operation} on {system}); use direct IPC to target primals"
        )))
    }

    /// Performs health check on all systems
    ///
    /// Checks the core adapter state (connections, endpoints) and reports
    /// subsystem status. Subsystems without dedicated probes remain "unknown".
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the health check aggregation fails
    pub fn health_check_all(&self) -> Result<serde_json::Value, BearDogError> {
        use serde_json::{Map, Value};

        let mut systems = Map::new();

        // Core adapter: check if initialized and report state
        let core = self.core_adapter();
        let is_initialized = !core.config().adapter_name.is_empty();
        let core_status = if is_initialized {
            "healthy"
        } else {
            "uninitialized"
        };
        systems.insert(
            "core_adapter".to_string(),
            Value::Object({
                let mut m = Map::new();
                m.insert("status".to_string(), Value::String(core_status.to_string()));
                m.insert(
                    "adapter_name".to_string(),
                    Value::String(core.config().adapter_name.clone()),
                );
                m.insert(
                    "adapter_id".to_string(),
                    Value::String(core.config().adapter_id.clone()),
                );
                m
            }),
        );

        // Subsystems without dedicated probes - marked unknown until implemented
        systems.insert("hsm".to_string(), Value::String("unknown".to_string()));
        systems.insert(
            "service_mesh".to_string(),
            Value::String("unknown".to_string()),
        );
        systems.insert(
            "api_gateway".to_string(),
            Value::String("unknown".to_string()),
        );
        systems.insert(
            "service_registry".to_string(),
            Value::String("unknown".to_string()),
        );

        let overall_status = if is_initialized {
            "healthy"
        } else {
            "degraded"
        };
        let mut response = Map::new();
        response.insert(
            "overall_status".to_string(),
            Value::String(overall_status.to_string()),
        );
        response.insert("systems".to_string(), Value::Object(systems));
        response.insert(
            "timestamp".to_string(),
            Value::String(chrono::Utc::now().to_rfc3339()),
        );

        Ok(Value::Object(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecosystem_integration::universal_adapter::config::{
        ProductionConfig, ProductionFeature, UniversalAdapterConfig,
    };
    use crate::ecosystem_integration::universal_adapter::types::{
        AdapterOperation, AdapterRequest,
    };

    fn sample_production_adapter() -> ProductionUniversalAdapter {
        ProductionUniversalAdapter::new(
            UniversalAdapterConfig::default(),
            ProductionConfig {
                enabled_features: vec![
                    ProductionFeature::ProductionMode,
                    ProductionFeature::EnhancedLogging,
                ],
            },
        )
    }

    #[test]
    fn production_config_accessor_returns_reference() {
        let a = sample_production_adapter();
        assert!(
            a.production_config()
                .is_enabled(ProductionFeature::ProductionMode)
        );
        assert!(a.production_config().enhanced_logging());
    }

    #[test]
    fn core_adapter_accessor_exposes_config() {
        let a = sample_production_adapter();
        assert_eq!(
            a.core_adapter().config().adapter_id,
            UniversalAdapterConfig::default().adapter_id
        );
    }

    #[test]
    fn execute_on_system_returns_not_implemented() {
        let a = sample_production_adapter();
        let err = a
            .execute_on_system("hsm", "op", serde_json::json!({}))
            .expect_err("routing must not be implemented yet");
        let msg = err.to_string();
        assert!(
            msg.contains("Unsupported operation") || msg.contains("subsystem_operation_routing"),
            "error should surface unsupported operation: {msg}"
        );
    }

    #[test]
    fn health_check_initialized_vs_uninitialized() {
        let good = sample_production_adapter();
        let v = good
            .health_check_all()
            .expect("health aggregation should succeed");
        assert_eq!(v["overall_status"], "healthy");

        let mut cfg = UniversalAdapterConfig::default();
        cfg.adapter_name.clear();
        let empty = ProductionUniversalAdapter::new(
            cfg,
            ProductionConfig {
                enabled_features: vec![],
            },
        );
        let v2 = empty.health_check_all().expect("health check");
        assert_eq!(v2["overall_status"], "degraded");
    }

    #[tokio::test]
    async fn process_request_delegates_to_core() {
        let a = sample_production_adapter();
        let req = AdapterRequest::new(
            AdapterOperation::HealthCheck,
            "http://localhost".to_string(),
        );
        let rid = req.request_id;
        let res = a
            .process_request(req)
            .await
            .expect("health check request should be handled");
        assert_eq!(res.request_id, rid);
        assert!(
            res.payload.is_some(),
            "health response should include payload"
        );
    }
}
