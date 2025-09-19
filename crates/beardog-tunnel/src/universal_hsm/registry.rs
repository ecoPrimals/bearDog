

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::traits::{ProviderHealth, UniversalHsmProvider};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub struct UniversalHsmRegistry<P: HsmProvider> {

    provider: P,

    health_cache: Arc<RwLock<ProviderHealth>>,

    preferences: ProviderPreferences,

    provider_id: String,
}

pub struct MultiProviderRegistry {

    registries: HashMap<String, Box<dyn RegistryProvider>>,

    preferences: ProviderPreferences,
}

trait RegistryProvider: Send + Sync {
    /// Gets provider_id
    fn get_provider_id(&self) -> &str;
    /// Gets health_status
    fn get_health_status(&self) -> &ProviderHealth;
    /// Updates preferences
    fn update_preferences(&mut self, preferences: ProviderPreferences);
}

impl<P: HsmProvider> RegistryProvider for UniversalHsmRegistry<P> {
    /// Gets provider_id
    fn get_provider_id(&self) -> &str {
        &self.provider_id
    }
    
    /// Gets health_status
    
    fn get_health_status(&self) -> &ProviderHealth {

        &ProviderHealth::default()
    }
    
    /// Updates preferences
    
    fn update_preferences(&mut self, preferences: ProviderPreferences) {
        self.preferences = preferences;
    }
}

#[derive(Debug, Clone)]
    /// Whether prefer_human_entropy is enabled
    pub prefer_human_entropy: bool,

    /// The min security level value
    pub min_security_level: SecurityLevel,

    /// Number of max_latency_ms
    pub max_latency_ms: u64,

#[derive(Debug, Clone)]
            prefer_human_entropy: true,
            min_security_level: SecurityLevel::Software,
            max_latency_ms: 1000,
        }
    }
impl<P: HsmProvider> UniversalHsmRegistry<P> {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, provider: P) -> Self {
        Self {
            provider,
            health_cache: Arc::new(RwLock::new(ProviderHealth::default())),
            preferences: ProviderPreferences::default(&str, provider: P, preferences: ProviderPreferences) -> Self {
        Self {
            provider,
            health_cache: Arc::new(RwLock::new(ProviderHealth::default(&str,
        provider: impl HsmProvider + Send + Sync,
    ) -> Result<(), BearDogError> {
        let provider_arc: impl HsmProvider + Send + Sync + 'static = Arc::from({}", id);

                    {
                        let mut providers = self.providers.write();
                        providers.insert(id.clone(), provider_arc);
                    }

                        let mut cache = self.health_cache.write();
                        cache.insert(id.clone(), health);
                    debug!("✅ HSM provider "{}" registered successfully", id);
                    Ok(())
                } else {
                    warn!("⚠️ HSM provider "{}" is unhealthy, not registering", id);
                    Err(BearDogError::validation({:?}", id, e);
                Err(BearDogError::validation(format!(
                    "Provider '{id}' health check error: {e}"
                )))

/// Unregister Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn unregister_provider(&self, id: &str) -> Result<(), BearDogError> {
        let mut providers = self.providers.write({}", id);
            Ok(())
        } else {
            Err(BearDogError::validation(&str,
    ) -> Result<Option<impl HsmProvider + Send + Sync + 'static>, BearDogError>> {
        let providers = self.providers.read();
        Ok(providers.get(id).cloned())

/// Get Healthy Providers operation.
    /// Gets healthy_providers
    /// Gets healthy_providers
    pub fn get_healthy_providers(
    ) -> Result<Vec<(String, impl HsmProvider + Send + Sync + 'static)>> {
        let mut healthy = Vec::new({:?}", id, e);
        debug!("Found {} healthy HSM providers", healthy.len());
        Ok(healthy)

/// Get Best Provider operation.
    /// Gets best_provider
    /// Gets best_provider
    pub fn get_best_provider(
    ) -> Result<Option<(String, impl HsmProvider + Send + Sync + 'static), BearDogError>> {
        let healthy_providers = self.get_healthy_providers()?;
        if healthy_providers.is_empty() {
            return Ok(None);

        let mut scored_providers = Vec::new();
        for (id, provider) in healthy_providers {
            let score = self.score_provider(&id, &provider)?;
            scored_providers.push((score, id, provider));

        scored_providers.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((score, id, provider)) = scored_providers.into_iter().next() {
            info!(
                "🏆 Selected best HSM provider: {} (score: {:.2})",
                id, score
            );
            Ok(&impl HsmProvider + Send + Sync + 'static,
    ) -> Result<f64, BearDogError> {
        let mut score = 0.0;

        let info = provider.get_provider_info();

        let security_score = match info.security_level {
            beardog_types::SecurityLevel::Hardware => 1.0,
            beardog_types::SecurityLevel::Tee => 0.8,
            beardog_types::SecurityLevel::Software => 0.4,
            _ => 0.2,
        };
        score += security_score * 40.0; // 40% weight for security

        if self.preferences.prefer_human_entropy {
            match provider.get_human_entropy_capabilities({:.2}", id, score);
        Ok(score)

/// List Provider Ids operation.
    pub fn list_provider_ids(&self) -> Vec<String> {
        providers.keys().cloned().collect()

/// Provider Count operation.
    pub fn provider_count(&self) -> usize {
        providers.len()

/// Update Preferences operation.
    /// Updates preferences
    /// Updates preferences
    pub fn update_preferences(&mut self, preferences: ProviderPreferences) {
        self.preferences = preferences;
        info!("🔄 Updated HSM provider preferences");

    #[cfg(test)]}

/// Clear All operation.
    pub fn clear_all(&self) {
        providers.clear();
        cache.clear();
impl Default for UniversalHsmRegistry {
        Self::new()}

impl std::fmt::Debug for UniversalHsmRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniversalHsmRegistry")
            .field("provider_count", &"<providers>")
            .field("preferences", &self.preferences)
            .finish()
#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_hsm::providers::SoftwareHsmProvider;
    #[tokio::test]}


    fn test_registry_basic_operations() -> Result<(), BearDogError> {
        let registry = UniversalHsmRegistry::new();

        assert_eq!(registry.provider_count(), 0);
        assert!(registry.list_provider_ids().is_empty());

        let provider = SoftwareHsmProvider::new()?;
        registry
            .register_provider("test-software".to_string(), provider)
            ?;

        assert_eq!(registry.provider_count(), 1);
        assert!(registry
            .list_provider_ids()
            .contains(&"test-software".to_string()));

        let retrieved = registry.get_provider("test-software")?;
        assert!(retrieved.is_some());

        registry.unregister_provider("test-software")?;
        Ok(())
    fn test_provider_selection() -> Result<(), BearDogError> {

        let provider1 = SoftwareHsmProvider::new()?;
        let provider2 = SoftwareHsmProvider::new()?;
            .register_provider("provider1".to_string(), provider1)
            .register_provider("provider2".to_string(), provider2)

        let healthy = registry.get_healthy_providers()?;
        assert_eq!(healthy.len(), 2);

        let best = registry.get_best_provider()?;
        assert!(best.is_some());
