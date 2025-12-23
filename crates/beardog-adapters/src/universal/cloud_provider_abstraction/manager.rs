

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use std::collections::HashMap;
use super::traits::CloudProvider;
use super::types::*;

pub struct CloudProviderManager {
    providers: HashMap<String, Box<dyn CloudProvider>>,
    primary_provider: String,
    fallback_providers: Vec<String>,
}

impl CloudProviderManager {


/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            providers: HashMap::with_capacity(16),
            primary_provider: "self-hosted".to_string(), provider: Box<dyn CloudProvider>) {
        self.providers.insert(name, provider);
    }


/// Set Primary operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Sets primary
    /// Sets primary
    pub fn set_primary(&mut self, provider_name: &str) -> Result<(), BearDogError> {
        if self.providers.contains_key(&provider_name) {
            self.primary_provider = provider_name;
            Ok(())
        } else {
            Err(BearDogError::configuration(format!(
                "Provider '{provider_name}' not found"
            )))
        }
    }


/// Add Fallback operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn add_fallback(&mut self, provider_name: &str) -> Result<(), BearDogError> {
        if self.providers.contains_key(&provider_name) {
            self.fallback_providers.push(provider_name);
            Ok(())
        } else {
            Err(BearDogError::configuration(format!(
                "Fallback provider '{provider_name}' not found"
            )))
        }
    }

/// Execute With Failover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Executes with_failover
    /// Executes with_failover
    pub fn execute_with_failover<F, T>(&self, operation: F) -> Result<T, BearDogError>
    where
        F: Fn(&dyn CloudProvider) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, BearDogError>> + Send + '_>>,
    {

        if let Some(provider) = self.providers.get(&self.primary_provider) {
            match operation(provider.as_ref()) {
                Ok(result) => return Ok(result),
                Err(_) => {

                }
            }
        }

        for fallback_name in &self.fallback_providers {
            if let Some(provider) = self.providers.get(fallback_name) {
                match operation(provider.as_ref()) {
                    Ok(result) => return Ok(result),
                    Err(_) => continue,
                }
            }
        }

        Err(BearDogError::configuration(
            "All cloud providers failed"))
    }

/// Get Provider Health operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets provider_health
    /// Gets provider_health
    pub fn get_provider_health(&self, provider_name: &str) -> Result<ProviderHealth, BearDogError> {
        if let Some(provider) = self.providers.get(provider_name) {
            provider.health_check()
        } else {
            Err(BearDogError::configuration(format!(
                "Provider '{provider_name}' not found"
            )))
        }
    }

/// List Providers operation.
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

/// Get Provider Capabilities operation.
    /// Gets provider_capabilities
    /// Gets provider_capabilities
    pub fn get_provider_capabilities(&self, provider_name: &str) -> Option<Vec<CloudServiceType>> {
        self.providers
            .get(provider_name)
            .map(|provider| provider.supported_services())
    }
}

impl Default for CloudProviderManager {
    fn default() -> Self {
        Self::new()
    }
}
