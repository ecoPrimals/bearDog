// Mobile HSM Configuration
//
// Consolidates mobile HSM configurations for Android and iOS platforms.

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
// Removed unused import: use std::collections::HashMap;

/// **UNIFIED MOBILE HSM CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMobileHsmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Android
    pub android: AndroidHsmConfig,
    /// Ios
    /// The ios value
    pub ios: IosHsmConfig,
    /// Auto Detection
    /// Whether `auto_detection` is enabled
    pub auto_detection: bool,
    /// Fallback Enabled
    /// Whether fallback is enabled
    pub fallback_enabled: bool,
}

impl Default for UnifiedMobileHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            android: AndroidHsmConfig::default(),
            ios: IosHsmConfig::default(),
            auto_detection: true,
            fallback_enabled: true,
        }
    }
}

/// Android HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Strongbox Enabled
    /// Whether strongbox is enabled
    pub strongbox_enabled: bool,
    /// Keystore Config
    pub keystore_config: AndroidKeystoreConfig,
    /// Attestation Config
    pub attestation_config: AndroidAttestationConfig,
    /// Biometric Config
    pub biometric_config: AndroidBiometricConfig,
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strongbox_enabled: true,
            keystore_config: AndroidKeystoreConfig::default(),
            attestation_config: AndroidAttestationConfig::default(),
            biometric_config: AndroidBiometricConfig::default(),
        }
    }
}

/// Android Keystore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidKeystoreConfig {
    /// Provider Name
    pub provider_name: String,
    /// Key Protection
    /// The key protection value
    pub key_protection: KeyProtectionLevel,
    /// User Authentication Required
    /// Whether `user_authentication_required` is enabled
    pub user_authentication_required: bool,
    /// User Presence Required
    /// Whether `user_presence_required` is enabled
    pub user_presence_required: bool,
    /// Unlocked Device Required
    /// Whether `unlocked_device_required` is enabled
    pub unlocked_device_required: bool,
}

impl Default for AndroidKeystoreConfig {
    fn default() -> Self {
        Self {
            provider_name: "AndroidKeyStore".to_string(),
            key_protection: KeyProtectionLevel::StrongBox,
            user_authentication_required: true,
            user_presence_required: false,
            unlocked_device_required: true,
        }
    }
}

/// Key protection levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyProtectionLevel {
    /// Software variant
    Software,
    /// `TrustedExecutionEnvironment` variant
    TrustedExecutionEnvironment,
    /// `StrongBox` variant
    StrongBox,
}

/// Android attestation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidAttestationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Include Device Properties
    /// Whether `include_device_properties` is enabled
    pub include_device_properties: bool,
    /// Challenge Required
    /// Whether `challenge_required` is enabled
    pub challenge_required: bool,
    /// Verify Attestation Chain
    /// Whether `verify_attestation_chain` is enabled
    pub verify_attestation_chain: bool,
}

impl Default for AndroidAttestationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_device_properties: true,
            challenge_required: true,
            verify_attestation_chain: true,
        }
    }
}

/// Android biometric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidBiometricConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Allowed Authenticators
    /// Collection of allowed authenticators
    pub allowed_authenticators: Vec<BiometricAuthenticator>,
    /// Require Confirmation
    /// Whether `require_confirmation` is enabled
    pub require_confirmation: bool,
    /// Allow Device Credential
    /// Whether `allow_device_credential` is enabled
    pub allow_device_credential: bool,
}

impl Default for AndroidBiometricConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_authenticators: vec![
                BiometricAuthenticator::Fingerprint,
                BiometricAuthenticator::Face,
            ],
            require_confirmation: false,
            allow_device_credential: true,
        }
    }
}

/// Biometric authenticator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiometricAuthenticator {
    /// Fingerprint variant
    Fingerprint,
    /// Face variant
    Face,
    /// Iris variant
    Iris,
    /// Voice variant
    Voice,
    /// `DeviceCredential` variant
    DeviceCredential,
}

/// iOS HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosHsmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Secure Enclave Config
    pub secure_enclave_config: SecureEnclaveConfig,
    /// Keychain Config
    pub keychain_config: KeychainConfig,
    /// Biometric Config
    pub biometric_config: IosBiometricConfig,
}

impl Default for IosHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            secure_enclave_config: SecureEnclaveConfig::default(),
            keychain_config: KeychainConfig::default(),
            biometric_config: IosBiometricConfig::default(),
        }
    }
}

/// Secure Enclave configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureEnclaveConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Key Type
    /// The key type value
    pub key_type: SecureEnclaveKeyType,
    /// User Presence Required
    /// Whether `user_presence_required` is enabled
    pub user_presence_required: bool,
    /// Biometry Required
    /// Whether `biometry_required` is enabled
    pub biometry_required: bool,
    /// Passcode Fallback
    /// Whether `passcode_fallback` is enabled
    pub passcode_fallback: bool,
}

impl Default for SecureEnclaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            key_type: SecureEnclaveKeyType::EcdsaSecp256r1,
            user_presence_required: true,
            biometry_required: false,
            passcode_fallback: true,
        }
    }
}

/// Secure Enclave key types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of secure enclave key
pub enum SecureEnclaveKeyType {
    /// ECDSA with secp256r1 curve
    EcdsaSecp256r1,
    /// ECDSA with secp384r1 curve
    EcdsaSecp384r1,
    /// ECDSA with secp521r1 curve
    EcdsaSecp521r1,
}

/// Keychain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeychainConfig {
    /// Accessibility
    /// The accessibility value
    pub accessibility: KeychainAccessibility,
    /// Synchronizable
    /// Whether synchronizable is enabled
    pub synchronizable: bool,
    /// Use Authentication Ui
    /// Whether `use_authentication_ui` is enabled
    pub use_authentication_ui: bool,
    /// Authentication Prompt
    /// The authentication prompt value
    pub authentication_prompt: String,
}

impl Default for KeychainConfig {
    fn default() -> Self {
        Self {
            accessibility: KeychainAccessibility::WhenUnlockedThisDeviceOnly,
            synchronizable: false,
            use_authentication_ui: true,
            authentication_prompt: "Authenticate to access secure keys".to_string(),
        }
    }
}

/// Keychain accessibility levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeychainAccessibility {
    /// `WhenUnlocked` variant
    WhenUnlocked,
    /// `AfterFirstUnlock` variant
    AfterFirstUnlock,
    /// Always variant
    Always,
    /// `WhenPasscodeSetThisDeviceOnly` variant
    WhenPasscodeSetThisDeviceOnly,
    /// `WhenUnlockedThisDeviceOnly` variant
    WhenUnlockedThisDeviceOnly,
    /// `AfterFirstUnlockThisDeviceOnly` variant
    AfterFirstUnlockThisDeviceOnly,
    /// `AlwaysThisDeviceOnly` variant
    AlwaysThisDeviceOnly,
}

/// iOS biometric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosBiometricConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Touch Id Enabled
    pub touch_id_enabled: bool,
    /// Face Id Enabled
    pub face_id_enabled: bool,
    /// Fallback To Passcode
    /// Whether `fallback_to_passcode` is enabled
    pub fallback_to_passcode: bool,
    /// Authentication Context
    /// The authentication context value
    pub authentication_context: BiometricAuthContext,
}

impl Default for IosBiometricConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            touch_id_enabled: true,
            face_id_enabled: true,
            fallback_to_passcode: true,
            authentication_context: BiometricAuthContext::default(),
        }
    }
}

/// Biometric authentication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricAuthContext {
    /// Local Authentication Prompt
    /// The local authentication prompt value
    pub local_authentication_prompt: String,
    /// Cancel Title
    /// The cancel title value
    pub cancel_title: String,
    /// Fallback Title
    /// The fallback title value
    pub fallback_title: String,
    /// Max Retry Count
    /// Number of `max_retry`
    pub max_retry_count: u32,
}

impl Default for BiometricAuthContext {
    fn default() -> Self {
        Self {
            local_authentication_prompt: "Authenticate to access BearDog".to_string(),
            cancel_title: "Cancel".to_string(),
            fallback_title: "Use Passcode".to_string(),
            max_retry_count: 3,
        }
    }
}

impl HsmConfigValidation for UnifiedMobileHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && !self.android.enabled && !self.ios.enabled {
            return Err(BearDogError::business(
                "At least one mobile platform must be enabled when mobile HSM is enabled"
                    .to_string(),
            ));
        }

        // Validate Android configuration
        if self.android.enabled && self.android.keystore_config.provider_name.is_empty() {
            return Err(BearDogError::business(
                "Android keystore provider name cannot be empty".to_string(),
            ));
        }

        // Validate iOS configuration
        if self.ios.enabled && self.ios.keychain_config.authentication_prompt.is_empty() {
            return Err(BearDogError::business(
                "iOS keychain authentication prompt cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
