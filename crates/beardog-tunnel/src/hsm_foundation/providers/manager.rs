use beardog_errors::BearDogError;

use super::super::traits::HsmManager;
use super::super::{
    error::HsmResult, 
    traits::ProviderInfo, 
    types::{HsmProviderType, HsmTier, BearDogError, HsmHealth, HsmHealthStatus}
};
use beardog_types::canonical::hsm::status::HealthMetrics;
use beardog_traits::canonical::HsmProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HsmProviderManager<P: HsmProvider + Clone + 'static> {

    primary_provider: P,

    fallback_providers: HashMap<HsmProviderType, P>,

    config: Arc<RwLock<Option<ManagerConfig>>>,

    metrics: Arc<RwLock<HashMap<String, ProviderMetrics>>>,
}

#[derive(Debug, Clone)]
pub struct ManagerConfig {

    pub preferred_providers: Vec<HsmProviderType>,

    pub min_security_tier: HsmTier,

    pub enable_discovery: bool,

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

#[derive(Debug, Clone, Default)]
pub struct ProviderMetrics {
    pub operations_count: u64,
    pub avg_response_time_ms: f64,
    pub error_count: u64,
    pub success_rate: f64,
}

impl<P: HsmProvider + Clone + 'static> HsmProviderManager<P> {

    pub fn new(primary_provider: P) -> Self {
        Self {
            primary_provider,
            fallback_providers: HashMap::with_capacity(16),
            config: Arc::new(RwLock::new(None)),
            metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }

    pub async fn initialize(&self, config: ManagerConfig) -> HsmResult<()> {
        let mut config_guard = self.config.write().await;
        *config_guard = Some(config);

        let mut metrics = self.metrics.write().await;
        metrics.insert("primary".to_string(), ProviderMetrics::default());
        
        Ok(())
    }

    pub async fn add_fallback_provider(&mut self, provider_type: HsmProviderType, provider: P) {
        self.fallback_providers.insert(provider_type, provider);

        let mut metrics = self.metrics.write().await;
        metrics.insert(format_args!("fallback_{:?}", provider_type).to_string(), ProviderMetrics::default());
    }

    pub fn get_best_provider(&self) -> &P {

        &self.primary_provider
    }

    fn get_provider_score(&self, provider_info: &ProviderInfo, config: &ManagerConfig) -> f64 {
        let mut score = 0f64;

        score += match provider_info.tier {
            HsmTier::Software => 10f64,
            HsmTier::BasicHardware => 20f64,
            HsmTier::CertifiedHardware => 30f64,
            HsmTier::HighSecurity => 40f64,
            HsmTier::HumanEntropyPremium => 50f64,
            _ => 1f64, // Default score for unknown tiers
        };

        if let Some(pos) = config
            .preferred_providers
            .iter()
            .position(|p| *p == provider_info.provider_type)
        {
            score += (config.preferred_providers.len() as f64 - pos as f64) * 100f64;
        }

        if provider_info.available {
            score += 1000f64;
        }
        
        score
    }

    fn meets_requirements(&self, provider_info: &ProviderInfo, config: &ManagerConfig) -> bool {

        let tier_order = |tier: &HsmTier| match tier {
            HsmTier::Software => 0,
            HsmTier::BasicHardware => 1,
            HsmTier::CertifiedHardware => 2,
            HsmTier::HighSecurity => 3,
            HsmTier::HumanEntropyPremium => 4,
            _ => 0, // Default order for unknown tiers
        };
        
        tier_order(&provider_info.tier) >= tier_order(&config.min_security_tier)
            && provider_info.available
    }

    pub async fn update_metrics(&self, provider_id: &str, operation_time_ms: f64, success: bool) {
        let mut metrics = self.metrics.write().await;
        if let Some(provider_metrics) = metrics.get_mut(provider_id) {
            provider_metrics.operations_count += 1;

            let count = provider_metrics.operations_count as f64;
            provider_metrics.avg_response_time_ms = 
                (provider_metrics.avg_response_time_ms * (count - 1.0) + operation_time_ms) / count;
            
            if !success {
                provider_metrics.error_count += 1;
            }
            
            provider_metrics.success_rate = 
                (provider_metrics.operations_count - provider_metrics.error_count) as f64 / count;
        }
    }
}

impl<P: HsmProvider + Clone + 'static> Default for HsmProviderManager<P> 
where 
    P: Default
{
    fn default() -> Self {
        Self::new(P::default())
    }
}

pub type DefaultHsmProviderManager = HsmProviderManager<crate::hsm_foundation::providers::software::SoftwareHsmProvider>;

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
        
        manager.initialize(config).await.map_err(|e| {
            beardog_errors::BearDogError::system(format_args!("Manager initialization failed: {:?}", e).to_string())
        })?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_scoring() -> Result<(), BearDogError> {
        let manager = create_default_manager();
        let config = ManagerConfig {
            preferred_providers: vec![HsmProviderType::Hardware, HsmProviderType::Software],
            min_security_tier: HsmTier::Software,
            enable_discovery: true,
            health_check_interval_secs: 60,
        };
        
        let provider_info_1 = ProviderInfo {
            name: "Software HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            provider_type: HsmProviderType::Software,
            tier: HsmTier::Software,
            available: true,
            description: "BearDog Software HSM Implementation".to_string(),
        };
        
        let provider_info_2 = ProviderInfo {
            name: "Hardware HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            provider_type: HsmProviderType::Hardware,
            tier: HsmTier::BasicHardware,
            available: true,
            description: "Hardware HSM Implementation".to_string(),
        };
        
        let score_1 = manager.get_provider_score(&provider_info_1, &config);
        let score_2 = manager.get_provider_score(&provider_info_2, &config);

        assert!(score_2 > score_1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_update() -> Result<(), BearDogError> {
        let manager = create_default_manager();

        manager.update_metrics("primary", 10.0, true).await;
        manager.update_metrics("primary", 20.0, true).await;
        manager.update_metrics("primary", 15.0, false).await; // One failure
        
        let metrics = manager.metrics.read().await;
        let primary_metrics = metrics.get("primary")
            .unwrap_or_else(|| {

                HsmProviderMetrics {
                    operations_count: 0,
                    success_rate: 0.0,
                    avg_response_time_ms: 0.0,
                    last_operation_time: None,
                }
            });
        
        assert_eq!(primary_metrics.operations_count, 3);
        assert_eq!(primary_metrics.error_count, 1);
        assert!((primary_metrics.avg_response_time_ms - 15.0).abs() < 0.1);
        assert!((primary_metrics.success_rate - 2.0/3.0).abs() < 0.1);
        
        Ok(())
    }
}
