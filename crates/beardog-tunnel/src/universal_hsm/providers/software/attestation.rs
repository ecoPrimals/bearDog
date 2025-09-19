

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::HsmKey;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use crate::universal_hsm::traits::{attestation::AttestationLevel, AttestationData};
use super::config::SoftwareHsmConfig;

#[derive(Debug, Clone)]
}
impl AttestationEngine {

/// New operation.
    /// Creates a new instance
    pub fn new(config: &SoftwareHsmConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

/// Attest Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn attest_key(&self, key: &HsmKey) -> Result<AttestationData, BearDogError> {
        if !self.config.enable_attestation {
            return Err(BearDogError::NotSupported {
                feature: "Attestation is disabled in configuration".to_string(),
            });

        let certificate = self.generate_attestation_certificate(key)?;

        let signature = self.create_attestation_signature(key, &certificate)?;

        let mut metadata = HashMap::with_capacity(16);
        metadata.insert(
            "hsm_type".to_string(),
            serde_json::Value::String("software".to_string()),
        );
            "key_type".to_string(),
            serde_json::Value::String(format!("{:?}", key.key_type)),
            "attestation_version".to_string(),
            serde_json::Value::String(AttestationLevel::Software,
            certificate_chain: vec![certificate.clone()],
            attestation_signature: signature.clone(vec![1, 2, 3, 4], // Simple nonce for software attestation
            generated_at: chrono::Utc::now(vec![], // No challenge for software attestation
            metadata,

            certificate,
            signature,
            attestation_time: chrono::Utc::now(),
        })


    fn generate_attestation_certificate(&self, key: &HsmKey) -> Result<Vec<u8>, BearDogError>> {

        let mut cert_data = Vec::new();

        cert_data.extend_from_slice(key.id.as_bytes());

        cert_data.extend_from_slice(format!("{:?}", key.key_type).as_bytes());

        match &key.material {
            beardog_types::canonical::hsm::KeyMaterial::PublicKey(pub_key) => {
                cert_data.extend_from_slice(pub_key);
            }
            beardog_types::canonical::hsm::KeyMaterial::PrivateKey(_priv_key) => {

                cert_data.extend_from_slice(b"derived_public_key");
            beardog_types::canonical::hsm::KeyMaterial::SoftwareHandle { handle, .. } => {
                cert_data.extend_from_slice(handle.as_bytes());
            beardog_types::canonical::hsm::KeyMaterial::HardwareReference {
                key_handle, ..
            } => {
                cert_data.extend_from_slice(key_handle.as_bytes());
            _ => {
                cert_data.extend_from_slice(b"software_key_material");

        let timestamp = chrono::Utc::now().timestamp().to_le_bytes();
        cert_data.extend_from_slice(&timestamp);

        cert_data.extend_from_slice(b"BearDog-Software-HSM-v1.0");

        let mut hasher = Sha256::new();
        hasher.update(&cert_data);
        let certificate_hash = hasher.finalize();

        let mut certificate = Vec::new(&HsmKey,
        certificate: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {

        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(certificate);
        signature_data.extend_from_slice(key.id.as_bytes());
        signature_data.extend_from_slice(b"BearDog-Software-HSM-Attestation");

        hasher.update(&signature_data);
        let signature_hash = hasher.finalize();

        let mut signature = Vec::new(&AttestationData,
    ) -> Result<bool, BearDogError> {

        if attestation.certificate.len(self.config.enable_attestation,
            supports_remote_attestation: false, // Software HSM doesn't support remote attestation
            attestation_algorithms: vec!["SHA256".to_string()],
            certificate_formats: vec!["BearDog-Custom-V1".to_string(),

/// Generate Batch Attestation operation.
    pub fn generate_batch_attestation(&[HsmKey],
    ) -> Result<BatchAttestationReport, BearDogError> {
        let mut attestations = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        for key in keys {
            match self.attest_key(key) {
                Ok(attestation) => {
                    attestations.push((key.id.clone(), Some(attestation)));
                    successful += 1;
                }
                Err(_) => {
                    attestations.push((key.id.clone(), None));
                    failed += 1;
        Ok(BatchAttestationReport {
            attestations,
            total_keys: keys.len(successful,
            failed_attestations: failed,

/// Validate Attestation Chain operation.
    /// Validates attestation_chain
    /// Validates attestation_chain
    pub fn validate_attestation_chain(&[AttestationData],
        if attestations.is_empty(bool,
    /// Whether supports_remote_attestation is enabled
    pub supports_remote_attestation: bool,
    /// Collection of attestation algorithms
    pub attestation_algorithms: Vec<String>,
    pub certificate_formats: Vec<String>,
    /// Number of max_certificate_size
    pub max_certificate_size: usize,

pub struct BatchAttestationReport {
    /// Collection of attestations
    pub attestations: Vec<(String, Option<AttestationData>)>, // (key_id, attestation)
    /// Number of total_keys
    pub total_keys: usize,
    /// Number of successful_attestations
    pub successful_attestations: usize,
    /// Number of failed_attestations
    pub failed_attestations: usize,
    /// The generated at value
    pub generated_at: chrono::DateTime<chrono::Utc>,
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    use std::collections::HashMap;}

    /// Creates test_key
    fn create_test_key() -> HsmKey {
        HsmKey {
            key_id: "test-key-123".to_string(),
            tags: HashMap::with_capacity(16),
    #[tokio::test]
    fn test_attestation_engine_creation() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let engine = AttestationEngine::new(&config);
        let capabilities = engine.get_attestation_capabilities();
        assert!(capabilities.supports_key_attestation);
        Ok(())}


    fn test_key_attestation() -> Result<(), BearDogError> {
        let key = create_test_key();
        let attestation = engine.attest_key(&key).map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        assert!(!attestation.certificate.is_empty());
        assert!(!attestation.signature.is_empty());
        assert!(!attestation.metadata.is_empty());
    fn test_attestation_verification() -> Result<(), BearDogError> {
        let is_valid = engine.verify_attestation(&attestation, &key).map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        assert!(is_valid);
    fn test_batch_attestation() -> Result<(), BearDogError> {
        let keys = vec![
            create_test_key(),
            HsmKey {
                key_id: "test-key-456".to_string(),
                tags: HashMap::with_capacity(16),
            },
        ];
        let report = engine.generate_batch_attestation(&keys).map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        assert_eq!(report.total_keys, 2);
        assert_eq!(report.successful_attestations, 2);
        assert_eq!(report.failed_attestations, 0);
    fn test_attestation_disabled() -> Result<(), BearDogError> {
        let mut config = SoftwareHsmConfig::default();
        config.enable_attestation = false;
        let result = engine.attest_key(&key);
        assert!(result.is_err());
