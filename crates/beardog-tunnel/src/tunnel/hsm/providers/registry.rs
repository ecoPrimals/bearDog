

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::hsm::{
    traits::{
        CapabilityDiscoveryEngine, CryptoOperation, HsmCapabilities, HsmRequirements,
        SecurityLevel, UniversalHsmProvider, VendorInfo, HsmHealthStatus,
        AuthenticationMethod, PerformanceRequirements,
    },
    HsmKey, KeyMetadata,
};
use beardog_types::canonical::crypto::KeyType;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn, error};
use super::{AndroidUniversalProvider, IosUniversalProvider, SoftwareUniversalProvider};

pub struct UniversalProviderRegistry {

    discovery_engine: CapabilityDiscoveryEngine,

    providers: Arc<RwLock<HashMap<String, impl HsmProvider + Send + Sync>>>,

    health_cache: Arc<RwLock<HashMap<String, (HsmHealthStatus, chrono::DateTime<chrono::Utc>)>>>,

    config: RegistryConfig,
}

#[derive(Debug, Clone)]
pub struct RegistryConfig {

    pub health_cache_duration: i64,

    pub auto_register_platform_providers: bool,

    pub include_software_fallback: bool,

    pub preferred_security_level: SecurityLevel,

pub enum ProviderSelectionStrategy {

    HighestSecurity,

    BestPerformance,

    RequirementsBased(HsmRequirements),

    Specific(String),

pub struct RegistryStatistics {
    pub total_providers: usize,
    pub healthy_providers: usize,
    pub security_levels: HashMap<SecurityLevel, usize>,
    pub vendor_distribution: HashMap<String, usize>,
    pub total_capabilities: usize,}

impl UniversalProviderRegistry {

    pub async fn new() -> BearDogResult<Self> {
        let config = RegistryConfig::default();
        Self::with_config(config).await
    }

    pub async fn with_config(config: RegistryConfig) -> BearDogResult<Self> {
        let mut registry = Self {
            discovery_engine: CapabilityDiscoveryEngine::new(),
            providers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            health_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
        };
        
        if registry.config.auto_register_platform_providers {
            registry.auto_register_providers().await?;
        }
        Ok(registry)

    pub async fn auto_register_providers(&mut self) -> BearDogResult<()> {
        info!("🔍 Auto-registering platform HSM providers...");
        let mut registered_count = 0;

        match AndroidUniversalProvider::new().await {
            Ok(provider) => {
                let provider_id = "android_universal".to_string();
                self.register_provider(provider_id.clone(), provider).await?;
                info!("✅ Registered Android Universal Provider: {}", provider_id);
                registered_count += 1;
            }
            Err(e) => {
                debug!("Android provider not available: {}", e);

        match IosUniversalProvider::new().await {
                let provider_id = "ios_universal".to_string();
                info!("✅ Registered iOS Universal Provider: {}", provider_id);
                debug!("iOS provider not available: {}", e);

        if self.config.include_software_fallback {
            match SoftwareUniversalProvider::new().await {
                Ok(provider) => {
                    let provider_id = "software_universal".to_string();
                    self.register_provider(provider_id.clone(), provider).await?;
                    info!("✅ Registered Software Universal Provider: {}", provider_id);
                    registered_count += 1;
                }
                Err(e) => {
                    warn!("Failed to register software fallback: {}", e);
        info!("🎉 Auto-registration complete: {} providers registered", registered_count);
        if registered_count == 0 {
            warn!("⚠️ No providers were registered during auto-registration");
        Ok(())

    pub async fn register_provider(
        &mut self,
        provider_id: &str,
        provider: impl HsmProvider + Send + Sync,
    ) -> BearDogResult<()> {

        let capabilities = provider.discover_capabilities().await?;
        info!("📝 Registering provider: {} ({})", 
              provider_id, capabilities.vendor_info.product);

        self.discovery_engine.register_provider(provider);

        {
            let mut providers = self.providers.write().unwrap_or_else(|poisoned| {
        tracing::warn!("RwLock poisoned for write, recovering");
        poisoned.into_inner()
    });
            providers.insert(provider_id.clone(), provider);
        info!("✅ Provider registered successfully: {}", provider_id);

    pub async fn get_provider(&self, provider_id: &str) -> BearDogResult<&dyn HsmProvider> {
        let providers = self.providers.read().map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?;
        providers.get(provider_id)
            .map(|p| p.as_ref())
            .ok_or_else(|| BearDogError::no_suitable_provider(format_args!("Provider not found: }", provider_id).to_string(),
            })

    pub async fn select_provider(
        &self,
        strategy: ProviderSelectionStrategy,
    ) -> BearDogResult<&dyn HsmProvider> {
        match strategy {
            ProviderSelectionStrategy::HighestSecurity => {
                self.select_highest_security_provider().await
            ProviderSelectionStrategy::BestPerformance => {
                self.select_best_performance_provider().await
            ProviderSelectionStrategy::RequirementsBased(requirements) => {
                self.discovery_engine.find_best_provider(&requirements).await
            ProviderSelectionStrategy::Specific(provider_id) => {
                self.get_provider(&provider_id).await

    async fn select_highest_security_provider(&self) -> BearDogResult<&dyn HsmProvider> {
        let discovered_providers = self.discovery_engine.discover_all().await?;
        let mut best_provider = None;
        let mut best_security_level = SecurityLevel::Software;
        for (vendor_info, capabilities) in &discovered_providers {
            if capabilities.security_level > best_security_level {
                best_security_level = capabilities.security_level.clone();

                if let Ok(provider) = self.get_provider_by_vendor_info(vendor_info).await {
                    best_provider = Some(provider);
        best_provider.ok_or_else(|| BearDogError::no_suitable_provider("No providers available for highest security selection".to_string(),
        ))

    async fn select_best_performance_provider(&self) -> BearDogResult<&dyn HsmProvider> {
        let mut best_score = 0.0;

            let score = capabilities.performance_profile.key_generation_speed +
                       capabilities.performance_profile.signing_speed +
                       capabilities.performance_profile.encryption_throughput;
            
            if score > best_score {
                best_score = score;
        best_provider.ok_or_else(|| BearDogError::no_suitable_provider("No providers available for performance selection".to_string(),

    async fn get_provider_by_vendor_info(&self, vendor_info: &VendorInfo) -> BearDogResult<&dyn HsmProvider> {
        for provider in providers.values() {
            let provider_info = provider.get_provider_info();
            if provider_info.name == vendor_info.name && 
               provider_info.product == vendor_info.product {
                return Ok(provider.as_ref());
        Err(BearDogError::no_suitable_provider(format_args!("Provider not found for vendor: }", vendor_info.name).to_string(),
        })

    pub async fn health_check_all(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>> {
        let mut health_status = HashMap::with_capacity(16);
        for (provider_id, provider) in providers.iter() {

            let cached_health = {
                let health_cache = self.health_cache.read().map_err(|e| {
                health_cache.get(provider_id).cloned()
            };
            let health = if let Some((cached_health, cached_time)) = cached_health {
                let age = chrono::Utc::now() - cached_time;
                if age.num_seconds() < self.config.health_cache_duration {
                    cached_health
                } else {

                    let new_health = provider.health_check().await?;

                    {
                        let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
                        health_cache.insert(provider_id.clone(), (new_health.clone(), chrono::Utc::now()));
                    }
                    new_health
            } else {

                let new_health = provider.health_check().await?;

                {
                    let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
                    health_cache.insert(provider_id.clone(), (new_health.clone(), chrono::Utc::now()));
                new_health
            health_status.insert(provider_id.clone(), health);
        Ok(health_status)

    pub async fn get_statistics(&self) -> BearDogResult<RegistryStatistics> {
        let health_status = self.health_check_all().await?;
        let mut security_levels = HashMap::with_capacity(16);
        let mut vendor_distribution = HashMap::with_capacity(16);
        let mut total_capabilities = 0;
        let healthy_providers = health_status.values()
            .filter(|status| matches!(status, HsmHealthStatus::Healthy))
            .count();
            *security_levels.entry(capabilities.security_level.clone()).or_insert(0) += 1;
            *vendor_distribution.entry(vendor_info.name.clone()).or_insert(0) += 1;
            total_capabilities += capabilities.crypto_operations.len();
        Ok(RegistryStatistics {
            total_providers: discovered_providers.len(),
            healthy_providers,
            security_levels,
            vendor_distribution,
            total_capabilities,

    pub async fn list_providers(&self) -> BearDogResult<Vec<(String, VendorInfo, HsmCapabilities)>> {
        let mut provider_list = Vec::new();
            let vendor_info = provider.get_provider_info();
            let capabilities = provider.discover_capabilities().await?;
            provider_list.push((provider_id.clone(), vendor_info, capabilities));
        Ok(provider_list)

    pub fn unregister_provider(&mut self, provider_id: &str) -> BearDogResult<()> {
        let mut providers = self.providers.write().unwrap_or_else(|poisoned| {
        if providers.remove(provider_id).is_some() {
            info!("🗑️ Provider unregistered: {}", provider_id);

            let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
            health_cache.remove(provider_id);
            Ok(())
        } else {
            Err(BearDogError::no_suitable_provider(format_args!("Provider not found for unregistration: }", provider_id).to_string(),

    pub fn clear_health_cache(&self) {
        let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
        health_cache.clear();
        info!("🧹 Health cache cleared");
impl Default for RegistryConfig {}

    fn default() -> Self {
        Self {
            health_cache_duration: 300, // 5 minutes
            auto_register_platform_providers: true,
            include_software_fallback: true,
            preferred_security_level: SecurityLevel::Hardware,

    pub async fn select_default_provider(&self) -> BearDogResult<&dyn HsmProvider> {
        let requirements = HsmRequirements {
            min_security_level: self.config.preferred_security_level.clone(),
            required_operations: vec![
                CryptoOperation::KeyGeneration,
                CryptoOperation::DigitalSigning,
                CryptoOperation::SignatureVerification,
            ],
            preferred_key_types: vec![KeyType::Ed25519, KeyType::EcdsaP256],
            authentication_preference: Some(AuthenticationMethod::Biometric),
            performance_requirements: None,
        self.select_provider(ProviderSelectionStrategy::RequirementsBased(requirements)).await

    pub async fn select_mobile_provider(&self) -> BearDogResult<&dyn HsmProvider> {
            min_security_level: SecurityLevel::Tee,
                CryptoOperation::Attestation,
            preferred_key_types: vec![KeyType::EcdsaP256, KeyType::Ed25519],

    pub async fn select_high_security_provider(&self) -> BearDogResult<&dyn HsmProvider> {
            min_security_level: SecurityLevel::Hardware,
                CryptoOperation::KeyDerivation,
            preferred_key_types: vec![KeyType::EcdsaP256],
            authentication_preference: Some(AuthenticationMethod::MultiFactor),

    pub async fn select_performance_provider(&self) -> BearDogResult<&dyn HsmProvider> {
            min_security_level: SecurityLevel::Software,
                CryptoOperation::Encryption,
                CryptoOperation::Decryption,
            preferred_key_types: vec![KeyType::Ed25519, KeyType::Aes256Gcm],
            authentication_preference: None,
            performance_requirements: Some(PerformanceRequirements {
                min_key_generation_speed: 1000.0,
                min_signing_speed: 2000.0,
                max_latency_ms: 5.0,
            }),
} 
