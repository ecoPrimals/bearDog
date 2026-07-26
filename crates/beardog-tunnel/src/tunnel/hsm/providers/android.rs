// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android HSM Provider
//!
//! Universal provider implementation for Android HSM capabilities,
//! including `StrongBox` and TEE support.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{debug, info};

/// Android Universal HSM Provider
pub struct AndroidUniversalProvider {
    /// HSM capabilities
    capabilities: Option<AndroidCapabilities>,
    /// `StrongBox` availability
    strongbox_available: bool,
    /// Trusted Execution Environment availability
    tee_available: bool,
    /// Device metadata
    device_metadata: HashMap<String, String>,
}

/// Android-specific HSM capabilities
#[derive(Debug, Clone)]
pub struct AndroidCapabilities {
    /// `StrongBox` support level
    pub strongbox_level: StrongBoxLevel,
    /// TEE type
    pub tee_type: Option<String>,
    /// Key attestation support
    pub attestation_supported: bool,
    /// Hardware-backed keystore
    pub hardware_backed: bool,
    /// Biometric authentication
    pub biometric_auth: bool,
}

/// `StrongBox` security levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrongBoxLevel {
    /// No `StrongBox` support
    None,
    /// Basic `StrongBox`
    Basic,
    /// `StrongBox` with attestation
    WithAttestation,
    /// Full `StrongBox` implementation
    Full,
}

impl AndroidUniversalProvider {
    /// Create a new Android HSM provider
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        let mut provider = Self {
            capabilities: None,
            strongbox_available: false,
            tee_available: false,
            device_metadata: HashMap::with_capacity(16),
        };

        // Detect platform capabilities
        provider.detect_strongbox();
        provider.detect_tee();

        let capabilities = provider.discover_capabilities()?;
        provider.capabilities = Some(capabilities);

        Ok(provider)
    }

    /// Check if running on Android platform
    const fn is_android_platform() -> bool {
        cfg!(target_os = "android")
    }

    /// Detect `StrongBox` availability
    fn detect_strongbox(&mut self) -> bool {
        if !Self::is_android_platform() {
            debug!("Not on Android platform, StrongBox not available");
            return false;
        }

        info!("Detecting StrongBox availability on Android device");

        let has_strongbox = self.simulate_strongbox_detection();
        if has_strongbox {
            info!("✅ StrongBox detected and available");
            self.device_metadata
                .insert("strongbox_version".to_string(), "1.0".to_string());
        } else {
            info!("❌ StrongBox not available on this device");
        }

        self.strongbox_available = has_strongbox;
        has_strongbox
    }

    /// Detect TEE availability.
    ///
    /// On non-Android platforms, TEE is reported as unavailable (fail closed).
    /// On Android, defers to `BEARDOG_ANDROID_TEE_AVAILABLE` env var until
    /// JNI bridge can query the Keymaster HAL.
    fn detect_tee(&mut self) -> bool {
        info!("Detecting TEE availability");

        let has_tee = if cfg!(target_os = "android") {
            beardog_errors::process_env::var(env_keys::ENV_ANDROID_TEE_AVAILABLE)
                .map(|v| v == "true")
                .unwrap_or(false)
        } else {
            false
        };

        if has_tee {
            info!("✅ TEE detected and available");
            self.device_metadata
                .insert("tee_version".to_string(), "trusty".to_string());
        } else {
            info!("TEE not available on this platform");
        }

        self.tee_available = has_tee;
        has_tee
    }

    /// Simulate `StrongBox` detection based on device model
    fn simulate_strongbox_detection(&mut self) -> bool {
        if let Ok(model) = beardog_errors::process_env::var(env_keys::ENV_ANDROID_MODEL) {
            self.device_metadata
                .insert("device_model".to_string(), model.clone());

            // Known StrongBox-capable devices (Pixel 8a = Titan M2 StrongBox on GrapheneOS)
            if model.contains("Pixel 8a")
                || model.contains("Pixel 8")
                || model.contains("Pixel")
                || model.contains("Galaxy S")
                || model.contains("Galaxy Note")
            {
                return true;
            }
        }

        // Default: assume basic TEE support
        false
    }

    /// Discover Android HSM capabilities
    fn discover_capabilities(&self) -> Result<AndroidCapabilities, BearDogError> {
        let strongbox_level = if self.strongbox_available {
            StrongBoxLevel::Full
        } else if self.tee_available {
            StrongBoxLevel::Basic
        } else {
            StrongBoxLevel::None
        };

        Ok(AndroidCapabilities {
            strongbox_level,
            tee_type: self.device_metadata.get("tee_version").cloned(),
            attestation_supported: self.strongbox_available,
            hardware_backed: self.strongbox_available || self.tee_available,
            biometric_auth: true, // Most modern Android devices support biometrics
        })
    }
    /// Get security level
    #[must_use]
    pub const fn get_security_level(&self) -> u8 {
        if self.strongbox_available {
            3 // Highest: StrongBox
        } else if self.tee_available {
            2 // Medium: TEE
        } else {
            1 // Basic: Software
        }
    }
    /// Get vendor information
    #[must_use]
    pub fn get_vendor_info(&self) -> VendorInfo {
        VendorInfo {
            name: "Android".to_string(),
            model: self
                .device_metadata
                .get("device_model")
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            version: self
                .device_metadata
                .get("android_version")
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
        }
    }
    /// Check if `StrongBox` is available
    #[must_use]
    pub const fn has_strongbox(&self) -> bool {
        self.strongbox_available
    }
    /// Check if TEE is available
    #[must_use]
    pub const fn has_tee(&self) -> bool {
        self.tee_available
    }
    /// Get capabilities
    #[must_use]
    pub const fn capabilities(&self) -> Option<&AndroidCapabilities> {
        self.capabilities.as_ref()
    }
}

/// Vendor information
#[derive(Debug, Clone)]
pub struct VendorInfo {
    /// Vendor name
    pub name: String,
    /// Device model
    pub model: String,
    /// Software version
    pub version: String,
}

#[cfg(test)]
#[path = "android_tests.rs"]
mod tests;
