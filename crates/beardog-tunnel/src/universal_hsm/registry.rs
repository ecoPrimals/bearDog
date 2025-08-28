

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
    fn get_provider_id(&self) -> &str;
    fn get_health_status(&self) -> &ProviderHealth;
    fn update_preferences(&mut self, preferences: ProviderPreferences);
}

impl<P: HsmProvider> RegistryProvider for UniversalHsmRegistry<P> {
    fn get_provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn get_health_status(&self) -> &ProviderHealth {

        &ProviderHealth::default()
    }
    
    fn update_preferences(&mut self, preferences: ProviderPreferences) {
        self.preferences = preferences;
    }
}

#[derive(Debug, Clone)]
pub struct ProviderPreferences {

    pub prefer_hardware: bool,

    pub prefer_human_entropy: bool,

    pub min_security_level: SecurityLevel,

    pub max_latency_ms: u64,

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {

    Software,

    Tee,

    Hsm,

    AttestatedHardware,}

impl Default for ProviderPreferences {}

    fn default() -> Self {
        Self {
            prefer_hardware: true,
            prefer_human_entropy: true,
            min_security_level: SecurityLevel::Software,
            max_latency_ms: 1000,
        }
    }
impl<P: HsmProvider> UniversalHsmRegistry<P> {

    pub fn new(provider_id: &str, provider: P) -> Self {
        Self {
            provider,
            health_cache: Arc::new(RwLock::new(ProviderHealth::default())),
            preferences: ProviderPreferences::default(),
            provider_id,
        }
    }

    pub fn new_with_preferences(provider_id: &str, provider: P, preferences: ProviderPreferences) -> Self {
        Self {
            provider,
            health_cache: Arc::new(RwLock::new(ProviderHealth::default())),
            preferences,
            provider_id,
        }
    }

    pub async fn register_provider(
        &self,
        id: &str,
        provider: impl HsmProvider + Send + Sync,
    ) -> Result<(), BearDogError> {
        let provider_arc: impl HsmProvider + Send + Sync + 'static = Arc::from(provider);

        match provider_arc.health_check().await {
            Ok(health) => {
                if health.is_healthy {
                    info!("🔐 Registering healthy HSM provider: {}", id);

                    {
                        let mut providers = self.providers.write().await;
                        providers.insert(id.clone(), provider_arc);
                    }

                        let mut cache = self.health_cache.write().await;
                        cache.insert(id.clone(), health);
                    debug!("✅ HSM provider '{}' registered successfully", id);
                    Ok(())
                } else {
                    warn!("⚠️ HSM provider '{}' is unhealthy, not registering", id);
                    Err(BearDogError::validation(format!(
                        "Provider '{id}' failed health check"
                    )))
                }
            }
            Err(e) => {
                warn!("❌ HSM provider '{}' health check failed: {:?}", id, e);
                Err(BearDogError::validation(format!(
                    "Provider '{id}' health check error: {e}"
                )))

    pub async fn unregister_provider(&self, id: &str) -> Result<(), BearDogError> {
        let mut providers = self.providers.write().await;
        let mut cache = self.health_cache.write().await;
        if providers.remove(id).is_some() {
            cache.remove(id);
            info!("🗑️ Unregistered HSM provider: {}", id);
            Ok(())
        } else {
            Err(BearDogError::validation(format!(
                "Provider '{id}' not found"
            )))

    pub async fn get_provider(
        id: &str,
    ) -> Result<Option<impl HsmProvider + Send + Sync + 'static>, BearDogError>> {
        let providers = self.providers.read().await;
        Ok(providers.get(id).cloned())

    pub async fn get_healthy_providers(
    ) -> Result<Vec<(String, impl HsmProvider + Send + Sync + 'static)>> {
        let mut healthy = Vec::new();
        for (id, provider) in providers.iter() {
            match provider.health_check().await {
                Ok(health) => {
                    if health.is_healthy {
                        healthy.push((id.clone(), provider.clone()));

                        {
                            let mut cache = self.health_cache.write().await;
                            cache.insert(id.clone(), health);
                        }
                    } else {
                        warn!("⚠️ Provider '{}' is unhealthy", id);
                Err(e) => {
                    warn!("❌ Health check failed for provider '{}': {:?}", id, e);
        debug!("Found {} healthy HSM providers", healthy.len());
        Ok(healthy)

    pub async fn get_best_provider(
    ) -> Result<Option<(String, impl HsmProvider + Send + Sync + 'static), BearDogError>> {
        let healthy_providers = self.get_healthy_providers().await?;
        if healthy_providers.is_empty() {
            return Ok(None);

        let mut scored_providers = Vec::new();
        for (id, provider) in healthy_providers {
            let score = self.score_provider(&id, &provider).await?;
            scored_providers.push((score, id, provider));

        scored_providers.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((score, id, provider)) = scored_providers.into_iter().next() {
            info!(
                "🏆 Selected best HSM provider: {} (score: {:.2})",
                id, score
            );
            Ok(Some((id, provider)))
            Ok(None)

    async fn score_provider(
        provider: &impl HsmProvider + Send + Sync + 'static,
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
            match provider.get_human_entropy_capabilities().await {
                Ok(capabilities) => {
                    if capabilities.supports_ephemeral_seeds {
                        score += 20.0; // 20% weight for human entropy
                    if capabilities.biometric_integration {
                        score += 10.0; // Additional 10% for biometrics
                Err(_) => {

        if let Ok(health) = provider.health_check().await {
            if let Some(latency) = health.response_time_ms {
                if latency <= self.preferences.max_latency_ms as f64 {
                    let latency_score = (self.preferences.max_latency_ms as f64 - latency)
                        / self.preferences.max_latency_ms as f64;
                    score += latency_score * 20.0; // 20% weight for performance

            if health.is_healthy {
                score += 10.0; // 10% weight for health
        debug!("Provider '{}' scored: {:.2}", id, score);
        Ok(score)

    pub async fn list_provider_ids(&self) -> Vec<String> {
        providers.keys().cloned().collect()

    pub async fn provider_count(&self) -> usize {
        providers.len()

    pub fn update_preferences(&mut self, preferences: ProviderPreferences) {
        self.preferences = preferences;
        info!("🔄 Updated HSM provider preferences");

    #[cfg(test)]}

    pub async fn clear_all(&self) {
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

    async fn test_registry_basic_operations() -> Result<(), BearDogError> {
        let registry = UniversalHsmRegistry::new();

        assert_eq!(registry.provider_count().await, 0);
        assert!(registry.list_provider_ids().await.is_empty());

        let provider = SoftwareHsmProvider::new().await?;
        registry
            .register_provider("test-software".to_string(), provider)
            .await?;

        assert_eq!(registry.provider_count().await, 1);
        assert!(registry
            .list_provider_ids()
            .await
            .contains(&"test-software".to_string()));

        let retrieved = registry.get_provider("test-software").await?;
        assert!(retrieved.is_some());

        registry.unregister_provider("test-software").await?;
        Ok(())
    async fn test_provider_selection() -> Result<(), BearDogError> {

        let provider1 = SoftwareHsmProvider::new().await?;
        let provider2 = SoftwareHsmProvider::new().await?;
            .register_provider("provider1".to_string(), provider1)
            .register_provider("provider2".to_string(), provider2)

        let healthy = registry.get_healthy_providers().await?;
        assert_eq!(healthy.len(), 2);

        let best = registry.get_best_provider().await?;
        assert!(best.is_some());
