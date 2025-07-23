//! Licensed External Function Implementations
//!
//! This module provides implementations for external system integrations that
//! require BearDog-signed licenses. All functions check license validity before
//! executing.

use crate::licensing::LicenseManager;
use async_trait::async_trait;
use beardog_errors::BearDogResult;
use serde_json::Value;
use std::collections::HashMap;

// Import all integration modules
pub mod aws_kms;
pub mod grafana;
pub mod kubernetes;
pub mod prometheus;

// Re-export all integration handlers
pub use aws_kms::AwsKmsIntegration;
pub use grafana::GrafanaDashboards;
pub use kubernetes::KubernetesIntegration;
pub use prometheus::PrometheusExport;

/// External function handler trait
#[async_trait]
pub trait ExternalFunctionHandler {
    /// Function name for licensing checks
    fn function_name(&self) -> &str;

    /// Execute external function with self-aware licensing
    async fn execute(
        &self,
        license_manager: &LicenseManager,
        operation: &str,
        payload: Value,
    ) -> BearDogResult<Value>;
}

/// Registry for managing external function handlers
pub struct ExternalFunctionRegistry {
    handlers: HashMap<String, Box<dyn ExternalFunctionHandler>>,
}

impl ExternalFunctionRegistry {
    /// Create new registry with all default handlers
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };

        // Register all available handlers
        registry.register("kubernetes", Box::new(KubernetesIntegration));
        registry.register("prometheus", Box::new(PrometheusExport));
        registry.register("grafana", Box::new(GrafanaDashboards));
        registry.register("aws_kms", Box::new(AwsKmsIntegration));

        registry
    }

    /// Register a new external function handler
    pub fn register(&mut self, name: &str, handler: Box<dyn ExternalFunctionHandler>) {
        self.handlers.insert(name.to_string(), handler);
    }

    /// Execute an external function by name
    pub async fn execute(
        &self,
        function_name: &str,
        payload: Value,
        license_manager: &LicenseManager,
    ) -> BearDogResult<Value> {
        match self.handlers.get(function_name) {
            Some(handler) => {
                handler
                    .execute(license_manager, function_name, payload)
                    .await
            }
            None => Ok(serde_json::json!({
                "error": "function_not_found",
                "message": format!("External function '{}' not found", function_name),
                "available_functions": self.list_functions()
            })),
        }
    }

    /// List all available external functions
    pub fn list_functions(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

impl Default for ExternalFunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
