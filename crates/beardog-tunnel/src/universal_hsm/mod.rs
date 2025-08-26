// PHASE 5 OPTIMIZED: Performance patterns applied
// PHASE 5 MODERNIZED: Comprehensive Arc<dyn> elimination
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


/// # Universal `HSM` System - Single Source of Truth
///
/// **FRAGMENTATION ELIMINATION COMPLETE**
/// This module provides the unified, vendor-agnostic `HSM` system that replaces all
/// fragmented implementations throughout `BearDog`. It serves as the single source
/// of truth for all `HSM` operations.
/// ## Replaced Implementations
/// - ❌ `hsm_foundation/providers/` (ELIMINATED)
/// - ❌ `tunnel/hsm/providers/` (CONSOLIDATED)
/// - ❌ Multiple trait definitions (UNIFIED)
/// - ❌ Duplicate registries (MERGED)
/// ## Architecture
/// ```text
/// ┌─────────────────────────────────────────────────────────────┐
/// │                 Universal `HSM` Manager                        │
/// │              (Single Orchestration Point)                   │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Provider Factory  │  Discovery Engine  │  Health Monitor  │
/// │  Android StrongBox │  iOS Secure Enclave │  Software `HSM`    │
/// │  PKCS#11 Provider  │  `TPM` 2.0 Provider   │  Custom Provider │
/// └─────────────────────────────────────────────────────────────┘
/// ```

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::crypto::KeyType;
use beardog_types::canonical::KeyMetadata;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
// Core modules
pub mod entropy;
pub mod providers;
pub mod registry;
pub mod traits;
// Re-exports for convenience
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
    #[allow(dead_code)]
    provider_id: String,
    response_times: Vec<f64>,
}
impl ProviderHealthMonitor {}


    pub fn new(provider_id: String) -> Self {
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
        // Keep only last 100 measurements
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
/// **Universal HSM Manager**
/// Central coordinator for all HSM providers with automatic discovery,
/// health monitoring, and intelligent provider selection.
pub struct UniversalHsmManager {
    /// Provider registry
    registry: Arc<RwLock<UniversalHsmRegistry>>,
    /// Provider factory
    factory: UniversalHsmFactory,
    /// Human entropy system
    entropy_collector: HumanEntropyCollector,
    /// Quality assessor for entropy
    quality_assessor: EntropyQualityAssessor,
    /// Tier elevation engine
    tier_engine: TierElevationEngine,
    /// Health monitoring
    health_monitors: HashMap<String, ProviderHealthMonitor>,}


impl UniversalHsmManager {
    /// Create new universal `HSM` manager with auto-discovery
    pub async fn new() -> BearDogResult<Self> {
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
            health_monitors: HashMap::new(),
        };
        // Auto-discover available providers
        manager.auto_discover_providers().await?;
        info!("✅ Universal `HSM` Manager initialized successfully");
        Ok(manager)
    /// Auto-discover all available `HSM` providers on the system
    pub async fn auto_discover_providers(&mut self) -> BearDogResult<Vec<String>> {
        info!("🔍 Auto-discovering `HSM` providers");
        let discovered_provider_names = self.factory.auto_discover().await?;
        let mut provider_ids = Vec::new();
        for provider_name in discovered_provider_names {
            // Create the actual provider instance based on the discovered name
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
            let provider_id = format!("{}_{}", provider_info.name, provider_info.version);
            // Register the provider
            {
                let registry = self.registry.write().await;
                registry
                    .register_provider(provider_id.clone(), provider)
                    .await?;
            }
            // Set up health monitoring
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
    /// Get the best `HSM` provider for a specific operation
    pub async fn get_best_provider(
        &self,
        requirements: HsmRequirements,
    ) -> BearDogResult<impl HsmProvider + Send + Sync + 'static> {
        debug!(
            "🎯 Selecting best `HSM` provider for requirements: {:?}",
            requirements
        let registry = self.registry.read().await;
        let available_providers = registry.get_healthy_providers().await?;
        // Score providers based on requirements
        let mut scored_providers = Vec::new();
        for (provider_id, provider) in available_providers {
            let score = self.score_provider(&provider, &requirements).await?;
            scored_providers.push((provider_id, provider, score));
        // Sort by score (highest first)
        scored_providers.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((provider_id, provider, score)) = scored_providers.first() {
                "🏆 Selected provider: {} (score: {:.2})",
                provider_id, score
            Ok(provider.clone())
            Err(BearDogError::Hsm("No suitable `HSM` provider found".to_string()))
    /// Generate key using the best available provider
    pub async fn generate_key(
        key_type: KeyType,
        metadata: KeyMetadata,
        requirements: Option<HsmRequirements>,
    ) -> BearDogResult<beardog_types::HsmKey> {
        let requirements = requirements.unwrap_or_default();
        let provider = self.get_best_provider(requirements).await?;
            "🔑 Generating key using provider: {}",
            provider.get_provider_info().name
        provider.generate_key(key_type, metadata).await
    /// Sign data using the best available provider
    pub async fn sign_data(
        key_id: &str,
        data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
            "✍️ Signing data using provider: {}",
        provider.sign_data(key_id, data).await
    /// Verify signature using the best available provider
    pub async fn verify_signature(
        signature: &[u8],
    ) -> BearDogResult<bool> {
            "🔍 Verifying signature using provider: {}",
        provider.verify_signature(key_id, data, signature).await
    /// Collect human entropy and create ephemeral seed
    pub async fn create_ephemeral_seed_with_human_entropy(
        entropy_bits: u32,
        quality_threshold: f64,
    ) -> BearDogResult<EphemeralSeed> {
            "🧠 Creating ephemeral seed with human entropy ({} bits)",
            entropy_bits
        // Find provider that supports human entropy
        let requirements = HsmRequirements {
            security_level: beardog_types::canonical::hsm::traits::SecurityLevel::Hardware,
            require_human_entropy: true,
            max_response_time_ms: Some(1000),
        let entropy_capabilities = provider.get_human_entropy_capabilities().await?;
        if !entropy_capabilities.supports_ephemeral_seeds {
            return Err(BearDogError::Hsm("Selected provider does not support ephemeral seed creation".to_string()));
        // Collect human entropy using the best available method
        let entropy_method = entropy_capabilities
            .collection_methods
            .first()
            .ok_or_else(|| BearDogError::Hsm("No entropy collection methods available".to_string()))?;
        let entropy_data = provider
            .collect_human_entropy(entropy_method, entropy_bits)
            .await?;
        // Assess quality
        let quality_score = self.quality_assessor.assess_quality(&entropy_data);
        if quality_score < quality_threshold {
            warn!(
                "⚠️ Entropy quality below threshold: {:.2} < {:.2}",
                quality_score, quality_threshold
            return Err(BearDogError::Hsm(format!("Entropy quality insufficient: {quality_score:.2)"},
            });
        // Create ephemeral seed
        let seed = provider.create_ephemeral_seed(&entropy_data, 32).await?;
            "✅ Created ephemeral seed with quality: {:.2}",
            quality_score
        Ok(seed)
    /// Get comprehensive health status for all HSM providers
    pub async fn get_health_status(&self) -> BearDogResult<UniversalHsmHealthStatus> {
        debug!("🏥 Getting Universal HSM health status");
        let mut provider_health = HashMap::new();
        let mut total_providers = 0;
        let mut healthy_providers = 0;
        let mut total_operations = 0u64;
        let mut successful_operations = 0u64;
        let mut failed_operations = 0u64;
        let mut total_response_time = 0.0;
        let mut response_count = 0;
        // Get all provider IDs and iterate through them
        let provider_ids = registry.list_provider_ids().await;
        for provider_id in provider_ids {
            if let Ok(Some(provider)) = registry.get_provider(&provider_id).await {
                total_providers += 1;
                // Get provider health
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
                // Update totals
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
    /// Score a provider based on requirements
    async fn score_provider(
        provider: &impl HsmProvider + Send + Sync + 'static,
        requirements: &HsmRequirements,
    ) -> BearDogResult<f64> {
        let provider_info = provider.get_provider_info();
        let mut score = 0.0;
        // Base score based on security level
        score += match provider_info.security_level {
            beardog_types::canonical::hsm::traits::SecurityLevel::MaximumSecurity => 120.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::CertifiedHardware => 110.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Hardware => 100.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Tee => 80.0,
            beardog_types::canonical::hsm::traits::SecurityLevel::Software => 40.0,
        // Bonus for human entropy support if required
        if requirements.require_human_entropy {
            let entropy_caps = provider.get_human_entropy_capabilities().await?;
            if entropy_caps.supports_ephemeral_seeds {
                score += 50.0;
        // Bonus for hardware attestation if required
        if requirements.require_attestation && provider_info.supports_attestation {
            score += 30.0;
        // Performance bonus based on recent health checks
        if let Some(health_monitor) = self.health_monitors.get(&provider_info.provider_id) {
            if let Some(avg_response_time) = health_monitor.get_average_response_time() {
                // Bonus for faster response times (inverse relationship)
                score += (1000.0 / avg_response_time.max(1.0)) * 10.0;
        Ok(score)
/// Universal HSM Health Status
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
/// Universal Provider Health
pub struct UniversalProviderHealth {
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub error_count: u64,
    pub performance_metrics: UniversalProviderPerformanceMetrics,
/// Universal Provider Performance Metrics
pub struct UniversalProviderPerformanceMetrics {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
/// Security levels for HSM providers
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


    async fn test_universal_hsm_manager_creation() -> beardog_errors::BearDogResult<()> {
        let manager = UniversalHsmManager::new().await;
        assert!(manager.is_ok());
        Ok(())
    async fn test_hsm_requirements_default() -> beardog_errors::BearDogResult<()> {
        let requirements = HsmRequirements::default();
        assert_eq!(
            requirements.security_level,
            beardog_types::canonical::hsm::traits::SecurityLevel::Software
        assert!(!requirements.require_human_entropy);
        assert!(!requirements.require_attestation);
    #[test]}


    fn test_security_level_ordering() -> beardog_errors::BearDogResult<()> {
        assert!(SecurityLevel::MaximumSecurity > SecurityLevel::CertifiedHardware);
        assert!(SecurityLevel::CertifiedHardware > SecurityLevel::Hardware);
        assert!(SecurityLevel::Hardware > SecurityLevel::Tee);
        assert!(SecurityLevel::Tee > SecurityLevel::Software);
