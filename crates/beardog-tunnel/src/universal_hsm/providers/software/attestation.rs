// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Attestation Engine
///
/// Key attestation and verification capabilities for the software HSM.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::HsmKey;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use crate::universal_hsm::traits::{attestation::AttestationLevel, AttestationData};
use super::config::SoftwareHsmConfig;
/// Attestation engine for key verification
#[derive(Debug)]
pub struct AttestationEngine {
    config: SoftwareHsmConfig,
}
impl AttestationEngine {
    /// Create a new attestation engine}


    pub fn new(config: &SoftwareHsmConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
    /// Attest a key and generate attestation data
    pub async fn attest_key(&self, key: &HsmKey) -> BearDogResult<AttestationData> {
        if !self.config.enable_attestation {
            return Err(BearDogError::NotSupported {
                feature: "Attestation is disabled in configuration".to_string(),
            });
        // Generate attestation certificate
        let certificate = self.generate_attestation_certificate(key).await?;
        // Create attestation signature
        let signature = self.create_attestation_signature(key, &certificate).await?;
        // Collect attestation metadata
        let mut metadata = HashMap::new();
        metadata.insert(
            "hsm_type".to_string(),
            serde_json::Value::String("software".to_string()),
        );
            "key_type".to_string(),
            serde_json::Value::String(format!("{:?}", key.key_type)),
            "attestation_version".to_string(),
            serde_json::Value::String("1.0".to_string()),
            "security_level".to_string(),
        Ok(AttestationData {
            level: AttestationLevel::Software,
            certificate_chain: vec![certificate.clone()],
            attestation_signature: signature.clone(),
            nonce: vec![1, 2, 3, 4], // Simple nonce for software attestation
            generated_at: chrono::Utc::now(),
            challenge: vec![], // No challenge for software attestation
            metadata,
            // Legacy compatibility fields
            certificate,
            signature,
            attestation_time: chrono::Utc::now(),
        })
    /// Generate attestation certificate for a key
    async fn generate_attestation_certificate(&self, key: &HsmKey) -> BearDogResult<Vec<u8>> {
        // Create certificate data structure
        let mut cert_data = Vec::new();
        // Add key identifier
        cert_data.extend_from_slice(key.id.as_bytes());
        // Add key type
        cert_data.extend_from_slice(format!("{:?}", key.key_type).as_bytes());
        // Add public key
        // Add key material (extract public key from material)
        match &key.material {
            beardog_types::canonical::hsm::KeyMaterial::PublicKey(pub_key) => {
                cert_data.extend_from_slice(pub_key);
            }
            beardog_types::canonical::hsm::KeyMaterial::PrivateKey(_priv_key) => {
                // For private keys, we'd typically derive the public key
                // For now, use a placeholder
                cert_data.extend_from_slice(b"derived_public_key");
            beardog_types::canonical::hsm::KeyMaterial::SoftwareHandle { handle, .. } => {
                cert_data.extend_from_slice(handle.as_bytes());
            beardog_types::canonical::hsm::KeyMaterial::HardwareReference {
                key_handle, ..
            } => {
                cert_data.extend_from_slice(key_handle.as_bytes());
            _ => {
                cert_data.extend_from_slice(b"software_key_material");
        // Add timestamp
        let timestamp = chrono::Utc::now().timestamp().to_le_bytes();
        cert_data.extend_from_slice(&timestamp);
        // Add HSM identifier
        cert_data.extend_from_slice(b"BearDog-Software-HSM-v1.0");
        // Hash the certificate data
        let mut hasher = Sha256::new();
        hasher.update(&cert_data);
        let certificate_hash = hasher.finalize();
        // Create certificate structure (simplified format)
        let mut certificate = Vec::new();
        certificate.extend_from_slice(b"BEARDOG_CERT_V1");
        certificate.extend_from_slice(&(cert_data.len() as u32).to_le_bytes());
        certificate.extend_from_slice(&cert_data);
        certificate.extend_from_slice(&certificate_hash);
        Ok(certificate)
    /// Create attestation signature for the certificate
    async fn create_attestation_signature(
        &self,
        key: &HsmKey,
        certificate: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        // In a real implementation, this would use the HSM's attestation key
        // For software HSM, we create a deterministic signature based on the certificate
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(certificate);
        signature_data.extend_from_slice(key.id.as_bytes());
        signature_data.extend_from_slice(b"BearDog-Software-HSM-Attestation");
        // Create signature hash
        hasher.update(&signature_data);
        let signature_hash = hasher.finalize();
        // Create signature structure
        let mut signature = Vec::new();
        signature.extend_from_slice(b"BEARDOG_SIG_V1");
        signature.extend_from_slice(&signature_hash);
        Ok(signature)
    /// Verify an attestation certificate}


    pub async fn verify_attestation(
        attestation: &AttestationData,
    ) -> BearDogResult<bool> {
        // Verify certificate structure
        if attestation.certificate.len() < 16 {
            return Ok(false);
        // Check certificate header
        if &attestation.certificate[..15] != b"BEARDOG_CERT_V1" {
        // Verify signature structure
        if attestation.signature.len() < 14 {
        // Check signature header
        if &attestation.signature[..14] != b"BEARDOG_SIG_V1" {
        // Regenerate expected certificate and signature
        let expected_cert = self.generate_attestation_certificate(key).await?;
        let expected_sig = self
            .create_attestation_signature(key, &expected_cert)
            .await?;
        // Compare with provided attestation
        let cert_valid = attestation.certificate == expected_cert;
        let sig_valid = attestation.signature == expected_sig;
        Ok(cert_valid && sig_valid)
    /// Get attestation capabilities}


    pub fn get_attestation_capabilities(&self) -> AttestationCapabilities {
        AttestationCapabilities {
            supports_key_attestation: self.config.enable_attestation,
            supports_remote_attestation: false, // Software HSM doesn't support remote attestation
            attestation_algorithms: vec!["SHA256".to_string()],
            certificate_formats: vec!["BearDog-Custom-V1".to_string()],
            max_certificate_size: 4096,
    /// Generate attestation report for multiple keys}


    pub async fn generate_batch_attestation(
        keys: &[HsmKey],
    ) -> BearDogResult<BatchAttestationReport> {
        let mut attestations = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        for key in keys {
            match self.attest_key(key).await {
                Ok(attestation) => {
                    attestations.push((key.id.clone(), Some(attestation)));
                    successful += 1;
                }
                Err(_) => {
                    attestations.push((key.id.clone(), None));
                    failed += 1;
        Ok(BatchAttestationReport {
            attestations,
            total_keys: keys.len(),
            successful_attestations: successful,
            failed_attestations: failed,
    /// Validate attestation chain
    pub async fn validate_attestation_chain(
        attestations: &[AttestationData],
        if attestations.is_empty() {
            return Ok(true);
        // For software HSM, we just check that all attestations are well-formed
        for attestation in attestations {
            if attestation.certificate.is_empty() || attestation.signature.is_empty() {
                return Ok(false);
            // Check certificate header
            if attestation.certificate.len() < 15
                || &attestation.certificate[..15] != b"BEARDOG_CERT_V1"
            {
            // Check signature header
            if attestation.signature.len() < 14 || &attestation.signature[..14] != b"BEARDOG_SIG_V1"
        Ok(true)
/// Attestation capabilities
#[derive(Debug, Clone)]
pub struct AttestationCapabilities {
    pub supports_key_attestation: bool,
    pub supports_remote_attestation: bool,
    pub attestation_algorithms: Vec<String>,
    pub certificate_formats: Vec<String>,
    pub max_certificate_size: usize,
/// Batch attestation report
pub struct BatchAttestationReport {
    pub attestations: Vec<(String, Option<AttestationData>)>, // (key_id, attestation)
    pub total_keys: usize,
    pub successful_attestations: usize,
    pub failed_attestations: usize,
    pub generated_at: chrono::DateTime<chrono::Utc>,
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    use std::collections::HashMap;}


    fn create_test_key() -> HsmKey {
        HsmKey {
            key_id: "test-key-123".to_string(),
            key_type: KeyType::Ed25519,
            public_key: vec![1, 2, 3, 4, 5, 6, 7, 8],
            tags: HashMap::new(),
    #[tokio::test]
    async fn test_attestation_engine_creation() -> beardog_errors::BearDogResult<()> {
        let config = SoftwareHsmConfig::default();
        let engine = AttestationEngine::new(&config);
        let capabilities = engine.get_attestation_capabilities();
        assert!(capabilities.supports_key_attestation);
        Ok(())}


    async fn test_key_attestation() -> beardog_errors::BearDogResult<()> {
        let key = create_test_key();
        let attestation = engine.attest_key(&key).await.map_err(|e| BearDogError::internal(format!("Operation failed: {:?}", e)))?;
        assert!(!attestation.certificate.is_empty());
        assert!(!attestation.signature.is_empty());
        assert!(!attestation.metadata.is_empty());
    async fn test_attestation_verification() -> beardog_errors::BearDogResult<()> {
        let is_valid = engine.verify_attestation(&attestation, &key).await.map_err(|e| BearDogError::internal(format!("Operation failed: {:?}", e)))?;
        assert!(is_valid);
    async fn test_batch_attestation() -> beardog_errors::BearDogResult<()> {
        let keys = vec![
            create_test_key(),
            HsmKey {
                key_id: "test-key-456".to_string(),
                key_type: KeyType::Aes256,
                public_key: vec![9, 10, 11, 12],
                tags: HashMap::new(),
            },
        ];
        let report = engine.generate_batch_attestation(&keys).await.map_err(|e| BearDogError::internal(format!("Operation failed: {:?}", e)))?;
        assert_eq!(report.total_keys, 2);
        assert_eq!(report.successful_attestations, 2);
        assert_eq!(report.failed_attestations, 0);
    async fn test_attestation_disabled() -> beardog_errors::BearDogResult<()> {
        let mut config = SoftwareHsmConfig::default();
        config.enable_attestation = false;
        let result = engine.attest_key(&key).await;
        assert!(result.is_err());
