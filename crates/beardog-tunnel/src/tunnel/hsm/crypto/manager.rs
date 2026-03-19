// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Provider Manager
//!
//! Manages multiple crypto providers and selects the best one for each operation.

use super::capabilities::CryptoCapabilities;
use super::provider::UniversalCryptoProvider;
use super::requirements::CryptoRequirements;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages multiple crypto providers and selects the best one
#[derive(Clone)]
pub struct CryptoProviderManager {
    providers: Arc<RwLock<Vec<Arc<dyn UniversalCryptoProvider>>>>,
    capabilities_cache: Arc<RwLock<HashMap<String, CryptoCapabilities>>>,
}

impl CryptoProviderManager {
    /// Create a new provider manager
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(Vec::new())),
            capabilities_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a crypto provider
    pub async fn register_provider(
        &self,
        provider: Arc<dyn UniversalCryptoProvider>,
    ) -> Result<(), BearDogError> {
        // Discover and cache capabilities
        let capabilities = provider.discover_capabilities().await?;

        {
            let mut cache = self.capabilities_cache.write().await;
            cache.insert(provider.provider_name().to_string(), capabilities);
        }

        {
            let mut providers = self.providers.write().await;
            providers.push(provider);
        }

        Ok(())
    }

    /// Find the best provider for specific requirements
    pub async fn select_provider(
        &self,
        requirements: &CryptoRequirements,
    ) -> Result<Arc<dyn UniversalCryptoProvider>, BearDogError> {
        let providers = self.providers.read().await;

        if providers.is_empty() {
            return Err(BearDogError::not_found(
                "No crypto providers registered".to_string(),
            ));
        }

        // Filter candidates that meet requirements
        let mut candidates = Vec::new();
        for provider in providers.iter() {
            if self.meets_requirements(provider, requirements).await {
                candidates.push(provider.clone());
            }
        }

        if candidates.is_empty() {
            return Err(BearDogError::not_found(format!(
                "No provider supports requirements: {:?}",
                requirements.algorithm
            )));
        }

        // Rank and select the best provider
        let ranked = self.rank_providers(candidates, requirements).await;
        ranked
            .first()
            .cloned()
            .ok_or_else(|| BearDogError::internal("Provider ranking failed".to_string()))
    }

    /// Check if a provider meets requirements
    async fn meets_requirements(
        &self,
        provider: &Arc<dyn UniversalCryptoProvider>,
        requirements: &CryptoRequirements,
    ) -> bool {
        // Check algorithm support
        if let Some(ref algorithm) = requirements.algorithm {
            if !provider.supports_algorithm(algorithm).await {
                return false;
            }
        }

        // Get capabilities
        let cache = self.capabilities_cache.read().await;
        let capabilities = match cache.get(provider.provider_name()) {
            Some(caps) => caps,
            None => return false,
        };

        // Check constant-time requirement
        if requirements.require_constant_time {
            if let Some(ref alg) = requirements.algorithm {
                let alg_name = alg.to_string();
                if !capabilities.constant_time_ops.contains(&alg_name) {
                    return false;
                }
            }
        }

        // Check performance requirements
        if let Some(max_latency) = requirements.max_latency_us {
            if let Some(ref alg) = requirements.algorithm {
                let alg_name = alg.to_string();
                if let Some(&latency) = capabilities.performance_profile.latency_us.get(&alg_name) {
                    if latency > max_latency as f64 {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Rank providers by score (higher is better)
    async fn rank_providers(
        &self,
        candidates: Vec<Arc<dyn UniversalCryptoProvider>>,
        requirements: &CryptoRequirements,
    ) -> Vec<Arc<dyn UniversalCryptoProvider>> {
        let cache = self.capabilities_cache.read().await;

        let mut scored: Vec<_> = candidates
            .into_iter()
            .map(|provider| {
                let score = self.calculate_score(&provider, requirements, &cache);
                (provider, score)
            })
            .collect();

        // Sort by score (descending)
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored.into_iter().map(|(p, _)| p).collect()
    }

    /// Calculate score for a provider (higher is better)
    fn calculate_score(
        &self,
        provider: &Arc<dyn UniversalCryptoProvider>,
        requirements: &CryptoRequirements,
        cache: &HashMap<String, CryptoCapabilities>,
    ) -> f64 {
        let mut score = 0.0;

        let capabilities = match cache.get(provider.provider_name()) {
            Some(caps) => caps,
            None => return 0.0,
        };

        // Base score for meeting requirements
        score += 100.0;

        // Performance scoring
        if requirements.prefer_performance {
            if let Some(ref alg) = requirements.algorithm {
                let alg_name = alg.to_string();

                // Higher throughput = better score
                if let Some(&throughput) = capabilities
                    .performance_profile
                    .throughput_mbps
                    .get(&alg_name)
                {
                    score += throughput / 10.0; // Scale down
                }

                // Lower latency = better score
                if let Some(&latency) = capabilities.performance_profile.latency_us.get(&alg_name) {
                    score += 100.0 / (latency + 1.0); // Inverse relationship
                }
            }
        } else {
            // Security scoring
            match capabilities.side_channel_resistance {
                super::capabilities::SideChannelResistance::Full => score += 50.0,
                super::capabilities::SideChannelResistance::Partial => score += 25.0,
                super::capabilities::SideChannelResistance::None => score += 0.0,
            }

            // Constant-time operations
            if let Some(ref alg) = requirements.algorithm {
                let alg_name = alg.to_string();
                if capabilities.constant_time_ops.contains(&alg_name) {
                    score += 30.0;
                }
            }
        }

        // Hardware acceleration bonus
        if requirements.require_hardware_accel && !capabilities.hardware_acceleration.is_empty() {
            score += 20.0;
        }

        score
    }

    /// Get all registered providers
    pub async fn get_providers(&self) -> Vec<Arc<dyn UniversalCryptoProvider>> {
        self.providers.read().await.clone()
    }

    /// Get capabilities for a specific provider
    pub async fn get_capabilities(&self, provider_name: &str) -> Option<CryptoCapabilities> {
        self.capabilities_cache
            .read()
            .await
            .get(provider_name)
            .cloned()
    }
}

impl Default for CryptoProviderManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CryptoProviderManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CryptoProviderManager")
            .field("providers", &"<providers>")
            .field("capabilities_cache", &"<cache>")
            .finish()
    }
}
