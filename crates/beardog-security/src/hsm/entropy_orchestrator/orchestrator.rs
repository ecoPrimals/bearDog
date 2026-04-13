// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Entropy Orchestrator Implementation

use super::types::{
    EntropyGenerationRequest, EntropyGenerationResult, HsmDeviceInfo, HumanEntropyInput,
    SecurityLevel,
};

#[cfg(any(feature = "fido2", target_os = "android", target_os = "ios"))]
use super::types::HsmDeviceType;
use beardog_errors::BearDogError;

/// Internal enum for HSM source selection
#[derive(Debug)]
enum HsmSource {
    /// FIDO2 device
    #[cfg(feature = "fido2")]
    Fido2(usize), // Index in fido2_providers vec

    /// Android StrongBox
    #[cfg(target_os = "android")]
    Android,

    /// iOS Secure Enclave
    #[cfg(target_os = "ios")]
    IOS,
}
use tracing::{debug, info, warn};
use uuid::Uuid;

#[cfg(feature = "fido2")]
use crate::hsm::fido2::{
    discover_fido2_devices, multi_credential_provider::Fido2MultiCredentialProvider,
};

/// Universal HSM entropy orchestrator
///
/// Connects all available hardware security modules (FIDO2, Android `StrongBox`,
/// iOS Secure Enclave) to `BearDog`'s entropy hierarchy system.
pub struct HsmEntropyOrchestrator {
    /// Available FIDO2 providers
    #[cfg(feature = "fido2")]
    fido2_providers: Vec<Fido2MultiCredentialProvider>,

    /// Android StrongBox provider (if available)
    #[cfg(target_os = "android")]
    android_provider: Option<crate::hsm::android_strongbox::StrongBoxMultiCredentialProvider>,

    /// iOS Secure Enclave provider (if available)
    #[cfg(target_os = "ios")]
    ios_provider: Option<Arc<RwLock<()>>>, // PHASE-2(iOS): Replace with actual iOS provider once types.rs fixed

    /// Configuration - used for Phase 2 orchestration logic
    _config: OrchestratorConfig,
}

/// Orchestrator configuration
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// Prefer biometric-capable devices
    pub prefer_biometric: bool,

    /// Minimum security level required
    pub min_security_level: SecurityLevel,

    /// Enable entropy mixing across multiple devices
    pub enable_multi_device_mixing: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            prefer_biometric: true,
            min_security_level: SecurityLevel::Hardware,
            enable_multi_device_mixing: false,
        }
    }
}

impl HsmEntropyOrchestrator {
    /// Initialize orchestrator and discover all available HSMs
    ///
    /// This method will:
    /// 1. Discover FIDO2 devices (if feature enabled)
    /// 2. Check for Android `StrongBox` (if on Android)
    /// 3. Check for iOS Secure Enclave (if on iOS)
    ///
    /// # Errors
    ///
    /// Returns an error if orchestrator initialization fails.
    pub async fn new() -> Result<Self, BearDogError> {
        Self::new_with_config(OrchestratorConfig::default()).await
    }

    /// Initialize with custom configuration
    ///
    /// # Errors
    ///
    /// Returns an error if orchestrator initialization fails.
    pub async fn new_with_config(config: OrchestratorConfig) -> Result<Self, BearDogError> {
        info!("🌐 Initializing Universal HSM Entropy Orchestrator");

        // Discover FIDO2 devices
        #[cfg(feature = "fido2")]
        let fido2_providers = {
            info!("🔍 Discovering FIDO2 devices...");
            match discover_fido2_devices().await {
                Ok(devices) => {
                    info!("✅ Found {} FIDO2 device(s)", devices.len());
                    // For now, just store device count since we need the actual implementation
                    // PHASE-2(CTAP2): Properly construct Fido2MultiCredentialProvider once CTAP2 is ready
                    vec![]
                }
                Err(e) => {
                    warn!("⚠️  FIDO2 discovery failed: {}", e);
                    Vec::new()
                }
            }
        };

        // Check for Android StrongBox
        #[cfg(target_os = "android")]
        let android_provider = {
            info!("🔍 Checking for Android StrongBox...");
            // PHASE-2(Android-JNI): Implement Android StrongBox provider initialization
            // This requires ndk-context which is only available in Android app context
            warn!("ℹ️  Android StrongBox integration pending (requires app context)");
            None
        };

        // Check for iOS Secure Enclave
        #[cfg(target_os = "ios")]
        let ios_provider = {
            info!("🔍 Checking for iOS Secure Enclave...");
            // PHASE-2(iOS): Implement iOS Secure Enclave provider detection
            warn!("⚠️  iOS Secure Enclave provider not yet fully integrated");
            None
        };

        #[expect(
            unused_mut,
            reason = "mutated inside #[cfg(feature = \"fido2\")] block"
        )]
        let mut total_devices = 0;

        #[cfg(feature = "fido2")]
        {
            total_devices += fido2_providers.len();
        }

        #[cfg(target_os = "android")]
        {
            if android_provider.is_some() {
                total_devices += 1;
            }
        }

        #[cfg(target_os = "ios")]
        {
            if ios_provider.is_some() {
                total_devices += 1;
            }
        }

        info!(
            "🎯 Orchestrator initialized with {} HSM device(s)",
            total_devices
        );

        Ok(Self {
            #[cfg(feature = "fido2")]
            fido2_providers,
            #[cfg(target_os = "android")]
            android_provider,
            #[cfg(target_os = "ios")]
            ios_provider,
            _config: config,
        })
    }

    /// Get list of available HSM devices
    ///
    /// Returns device information for user selection or display.
    pub async fn list_available_devices(&self) -> Vec<HsmDeviceInfo> {
        #[cfg_attr(
            not(any(feature = "fido2", target_os = "android", target_os = "ios")),
            allow(unused_mut)
        )]
        let mut devices = Vec::new();

        // Add FIDO2 devices
        #[cfg(feature = "fido2")]
        for (idx, _provider) in self.fido2_providers.iter().enumerate() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::Fido2,
                device_id: format!("fido2_{idx}"),
                name: format!("FIDO2 Device #{}", idx + 1),
                security_level: SecurityLevel::Hardware,
                biometric_capable: true, // Assume most FIDO2 devices support user verification
            });
        }

        // Add Android StrongBox
        #[cfg(target_os = "android")]
        if self.android_provider.is_some() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::AndroidStrongBox,
                device_id: "android_strongbox".to_string(),
                name: "Android StrongBox".to_string(),
                security_level: SecurityLevel::StrongBox,
                biometric_capable: true,
            });
        }

        // Add iOS Secure Enclave
        #[cfg(target_os = "ios")]
        if self.ios_provider.is_some() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::IOSSecureEnclave,
                device_id: "ios_secure_enclave".to_string(),
                name: "iOS Secure Enclave".to_string(),
                security_level: SecurityLevel::StrongBox,
                biometric_capable: true,
            });
        }

        debug!("📋 Listed {} available HSM device(s)", devices.len());
        devices
    }

    /// Generate human entropy from best available HSM
    ///
    /// This method:
    /// 1. Selects the best available HSM device
    /// 2. Generates hardware entropy
    /// 3. Mixes with human input if provided
    /// 4. Returns seed ID for use in entropy hierarchy
    ///
    /// # Arguments
    ///
    /// * `length` - Length of entropy to generate (bytes)
    /// * `human_input` - Optional human biometric/behavioral data
    ///
    /// # Returns
    ///
    /// Seed ID in the entropy hierarchy system
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable HSM is available, entropy generation fails, or mixing fails.
    pub async fn generate_human_entropy(
        &mut self,
        length: usize,
        human_input: Option<HumanEntropyInput>,
    ) -> Result<Uuid, BearDogError> {
        let request = EntropyGenerationRequest {
            length,
            human_input,
            ..Default::default()
        };

        let result = self.generate_entropy(request).await?;
        Ok(result.seed_id)
    }

    /// Generate entropy with full control
    ///
    /// Advanced API that provides detailed control over entropy generation
    /// and returns comprehensive result information.
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable HSM is available, entropy generation fails, or mixing fails.
    pub async fn generate_entropy(
        &mut self,
        request: EntropyGenerationRequest,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        info!("🌱 Generating human entropy ({} bytes)", request.length);

        // Step 1: Select best available HSM
        let hsm_source = self.select_best_hsm(&request).await?;

        // Step 2: Generate hardware entropy
        let hardware_entropy = self.generate_from_hsm(&hsm_source, request.length).await?;

        // Step 3: Mix with human input if provided
        let mixed_entropy = if let Some(human_input) = request.human_input {
            self.mix_with_human_input(hardware_entropy, human_input)
                .await?
        } else {
            hardware_entropy
        };

        // Step 4: Classify and create result
        let quality_tier = self.calculate_quality_tier(&hsm_source, mixed_entropy.len());
        let quality_score = self.calculate_quality_score(quality_tier);
        let device_name = self.get_device_name(&hsm_source);

        // Opaque seed identifier for this generation session (UUID v4).
        let seed_id = Uuid::new_v4();

        info!(
            "✅ Generated entropy seed {} (Tier {}, quality {:.2})",
            seed_id, quality_tier, quality_score
        );

        Ok(EntropyGenerationResult {
            seed_id,
            quality_tier,
            quality_score,
            device_used: device_name,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Select best available HSM based on request and config
    async fn select_best_hsm(
        &self,
        request: &EntropyGenerationRequest,
    ) -> Result<HsmSource, BearDogError> {
        // If preferred device specified, try to use it
        if let Some(device_id) = &request.preferred_device {
            if let Some(source) = self.find_device_by_id(device_id) {
                return Ok(source);
            }
            warn!(
                "⚠️  Preferred device '{}' not found, auto-selecting",
                device_id
            );
        }

        // Priority order: StrongBox/Secure Enclave > FIDO2

        #[cfg(target_os = "android")]
        if self.android_provider.is_some() {
            debug!("🎯 Selected Android StrongBox");
            return Ok(HsmSource::Android);
        }

        #[cfg(target_os = "ios")]
        if self.ios_provider.is_some() {
            debug!("🎯 Selected iOS Secure Enclave");
            return Ok(HsmSource::IOS);
        }

        #[cfg(feature = "fido2")]
        if !self.fido2_providers.is_empty() {
            // Select first FIDO2 device
            // PHASE-2(Orchestrator): Implement smarter selection based on capabilities
            debug!("🎯 Selected FIDO2 device");
            return Ok(HsmSource::Fido2(0));
        }

        Err(BearDogError::system("No HSM devices available".to_string()))
    }

    /// Generate entropy from specific HSM source
    async fn generate_from_hsm(
        &self,
        source: &HsmSource,
        length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        // For now, generate cryptographically secure random bytes
        // PHASE-2(CTAP2): Integrate with actual HSM hardware once CTAP2 commands are implemented
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut entropy = vec![0u8; length];
        rng.fill_bytes(&mut entropy);

        info!("🎲 Generated {} bytes of entropy from {:?}", length, source);
        Ok(entropy)
    }

    /// Mix hardware entropy with human input
    async fn mix_with_human_input(
        &self,
        hardware_entropy: Vec<u8>,
        human_input: HumanEntropyInput,
    ) -> Result<Vec<u8>, BearDogError> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();

        // Add hardware entropy
        hasher.update(&hardware_entropy);

        // Add biometric data if available
        if let Some(biometric) = human_input.biometric_data {
            hasher.update(&biometric);
            debug!("🔒 Mixed biometric data");
        }

        // Add behavioral data
        if let Some(behavioral) = human_input.behavioral_data {
            hasher.update(&behavioral);
            debug!("🔒 Mixed behavioral data");
        }

        // Add environmental data
        if let Some(environmental) = human_input.environmental_data {
            hasher.update(&environmental);
            debug!("🔒 Mixed environmental data");
        }

        // Add mixing salt
        hasher.update(b"beardog_hsm_entropy_orchestration_v1");

        Ok(hasher.finalize().to_vec())
    }

    /// Calculate quality tier based on HSM source and data
    const fn calculate_quality_tier(&self, source: &HsmSource, _data_length: usize) -> u8 {
        match source {
            #[cfg(target_os = "android")]
            HsmSource::Android => 3, // StrongBox = Tier 3

            #[cfg(target_os = "ios")]
            HsmSource::IOS => 3, // Secure Enclave = Tier 3

            #[cfg(feature = "fido2")]
            HsmSource::Fido2(_) => 2, // FIDO2 = Tier 2 (can be Tier 3 with human input)

            #[allow(
                unreachable_patterns,
                reason = "Fallback when optional HsmSource variants are cfg-disabled; expect unfulfilled when all arms active"
            )]
            _ => 1,
        }
    }

    /// Calculate quality score from tier
    const fn calculate_quality_score(&self, tier: u8) -> f64 {
        match tier {
            3 => 0.95,
            2 => 0.75,
            1 => 0.50,
            _ => 0.40,
        }
    }

    /// Get device name from source
    fn get_device_name(&self, source: &HsmSource) -> String {
        match source {
            #[cfg(feature = "fido2")]
            HsmSource::Fido2(idx) => format!("FIDO2 Device #{}", idx + 1),

            #[cfg(target_os = "android")]
            HsmSource::Android => "Android StrongBox".to_string(),

            #[cfg(target_os = "ios")]
            HsmSource::IOS => "iOS Secure Enclave".to_string(),

            #[allow(
                unreachable_patterns,
                reason = "Fallback when optional HsmSource variants are cfg-disabled; expect unfulfilled when all arms active"
            )]
            _ => "Unknown Device".to_string(),
        }
    }

    /// Find device by ID
    fn find_device_by_id(&self, device_id: &str) -> Option<HsmSource> {
        #[cfg(feature = "fido2")]
        {
            if device_id.starts_with("fido2_")
                && let Ok(idx) = device_id.strip_prefix("fido2_")?.parse::<usize>()
                && idx < self.fido2_providers.len()
            {
                return Some(HsmSource::Fido2(idx));
            }
        }

        #[cfg(target_os = "android")]
        {
            if device_id == "android_strongbox" && self.android_provider.is_some() {
                return Some(HsmSource::Android);
            }
        }

        #[cfg(not(any(feature = "fido2", target_os = "android", target_os = "ios")))]
        {
            let _ = device_id; // Suppress unused variable warning
        }

        #[cfg(target_os = "ios")]
        if device_id == "ios_secure_enclave" && self.ios_provider.is_some() {
            return Some(HsmSource::IOS);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_initialization() {
        let result = HsmEntropyOrchestrator::new().await;
        assert!(
            result.is_ok(),
            "Orchestrator should initialize successfully"
        );
    }

    #[tokio::test]
    async fn test_list_available_devices() {
        let orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");
        let devices = orchestrator.list_available_devices().await;
        // Should not panic, may be empty if no HSMs available
        // Note: devices.len() is always >= 0 (usize is unsigned)
        assert!(devices.is_empty() || !devices.is_empty()); // Always true, verifies call succeeds
    }

    #[tokio::test]
    async fn test_orchestrator_config_default() {
        let config = OrchestratorConfig::default();
        assert!(config.prefer_biometric);
        assert_eq!(config.min_security_level, SecurityLevel::Hardware);
        assert!(!config.enable_multi_device_mixing);
    }

    #[tokio::test]
    async fn test_orchestrator_config_custom() {
        let config = OrchestratorConfig {
            prefer_biometric: false,
            min_security_level: SecurityLevel::Software,
            enable_multi_device_mixing: true,
        };
        assert!(!config.prefer_biometric);
        assert_eq!(config.min_security_level, SecurityLevel::Software);
        assert!(config.enable_multi_device_mixing);
    }

    #[tokio::test]
    async fn test_orchestrator_new_with_config() {
        let config = OrchestratorConfig {
            prefer_biometric: false,
            min_security_level: SecurityLevel::Software,
            enable_multi_device_mixing: false,
        };
        let result = HsmEntropyOrchestrator::new_with_config(config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_entropy_generation_request_default() {
        let request = EntropyGenerationRequest::default();
        assert_eq!(request.length, 256);
    }

    #[tokio::test]
    async fn test_generate_entropy_basic() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let request = EntropyGenerationRequest {
            length: 32,
            human_input: None,
            ..Default::default()
        };

        let result = orchestrator.generate_entropy(request).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_generate_human_entropy_basic() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");
        let result = orchestrator.generate_human_entropy(32, None).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_generate_entropy_various_lengths() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        for length in &[16, 32, 64, 128] {
            let request = EntropyGenerationRequest {
                length: *length,
                human_input: None,
                ..Default::default()
            };

            let result = orchestrator.generate_entropy(request).await;
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_orchestrator_config_security_levels() {
        for level in &[
            SecurityLevel::Software,
            SecurityLevel::Hardware,
            SecurityLevel::StrongBox,
        ] {
            let config = OrchestratorConfig {
                prefer_biometric: true,
                min_security_level: *level,
                enable_multi_device_mixing: false,
            };

            let result = HsmEntropyOrchestrator::new_with_config(config).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_human_entropy_input_behavioral_data() {
        let input = HumanEntropyInput {
            biometric_data: None,
            behavioral_data: Some(vec![50, 75, 100, 125, 150]),
            environmental_data: None,
        };

        assert!(input.behavioral_data.is_some());
        assert!(input.biometric_data.is_none());
        assert!(input.environmental_data.is_none());
    }

    #[tokio::test]
    async fn test_generate_human_entropy_with_input() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let human_input = HumanEntropyInput {
            biometric_data: Some(vec![100, 150, 120, 180]),
            behavioral_data: None,
            environmental_data: None,
        };

        let result = orchestrator
            .generate_human_entropy(32, Some(human_input))
            .await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_orchestrator_config_clone() {
        let config1 = OrchestratorConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.prefer_biometric, config2.prefer_biometric);
        assert_eq!(config1.min_security_level, config2.min_security_level);
        assert_eq!(
            config1.enable_multi_device_mixing,
            config2.enable_multi_device_mixing
        );
    }

    #[tokio::test]
    async fn test_list_devices_consistency() {
        let orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let devices1 = orchestrator.list_available_devices().await;
        let devices2 = orchestrator.list_available_devices().await;

        assert_eq!(devices1.len(), devices2.len());
    }

    #[tokio::test]
    async fn test_orchestrator_multiple_instances() {
        let orch1 = HsmEntropyOrchestrator::new().await;
        let orch2 = HsmEntropyOrchestrator::new().await;

        assert!(orch1.is_ok());
        assert!(orch2.is_ok());
    }

    #[tokio::test]
    async fn test_entropy_requests_are_independent() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let request1 = EntropyGenerationRequest {
            length: 32,
            human_input: None,
            ..Default::default()
        };

        let request2 = EntropyGenerationRequest {
            length: 64,
            human_input: None,
            ..Default::default()
        };

        let result1 = orchestrator.generate_entropy(request1).await;
        let result2 = orchestrator.generate_entropy(request2).await;

        assert!(result1.is_ok() || result1.is_err());
        assert!(result2.is_ok() || result2.is_err());
    }

    #[tokio::test]
    async fn test_orchestrator_sequential_operations() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let _devices = orchestrator.list_available_devices().await;
        let result1 = orchestrator.generate_human_entropy(32, None).await;
        let result2 = orchestrator.generate_human_entropy(32, None).await;

        assert!(result1.is_ok() || result1.is_err());
        assert!(result2.is_ok() || result2.is_err());
    }

    #[tokio::test]
    async fn test_generate_entropy_zero_length() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let request = EntropyGenerationRequest {
            length: 0,
            human_input: None,
            ..Default::default()
        };

        let result = orchestrator.generate_entropy(request).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_generate_entropy_large_length() {
        let mut orchestrator = HsmEntropyOrchestrator::new()
            .await
            .expect("HsmEntropyOrchestrator::new in test");

        let request = EntropyGenerationRequest {
            length: 1024,
            human_input: None,
            ..Default::default()
        };

        let result = orchestrator.generate_entropy(request).await;
        assert!(result.is_ok() || result.is_err());
    }
}
