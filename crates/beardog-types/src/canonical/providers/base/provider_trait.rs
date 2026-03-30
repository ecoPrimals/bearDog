// SPDX-License-Identifier: AGPL-3.0-only

//! [`BaseProvider`] — lifecycle, health, and capability surface for all providers.

use beardog_errors::BearDogError;

use super::configuration::ProviderConfiguration;
use super::schema::ConfigurationSchema;
use super::super::{PerformanceMetrics, ProviderCapability, ProviderHealth, ProviderInfo};

/// Base trait that all BearDog providers must implement
///
/// This trait establishes the fundamental interface for all providers in the
/// BearDog ecosystem, ensuring consistent behavior for lifecycle management,
/// health monitoring, and capability discovery.
pub trait BaseProvider: Send + Sync {
    /// Get provider information
    fn provider_info(&self) -> ProviderInfo;

    /// Perform a health check on the provider
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderHealth, BearDogError>> + Send;

    /// Get provider performance metrics
    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<PerformanceMetrics, BearDogError>> + Send;

    /// Get provider capabilities
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Initialize the provider with configuration
    fn initialize(
        &mut self,
        config: ProviderConfiguration,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Shutdown the provider gracefully
    fn shutdown(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Check if provider supports a specific capability
    fn supports_capability(&self, capability: &str) -> bool {
        self.capabilities().iter().any(|c| c.name == capability)
    }

    /// Get provider configuration schema
    fn configuration_schema(&self) -> ConfigurationSchema {
        ConfigurationSchema::default()
    }

    /// Validate provider configuration
    fn validate_configuration(
        &self,
        config: &ProviderConfiguration,
    ) -> Result<(), BearDogError> {
        // Default implementation - providers can override for custom validation
        if config.provider_id.is_empty() {
            return Err(BearDogError::business("Provider ID cannot be empty".to_string()));
        }
        Ok(())
    }
}
