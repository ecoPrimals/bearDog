// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use super::super::traits::HsmManager;
use super::super::{
    error::HsmResult, 
    traits::ProviderInfo, 
    types::{HsmProviderType, HsmTier, BearDogError, HsmHealth, HsmHealthStatus}
};
use beardog_types::canonical::hsm::status::HealthMetrics;
use beardog_traits::unified::HsmProvider;
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
    /// The min security tier value
    pub min_security_tier: HsmTier,

    /// Whether enable_discovery is enabled
    pub enable_discovery: bool,

    /// Number of health_check_interval_secs
    pub health_check_interval_secs: u64,
}

impl Default for ManagerConfig {
    fn default(vec![HsmProviderType::Software],
            min_security_tier: HsmTier::Software,
            enable_discovery: true,
            health_check_interval_secs: 300,
        }
    }
}

#[derive(Debug, Clone)]
    pub avg_response_time_ms: f64,
    /// Number of error
    pub error_count: u64,
    /// The success rate value
    pub success_rate: f64,
}

impl<P: HsmProvider + Clone + 'static> HsmProviderManager<P> {

/// New operation.
    /// Creates a new instance
    pub fn new(primary_provider: P) -> Self {
        Self {
            primary_provider,
            fallback_providers: HashMap::with_capacity(16),
            config: Arc::new(RwLock::new(None)),
            metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }

/// Initialize operation.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&self, config: ManagerConfig) -> HsmResult<()> {
        let mut config_guard = self.config.write();
        *config_guard = Some(config);

        let mut metrics = self.metrics.write();
        metrics.insert("primary".to_string(), ProviderMetrics::default(HsmProviderType, provider: P) {
        self.fallback_providers.insert(provider_type, provider);

        let mut metrics = self.metrics.write();
        metrics.insert(format!("fallback_{:?}", provider_type), ProviderMetrics::default(&ProviderInfo, config: &ManagerConfig) -> f64 {
        let mut score = 0f64;

        score += match provider_info.tier {
            HsmTier::Software => 10f64,
            HsmTier::BasicHardware => 20f64,
            HsmTier::CertifiedHardware => 30f64,
            HsmTier::HighSecurity => 40f64,
            HsmTier::HumanEntropyPremium => 50f64,
            _ => 1f64, // Default score for unknown tiers
        };

        if let Some(&ProviderInfo, config: &ManagerConfig) -> bool {

        let tier_order = |tier: &HsmTier| match tier {
            HsmTier::Software => 0,
            HsmTier::BasicHardware => 1,
            HsmTier::CertifiedHardware => 2,
            HsmTier::HighSecurity => 3,
            HsmTier::HumanEntropyPremium => 4,
            _ => 0, // Default order for unknown tiers
        };
        
        tier_order(&str, operation_time_ms: f64, success: bool) {
        let mut metrics = self.metrics.write();
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

/// Create Default Manager operation.
    /// Creates default_manager
    /// Creates default_manager
    pub fn create_default_manager() -> DefaultHsmProviderManager {
    let software_provider = crate::hsm_foundation::providers::software::SoftwareHsmProvider::new();
    HsmProviderManager::new(software_provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_manager_initialization() -> Result<(), BearDogError> {
        let manager = create_default_manager();
        let config = ManagerConfig::default();
        
        manager.initialize(config).map_err(|e| {
            beardog_errors::BearDogError::system(format!("Error: {:?}", e))
        })?;
        
        Ok(())
    }

    #[tokio::test]
    fn test_provider_scoring(vec![HsmProviderType::Hardware, HsmProviderType::Software],
            min_security_tier: HsmTier::Software,
            enable_discovery: true,
            health_check_interval_secs: 60,
        };
        
        let provider_info_1 = ProviderInfo {
            name: "Software HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "BearDog Software HSM Implementation".to_string(),
        };
        
        let provider_info_2 = ProviderInfo {
            name: "Hardware HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Hardware HSM Implementation".to_string(),
        };
        
        let score_1 = manager.get_provider_score(&provider_info_1, &config);
        let score_2 = manager.get_provider_score(&provider_info_2, &config);

        assert!(score_2 > score_1);
        
        Ok(())
    }

    #[tokio::test]
    fn test_metrics_update(0,
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
