// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Universal HSM Registry
///
/// **UNIFIED PROVIDER MANAGEMENT**
/// This module provides the central registry for managing HSM providers across
/// all platforms and implementations. It replaces all fragmented registries
/// with a single, canonical provider management system.

use super::traits::{ProviderHealth, UniversalHsmProvider};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
/// Universal HSM provider registry
/// Central registry for managing all HSM providers across platforms.
/// Provides provider discovery, health monitoring, and selection capabilities.
pub struct UniversalHsmRegistry {
    /// Registered providers by ID
    providers: Arc<RwLock<HashMap<String, Arc<dyn HsmProvider>>>>,
    /// Provider health status cache
    health_cache: Arc<RwLock<HashMap<String, ProviderHealth>>>,
    /// Provider selection preferences
    preferences: ProviderPreferences,
}
/// Provider selection preferences
#[derive(Debug, Clone)]
pub struct ProviderPreferences {
    /// Prefer hardware-backed providers over software
    pub prefer_hardware: bool,
    /// Prefer providers with human entropy capabilities
    pub prefer_human_entropy: bool,
    /// Minimum required security level
    pub min_security_level: SecurityLevel,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
/// Security level classification for providers
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Software-only implementation
    Software,
    /// Trusted Execution Environment
    Tee,
    /// Hardware Security Module
    Hsm,
    /// Hardware with attestation
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
impl UniversalHsmRegistry {
    /// Create a new registry}


    pub fn new() -> Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            preferences: ProviderPreferences::default(),
    /// Create a new registry with custom preferences}


    pub fn new_with_preferences(preferences: ProviderPreferences) -> Self {
            preferences,
    /// Register a new HSM provider
    pub async fn register_provider(
        &self,
        id: String,
        provider: Box<dyn HsmProvider>,
    ) -> BearDogResult<()> {
        let provider_arc: Arc<dyn HsmProvider> = Arc::from(provider);
        // Check provider health before registration
        match provider_arc.health_check().await {
            Ok(health) => {
                if health.is_healthy {
                    info!("🔐 Registering healthy HSM provider: {}", id);
                    // Register the provider
                    {
                        let mut providers = self.providers.write().await;
                        providers.insert(id.clone(), provider_arc);
                    }
                    // Cache health status
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
    /// Unregister an HSM provider
    pub async fn unregister_provider(&self, id: &str) -> BearDogResult<()> {
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
    /// Get a specific provider by ID
    pub async fn get_provider(
        id: &str,
    ) -> BearDogResult<Option<Arc<dyn HsmProvider>>> {
        let providers = self.providers.read().await;
        Ok(providers.get(id).cloned())
    /// Get all healthy providers}


    pub async fn get_healthy_providers(
    ) -> BearDogResult<Vec<(String, Arc<dyn HsmProvider>)>> {
        let mut healthy = Vec::new();
        for (id, provider) in providers.iter() {
            match provider.health_check().await {
                Ok(health) => {
                    if health.is_healthy {
                        healthy.push((id.clone(), provider.clone()));
                        // Update health cache
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
    /// Get the best available provider based on preferences
    pub async fn get_best_provider(
    ) -> BearDogResult<Option<(String, Arc<dyn HsmProvider>)>> {
        let healthy_providers = self.get_healthy_providers().await?;
        if healthy_providers.is_empty() {
            return Ok(None);
        // Score providers based on preferences
        let mut scored_providers = Vec::new();
        for (id, provider) in healthy_providers {
            let score = self.score_provider(&id, &provider).await?;
            scored_providers.push((score, id, provider));
        // Sort by score (highest first)
        scored_providers.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((score, id, provider)) = scored_providers.into_iter().next() {
            info!(
                "🏆 Selected best HSM provider: {} (score: {:.2})",
                id, score
            );
            Ok(Some((id, provider)))
            Ok(None)
    /// Score a provider based on preferences
    async fn score_provider(
        provider: &Arc<dyn HsmProvider>,
    ) -> BearDogResult<f64> {
        let mut score = 0.0;
        // Get provider info
        let info = provider.get_provider_info();
        // Score based on security level
        let security_score = match info.security_level {
            beardog_types::SecurityLevel::Hardware => 1.0,
            beardog_types::SecurityLevel::Tee => 0.8,
            beardog_types::SecurityLevel::Software => 0.4,
            _ => 0.2,
        };
        score += security_score * 40.0; // 40% weight for security
        // Score based on human entropy capabilities
        if self.preferences.prefer_human_entropy {
            match provider.get_human_entropy_capabilities().await {
                Ok(capabilities) => {
                    if capabilities.supports_ephemeral_seeds {
                        score += 20.0; // 20% weight for human entropy
                    if capabilities.biometric_integration {
                        score += 10.0; // Additional 10% for biometrics
                Err(_) => {
                    // No human entropy capabilities
        // Score based on performance (lower latency is better)
        if let Ok(health) = provider.health_check().await {
            if let Some(latency) = health.response_time_ms {
                if latency <= self.preferences.max_latency_ms as f64 {
                    let latency_score = (self.preferences.max_latency_ms as f64 - latency)
                        / self.preferences.max_latency_ms as f64;
                    score += latency_score * 20.0; // 20% weight for performance
        // Score based on reliability
            if health.is_healthy {
                score += 10.0; // 10% weight for health
        debug!("Provider '{}' scored: {:.2}", id, score);
        Ok(score)
    /// List all registered provider IDs
    pub async fn list_provider_ids(&self) -> Vec<String> {
        providers.keys().cloned().collect()
    /// Get provider count}


    pub async fn provider_count(&self) -> usize {
        providers.len()
    /// Update provider preferences
    pub fn update_preferences(&mut self, preferences: ProviderPreferences) {
        self.preferences = preferences;
        info!("🔄 Updated HSM provider preferences");
    /// Clear all providers (for testing)
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


    async fn test_registry_basic_operations() -> BearDogResult<()> {
        let registry = UniversalHsmRegistry::new();
        // Test empty registry
        assert_eq!(registry.provider_count().await, 0);
        assert!(registry.list_provider_ids().await.is_empty());
        // Create and register a software provider
        let provider = SoftwareHsmProvider::new().await?;
        registry
            .register_provider("test-software".to_string(), Box::new(provider))
            .await?;
        // Test provider registration
        assert_eq!(registry.provider_count().await, 1);
        assert!(registry
            .list_provider_ids()
            .await
            .contains(&"test-software".to_string()));
        // Test provider retrieval
        let retrieved = registry.get_provider("test-software").await?;
        assert!(retrieved.is_some());
        // Test unregistration
        registry.unregister_provider("test-software").await?;
        Ok(())
    async fn test_provider_selection() -> BearDogResult<()> {
        // Register multiple providers
        let provider1 = SoftwareHsmProvider::new().await?;
        let provider2 = SoftwareHsmProvider::new().await?;
            .register_provider("provider1".to_string(), Box::new(provider1))
            .register_provider("provider2".to_string(), Box::new(provider2))
        // Test getting healthy providers
        let healthy = registry.get_healthy_providers().await?;
        assert_eq!(healthy.len(), 2);
        // Test getting best provider
        let best = registry.get_best_provider().await?;
        assert!(best.is_some());
