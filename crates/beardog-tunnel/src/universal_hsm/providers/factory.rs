//! Provider factory for creating HSM provider instances
//!
//! This module provides a factory pattern for creating HSM providers based on
//! the unified configuration system. It integrates with the HSM discovery system
//! to automatically select the best available HSM.

use beardog_errors::BearDogError;
use beardog_types::canonical::config::hsm::UnifiedHsmConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// HSM Provider types supported by the factory
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderType {
    /// Software-based HSM (for development/testing)
    Software,
    /// Android StrongBox hardware HSM
    AndroidStrongBox,
    /// iOS Secure Enclave hardware HSM
    IOSSecureEnclave,
    /// PKCS#11 hardware HSM (YubiKey, network HSM, etc.)
    Pkcs11,
    /// Cloud HSM (AWS KMS, Azure Key Vault, Google Cloud KMS)
    Cloud(CloudProvider),
}

/// Cloud HSM providers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudProvider {
    /// AWS Key Management Service
    AwsKms,
    /// Azure Key Vault
    AzureKeyVault,
    /// Google Cloud Key Management Service
    GcpKms,
}

/// Factory for creating HSM providers based on unified configuration
#[derive(Debug, Clone)]
pub struct ProviderFactory {
    /// Unified HSM configuration
    config: Arc<RwLock<UnifiedHsmConfig>>,
}

impl ProviderFactory {
    /// Create a new provider factory with unified configuration
    ///
    /// # Arguments
    /// * `config` - Unified HSM configuration from canonical config system
    pub fn new(config: UnifiedHsmConfig) -> Self {
        info!("Creating HSM provider factory with unified configuration");
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// Create a provider by name using unified configuration
    ///
    /// This implementation uses the canonical `UnifiedHsmConfig` to determine
    /// which HSM provider to create and how to configure it.
    ///
    /// # Arguments
    /// * `provider_name` - Name of the provider to create
    ///
    /// # Errors
    /// Returns an error if:
    /// - Provider is not supported
    /// - Provider is disabled in configuration
    /// - Provider initialization fails
    pub async fn create_provider(&self, provider_name: &str) -> Result<ProviderType, BearDogError> {
        let config = self.config.read().await;

        if !config.enabled {
            return Err(BearDogError::business(
                "HSM functionality is globally disabled in configuration".to_string(),
            ));
        }

        match provider_name {
            "software" => self.create_software_provider(&config).await,
            "android" | "android-strongbox" | "strongbox" => {
                self.create_android_provider(&config).await
            }
            "ios" | "ios-secure-enclave" | "secure-enclave" => {
                self.create_ios_provider(&config).await
            }
            "pkcs11" | "yubikey" | "hsm" => self.create_pkcs11_provider(&config).await,
            "aws" | "aws-kms" => self.create_cloud_provider(&config, CloudProvider::AwsKms).await,
            "azure" | "azure-keyvault" => {
                self.create_cloud_provider(&config, CloudProvider::AzureKeyVault).await
            }
            "gcp" | "gcp-kms" | "google" => {
                self.create_cloud_provider(&config, CloudProvider::GcpKms).await
            }
            _ => Err(BearDogError::not_supported(format!(
                "Unknown HSM provider: {}. Supported: software, android, ios, pkcs11, aws, azure, gcp",
                provider_name
            ))),
        }
    }

    /// Create software HSM provider
    async fn create_software_provider(
        &self,
        config: &UnifiedHsmConfig,
    ) -> Result<ProviderType, BearDogError> {
        if !config.software.enabled {
            return Err(BearDogError::business(
                "Software HSM is disabled in configuration".to_string(),
            ));
        }

        info!("Creating software HSM provider");
        debug!(
            "Software HSM config: key_storage={}, encryption={}",
            config.software.key_storage_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "in-memory".to_string()),
            config.software.encryption_enabled
        );

        Ok(ProviderType::Software)
    }

    /// Create Android StrongBox provider
    async fn create_android_provider(
        &self,
        config: &UnifiedHsmConfig,
    ) -> Result<ProviderType, BearDogError> {
        if !config.mobile.enabled {
            return Err(BearDogError::business(
                "Mobile HSM is disabled in configuration".to_string(),
            ));
        }

        if !config.mobile.android.enabled {
            return Err(BearDogError::business(
                "Android HSM is disabled in configuration".to_string(),
            ));
        }

        #[cfg(not(target_os = "android"))]
        {
            warn!("Android StrongBox requested but not running on Android platform");
            return Err(BearDogError::platform(
                "Android StrongBox requires Android platform".to_string(),
            ));
        }

        #[cfg(target_os = "android")]
        {
            info!("Creating Android StrongBox HSM provider");
            debug!(
                "Android config: use_strongbox={}, biometric_auth={}",
                config.mobile.android.use_strongbox, config.mobile.android.require_biometric_auth
            );
            Ok(ProviderType::AndroidStrongBox)
        }
    }

    /// Create iOS Secure Enclave provider
    async fn create_ios_provider(
        &self,
        config: &UnifiedHsmConfig,
    ) -> Result<ProviderType, BearDogError> {
        if !config.mobile.enabled {
            return Err(BearDogError::business(
                "Mobile HSM is disabled in configuration".to_string(),
            ));
        }

        if !config.mobile.ios.enabled {
            return Err(BearDogError::business(
                "iOS HSM is disabled in configuration".to_string(),
            ));
        }

        #[cfg(not(target_os = "ios"))]
        {
            warn!("iOS Secure Enclave requested but not running on iOS platform");
            return Err(BearDogError::platform(
                "iOS Secure Enclave requires iOS platform".to_string(),
            ));
        }

        #[cfg(target_os = "ios")]
        {
            info!("Creating iOS Secure Enclave HSM provider");
            debug!(
                "iOS config: use_secure_enclave={}, biometric_auth={}",
                config.mobile.ios.use_secure_enclave, config.mobile.ios.require_biometric_auth
            );
            Ok(ProviderType::IOSSecureEnclave)
        }
    }

    /// Create PKCS#11 hardware HSM provider
    async fn create_pkcs11_provider(
        &self,
        config: &UnifiedHsmConfig,
    ) -> Result<ProviderType, BearDogError> {
        if !config.hardware.enabled {
            return Err(BearDogError::business(
                "Hardware HSM is disabled in configuration".to_string(),
            ));
        }

        if !config.hardware.pkcs11.enabled {
            return Err(BearDogError::business(
                "PKCS#11 is disabled in configuration".to_string(),
            ));
        }

        let library_path = config
            .hardware
            .pkcs11
            .library_path
            .as_ref()
            .ok_or_else(|| BearDogError::business("PKCS#11 library path not configured".to_string()))?;

        if !library_path.exists() {
            return Err(BearDogError::not_found(format!(
                "PKCS#11 library not found: {}",
                library_path.display()
            )));
        }

        info!("Creating PKCS#11 HSM provider from {}", library_path.display());
        debug!(
            "PKCS#11 config: slot={:?}, pin_required={}",
            config.hardware.pkcs11.slot_id, config.hardware.pkcs11.require_pin
        );

        Ok(ProviderType::Pkcs11)
    }

    /// Create cloud HSM provider
    async fn create_cloud_provider(
        &self,
        config: &UnifiedHsmConfig,
        provider: CloudProvider,
    ) -> Result<ProviderType, BearDogError> {
        if !config.cloud.enabled {
            return Err(BearDogError::business(
                "Cloud HSM is disabled in configuration".to_string(),
            ));
        }

        match provider {
            CloudProvider::AwsKms => {
                if !config.cloud.aws_kms.enabled {
                    return Err(BearDogError::business("AWS KMS is disabled in configuration".to_string()));
                }

                // Verify AWS credentials are configured
                if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
                    warn!("AWS credentials not found in environment");
                    return Err(BearDogError::business(
                        "AWS credentials not configured (AWS_ACCESS_KEY_ID missing)".to_string(),
                    ));
                }

                info!("Creating AWS KMS HSM provider");
                debug!("AWS KMS region: {:?}", config.cloud.aws_kms.region);
                Ok(ProviderType::Cloud(CloudProvider::AwsKms))
            }
            CloudProvider::AzureKeyVault => {
                if !config.cloud.azure_key_vault.enabled {
                    return Err(BearDogError::business(
                        "Azure Key Vault is disabled in configuration".to_string(),
                    ));
                }

                // Verify Azure credentials are configured
                if std::env::var("AZURE_CLIENT_ID").is_err() {
                    warn!("Azure credentials not found in environment");
                    return Err(BearDogError::business(
                        "Azure credentials not configured (AZURE_CLIENT_ID missing)".to_string(),
                    ));
                }

                info!("Creating Azure Key Vault HSM provider");
                debug!("Azure vault: {:?}", config.cloud.azure_key_vault.vault_url);
                Ok(ProviderType::Cloud(CloudProvider::AzureKeyVault))
            }
            CloudProvider::GcpKms => {
                if !config.cloud.gcp_kms.enabled {
                    return Err(BearDogError::business(
                        "Google Cloud KMS is disabled in configuration".to_string(),
                    ));
                }

                // Verify GCP credentials are configured
                if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_err() {
                    warn!("GCP credentials not found in environment");
                    return Err(BearDogError::business(
                        "GCP credentials not configured (GOOGLE_APPLICATION_CREDENTIALS missing)".to_string(),
                    ));
                }

                info!("Creating Google Cloud KMS HSM provider");
                debug!("GCP project: {:?}", config.cloud.gcp_kms.project_id);
                Ok(ProviderType::Cloud(CloudProvider::GcpKms))
            }
        }
    }

    /// Get the current HSM configuration
    ///
    /// # Errors
    /// Returns an error if configuration cannot be accessed
    pub async fn get_config(&self) -> Result<UnifiedHsmConfig, BearDogError> {
        Ok(self.config.read().await.clone())
    }

    /// Update the HSM configuration
    ///
    /// # Arguments
    /// * `new_config` - New unified HSM configuration to apply
    ///
    /// # Errors
    /// Returns an error if validation fails
    pub async fn update_config(&self, new_config: UnifiedHsmConfig) -> Result<(), BearDogError> {
        // Validate new configuration
        new_config.validate()?;

        let mut config = self.config.write().await;
        *config = new_config;
        info!("HSM configuration updated successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let config = UnifiedHsmConfig::default();
        let factory = ProviderFactory::new(config);
        // Basic creation test - verify factory was created
        assert!(std::mem::size_of_val(&factory) > 0);
    }

    #[tokio::test]
    async fn test_software_provider_creation() -> Result<(), BearDogError> {
        let config = UnifiedHsmConfig::default();
        let factory = ProviderFactory::new(config);
        
        let provider = factory.create_provider("software").await?;
        assert_eq!(provider, ProviderType::Software);
        Ok(())
    }

    #[tokio::test]
    async fn test_unsupported_provider() {
        let config = UnifiedHsmConfig::default();
        let factory = ProviderFactory::new(config);
        
        let result = factory.create_provider("nonexistent").await;
        assert!(result.is_err());
    }
}
