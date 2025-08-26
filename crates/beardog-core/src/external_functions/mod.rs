

use crate::licensing::LicenseManager;
use beardog_errors::BearDogResult;
use serde_json::Value;
use std::collections::HashMap;

pub mod aws_kms;
pub mod grafana;
pub mod kubernetes;
pub mod prometheus;

pub use aws_kms::AwsKmsIntegration;
pub use grafana::GrafanaDashboards;
pub use kubernetes::KubernetesIntegration;
pub use prometheus::PrometheusExport;

#[allow(async_fn_in_trait)]
pub trait ExternalFunctionHandler: Send + Sync + Clone {

    fn function_name(&self) -> &str;

    async fn execute(
        &self,
        license_manager: &LicenseManager,
        operation: &str,
        payload: Value,
    ) -> BearDogResult<Value>;
}

pub struct ExternalFunctionRegistry<H: ExternalFunctionHandler> {
    handlers: HashMap<String, H>,
}

impl<H: ExternalFunctionHandler> ExternalFunctionRegistry<H> {

    pub fn new() -> Self {
        Self {
            handlers: ahash::HashMap::default(),
        }
    }

    pub fn register_handler(&mut self, name: &str, handler: H) {
        self.handlers.insert(name, handler);
    }

    pub async fn execute_function(
        &self,
        function_name: &str,
        license_manager: &LicenseManager,
        operation: &str,
        payload: Value,
    ) -> BearDogResult<Value> {
        let handler = self.handlers
            .get(function_name)
            .ok_or_else(|| BearDogError::not_found(format_args!("Function handler not found: {}", function_name).to_string()))?;
        
        handler.execute(license_manager, operation, payload).await
    }

    pub fn get_function_names(&self) -> Vec<&String> {
        self.handlers.keys().collect()
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

impl<H: ExternalFunctionHandler> Default for ExternalFunctionRegistry<H> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub enum UnifiedExternalHandler {
    Kubernetes(KubernetesIntegration),
    Prometheus(PrometheusExport),
    Grafana(GrafanaDashboards),
    AwsKms(AwsKmsIntegration),
}

impl ExternalFunctionHandler for UnifiedExternalHandler {
    fn function_name(&self) -> &str {
        match self {
            UnifiedExternalHandler::Kubernetes(h) => h.function_name(),
            UnifiedExternalHandler::Prometheus(h) => h.function_name(),
            UnifiedExternalHandler::Grafana(h) => h.function_name(),
            UnifiedExternalHandler::AwsKms(h) => h.function_name(),
        }
    }

    async fn execute(
        &self,
        license_manager: &LicenseManager,
        operation: &str,
        payload: Value,
    ) -> BearDogResult<Value> {
        match self {
            UnifiedExternalHandler::Kubernetes(h) => h.execute(license_manager, operation, payload).await,
            UnifiedExternalHandler::Prometheus(h) => h.execute(license_manager, operation, payload).await,
            UnifiedExternalHandler::Grafana(h) => h.execute(license_manager, operation, payload).await,
            UnifiedExternalHandler::AwsKms(h) => h.execute(license_manager, operation, payload).await,
        }
    }
}

pub type UnifiedExternalFunctionRegistry = ExternalFunctionRegistry<UnifiedExternalHandler>;

impl UnifiedExternalFunctionRegistry {

    pub fn with_default_handlers() -> Self {
        let mut registry = Self::new();

        registry.register_handler(
            "kubernetes".to_string(),
            UnifiedExternalHandler::Kubernetes(KubernetesIntegration),
        );
        registry.register_handler(
            "prometheus".to_string(),
            UnifiedExternalHandler::Prometheus(PrometheusExport),
        );
        registry.register_handler(
            "grafana".to_string(),
            UnifiedExternalHandler::Grafana(GrafanaDashboards),
        );
        registry.register_handler(
            "aws_kms".to_string(),
            UnifiedExternalHandler::AwsKms(AwsKmsIntegration),
        );
        
        registry
    }
}

