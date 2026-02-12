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
    /// # Important: Returns Stub Status
    ///
    /// This method returns a response indicating that health checks are not yet
    /// implemented with real system probing. The status is marked as "unknown"
    /// rather than falsely claiming "healthy".
    ///
    /// # Implementation Required
    ///
    /// To implement real health checks:
    /// 1. Define health check endpoints for each subsystem
    /// 2. Implement timeout-based probing
    /// 3. Aggregate results with proper error handling
    /// 4. Consider circuit breaker patterns for failing systems
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the health check aggregation fails
    pub fn health_check_all(&self) -> Result<serde_json::Value, BearDogError> {
        tracing::warn!("⚠️ health_check_all: Real health probing not implemented");

        use serde_json::{Map, Value};

        // Return honest "unknown" status instead of fake "healthy"
        let mut systems = Map::new();
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

        let mut response = Map::new();
        response.insert(
            "overall_status".to_string(),
            Value::String("unknown".to_string()),
        );
        response.insert(
            "note".to_string(),
            Value::String("Health probing not implemented - status is assumed unknown".to_string()),
        );
        response.insert("systems".to_string(), Value::Object(systems));
        response.insert(
            "timestamp".to_string(),
            Value::String(chrono::Utc::now().to_rfc3339()),
        );

        Ok(Value::Object(response))
    }
}
