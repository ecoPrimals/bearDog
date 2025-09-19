

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
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
    pub auto_register_platform_providers: bool,

    /// Whether include_software_fallback is enabled
    pub include_software_fallback: bool,

    /// The preferred security level value
    pub preferred_security_level: SecurityLevel,

pub enum ProviderSelectionStrategy {


    /// Represents highest security variant
    HighestSecurity,


    BestPerformance,

    /// State indicating requirementsbased
    RequirementsBased(usize,
    pub healthy_providers: usize,
    /// Mapping of security levels
    pub security_levels: HashMap<SecurityLevel, usize>,
    /// Mapping of vendor distribution
    pub vendor_distribution: HashMap<String, usize>,
    /// Number of total_capabilities
    pub total_capabilities: usize,}
    pub total_capabilities: usize,}
    pub total_capabilities: usize,}

impl UniversalProviderRegistry {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        let config = RegistryConfig::default();
        Self::with_config(config)
    }

/// With Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates instance with config
    pub fn with_config(config: RegistryConfig) -> Result<Self, BearDogError> {
        let mut registry = Self {
            discovery_engine: CapabilityDiscoveryEngine::new(),
            providers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            health_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
        };
        
        if registry.config.auto_register_platform_providers {
            registry.auto_register_providers()?;
        }
        Ok(registry)

/// Auto Register Providers operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn auto_register_providers(&mut self) -> Result<(), BearDogError> {
        info!("🔍 Auto-registering platform HSM providers...");
        let mut registered_count = 0;

        match AndroidUniversalProvider::new({}", provider_id);
                registered_count += 1;
            }
            Err({}", e);

        match IosUniversalProvider::new({}", provider_id);
                debug!("iOS provider not available: {}", e);

        if self.config.include_software_fallback {
            match SoftwareUniversalProvider::new({}", provider_id);
                    registered_count += 1;
                }
                Err({}", e);
        info!("🎉 Auto-registration complete: {} providers registered", registered_count);
        if registered_count == 0 {
            warn!("⚠️ No providers were registered during auto-registration");
        Ok(&str,
        provider: impl HsmProvider + Send + Sync,
    ) -> Result<(), BearDogError> {

        let capabilities = provider.discover_capabilities()?;
        info!("📝 Registering provider: {} ({})", 
              provider_id, capabilities.vendor_info.product);

        self.discovery_engine.register_provider(provider);

        {
            let mut providers = self.providers.write().unwrap_or_else(|poisoned| {
        tracing::warn!("RwLock poisoned for write, recovering");
        poisoned.into_inner({}", provider_id);

/// Get Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets provider
    /// Gets provider
    pub fn get_provider(&self, provider_id: &str) -> Result<&dyn HsmProvider, BearDogError> {
        let providers = self.providers.read().map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?;
        providers.get(provider_id)
            .map(|p| p.as_ref())
            .ok_or_else(|| BearDogError::no_suitable_provider(}", provider_id).to_string(),
    ) -> Result<&dyn HsmProvider, BearDogError> {
        match strategy {
            ProviderSelectionStrategy::HighestSecurity => {
                self.select_highest_security_provider()
            ProviderSelectionStrategy::BestPerformance => {
                self.select_best_performance_provider()
            ProviderSelectionStrategy::RequirementsBased(requirements) => {
                self.discovery_engine.find_best_provider(&requirements)
            ProviderSelectionStrategy::Specific(provider_id) => {
                self.get_provider(&provider_id)


    fn select_highest_security_provider(&self) -> Result<&dyn HsmProvider, BearDogError> {
        let discovered_providers = self.discovery_engine.discover_all()?;
        let mut best_provider = None;
        let mut best_security_level = SecurityLevel::Software;
        for (vendor_info, capabilities) in &discovered_providers {
            if capabilities.security_level > best_security_level {
                best_security_level = &capabilities.security_level;

                if let Ok(provider) = self.get_provider_by_vendor_info(vendor_info) {
                    best_provider = Some(provider);
        best_provider.ok_or_else(|| BearDogError::no_suitable_provider("No providers available for highest security selection"))


    fn select_best_performance_provider(&self) -> Result<&dyn HsmProvider, BearDogError> {
        let mut best_score = 0.0;

            let score = capabilities.performance_profile.key_generation_speed +
                       capabilities.performance_profile.signing_speed +
                       capabilities.performance_profile.encryption_throughput;
            
            if score > best_score {
                best_score = score;
        best_provider.ok_or_else(|| BearDogError::no_suitable_provider("No providers available for performance selection".to_string(),

    /// Gets provider_by_vendor_info
    fn get_provider_by_vendor_info(&self, vendor_info: &VendorInfo) -> Result<&dyn HsmProvider, BearDogError> {
        for provider in providers.values() {
            let provider_info = provider.get_provider_info();
            if provider_info.name == vendor_info.name && 
               provider_info.product == vendor_info.product {
                return Ok(provider.as_ref());
        Err(BearDogError::no_suitable_provider(}", vendor_info.name).to_string(),
        })

/// Health Check All operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check_all(&self) -> Result<HashMap<String, HsmHealthStatus, BearDogError>> {
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

                    let new_health = provider.health_check()?;

                    {
                        let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
                        health_cache.insert(provider_id.clone(), (new_health, chrono::Utc::now()));
                    }
                    new_health
            } else {

                let new_health = provider.health_check()?;

                {
                    let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
                    health_cache.insert(provider_id.clone(), (new_health, chrono::Utc::now()));
                new_health
            health_status.insert(provider_id.clone(), health);
        Ok(health_status)

/// Get Statistics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> Result<RegistryStatistics, BearDogError> {
        let health_status = self.health_check_all()?;
        let mut security_levels = HashMap::with_capacity(16);
        let mut vendor_distribution = HashMap::with_capacity(16);
        let mut total_capabilities = 0;
        let healthy_providers = health_status.values()
            .filter(|status| matches!(status, HsmHealthStatus::Healthy))
            .count();
            *security_levels.entry(capabilities.security_level).or_insert(0) += 1;
            *vendor_distribution.entry(vendor_info.name).or_insert(0) += 1;
            total_capabilities += capabilities.crypto_operations.len();
        Ok(RegistryStatistics {
            total_providers: discovered_providers.len(),
            healthy_providers,
            security_levels,
            vendor_distribution,
            total_capabilities,

/// List Providers operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_providers(&self) -> Result<Vec<(String, VendorInfo, HsmCapabilities), BearDogError>> {
        let mut provider_list = Vec::new();
            let vendor_info = provider.get_provider_info();
            let capabilities = provider.discover_capabilities()?;
            provider_list.push((provider_id.clone(), vendor_info, capabilities));
        Ok(provider_list)

/// Unregister Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn unregister_provider(&mut self, provider_id: &str) -> Result<(), BearDogError> {
        let mut providers = self.providers.write({}", provider_id);

            let mut health_cache = self.health_cache.write().unwrap_or_else(|poisoned| {
            health_cache.remove(provider_id);
            Ok(())
        } else {
            Err(BearDogError::no_suitable_provider(}", provider_id).to_string(), // 5 minutes
            auto_register_platform_providers: true,
            include_software_fallback: true,
            preferred_security_level: SecurityLevel::Hardware,

/// Select Default Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn select_default_provider(self.&config.preferred_security_level,
            required_operations: vec![
                CryptoOperation::KeyGeneration,
                CryptoOperation::DigitalSigning,
                CryptoOperation::SignatureVerification,
            ],
            preferred_key_types: vec![KeyType::Ed25519, KeyType::EcdsaP256],
            authentication_preference: Some(AuthenticationMethod::Biometric),
            performance_requirements: None,
        self.select_provider(ProviderSelectionStrategy::RequirementsBased(SecurityLevel::Tee,
                CryptoOperation::Attestation,
            preferred_key_types: vec![KeyType::EcdsaP256, KeyType::Ed25519],

/// Select High Security Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn select_high_security_provider(SecurityLevel::Hardware,
                CryptoOperation::KeyDerivation,
            preferred_key_types: vec![KeyType::EcdsaP256],
            authentication_preference: Some(AuthenticationMethod::MultiFactor),

///
/// # Errors
/// Returns an error if the operation fails.
    pub fn select_performance_provider(SecurityLevel::Software,
                CryptoOperation::Encryption,
                CryptoOperation::Decryption,
            preferred_key_types: vec![KeyType::Ed25519, KeyType::Aes256Gcm],
            authentication_preference: None,
            performance_requirements: Some(1000.0,
                min_signing_speed: 2000.0,
                max_latency_ms: 5.0,
            }),
} 
