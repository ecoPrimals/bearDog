

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
#[derive(Debug)]};

pub struct ProviderHealthMonitor {
        provider_id: String,
    response_times: Vec<f64>,
}
impl ProviderHealthMonitor {}

    pub fn new(provider_id: &str) -> Self {
        Self {
            provider_id,
            response_times: Vec::new(),
        }
    }
    pub fn get_average_response_time(&self) -> Option<f64> {
        if self.response_times.is_empty() {
            None
        } else {
            Some(self.response_times.iter().sum::<f64>() / self.response_times.len() as f64)
    pub fn record_response_time(&mut self, time_ms: f64) {
        self.response_times.push(time_ms);

        if self.response_times.len() > 100 {
            self.response_times.remove(0);
pub struct HsmRequirements {
    pub security_level: beardog_types::canonical::hsm::traits::SecurityLevel,
    pub key_type: beardog_types::canonical::KeyType,
    pub require_human_entropy: bool,
    pub require_attestation: bool,
    pub require_biometric: bool,
    pub preferred_key_types: Vec<beardog_types::canonical::KeyType>,
    pub max_response_time_ms: Option<u64>,}

impl Default for HsmRequirements {}

    fn default() -> Self {
            security_level: beardog_types::canonical::hsm::traits::SecurityLevel::Software,
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

    pub async fn new() -> Result<Self, BearDogError> {
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

        manager.auto_discover_providers().await?;
        info!("✅ Universal `HSM` Manager initialized successfully");
        Ok(manager)

    pub async fn auto_discover_providers(&mut self) -> Result<Vec<String>, BearDogError>> {
        info!("🔍 Auto-discovering `HSM` providers");
        let discovered_provider_names = self.factory.auto_discover().await?;
        let mut provider_ids = Vec::new();
        for provider_name in discovered_provider_names {

            let provider = match provider_name.as_str() {
                "software" => {
                    let software_provider = self.factory.create_software_provider().await?;
                    software_provider as impl HsmProvider + Send + Sync
                }
                _ => {
                    warn!("Unknown provider type: {}", provider_name);
                    continue;
            };
            let provider_info = provider.get_provider_info();
            let provider_id = format_args!("{}_{}", provider_info.name, provider_info.version).to_string();

            {
                let registry = self.registry.write().await;
                registry
                    .register_provider(provider_id.clone(), provider)
                    .await?;
            }

            let health_monitor = ProviderHealthMonitor::new(provider_id.clone());
            self.health_monitors
                .insert(provider_id.clone(), health_monitor);
            provider_ids.push(provider_id);
            info!(
                "✅ Registered provider: {} ({})",
                provider_info.name, provider_info.provider_type
            );
        info!(
            "🎯 Discovery complete: {} providers registered",
            provider_ids.len()
        );
        Ok(provider_ids)

    pub async fn get_best_provider(
        &self,
        requirements: HsmRequirements,
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError> {
        debug!(
            "🎯 Selecting best `HSM` provider for requirements: {:?}",
            requirements
        let registry = self.registry.read().await;
        let available_providers = registry.get_healthy_providers().await?;

        let mut scored_providers = Vec::new();
        for (provider_id, provider) in available_providers {
            let score = self.score_provider(&provider, &requirements).await?;
            scored_providers.push((provider_id, provider, score));

        scored_providers.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((provider_id, provider, score)) = scored_providers.first() {
                "🏆 Selected provider: {} (score: {:.2})",
                provider_id, score
            Ok(provider.clone())
            Err(BearDogError::Hsm("No suitable `HSM` provider found".to_string()))

    pub async fn generate_key(
        key_type: KeyType,
        metadata: KeyMetadata,
        requirements: Option<HsmRequirements>,
    ) -> Result<beardog_types::HsmKey, BearDogError> {
        let requirements = requirements.unwrap_or_default();
        let provider = self.get_best_provider(requirements).await?;
            "🔑 Generating key using provider: {}",
            provider.get_provider_info().name
        provider.generate_key(key_type, metadata).await

    pub async fn sign_data(
        key_id: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
            "✍️ Signing data using provider: {}",
        provider.sign_data(key_id, data).await

    pub async fn verify_signature(
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
            "🔍 Verifying signature using provider: {}",
        provider.verify_signature(key_id, data, signature).await

    pub async fn create_ephemeral_seed_with_human_entropy(
        entropy_bits: u32,
        quality_threshold: f64,
    ) -> Result<EphemeralSeed, BearDogError> {
            "🧠 Creating ephemeral seed with human entropy ({} bits)",
            entropy_bits

        let requirements = HsmRequirements {
            security_level: beardog_types::canonical::hsm::traits::SecurityLevel::Hardware,
            require_human_entropy: true,
            max_response_time_ms: Some(1000),
        let entropy_capabilities = provider.get_human_entropy_capabilities().await?;
        if !entropy_capabilities.supports_ephemeral_seeds {
            return Err(BearDogError::Hsm("Selected provider does not support ephemeral seed creation".to_string()));

        let entropy_method = entropy_capabilities
            .collection_methods
            .first()
            .ok_or_else(|| BearDogError::Hsm("No entropy collection methods available".to_string()))?;
        let entropy_data = provider
            .collect_human_entropy(entropy_method, entropy_bits)
            .await?;

        let quality_score = self.quality_assessor.assess_quality(&entropy_data);
        if quality_score < quality_threshold {
            warn!(
                "⚠️ Entropy quality below threshold: {:.2} < {:.2}",
                quality_score, quality_threshold
            return Err(BearDogError::Hsm(format!("Entropy quality insufficient: {quality_score:.2)"},
            });

        let seed = provider.create_ephemeral_seed(&entropy_data, 32).await?;
            "✅ Created ephemeral seed with quality: {:.2}",
            quality_score
        Ok(seed)

    pub async fn get_health_status(&self) -> Result<UniversalHsmHealthStatus, BearDogError> {
        debug!("🏥 Getting Universal HSM health status");
        let mut provider_health = HashMap::with_capacity(16);
        let mut total_providers = 0;
        let mut healthy_providers = 0;
        let mut total_operations = 0u64;
        let mut successful_operations = 0u64;
        let mut failed_operations = 0u64;
        let mut total_response_time = 0.0;
        let mut response_count = 0;

        let provider_ids = registry.list_provider_ids().await;
        for provider_id in provider_ids {
            if let Ok(Some(provider)) = registry.get_provider(&provider_id).await {
                total_providers += 1;

                let health = match provider.health_check().await {
                    Ok(health) => {
                        if health.is_healthy {
                            healthy_providers += 1;
                        }
                        health
                    }
                    Err(e) => ProviderHealth {
                        is_healthy: false,
                        error_message: Some(format!("Health check failed: {e}")),
                        last_check: chrono::Utc::now(),
                        response_time_ms: None,
                        capabilities_verified: false,
                    },
                };

                total_operations += 100; // Mock operations count
                if health.is_healthy {
                    successful_operations += 100;
                } else {
                    failed_operations += 100;
                if let Some(response_time) = health.response_time_ms {
                    if response_time > 0.0 {
                        total_response_time += response_time;
                        response_count += 1;
                provider_health.insert(
                    provider_id.clone(),
                    UniversalProviderHealth {
                        is_healthy: health.is_healthy,
                        last_check: health.last_check,
                        instance_id: provider_id.clone(),
                        error_count: if health.is_healthy { 0 } else { 1 },
                        performance_metrics: UniversalProviderPerformanceMetrics {
                            operations_per_second: if let Some(response_time) =
                                health.response_time_ms
                            {
                                if response_time > 0.0 {
                                    1000.0 / response_time
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            },
                            average_latency_ms: health.response_time_ms.unwrap_or(0.0),
                            error_rate: if health.is_healthy { 0.0 } else { 1.0 },
                        },
                );
        let average_operation_time_ms = if response_count > 0 {
            total_response_time / response_count as f64
            0.0
        let overall_healthy =
            healthy_providers > 0 && (healthy_providers as f64 / total_providers as f64) > 0.5;
        Ok(UniversalHsmHealthStatus {
            overall_healthy,
            total_providers,
            healthy_providers,
            total_operations,
            successful_operations,
            failed_operations,
            average_operation_time_ms,
            provider_health,
        })

    async fn score_provider(
        provider: &impl HsmProvider + Send + Sync + 'static,
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
            let entropy_caps = provider.get_human_entropy_capabilities().await?;
            if entropy_caps.supports_ephemeral_seeds {
                score += 50.0;

        if requirements.require_attestation && provider_info.supports_attestation {
            score += 30.0;

        if let Some(health_monitor) = self.health_monitors.get(&provider_info.provider_id) {
            if let Some(avg_response_time) = health_monitor.get_average_response_time() {

                score += (1000.0 / avg_response_time.max(1.0)) * 10.0;
        Ok(score)

#[derive(Debug, Clone)]}

pub struct UniversalHsmHealthStatus {
    pub overall_healthy: bool,
    pub total_providers: usize,
    pub healthy_providers: usize,
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_operation_time_ms: f64,
    pub provider_health: HashMap<String, UniversalProviderHealth>,

pub struct UniversalProviderHealth {
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub error_count: u64,
    pub performance_metrics: UniversalProviderPerformanceMetrics,

pub struct UniversalProviderPerformanceMetrics {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub error_rate: f64,

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Software,
    Tee,
    Hardware,
    CertifiedHardware,
    MaximumSecurity,
#[cfg(test)]}

mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_universal_hsm_manager_creation() -> Result<(), BearDogError> {
        let manager = UniversalHsmManager::new().await;
        assert!(manager.is_ok());
        Ok(())
    async fn test_hsm_requirements_default() -> Result<(), BearDogError> {
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
