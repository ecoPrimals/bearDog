// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Provider Registry
//!
//! Central registry for managing and selecting HSM providers across platforms.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::info;

/// Universal HSM Provider Registry
pub struct UniversalProviderRegistry {
    /// Registered providers
    providers: HashMap<String, ProviderInfo>,
    /// Provider selection strategy
    strategy: SelectionStrategy,
    /// Registry statistics
    stats: RegistryStats,
}

/// Provider information
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    /// Provider name/ID
    pub id: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Security level
    pub security_level: u8,
    /// Whether provider is available
    pub available: bool,
    /// Provider metadata
    pub metadata: HashMap<String, String>,
}

/// Provider type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProviderType {
    /// Software HSM
    Software,
    /// Android StrongBox/TEE
    Android,
    /// iOS Secure Enclave
    Ios,
    /// PKCS#11 Hardware HSM
    Pkcs11,
    /// TPM 2.0
    Tpm,
    /// Cloud HSM
    Cloud,
}

/// Provider selection strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionStrategy {
    /// Highest security level
    HighestSecurity,
    /// Best performance
    BestPerformance,
    /// Requirements-based selection
    RequirementsBased,
}

/// Registry statistics
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    /// Total registered providers
    pub total_providers: usize,
    /// Healthy/available providers
    pub healthy_providers: usize,
    /// Security level distribution
    pub security_levels: HashMap<u8, usize>,
    /// Vendor distribution
    pub vendor_distribution: HashMap<String, usize>,
    /// Total capabilities
    pub total_capabilities: usize,
}

impl UniversalProviderRegistry {
    /// Create a new provider registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            strategy: SelectionStrategy::HighestSecurity,
            stats: RegistryStats::default(),
        }
    }

    /// Register a provider
    ///
    /// # Errors
    /// Returns an error if registration fails
    pub fn register_provider(&mut self, info: ProviderInfo) -> Result<(), BearDogError> {
        info!(
            "📝 Registering HSM provider: {} ({:?})",
            info.id, info.provider_type
        );

        // Update stats
        self.stats.total_providers += 1;
        if info.available {
            self.stats.healthy_providers += 1;
        }

        *self
            .stats
            .security_levels
            .entry(info.security_level)
            .or_insert(0) += 1;

        if let Some(vendor) = info.metadata.get("vendor") {
            *self
                .stats
                .vendor_distribution
                .entry(vendor.clone())
                .or_insert(0) += 1;
        }

        self.providers.insert(info.id.clone(), info);
        Ok(())
    }

    /// Unregister a provider
    pub fn unregister_provider(&mut self, provider_id: &str) -> Option<ProviderInfo> {
        if let Some(info) = self.providers.remove(provider_id) {
            info!("🗑️ Unregistered HSM provider: {}", provider_id);

            // Update stats
            self.stats.total_providers = self.stats.total_providers.saturating_sub(1);
            if info.available {
                self.stats.healthy_providers = self.stats.healthy_providers.saturating_sub(1);
            }

            Some(info)
        } else {
            None
        }
    }

    /// Select best provider based on strategy
    pub fn select_provider(&self) -> Option<&ProviderInfo> {
        match self.strategy {
            SelectionStrategy::HighestSecurity => self.select_highest_security(),
            SelectionStrategy::BestPerformance => self.select_best_performance(),
            SelectionStrategy::RequirementsBased => self.select_by_requirements(),
        }
    }

    /// Select provider with highest security level
    fn select_highest_security(&self) -> Option<&ProviderInfo> {
        self.providers
            .values()
            .filter(|p| p.available)
            .max_by_key(|p| p.security_level)
    }

    /// Select provider with best performance
    fn select_best_performance(&self) -> Option<&ProviderInfo> {
        // Prefer software for performance, then mobile, then hardware
        let preference_order = [
            ProviderType::Software,
            ProviderType::Android,
            ProviderType::Ios,
            ProviderType::Tpm,
            ProviderType::Pkcs11,
        ];

        for provider_type in &preference_order {
            if let Some(provider) = self
                .providers
                .values()
                .find(|p| p.available && &p.provider_type == provider_type)
            {
                return Some(provider);
            }
        }

        None
    }

    /// Select provider by requirements
    fn select_by_requirements(&self) -> Option<&ProviderInfo> {
        // Default to highest security for requirements-based
        self.select_highest_security()
    }

    /// Set selection strategy
    pub fn set_strategy(&mut self, strategy: SelectionStrategy) {
        info!("🎯 Setting provider selection strategy: {:?}", strategy);
        self.strategy = strategy;
    }

    /// Get all registered providers
    pub fn list_providers(&self) -> Vec<&ProviderInfo> {
        self.providers.values().collect()
    }

    /// Get provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&ProviderInfo> {
        self.providers.get(provider_id)
    }

    /// Get registry statistics
    pub const fn stats(&self) -> &RegistryStats {
        &self.stats
    }

    /// Get providers by type
    pub fn get_providers_by_type(&self, provider_type: &ProviderType) -> Vec<&ProviderInfo> {
        self.providers
            .values()
            .filter(|p| &p.provider_type == provider_type)
            .collect()
    }

    /// Get available providers
    pub fn get_available_providers(&self) -> Vec<&ProviderInfo> {
        self.providers.values().filter(|p| p.available).collect()
    }
}

impl Default for UniversalProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── Canonical HsmProviderRegistry ───────────────────────────────────────

use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{HsmProviderType, SelectionPreference};
use std::sync::Arc;

/// Runtime registry of `HsmKeyProvider` backends.
///
/// Holds [`crate::tunnel::hsm::HsmKeyProviderBackend`] instances and selects the best one
/// according to [`SelectionPreference`].
pub struct HsmProviderRegistry {
    providers: Vec<Arc<crate::tunnel::hsm::HsmKeyProviderBackend>>,
}

impl HsmProviderRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Probe the current platform and register all available providers.
    ///
    /// On non-Android hosts this registers only the software provider.
    /// On Android it additionally probes for `StrongBox` availability.
    pub async fn discover() -> Self {
        let mut registry = Self::new();

        let sw = crate::tunnel::hsm::software_hsm::create_default_software_hsm()
            .await
            .map(|hsm| Arc::new(crate::tunnel::hsm::HsmKeyProviderBackend::Software(hsm)));

        if let Ok(provider) = sw {
            info!("HSM registry: registered software-rustcrypto provider");
            registry.providers.push(provider);
        }

        // On Android, try to register StrongBox
        #[cfg(target_os = "android")]
        {
            if let Ok(sb) =
                crate::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm::with_defaults()
            {
                if beardog_traits::hsm::HsmKeyProvider::is_available(&sb) {
                    info!("HSM registry: registered android-strongbox provider");
                    registry.providers.push(Arc::new(
                        crate::tunnel::hsm::HsmKeyProviderBackend::AndroidStrongBox(sb),
                    ));
                }
            }
        }

        registry
    }

    /// Manually register a provider.
    pub fn register(&mut self, provider: Arc<crate::tunnel::hsm::HsmKeyProviderBackend>) {
        info!(
            "HSM registry: manually registered provider '{}'",
            provider.provider_id()
        );
        self.providers.push(provider);
    }

    /// # Errors
    ///
    /// Returns an error if the Tor-related operation fails.
    /// Select the best available provider according to `preference`.
    ///
    /// Falls back to the software provider when no hardware provider
    /// is available and `PreferHardware` was requested.
    pub fn select(
        &self,
        preference: SelectionPreference,
    ) -> Result<Arc<crate::tunnel::hsm::HsmKeyProviderBackend>, BearDogError> {
        let available: Vec<_> = self
            .providers
            .iter()
            .filter(|p| p.is_available())
            .cloned()
            .collect();

        if available.is_empty() {
            return Err(BearDogError::system(
                "No HSM providers are available".to_string(),
            ));
        }

        match preference {
            SelectionPreference::PreferHardware => {
                if let Some(hw) = available
                    .iter()
                    .find(|p| p.provider_type() != HsmProviderType::Software)
                {
                    return Ok(Arc::clone(hw));
                }
                Ok(Arc::clone(&available[0]))
            }
            SelectionPreference::RequireHardware => available
                .iter()
                .find(|p| p.provider_type() != HsmProviderType::Software)
                .cloned()
                .ok_or_else(|| {
                    BearDogError::system("No hardware-backed HSM provider available".to_string())
                }),
            SelectionPreference::SoftwareOnly => available
                .iter()
                .find(|p| p.provider_type() == HsmProviderType::Software)
                .cloned()
                .ok_or_else(|| {
                    BearDogError::system("No software HSM provider available".to_string())
                }),
        }
    }

    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    /// Convenience: always returns the software fallback.
    pub fn software_fallback(
        &self,
    ) -> Result<Arc<crate::tunnel::hsm::HsmKeyProviderBackend>, BearDogError> {
        self.select(SelectionPreference::SoftwareOnly)
    }

    /// Number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Iterate over registered providers.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<crate::tunnel::hsm::HsmKeyProviderBackend>> {
        self.providers.iter()
    }
}

impl Default for HsmProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_provider(
        id: &str,
        provider_type: ProviderType,
        security_level: u8,
    ) -> ProviderInfo {
        ProviderInfo {
            id: id.to_string(),
            provider_type,
            security_level,
            available: true,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_registry_creation() -> Result<(), Box<dyn std::error::Error>> {
        let registry = UniversalProviderRegistry::new();
        assert_eq!(registry.stats().total_providers, 0);
        Ok(())
    }

    #[test]
    fn test_register_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();
        let provider = create_test_provider("software-1", ProviderType::Software, 1);

        let result = registry.register_provider(provider);
        assert!(result.is_ok());
        assert_eq!(registry.stats().total_providers, 1);
        assert_eq!(registry.stats().healthy_providers, 1);
        Ok(())
    }

    #[test]
    fn test_unregister_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();
        let provider = create_test_provider("software-1", ProviderType::Software, 1);

        registry.register_provider(provider)?;
        let removed = registry.unregister_provider("software-1");

        assert!(removed.is_some());
        assert_eq!(registry.stats().total_providers, 0);
        Ok(())
    }

    #[test]
    fn test_select_highest_security() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();

        registry.register_provider(create_test_provider("software", ProviderType::Software, 1))?;
        registry.register_provider(create_test_provider("android", ProviderType::Android, 2))?;
        registry.register_provider(create_test_provider("ios", ProviderType::Ios, 3))?;

        registry.set_strategy(SelectionStrategy::HighestSecurity);
        let selected = registry.select_provider();

        assert!(selected.is_some());
        let sel = selected.ok_or("provider not found")?;
        assert_eq!(sel.id, "ios");
        assert_eq!(sel.security_level, 3);
        Ok(())
    }

    #[test]
    fn test_select_best_performance() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();

        registry.register_provider(create_test_provider("ios", ProviderType::Ios, 3))?;
        registry.register_provider(create_test_provider("software", ProviderType::Software, 1))?;

        registry.set_strategy(SelectionStrategy::BestPerformance);
        let selected = registry.select_provider();

        assert!(selected.is_some());
        assert_eq!(selected.ok_or("provider not found")?.id, "software");
        Ok(())
    }

    #[test]
    fn test_list_providers() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();

        registry.register_provider(create_test_provider(
            "provider-1",
            ProviderType::Software,
            1,
        ))?;
        registry.register_provider(create_test_provider("provider-2", ProviderType::Android, 2))?;

        let providers = registry.list_providers();
        assert_eq!(providers.len(), 2);
        Ok(())
    }

    #[test]
    fn test_get_providers_by_type() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();

        registry.register_provider(create_test_provider("soft-1", ProviderType::Software, 1))?;
        registry.register_provider(create_test_provider("soft-2", ProviderType::Software, 1))?;
        registry.register_provider(create_test_provider("android-1", ProviderType::Android, 2))?;

        let software_providers = registry.get_providers_by_type(&ProviderType::Software);
        assert_eq!(software_providers.len(), 2);
        Ok(())
    }

    #[test]
    fn test_get_available_providers() -> Result<(), Box<dyn std::error::Error>> {
        let mut registry = UniversalProviderRegistry::new();

        let mut provider1 = create_test_provider("provider-1", ProviderType::Software, 1);
        provider1.available = false;

        registry.register_provider(provider1)?;
        registry.register_provider(create_test_provider("provider-2", ProviderType::Android, 2))?;

        let available = registry.get_available_providers();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].id, "provider-2");
        Ok(())
    }

    #[test]
    fn test_provider_types() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(ProviderType::Software, ProviderType::Software);
        assert_ne!(ProviderType::Software, ProviderType::Android);
        Ok(())
    }

    #[test]
    fn test_selection_strategies() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            SelectionStrategy::HighestSecurity,
            SelectionStrategy::HighestSecurity
        );
        assert_ne!(
            SelectionStrategy::HighestSecurity,
            SelectionStrategy::BestPerformance
        );
        Ok(())
    }

    // ── canonical HsmProviderRegistry tests ────────────────────────────

    use beardog_types::hsm::HsmProviderType as CanonicalType;

    #[test]
    fn canonical_registry_empty_select_fails() {
        let reg = HsmProviderRegistry::new();
        assert!(reg.is_empty());
        assert!(reg.select(SelectionPreference::PreferHardware).is_err());
    }

    #[test]
    fn canonical_registry_prefer_hardware_picks_hw() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubHardware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeHwProvider,
            ),
        ));

        let p = reg.select(SelectionPreference::PreferHardware).unwrap();
        assert_eq!(p.provider_type(), CanonicalType::AndroidStrongBox);
    }

    #[test]
    fn canonical_registry_prefer_hardware_falls_back_to_sw() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));

        let p = reg.select(SelectionPreference::PreferHardware).unwrap();
        assert_eq!(p.provider_type(), CanonicalType::Software);
    }

    #[test]
    fn canonical_registry_require_hardware_fails_without_hw() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));

        assert!(reg.select(SelectionPreference::RequireHardware).is_err());
    }

    #[test]
    fn canonical_registry_software_only() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubHardware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeHwProvider,
            ),
        ));

        let p = reg.select(SelectionPreference::SoftwareOnly).unwrap();
        assert_eq!(p.provider_type(), CanonicalType::Software);
    }

    #[test]
    fn canonical_registry_software_fallback() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubHardware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeHwProvider,
            ),
        ));

        let p = reg.software_fallback().unwrap();
        assert_eq!(p.provider_id(), "fake-sw");
    }

    #[test]
    fn canonical_registry_len_and_iter() {
        let mut reg = HsmProviderRegistry::new();
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubSoftware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeSwProvider,
            ),
        ));
        reg.register(Arc::new(
            crate::tunnel::hsm::HsmKeyProviderBackend::StubHardware(
                crate::tunnel::hsm::hsm_key_provider_backend::tests::FakeHwProvider,
            ),
        ));
        assert_eq!(reg.len(), 2);

        let ids: Vec<_> = reg.iter().map(|p| p.provider_id()).collect();
        assert!(ids.contains(&"fake-sw"));
        assert!(ids.contains(&"fake-hw"));
    }

    #[tokio::test]
    async fn canonical_registry_discover_creates_software() {
        let reg = HsmProviderRegistry::discover().await;
        assert!(!reg.is_empty());
        let sw = reg.software_fallback().unwrap();
        assert_eq!(sw.provider_type(), CanonicalType::Software);
    }
}
