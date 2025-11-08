//! HSM configuration

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// HSM configuration for hardware and software providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Auto-detect available HSMs
    #[serde(default = "default_true")]
    pub auto_detect: bool,

    /// Prefer hardware over software
    #[serde(default = "default_true")]
    pub prefer_hardware: bool,

    /// Enable SoftHSM2 provider
    #[serde(default = "default_true")]
    pub enable_softhsm: bool,

    /// Enable YubiHSM provider
    #[serde(default)]
    pub enable_yubihsm: bool,

    /// Enable TPM provider
    #[serde(default)]
    pub enable_tpm: bool,

    /// Enable Android StrongBox (if on Android)
    #[serde(default)]
    pub enable_strongbox: bool,

    /// SoftHSM2 configuration file path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softhsm_config: Option<PathBuf>,

    /// YubiHSM connector URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yubihsm_connector: Option<String>,

    /// Preferred provider order
    #[serde(default = "default_provider_order")]
    pub provider_order: Vec<String>,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            prefer_hardware: true,
            enable_softhsm: true,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            softhsm_config: None,
            yubihsm_connector: None,
            provider_order: default_provider_order(),
        }
    }
}

impl HsmConfig {
    /// Validate HSM configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Check that at least one provider is enabled
        if !self.auto_detect
            && !self.enable_softhsm
            && !self.enable_yubihsm
            && !self.enable_tpm
            && !self.enable_strongbox
        {
            return Err(ConfigError::validation(
                "At least one HSM provider must be enabled or auto_detect must be true",
            ));
        }

        // Validate SoftHSM config path if specified
        if let Some(ref config) = self.softhsm_config {
            if !config.exists() {
                return Err(ConfigError::PathNotFound(format!(
                    "SoftHSM2 config not found: {}",
                    config.display()
                )));
            }
        }

        // Validate YubiHSM connector URL if specified
        if let Some(ref url) = self.yubihsm_connector {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(ConfigError::invalid_value(
                    "hsm.yubihsm_connector",
                    "Must be a valid HTTP(S) URL",
                ));
            }
        }

        Ok(())
    }

    /// Get enabled providers in order of preference
    pub fn get_enabled_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();

        for provider in &self.provider_order {
            match provider.as_str() {
                "softhsm" if self.enable_softhsm => providers.push(provider.clone()),
                "yubihsm" if self.enable_yubihsm => providers.push(provider.clone()),
                "tpm" if self.enable_tpm => providers.push(provider.clone()),
                "strongbox" if self.enable_strongbox => providers.push(provider.clone()),
                _ => {}
            }
        }

        providers
    }
}

fn default_true() -> bool {
    true
}

fn default_provider_order() -> Vec<String> {
    vec![
        "strongbox".to_string(),
        "tpm".to_string(),
        "yubihsm".to_string(),
        "softhsm".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hsm_config() {
        let config = HsmConfig::default();
        assert!(config.validate().is_ok());
        assert!(config.auto_detect);
        assert!(config.prefer_hardware);
    }

    #[test]
    fn test_no_providers_enabled() {
        let mut config = HsmConfig::default();
        config.auto_detect = false;
        config.enable_softhsm = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_enabled_providers() {
        let mut config = HsmConfig::default();
        config.enable_softhsm = true;
        config.enable_yubihsm = true;
        config.enable_tpm = false;
        config.enable_strongbox = false;

        let providers = config.get_enabled_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"softhsm".to_string()));
        assert!(providers.contains(&"yubihsm".to_string()));
    }
}

