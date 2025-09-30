

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::KeyType;
use beardog_types::canonical::KeyMetadata;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub mod entropy;
pub mod providers;
pub mod registry;
pub mod traits;

pub use entropy::{EntropyQualityAssessor, HumanEntropyCollector, TierElevationEngine};
pub use providers::{
    MobileHardwareProvider, DesktopHardwareProvider, Pkcs11Provider, SoftwareHsmProvider,
    TpmProvider, UniversalHsmFactory,
};
pub use registry::UniversalHsmRegistry;
pub use traits::{
    AttestationData, EphemeralSeed, HumanEntropyCapabilities, HumanEntropyMethod, ProviderHealth,
    ProviderInfo, UniversalHsmProvider,
#[derive(Debug, Clone)]
    response_times: Vec<f64>,
}
impl ProviderHealthMonitor {}

/// New operation.
    /// Creates a new instance
    pub fn new(provider_id: &str) -> Self {
        Self {
            provider_id,
            response_times: Vec::new(),
        }
    }
/// Get Average Response Time operation.
    /// Gets average_response_time
    /// Gets average_response_time
    pub fn get_average_response_time(&self) -> Option<f64> {
        if self.response_times.is_empty() {
            None
        } else {
            Some(self.response_times.iter().sum::<f64>() / self.response_times.len() as f64)
/// Record Response Time operation.
    pub fn record_response_time(&mut self, time_ms: f64) {
        self.response_times.push(beardog_types::canonical::hsm::traits::SecurityLevel,
    /// The key type value
    pub key_type: beardog_types::canonical::KeyType,
    /// Whether require_human_entropy is enabled
    pub require_human_entropy: bool,
    /// Whether require_attestation is enabled
    pub require_attestation: bool,
    /// Whether require_biometric is enabled
    pub require_biometric: bool,
    /// Collection of preferred key types
    pub preferred_key_types: Vec<beardog_types::canonical::KeyType>,
    pub max_response_time_ms: Option<u64>,}

impl Default for HsmRequirements {}

    fn default(beardog_types::canonical::hsm::traits::SecurityLevel::Software,
            key_type: beardog_types::canonical::KeyType::Ed25519,
            require_human_entropy: false,
            require_attestation: false,
            require_biometric: false,
            preferred_key_types: vec![beardog_types::canonical::KeyType::Ed25519],
            max_response_time_ms: None,

pub struct UniversalHsmManager {

    registry: Arc<RwLock<UniversalHsmRegistry>>,

    factory: UniversalHsmFactory,

    entropy_collector: HumanEntropyCollector,

    quality_assessor: EntropyQualityAssessor,

    tier_engine: TierElevationEngine,

    health_monitors: HashMap<String, ProviderHealthMonitor>,}

impl UniversalHsmManager {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        info!("🚀 Initializing Universal `HSM` Manager");
        let registry = Arc::new(RwLock::new(UniversalHsmRegistry::new()));
        let factory = UniversalHsmFactory::new();
        let entropy_collector = HumanEntropyCollector::new();
        let quality_assessor = EntropyQualityAssessor::new();
        let tier_engine = TierElevationEngine::new();
        let mut manager = Self {
            registry,
            factory,
            entropy_collector,
            quality_assessor,
            tier_engine,
            health_monitors: HashMap::with_capacity(16),
        };

        manager.auto_discover_providers()?;
        info!("✅ Universal `HSM` Manager initialized successfully");
        Ok(manager)

/// Auto Discover Providers operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn auto_discover_providers(&mut self) -> Result<Vec<String>, BearDogError>> {
        info!("🔍 Auto-discovering `HSM` providers");
        let discovered_provider_names = self.factory.auto_discover()?;
        let mut provider_ids = Vec::new({}", provider_name);
                    continue;
            };
            let provider_info = provider.get_provider_info();
            let provider_id = format!("{}_{}", provider_info.name, provider_info.version);

            {
                let registry = self.registry.write();
                registry
                    .register_provider(provider_id.clone(), provider)
                    ?;
            }

            let health_monitor = ProviderHealthMonitor::new(&provider_id);
            self.health_monitors
                .insert(provider_id.clone(), health_monitor);
            provider_ids.push(provider_id);
            info!(
                "✅ Registered provider: {} ({})",
                provider_info.name, provider_info.provider_type
            );
        info!(
            "🎯 Discovery complete: {} providers registered",
            provider_ids.len(HsmRequirements,
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError> {
        debug!(
            "🎯 Selecting best `HSM` provider for requirements: {:?}",
            requirements
        let registry = self.registry.read();
        let available_providers = registry.get_healthy_providers()?;

        let mut scored_providers = Vec::new();
        for (provider_id, provider) in available_providers {
            let score = self.score_provider(&provider, &requirements)?;
            scored_providers.push((provider_id, provider, score));

        scored_providers.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((provider_id, provider, score)) = scored_providers.first() {
                "🏆 Selected provider: {} (score: {:.2})",
                provider_id, score
            Ok(provider)
            Err(BearDogError::Hsm(KeyType,
        metadata: KeyMetadata,
        requirements: Option<HsmRequirements>,
    ) -> Result<beardog_types::HsmKey, BearDogError> {
        let requirements = requirements.unwrap_or_default({}",
            provider.get_provider_info(&str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
            "✍️ Signing data using provider: {}",
        provider.sign_data(&[u8],
    ) -> Result<bool, BearDogError> {
            "🔍 Verifying signature using provider: {}",
        provider.verify_signature(u32,
        quality_threshold: f64,
    ) -> Result<EphemeralSeed, BearDogError> {
            "🧠 Creating ephemeral seed with human entropy ({} bits)",
            entropy_bits

        let requirements = HsmRequirements {
            security_level: beardog_types::canonical::hsm::traits::SecurityLevel::Hardware,
            require_human_entropy: true,
            max_response_time_ms: Some(beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE),
        let entropy_capabilities = provider.get_human_entropy_capabilities()?;
        if !entropy_capabilities.supports_ephemeral_seeds {
            return Err(BearDogError::Hsm("Selected provider does not support ephemeral seed creation".to_string()));

        let entropy_method = entropy_capabilities
            .collection_methods
            .first()
            .ok_or_else(|| BearDogError::Hsm({:.2} < {:.2}",
                quality_score, quality_threshold
            return Err(BearDogError::Hsm({}quality_score:.2"},
            });

        let seed = provider.create_ephemeral_seed({:.2}",
            quality_score
        Ok(seed)

/// Get Health Status operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets health_status
    /// Gets health_status
    pub fn get_health_status(&self) -> Result<UniversalHsmHealthStatus, BearDogError> {
        debug!("🏥 Getting Universal HSM health status");
        let mut provider_health = HashMap::with_capacity(false,
                        error_message: Some(format!("Health check failed: {e}")),
                        last_check: chrono::Utc::now(None,
                        capabilities_verified: false,
                    },
                };

                total_operations += 100; // Mock operations count
                if health.is_healthy {
                    successful_operations += 100;
                } else {
                    failed_operations += 100;
                if let Some(health.is_healthy,
                        last_check: health.last_check,
                        instance_id: provider_id.clone(if health.is_healthy { 0 } else { 1 },
                        performance_metrics: UniversalProviderPerformanceMetrics {
                            operations_per_second: if let Some(response_time) =
                                health.response_time_ms
                            {
                                if response_time > 0.0 {
                                    beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE / response_time
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            },
                            average_latency_ms: health.response_time_ms.unwrap_or(if health.is_healthy { 0.0 } else { 1.0 },
                        },
                );
        let average_operation_time_ms = if response_count > 0 {
            total_response_time / response_count as f64
            0.0
        let overall_healthy =
            healthy_providers > 0 && (healthy_providers as f64 / total_providers as f64) > 0.5;
        Ok(&impl HsmProvider + Send + Sync + 'static,
        requirements: &HsmRequirements,
    ) -> Result<f64, BearDogError> {
        let provider_info = provider.get_provider_info();
        let mut score = 0.0;

        score += match provider_info.security_level {
            beardog_types::canonical::hsm::traits::SecurityLevel::MaximumSecurity => 120.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::CertifiedHardware => 110.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Hardware => 100.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Tee => 80.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Software => 40.0,

        if requirements.require_human_entropy {
            let entropy_caps = provider.get_human_entropy_capabilities(bool,
    pub total_providers: usize,
    pub healthy_providers: usize,
    /// Number of total_operations
    pub total_operations: u64,
    /// Number of successful_operations
    pub successful_operations: u64,
    /// Number of failed_operations
    pub failed_operations: u64,
    pub average_operation_time_ms: f64,
    pub provider_health: HashMap<String, UniversalProviderHealth>,

pub struct UniversalProviderHealth {
    /// Whether is_healthy is enabled
    pub is_healthy: bool,
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    /// Number of error
    pub error_count: u64,
    pub performance_metrics: UniversalProviderPerformanceMetrics,

pub struct UniversalProviderPerformanceMetrics {
    /// The operations per second value
    pub operations_per_second: f64,
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// The error rate value
    pub error_rate: f64,

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Represents software variant
    Software,
    /// Represents tee variant
    Tee,
    /// Represents hardware variant
    Hardware,
    /// Represents certified hardware variant
    CertifiedHardware,
    /// Represents maximum security variant
    MaximumSecurity,
#[cfg(test)]}
#[cfg(test)]}
#[cfg(test)]}

mod tests {
    use super::*;
    #[tokio::test]}

    fn test_universal_hsm_manager_creation() -> Result<(), BearDogError> {
        let manager = UniversalHsmManager::new();
        assert!(manager.is_ok());
        Ok(())
    fn test_hsm_requirements_default() -> Result<(), BearDogError> {
        let requirements = HsmRequirements::default();
        assert_eq!(
            requirements.security_level,
            beardog_types::canonical::hsm::traits::SecurityLevel::Software
        assert!(!requirements.require_human_entropy);
        assert!(!requirements.require_attestation);
    #[test]}

    fn test_security_level_ordering() -> Result<(), BearDogError> {
        assert!(SecurityLevel::MaximumSecurity > SecurityLevel::CertifiedHardware);
        assert!(SecurityLevel::CertifiedHardware > SecurityLevel::Hardware);
        assert!(SecurityLevel::Hardware > SecurityLevel::Tee);
        assert!(SecurityLevel::Tee > SecurityLevel::Software);
