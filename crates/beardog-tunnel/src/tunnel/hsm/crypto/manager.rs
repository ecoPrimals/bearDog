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

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
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

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
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
        if let Some(ref algorithm) = requirements.algorithm
            && !provider.supports_algorithm(algorithm).await
        {
            return false;
        }

        // Get capabilities
        let cache = self.capabilities_cache.read().await;
        let Some(capabilities) = cache.get(provider.provider_name()) else {
            return false;
        };

        // Check constant-time requirement
        if requirements.require_constant_time
            && let Some(ref alg) = requirements.algorithm
        {
            let alg_name = alg.to_string();
            if !capabilities.constant_time_ops.contains(&alg_name) {
                return false;
            }
        }

        // Check performance requirements
        if let Some(max_latency) = requirements.max_latency_us
            && let Some(ref alg) = requirements.algorithm
        {
            let alg_name = alg.to_string();
            if let Some(&latency) = capabilities.performance_profile.latency_us.get(&alg_name) {
                #[expect(clippy::cast_precision_loss, reason = "compare latency budget in f64")]
                let max_us = max_latency as f64;
                if latency > max_us {
                    return false;
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

        let Some(capabilities) = cache.get(provider.provider_name()) else {
            return 0.0;
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

#[cfg(test)]
mod tests {
    use super::super::UniversalCryptoProvider;
    use super::super::algorithms::{
        AesMode, AsymmetricAlgorithm, CryptoAlgorithm, CryptoOperation, HashAlgorithm,
        SignatureAlgorithm, SymmetricAlgorithm,
    };
    use super::super::providers::RustCryptoProvider;
    use super::super::requirements::CryptoRequirements;
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn crypto_provider_manager_new_default_register_and_select() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov)
            .await
            .expect("register RustCrypto provider");

        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes {
            mode: AesMode::Gcm,
            key_size: 256,
        });
        let reqs = CryptoRequirements {
            algorithm: Some(alg.clone()),
            require_constant_time: false,
            prefer_performance: true,
            ..CryptoRequirements::default()
        };
        let chosen = mgr
            .select_provider(&reqs)
            .await
            .expect("select provider for AES-256-GCM");
        assert_eq!(chosen.provider_name(), "RustCrypto");

        let list = mgr.get_providers().await;
        assert_eq!(list.len(), 1);

        let caps = mgr
            .get_capabilities("RustCrypto")
            .await
            .expect("cached capabilities");
        assert_eq!(caps.provider_name, "RustCrypto");
    }

    #[tokio::test]
    async fn select_provider_errors_when_empty() {
        let mgr = CryptoProviderManager::default();
        let err = mgr
            .select_provider(&CryptoRequirements::default())
            .await
            .expect_err("no providers");
        let msg = format!("{err}");
        assert!(
            msg.contains("No crypto providers") || msg.contains("not found"),
            "{msg}"
        );
    }

    #[tokio::test]
    async fn select_provider_no_candidate_when_constant_time_required_for_hash() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov).await.expect("register");

        let reqs = CryptoRequirements {
            operation: CryptoOperation::Hashing,
            algorithm: Some(CryptoAlgorithm::Hash(HashAlgorithm::Sha256)),
            require_constant_time: true,
            ..CryptoRequirements::default()
        };
        let err = mgr
            .select_provider(&reqs)
            .await
            .expect_err("SHA-256 not in constant_time_ops list");
        let m = format!("{err}");
        assert!(
            m.contains("requirements") || m.contains("No provider"),
            "{m}"
        );
    }

    #[tokio::test]
    async fn select_provider_rejects_excessive_latency_budget() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov).await.expect("register");

        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes {
            mode: AesMode::Gcm,
            key_size: 256,
        });
        let reqs = CryptoRequirements {
            algorithm: Some(alg),
            max_latency_us: Some(1),
            require_constant_time: false,
            ..CryptoRequirements::default()
        };
        mgr.select_provider(&reqs)
            .await
            .expect_err("latency 5us exceeds 1us budget");
    }

    #[tokio::test]
    async fn select_provider_prefers_hardware_when_required_and_available() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov).await.expect("register");

        let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519);
        let reqs = CryptoRequirements {
            operation: CryptoOperation::Signing,
            algorithm: Some(alg),
            require_hardware_accel: true,
            require_constant_time: false,
            prefer_performance: false,
            ..CryptoRequirements::default()
        };
        let p = mgr
            .select_provider(&reqs)
            .await
            .expect("hardware bonus path");
        assert_eq!(p.provider_name(), "RustCrypto");
    }

    #[tokio::test]
    async fn select_provider_security_scoring_branch_non_performance() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov).await.expect("register");

        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::ChaCha20Poly1305);
        let reqs = CryptoRequirements {
            algorithm: Some(alg),
            prefer_performance: false,
            require_constant_time: false,
            ..CryptoRequirements::default()
        };
        mgr.select_provider(&reqs)
            .await
            .expect("side-channel scoring branch");
    }

    #[tokio::test]
    async fn crypto_provider_manager_debug_smoke() {
        let mgr = CryptoProviderManager::new();
        let s = format!("{mgr:?}");
        assert!(s.contains("CryptoProviderManager"));
    }

    #[tokio::test]
    async fn select_provider_no_asymmetric_support_in_rustcrypto() {
        let mgr = CryptoProviderManager::new();
        let prov = Arc::new(RustCryptoProvider::new()) as Arc<dyn UniversalCryptoProvider>;
        mgr.register_provider(prov).await.expect("register");

        let reqs = CryptoRequirements {
            operation: CryptoOperation::AsymmetricEncryption,
            algorithm: Some(CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::EciesP256)),
            require_constant_time: false,
            ..CryptoRequirements::default()
        };
        mgr.select_provider(&reqs)
            .await
            .expect_err("RustCrypto has no asymmetric algorithms registered");
    }
}
