//! # HSM Provider Manager
//!
//! This module provides management of HSM providers, including
//! primary/fallback selection, health monitoring, and metrics collection.

use beardog_errors::BearDogError;

use super::super::traits::HsmManager;
use super::super::{
    error::HsmResult,
    traits::ProviderInfo,
    types::{HsmHealth, HsmHealthStatus, HsmProviderType, HsmTier},
};
use beardog_types::canonical::hsm::status::HealthMetrics;
use beardog_types::canonical::providers_unified::traits::UnifiedHsmProvider as HsmProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================
// Configuration
// ============================================================

/// Manager configuration
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    /// Preferred provider types in order
    pub preferred_providers: Vec<HsmProviderType>,

    /// Minimum required security tier
    pub min_security_tier: HsmTier,

    /// Enable auto-discovery of providers
    pub enable_discovery: bool,

    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            preferred_providers: vec![HsmProviderType::Software],
            min_security_tier: HsmTier::Software,
            enable_discovery: true,
            health_check_interval_secs: 300,
        }
    }
}

// ============================================================
// Metrics
// ============================================================

/// Provider metrics
#[derive(Debug, Clone, Default)]
pub struct ProviderMetrics {
    /// Total operations count
    pub operations_count: u64,

    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,

    /// Error count
    pub error_count: u64,

    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,

    /// Last operation timestamp
    pub last_operation_time: Option<chrono::DateTime<chrono::Utc>>,
}

// ============================================================
// HSM Provider Manager
// ============================================================

/// HSM Provider Manager
pub struct HsmProviderManager<P: HsmProvider + Clone + 'static> {
    /// Primary HSM provider
    primary_provider: P,

    /// Fallback providers by type
    fallback_providers: HashMap<HsmProviderType, P>,

    /// Configuration
    config: Arc<RwLock<Option<ManagerConfig>>>,

    /// Provider metrics
    metrics: Arc<RwLock<HashMap<String, ProviderMetrics>>>,
}

impl<P: HsmProvider + Clone + 'static> HsmProviderManager<P> {
    /// Create a new HSM provider manager
    pub fn new(primary_provider: P) -> Self {
        Self {
            primary_provider,
            fallback_providers: HashMap::with_capacity(16),
            config: Arc::new(RwLock::new(None)),
            metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }

    /// Initialize the manager with configuration
    pub async fn initialize(&self, config: ManagerConfig) -> HsmResult<()> {
        let mut config_guard = self.config.write().await;
        *config_guard = Some(config);

        let mut metrics = self.metrics.write().await;
        metrics.insert("primary".to_string(), ProviderMetrics::default());

        Ok(())
    }

    /// Add a fallback provider
    pub async fn add_fallback_provider(&mut self, provider_type: HsmProviderType, provider: P) {
        self.fallback_providers.insert(provider_type, provider);

        let mut metrics = self.metrics.write().await;
        metrics.insert(
            format!("fallback_{:?}", provider_type),
            ProviderMetrics::default(),
        );
    }

    /// Get provider score based on info and config
    pub fn get_provider_score(&self, provider_info: &ProviderInfo, config: &ManagerConfig) -> f64 {
        let mut score = 0.0_f64;

        // Score based on tier
        score += match provider_info.tier {
            HsmTier::Software => 10.0,
            HsmTier::BasicHardware => 20.0,
            HsmTier::CertifiedHardware => 30.0,
            HsmTier::HighSecurity => 40.0,
            HsmTier::HumanEntropyPremium => 50.0,
            _ => 1.0, // Default score for unknown tiers
        };

        // Bonus for preferred providers
        if let Some(provider_type) = &provider_info.provider_type {
            if config.preferred_providers.contains(provider_type) {
                score += 20.0;
            }
        }

        score
    }

    /// Check if provider meets minimum requirements
    pub fn meets_requirements(&self, provider_info: &ProviderInfo, config: &ManagerConfig) -> bool {
        let tier_order = |tier: &HsmTier| match tier {
            HsmTier::Software => 0,
            HsmTier::BasicHardware => 1,
            HsmTier::CertifiedHardware => 2,
            HsmTier::HighSecurity => 3,
            HsmTier::HumanEntropyPremium => 4,
            _ => 0, // Default order for unknown tiers
        };

        tier_order(&provider_info.tier) >= tier_order(&config.min_security_tier)
    }

    /// Update metrics for a provider
    pub async fn update_metrics(
        &self,
        provider_id: &str,
        operation_time_ms: f64,
        success: bool,
    ) {
        let mut metrics = self.metrics.write().await;
        if let Some(provider_metrics) = metrics.get_mut(provider_id) {
            provider_metrics.operations_count += 1;

            // Update average response time using running average
            let count = provider_metrics.operations_count as f64;
            provider_metrics.avg_response_time_ms =
                (provider_metrics.avg_response_time_ms * (count - 1.0) + operation_time_ms) / count;

            if !success {
                provider_metrics.error_count += 1;
            }

            // Update success rate
            provider_metrics.success_rate =
                (provider_metrics.operations_count - provider_metrics.error_count) as f64 / count;

            provider_metrics.last_operation_time = Some(chrono::Utc::now());
        }
    }

    /// Get metrics for a provider
    pub async fn get_metrics(&self, provider_id: &str) -> Option<ProviderMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(provider_id).cloned()
    }

    /// Get all provider metrics
    pub async fn get_all_metrics(&self) -> HashMap<String, ProviderMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Get the primary provider
    pub fn primary(&self) -> &P {
        &self.primary_provider
    }

    /// Get a fallback provider by type
    pub fn fallback(&self, provider_type: &HsmProviderType) -> Option<&P> {
        self.fallback_providers.get(provider_type)
    }
}

impl<P: HsmProvider + Clone + 'static> Default for HsmProviderManager<P>
where
    P: Default,
{
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: HsmProvider + Clone + 'static> std::fmt::Debug for HsmProviderManager<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HsmProviderManager")
            .field("fallback_count", &self.fallback_providers.len())
            .finish()
    }
}

/// Type alias for default manager
pub type DefaultHsmProviderManager =
    HsmProviderManager<crate::hsm_foundation::providers::software::SoftwareHsmProvider>;

/// Create a default HSM provider manager
pub fn create_default_manager() -> DefaultHsmProviderManager {
    let software_provider = crate::hsm_foundation::providers::software::SoftwareHsmProvider::new();
    HsmProviderManager::new(software_provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_initialization() -> Result<(), BearDogError> {
        let manager = create_default_manager();
        let config = ManagerConfig::default();

        manager
            .initialize(config)
            .await
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_update() -> Result<(), BearDogError> {
        let manager = create_default_manager();
        let config = ManagerConfig::default();

        manager
            .initialize(config)
            .await
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        // Update metrics
        manager.update_metrics("primary", 10.0, true).await;
        manager.update_metrics("primary", 15.0, true).await;
        manager.update_metrics("primary", 20.0, false).await;

        let metrics = manager.get_metrics("primary").await.unwrap_or_default();

        assert_eq!(metrics.operations_count, 3);
        assert_eq!(metrics.error_count, 1);
        assert!((metrics.avg_response_time_ms - 15.0).abs() < 0.1);
        assert!((metrics.success_rate - 2.0 / 3.0).abs() < 0.1);

        Ok(())
    }

    #[test]
    fn test_config_default() {
        let config = ManagerConfig::default();
        assert!(config.enable_discovery);
        assert_eq!(config.health_check_interval_secs, 300);
    }

    #[test]
    fn test_provider_metrics_default() {
        let metrics = ProviderMetrics::default();
        assert_eq!(metrics.operations_count, 0);
        assert_eq!(metrics.error_count, 0);
    }
}
