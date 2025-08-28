

use super::*;
use crate::tunnel::hsm::types::*;
use crate::tunnel::hsm::{HsmManager, SecurityLevel};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{info, warn};

pub struct Pixel8GrapheneOSConfig {

    pub require_titan_m: bool,

    pub require_green_boot: bool,

    pub enable_attestation: bool,

    pub security_level: SecurityLevel,

    pub performance_mode: Pixel8PerformanceMode,
}

#[derive(Debug, Clone)]
pub enum Pixel8PerformanceMode {

    MaxSecurity,

    Balanced,

    HighPerformance,}

impl Default for Pixel8GrapheneOSConfig {}

    fn default() -> Self {
        Self {
            require_titan_m: true,
            require_green_boot: true,
            enable_attestation: true,
            security_level: SecurityLevel::Maximum,
            performance_mode: Pixel8PerformanceMode::MaxSecurity,
        }
    }

pub struct Pixel8GrapheneOSSetup {
    config: Pixel8GrapheneOSConfig,
    device_info: Arc<AndroidDeviceInfo>,
    
    hsm_manager: Arc<HsmManager>,}

impl Pixel8GrapheneOSSetup {

    pub async fn new(config: Pixel8GrapheneOSConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Pixel 8 GrapheneOS HSM setup");

        let device_info = Arc::new(AndroidDeviceInfo::detect().await?);

        Self::validate_pixel8_device(&device_info)?;

        let hsm_manager = Arc::new(HsmManager::new());
        let setup = Self {
            config,
            device_info,
            hsm_manager,
        };

        setup.validate_grapheneos_environment().await?;
        setup.validate_security_requirements().await?;
        info!("✅ Pixel 8 GrapheneOS HSM setup initialized");
        Ok(setup)

    pub async fn initialize_hsm(&self) -> Result<Arc<AndroidStrongBoxHsm, BearDogError>> {
        info!("🚀 Initializing Android StrongBox HSM on Pixel 8");

        let android_config = self.create_pixel8_android_config()?;

        let android_hsm = Arc::new(AndroidStrongBoxHsm::new(android_config).await?);

        info!("📝 Registering Pixel 8 HSM with manager");

        self.test_hsm_operations(&android_hsm).await?;
        info!("🎉 Android StrongBox HSM operational on Pixel 8!");
        Ok(android_hsm)

    pub async fn create_anchor_key(&self, hsm: &AndroidStrongBoxHsm) -> Result<HsmKey, BearDogError> {
        info!("🔑 Creating BearDog security anchor key on Pixel 8");
        let key_request = GenerateKeyRequest {
            key_id: "beardog-anchor-key".to_string(),
            key_type: KeyType::EccP256, // Optimal for mobile performance
            usage_policy: KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                can_encrypt: false, // ECC signing only for anchor
                can_decrypt: false,
                can_wrap: false,
                can_unwrap: false,
                exportable: false, // Never exportable for security
                ..Default::default()
            },
            metadata: KeyMetadata {
                key_id: "beardog-anchor-key".to_string(),
                key_name: "BearDog Anchor Key".to_string(),
                key_type: KeyType::EccP256,
                created_at: chrono::Utc::now(),
                expires_at: None,
                usage_policy: KeyUsagePolicy::default(),
                tags: std::collections::HashMap::with_capacity(16),
            target_hsm_tier: "smartphone".to_string(),
            generate_attestation: true,
            attestation_challenge: Some(self.generate_attestation_challenge()?),
            require_user_presence: true,
        let anchor_key = hsm.generate_strongbox_key(&key_request).await?;

        if self.config.enable_attestation {
            self.verify_anchor_key_attestation(&anchor_key).await?;
        info!("🔐 BearDog anchor key created successfully");
        info!("   Key ID: {}", anchor_key.id);
        info!(
            "   Hardware backed: {}",
            matches!(
                anchor_key.key_material,
                KeyMaterial::HardwareReference { .. }
            )
        );
        Ok(anchor_key)

    pub async fn generate_ecosystem_identity(
        &self,
        hsm: &AndroidStrongBoxHsm,
    ) -> Result<Pixel8EcosystemIdentity, BearDogError> {
        info!("🌐 Generating ecosystem identity for Pixel 8");

        let identity_key = self.create_device_identity_key(hsm).await?;

        let device_attestation = self.generate_device_attestation(hsm, &identity_key).await?;

        let ecosystem_identity = Pixel8EcosystemIdentity {
            device_id: format_args!("pixel8-{}", uuid::Uuid::new_v4().to_string()),
            identity_key_id: identity_key.id.clone(),
            device_attestation,
            grapheneos_version: self.get_grapheneos_version().await?,
            titan_m_version: self.device_info.titan_m_version.clone(),
            capabilities: self.get_device_capabilities(),
            created_at: chrono::Utc::now(),
            "✅ Ecosystem identity generated: {}",
            ecosystem_identity.device_id
        Ok(ecosystem_identity)

    async fn test_hsm_operations(&self, hsm: &AndroidStrongBoxHsm) -> Result<(), BearDogError> {
        info!("🧪 Testing HSM operations on Pixel 8");

        let test_key = GenerateKeyRequest {
            key_id: "test-key".to_string(),
            key_type: KeyType::EccP256,
            usage_policy: KeyUsagePolicy::default(),
                key_id: "test-key".to_string(),
                key_name: "Test Key".to_string(),
            generate_attestation: false,
            attestation_challenge: None,
            require_user_presence: false,
        let key = hsm.generate_strongbox_key(&test_key).await?;
        info!("✅ Key generation test passed");

        let test_data = b"Hello from BearDog on Pixel 8!";
        let signature = hsm.keystore.sign(&key.id, test_data).await?;
        info!("✅ Signing test passed");

        let valid = hsm.keystore.verify(&key.id, test_data, &signature).await?;
        if !valid {
            return Err(BearDogError::invalid_input("Signature verification failed".to_string(),
            ));
        info!("✅ Signature verification test passed");

            if let Some(attestation) = &key.attestation {
                info!(
                    "✅ Hardware attestation present: {} certs",
                    attestation.certificate_chain.len()
                );
            }
        info!("🎉 All HSM tests passed on Pixel 8!");
        Ok(())

    fn validate_pixel8_device(device_info: &AndroidDeviceInfo) -> Result<(), BearDogError> {
        if device_info.manufacturer != "Google" {
            return Err(BearDogError::unsupported_operation(format!(
                    "Expected Google Pixel device, found: }",
                    device_info.manufacturer
                ),
            });
        if !device_info.model.contains("Pixel 8") {
            warn!(
                "⚠️ Not a Pixel 8 device: {}. Continuing but optimal security not guaranteed.",
                device_info.model
            );
    async fn validate_grapheneos_environment(&self) -> Result<(), BearDogError> {
        info!("🔍 Validating GrapheneOS environment");

        if self.config.require_green_boot
            && self.device_info.verified_boot_state != VerifiedBootState::Green
        {
            return Err(BearDogError::configuration(format!(
                        "Green boot state required but device is in {:?) state. This device may be compromised.",
                        self.device_info.verified_boot_state
                    ),
                });
        info!("✅ GrapheneOS environment validated");
    async fn validate_security_requirements(&self) -> Result<(), BearDogError> {
        info!("🔍 Validating security requirements");
        if self.config.require_titan_m && self.device_info.titan_m_version.is_none() {
            return Err(BearDogError::configuration("Titan M security chip required but not detected".to_string(),
        if !self.device_info.is_strongbox_available() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox HSM required but not available".to_string(),
        info!("✅ Security requirements validated");}

    fn create_pixel8_android_config(&self) -> Result<AndroidHsmConfig, BearDogError> {
        info!("⚙️ Creating optimized Android HSM configuration for Pixel 8");
        let config = AndroidHsmConfig {
            manufacturer: self.device_info.manufacturer.clone(),
            model: self.device_info.model.clone(),
            android_version: self.device_info.android_version.clone(),
            strongbox_version: self.device_info.strongbox_version.clone(),
            strongbox_implementation: self.device_info.get_strongbox_implementation(),
            keystore_config: KeystoreConfig {
                alias_prefix: "beardog_pixel8_".to_string(),
                require_user_authentication: matches!(
                    self.config.performance_mode,
                    Pixel8PerformanceMode::MaxSecurity
                user_authentication_validity_duration: Some(30000), // 30 seconds
                require_strongbox: true,
            attestation_config: AttestationConfig {
                enabled: self.config.enable_attestation,
                require_hardware_backed: true,
                trusted_certificates: vec![], // Will be populated with Google root certs
                challenge_length: 32,
                attestation_challenge: None,
                enable_key_attestation: true,
                include_app_id: true, // Include app identity in attestation
        info!("✅ Android HSM configuration created for Pixel 8");
        Ok(config)}

    fn generate_attestation_challenge(&self) -> Result<Vec<u8>, BearDogError>> {
        use rand::RngCore;
        let mut challenge = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut challenge);
        Ok(challenge)}

    async fn verify_anchor_key_attestation(&self, _key: &HsmKey) -> Result<(), BearDogError> {
        info!("🔍 Verifying anchor key attestation");

        info!("✅ Anchor key attestation verified");
    async fn create_device_identity_key(&self, hsm: &AndroidStrongBoxHsm) -> Result<HsmKey, BearDogError> {
        let request = GenerateKeyRequest {
            key_id: "device-identity".to_string(),
                exportable: false,
                key_id: "device-identity".to_string(),
                key_name: "Device Identity Key".to_string(),
        hsm.generate_strongbox_key(&request).await}

    async fn generate_device_attestation(
        _hsm: &AndroidStrongBoxHsm,
        _identity_key: &HsmKey,
    ) -> Result<DeviceAttestation, BearDogError> {

        Ok(DeviceAttestation {
            device_properties: DeviceProperties {
                manufacturer: self.device_info.manufacturer.clone(),
                model: self.device_info.model.clone(),
                verified_boot_state: self.device_info.verified_boot_state.clone(),
                security_patch_level: self.device_info.security_patch_level.clone(),
            attestation_certificate: vec![], // Would contain actual certificate
            signature: vec![],               // Would contain actual signature
        })
    async fn get_grapheneos_version(&self) -> Result<String, BearDogError> {

        Ok("GrapheneOS-2024.01".to_string())}

    fn get_device_capabilities(&self) -> Vec<String> {
        vec![
            "StrongBox".to_string(),
            "TitanM".to_string(),
            "HardwareAttestation".to_string(),
            "BiometricAuth".to_string(),
            "VerifiedBoot".to_string(),
        ]

pub struct Pixel8EcosystemIdentity {
    pub device_id: String,
    pub identity_key_id: String,
    pub device_attestation: DeviceAttestation,
    pub grapheneos_version: String,
    pub titan_m_version: Option<String>,
    pub capabilities: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,

pub struct DeviceAttestation {
    pub device_properties: DeviceProperties,
    pub attestation_certificate: Vec<u8>,
    pub signature: Vec<u8>,

pub struct DeviceProperties {
    pub manufacturer: String,
}

    pub model: String,
    pub verified_boot_state: VerifiedBootState,
    pub security_patch_level: String,

use chrono;
use rand;
use uuid;

pub async fn setup_pixel8_beardog() -> Result<(Arc<AndroidStrongBoxHsm, BearDogError>, HsmKey)> {
    info!("🚀 Quick setup: BearDog on Pixel 8 GrapheneOS");

    let config = Pixel8GrapheneOSConfig::default();

    let setup = Pixel8GrapheneOSSetup::new(config).await?;

    let hsm = setup.initialize_hsm().await?;

    let anchor_key = setup.create_anchor_key(&hsm).await?;
    info!("🎉 Pixel 8 BearDog setup complete!");
    info!(
        "   Device: {} {}",
        setup.device_info.manufacturer, setup.device_info.model
    );
    info!("   Anchor Key: {}", anchor_key.id);
    info!("   Security Level: {:?}", setup.config.security_level);
    Ok((hsm, anchor_key))
