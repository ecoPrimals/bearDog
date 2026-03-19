// SPDX-License-Identifier: AGPL-3.0-only

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
    pub fn production_config(&self) -> &ProductionConfig {
        &self.production_config
    }

    /// Get underlying core adapter
    #[must_use]
    pub const fn core_adapter(&self) -> &UniversalAdapter {
        &self.core_adapter
    }

    /// Execute operation on a specific system
    ///
    /// # Important: Not Yet Implemented
    ///
    /// This method is a placeholder for routing operations to specific subsystems.
    /// Currently returns an explicit error to prevent silent failures in production.
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
    /// Returns `Err(BearDogError::not_implemented)` until real routing is implemented
    pub fn execute_on_system(
        &self,
        system: &str,
        operation: &str,
        _params: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        // Log the attempted operation for debugging
        tracing::warn!(
            "⚠️ execute_on_system called but not implemented: system={}, operation={}",
            system,
            operation
        );

        // Return explicit error instead of fake success
        // This prevents silent failures in production
        Err(BearDogError::not_implemented(&format!(
            "System operation routing not implemented. \
             Attempted: {operation} on {system}. \
             Use direct IPC to target primals instead."
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
