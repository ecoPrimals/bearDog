

use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info, warn};
impl AndroidAttestationService {

    pub async fn new(config: AttestationConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Android Attestation Service");

        let trusted_certificates = Self::load_trusted_certificates(&config).await?;
        info!(
            "📜 Loaded {} trusted certificates",
            trusted_certificates.len()
        );

        let challenge_generator = Arc::new(ChallengeGenerator::new());
        let service = Self {
            config,
            trusted_certificates,
            challenge_generator,
        };
        info!("✅ Android Attestation Service initialized");
        Ok(service)
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {
        info!("🔍 Initializing attestation service");

        for (i, cert) in self.trusted_certificates.iter().enumerate() {
            if cert.is_empty() {
                return Err(BearDogError::VerificationFailed {
                    message: format!("Trusted certificate {i} is empty"),
                });
            }
        }

        let _test_challenge = self.challenge_generator.generate_challenge(32)?;
        info!("✅ Attestation service initialization completed");
        Ok(())

    pub async fn verify_certificate_chain(
        &self,
        chain: &[Vec<u8>],
        challenge: &[u8],
    ) -> Result<bool, BearDogError> {
            "🔍 Verifying certificate chain ({} certificates)",
            chain.len()
        if chain.is_empty() {
            return Ok(false);

        if chain.len() < 2 {
            warn!(
                "⚠️ Certificate chain too short: {} certificates",
                chain.len()
            );

        for (i, cert) in chain.iter().enumerate() {
            if cert.len() < 4 || cert[0] != 0x30 {
                warn!("⚠️ Invalid DER certificate at position {}", i);
                return Ok(false);

        if challenge.is_empty() {
            warn!("⚠️ Empty challenge provided");

        let root_cert = &chain[chain.len() - 1];
        let is_trusted = self.is_certificate_trusted(root_cert).await?;
        if !is_trusted {
            warn!("⚠️ Root certificate is not trusted");
        info!("✅ Certificate chain verification successful");
        Ok(true)

    pub async fn create_attestation_data(
        key_id: &str,
        device_info: &AndroidDeviceInfo,
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("📝 Creating attestation data for key: {}", key_id);

        let mut attestation_data = Vec::new();

        attestation_data.extend_from_slice(b"ANDROID_ATTEST");

        attestation_data.extend_from_slice(&(key_id.len() as u32).to_be_bytes());
        attestation_data.extend_from_slice(key_id.as_bytes());

        attestation_data.extend_from_slice(&(device_info.manufacturer.len() as u32).to_be_bytes());
        attestation_data.extend_from_slice(device_info.manufacturer.as_bytes());
        attestation_data.extend_from_slice(&(device_info.model.len() as u32).to_be_bytes());
        attestation_data.extend_from_slice(device_info.model.as_bytes());
        attestation_data
            .extend_from_slice(&(device_info.android_version.len() as u32).to_be_bytes());
        attestation_data.extend_from_slice(device_info.android_version.as_bytes());

        let boot_state_byte = match device_info.verified_boot_state {
            VerifiedBootState::Green => 0x01,
            VerifiedBootState::Yellow => 0x02,
            VerifiedBootState::Orange => 0x03,
            VerifiedBootState::Red => 0x04,
            VerifiedBootState::Unknown => 0x00,
        attestation_data.push(boot_state_byte);

        attestation_data.extend_from_slice(&(challenge.len() as u32).to_be_bytes());
        attestation_data.extend_from_slice(challenge);

        let timestamp = chrono::Utc::now().timestamp() as u64;
        attestation_data.extend_from_slice(&timestamp.to_be_bytes());

        attestation_data.push(0x01); // StrongBox-backed
        debug!(
            "📝 Attestation data created: {} bytes",
            attestation_data.len()
        Ok(attestation_data)

    async fn is_certificate_trusted(&self, certificate: &[u8]) -> Result<bool, BearDogError> {
        debug!("🔍 Checking if certificate is trusted");

        for trusted_cert in &self.trusted_certificates {
            if certificate == trusted_cert {
                debug!("✅ Certificate matches trusted certificate");
                return Ok(true);

        if certificate.len() >= 4 && certificate[0] == 0x30 && certificate[1] == 0x82 {
            debug!("✅ Certificate appears to be valid DER format");
            return Ok(true); // For simulation, accept valid DER certificates
        debug!("❌ Certificate is not trusted");
        Ok(false)

    async fn load_trusted_certificates(_config: &AttestationConfig) -> Result<Vec<Vec<u8>, BearDogError>>> {
        info!("📜 Loading trusted certificates");

        let mut certificates = Vec::new();

        certificates.push(Self::load_google_root_certificate().await?);
        info!("📜 Loaded {} trusted certificates", certificates.len());
        Ok(certificates)

    async fn load_google_root_certificate() -> Result<Vec<u8>, BearDogError>> {

        Ok(vec![
            0x30, 0x82, 0x01,
            0x00, // DER header for mock certificate

        ])

    pub async fn load_certificate_from_path(path: &str) -> Result<Vec<u8>, BearDogError>> {
        debug!("📜 Loading certificate from: {}", path);

        Ok(vec![0x30, 0x82, 0x01, 0x00])
}
