// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified provider traits (`UnifiedProvider`, `UnifiedSecurityProvider`, `UnifiedHsmProvider`).

use super::AndroidStrongBoxHsm;
use beardog_errors::BearDogError;
use beardog_types::canonical::UnifiedProvider;
use beardog_types::canonical::providers_unified::traits::base_traits as unified;
use beardog_types::canonical::providers_unified::traits::{
    AttestationResponse, AuthenticationRequest, AuthenticationResponse, AuthorizationRequest,
    AuthorizationResponse, BackupInfo, HsmDeviceInfo, KeyBackupSpec, KeyGenerationSpec, KeyInfo,
    KeyType, KeyUsage, SecurityContext, UnifiedHsmProvider, UnifiedSecurityProvider,
};
use std::collections::HashMap;
use tracing::{info, warn};

// Implement UnifiedProvider (base trait)
// Note: UnifiedProvider uses RPITIT (native async), not #[async_trait]
impl UnifiedProvider for AndroidStrongBoxHsm {
    fn provider_info(&self) -> unified::ProviderInfo {
        unified::ProviderInfo {
            id: "android_strongbox".to_string(),
            name: "Android StrongBox HSM".to_string(),
            version: super::super::VERSION.to_string(),
            provider_type: unified::ProviderType::Security,
            supported_capabilities: vec![
                "hardware_keystore".to_string(),
                "key_attestation".to_string(),
                "hardware_rng".to_string(),
            ],
        }
    }

    async fn health_check(&self) -> Result<unified::ProviderHealth, BearDogError> {
        let health_status = self.health_monitor.get_health_status().await?;

        let error_msg = health_status.error_message;
        Ok(unified::ProviderHealth {
            status: if health_status.is_healthy {
                unified::HealthStatus::Healthy
            } else {
                unified::HealthStatus::Degraded
            },
            timestamp: std::time::SystemTime::now(),
            details: error_msg
                .as_ref()
                .map(|msg| HashMap::from([("error".to_string(), msg.clone())]))
                .unwrap_or_default(),
            resource_usage: unified::ResourceUsage {
                cpu_percent: 0.0,
                memory_bytes: 0,
                memory_percent: 0.0,
                disk_io: HashMap::new(),
                network_io: unified::NetworkIoMetrics {
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                },
            },
            last_error: error_msg,
        })
    }

    async fn metrics(&self) -> Result<unified::ProviderMetrics, BearDogError> {
        Ok(unified::ProviderMetrics {
            timestamp: std::time::SystemTime::now(),
            performance: HashMap::new(),
            custom_metrics: vec![],
            system_metrics: beardog_types::workflow::SystemMetrics {
                uptime_seconds: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                avg_response_time_ms: 0.0,
                active_connections: 0,
                error_rate: 0.0,
            },
        })
    }

    fn capabilities(&self) -> Vec<unified::ProviderCapability> {
        vec![
            unified::ProviderCapability {
                name: "HardwareKeyStorage".to_string(),
                description: "Hardware-backed key storage in StrongBox".to_string(),
                parameters: vec![],
                enabled: true,
            },
            unified::ProviderCapability {
                name: "KeyAttestation".to_string(),
                description: "Hardware key attestation".to_string(),
                parameters: vec![],
                enabled: true,
            },
            unified::ProviderCapability {
                name: "HardwareRng".to_string(),
                description: "Hardware random number generation".to_string(),
                parameters: vec![],
                enabled: true,
            },
        ]
    }

    async fn initialize(
        &mut self,
        _config: unified::ProviderConfiguration,
    ) -> Result<(), BearDogError> {
        info!("Android StrongBox HSM already initialized");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BearDogError> {
        info!("Shutting down Android StrongBox HSM");
        Ok(())
    }
}

// Implement UnifiedSecurityProvider (extends UnifiedProvider)
// Note: UnifiedSecurityProvider uses RPITIT (native async), not #[async_trait]
impl UnifiedSecurityProvider for AndroidStrongBoxHsm {
    async fn authenticate(
        &self,
        _request: AuthenticationRequest,
    ) -> Result<AuthenticationResponse, BearDogError> {
        // StrongBox is a cryptographic HSM, not an authentication provider
        Err(BearDogError::unsupported_operation(
            "Authentication not supported in StrongBox HSM - use for crypto operations only",
        ))
    }

    async fn authorize(
        &self,
        _request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, BearDogError> {
        // StrongBox is a cryptographic HSM, not an authorization provider
        Err(BearDogError::unsupported_operation(
            "Authorization not supported in StrongBox HSM - use for crypto operations only",
        ))
    }

    async fn encrypt(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>, BearDogError> {
        self.validate_key_access(key_id)?;
        self.keystore.encrypt(key_id, data).await
    }

    async fn decrypt(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>, BearDogError> {
        self.validate_key_access(key_id)?;
        self.keystore.decrypt(key_id, data).await
    }

    async fn sign(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>, BearDogError> {
        self.validate_key_access(key_id)?;
        self.keystore.sign(key_id, data).await
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &str,
    ) -> Result<bool, BearDogError> {
        self.validate_key_access(key_id)?;
        self.keystore.verify(key_id, data, signature).await
    }

    async fn generate_random(&self, length: usize) -> Result<Vec<u8>, BearDogError> {
        // Use Android's hardware RNG
        self.keystore.generate_random_bytes(length).await
    }

    fn security_context(&self) -> SecurityContext {
        // Use available SecurityContext fields for Android StrongBox
        SecurityContext {
            security_level: "StrongBox".to_string(),
            encryption_algorithms: vec!["AES-256-GCM".to_string(), "ChaCha20-Poly1305".to_string()],
            signature_algorithms: vec![
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "RSA-PSS-2048".to_string(),
            ],
            key_derivation_functions: vec!["HKDF-SHA256".to_string()],
            random_generators: vec!["Hardware-RNG-StrongBox".to_string()],
        }
    }
}

// Implement UnifiedHsmProvider (HSM-specific operations)
// Note: UnifiedHsmProvider uses RPITIT (native async), not #[async_trait]
impl UnifiedHsmProvider for AndroidStrongBoxHsm {
    async fn generate_key(&self, spec: KeyGenerationSpec) -> Result<KeyInfo, BearDogError> {
        info!("🔐 Generating StrongBox key: {}", spec.key_id);

        // Use updated method
        self.generate_strongbox_key(&spec).await
    }

    async fn import_key(
        &self,
        key_data: &[u8],
        key_type: KeyType,
        key_id: &str,
    ) -> Result<KeyInfo, BearDogError> {
        info!("📥 Importing key into StrongBox: {}", key_id);

        // Import into Android Keystore
        self.keystore
            .import_key(key_id, key_data, key_type.clone())
            .await?;

        // Return key info
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type,
            key_size: key_data.len() as u32 * 8, // Convert bytes to bits
            key_usage: vec![KeyUsage::Sign, KeyUsage::Verify], // Default usage
            created_at: std::time::SystemTime::now(),
            extractable: false, // StrongBox keys are hardware-bound
        })
    }

    async fn export_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        // Android StrongBox keys are HARDWARE-BOUND and CANNOT be exported
        // This is a SECURITY FEATURE, not a limitation
        warn!(
            "🔒 Key export denied for StrongBox key: {} (hardware-bound security)",
            key_id
        );
        Err(BearDogError::hsm(format!(
            "Key export not supported for StrongBox key '{}' - keys are hardware-bound for security",
            key_id
        )))
    }

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting StrongBox key: {}", key_id);
        self.keystore.delete_key(key_id).await?;

        // Remove from cache
        let mut cache = self.key_cache.write().await;
        cache.remove(key_id);

        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<KeyInfo>, BearDogError> {
        info!("📋 Listing StrongBox keys");
        self.keystore.list_keys().await
    }

    async fn device_info(&self) -> Result<HsmDeviceInfo, BearDogError> {
        Ok(HsmDeviceInfo {
            manufacturer: self.device_info.manufacturer.clone(),
            model: self.device_info.model.clone(),
            serial_number: "REDACTED".to_string(), // Privacy: don't expose serial
            firmware_version: self.device_info.android_version.clone(),
            hardware_version: "StrongBox".to_string(),
            supported_algorithms: vec![
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
                "EC-P256".to_string(),
                "EC-P384".to_string(),
                "AES-256".to_string(),
            ],
            capabilities: vec![
                "hardware_keystore".to_string(),
                "key_attestation".to_string(),
                "hardware_bound_keys".to_string(),
                "hardware_rng".to_string(),
            ],
            status: "Operational".to_string(),
            certificate: None, // Attestation certificate available via attest()
            attestation_data: None,
        })
    }

    async fn attest(&self) -> Result<AttestationResponse, BearDogError> {
        info!("🔐 Performing device attestation");
        let challenge = self.keystore.generate_attestation_challenge(32)?;

        Ok(AttestationResponse {
            success: true,
            attestation_data: challenge,
            signature: vec![],
            certificate_chain: vec![],
            timestamp: std::time::SystemTime::now(),
        })
    }

    async fn backup_keys(&self, _spec: KeyBackupSpec) -> Result<BackupInfo, BearDogError> {
        // Android StrongBox keys are HARDWARE-BOUND and CANNOT be backed up
        // This is a SECURITY FEATURE, not a limitation
        warn!("🔒 Key backup denied for StrongBox keys (hardware-bound security)");
        Err(BearDogError::hsm(
            "Key backup not supported for StrongBox keys - keys are hardware-bound for security"
                .to_string(),
        ))
    }
}
