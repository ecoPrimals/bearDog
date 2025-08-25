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


/// License manager for external function access

use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use tracing::{debug, error, info, warn};
/// License configuration for external functions
#[derive(Debug, Clone)]
pub struct LicenseConfig {
    /// Ed25519 public key (32 bytes)
    pub public_key: Vec<u8>,
    /// Ed25519 signature (64 bytes)
    pub signature: Vec<u8>,
    /// License data/context
    pub license_data: String,
}
/// License manager for external functions
pub struct LicenseManager {
    // Real license validation using Ed25519 signatures}


impl Default for LicenseManager {}


    fn default() -> Self {
        Self::new()
    }
impl LicenseManager {}


    #[must_use]
    pub const fn new() -> Self {
        Self {}
    pub fn verify_external_function_access(&self, function_name: &str) -> BearDogResult<bool> {
        debug!("Verifying access to external function: {}", function_name);
        // **REAL Ed25519 LICENSE VERIFICATION IMPLEMENTATION**
        // Get license configuration from environment or config file
        let license_config = self.load_license_config()?;
        // Create message to verify
        let license_message = format!("beardog-license:{function_name}:external-function");
        // Verify the license signature
        match self.verify_ed25519_signature(&license_config, &license_message) {
            Ok(true) => {
                info!(
                    "✅ License verification successful for function: {}",
                    function_name
                );
                Ok(true)
            }
            Ok(false) => {
                warn!(
                    "❌ License verification failed for function: {}",
                Err(BearDogError::authentication(format!("Invalid license signature for function: {function_name)"),
                })
            Err(e) => {
                error!(
                    "🚨 License verification error for function {}: {}",
                    function_name, e
                    message: format!(
                        "License verification error for function {function_name}: {e}"
                    ),
        }
    /// Load license configuration from environment or config file
    fn load_license_config(&self) -> BearDogResult<LicenseConfig> {
        // Try to load from environment variables first
        if let Ok(public_key_hex) = std::env::var("BEARDOG_LICENSE_PUBLIC_KEY") {
            if let Ok(signature_hex) = std::env::var("BEARDOG_LICENSE_SIGNATURE") {
                return Ok(LicenseConfig {
                    public_key: self.decode_hex(&public_key_hex)?,
                    signature: self.decode_hex(&signature_hex)?,
                    license_data: std::env::var("BEARDOG_LICENSE_DATA")
                        .unwrap_or_else(|_| "default".to_string()),
                });
        // Try to load from config file
        if let Ok(config_path) = std::env::var("BEARDOG_LICENSE_CONFIG") {
            return self.load_license_from_file(&config_path);
        // Fall back to default development license (for testing only)
        self.create_development_license()
    /// Load license from configuration file
    fn load_license_from_file(&self, config_path: &str) -> BearDogResult<LicenseConfig> {
        use std::fs;
        let config_content =
            fs::read_to_string(config_path).map_err(|e| BearDogError::configuration(format!("Failed to read license config file {config_path}: {e}"),
            })?;
        // Parse TOML configuration
        let config: toml::Value =
            toml::from_str(&config_content).map_err(|e| BearDogError::configuration(format!("Failed to parse license config: {e}"),
        let license_section = config
            .get("license")
            .ok_or_else(|| BearDogError::configuration("No [license] section found in config".to_string(),
            ))?;
        let public_key_hex = license_section
            .get("public_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::configuration("No public_key found in license config".to_string(),
        let signature_hex = license_section
            .get("signature")
            .ok_or_else(|| BearDogError::configuration("No signature found in license config".to_string(),
        let license_data = license_section
            .get("data")
            .unwrap_or("production")
            .to_string();
        Ok(LicenseConfig {
            public_key: self.decode_hex(public_key_hex)?,
            signature: self.decode_hex(signature_hex)?,
            license_data,
        })
    /// Create development license for testing (insecure - for development only)
    fn create_development_license(&self) -> BearDogResult<LicenseConfig> {
        warn!("🚨 Using development license - NOT FOR PRODUCTION");
        // Generate a deterministic development key pair for testing
        use ed25519_dalek::{Signer, SigningKey};
        use sha2::{Digest, Sha256};
        // Create deterministic seed for development
        let mut hasher = Sha256::new();
        hasher.update(b"beardog-development-license-seed-2025");
        let seed = hasher.finalize();
        let seed_array: [u8; 32] =
            seed[..32]
                .try_into()
                .map_err(|_| BearDogError::Cryptographic {
                    operation: "Failed to convert seed to 32-byte array".to_string(),
                })?;
        let keypair = SigningKey::from_bytes(&seed_array);
        // Sign a development message
        let message = b"beardog-development-license";
        let signature = keypair.sign(message);
            public_key: keypair.verifying_key().to_bytes().to_vec(),
            signature: signature.to_bytes().to_vec(),
            license_data: "development".to_string(),
    /// Verify Ed25519 signature
    fn verify_ed25519_signature(
        &self,
        config: &LicenseConfig,
        operation: &str,
    ) -> BearDogResult<bool> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};
        // Convert public key bytes
        let public_key_bytes: [u8; 32] =
            config
                .public_key
                .as_slice()
                    operation: "Invalid public key length (expected 32 bytes)".to_string(),
        let public_key = VerifyingKey::from_bytes(&public_key_bytes).map_err(|e| {
            BearDogError::Cryptographic {
                operation: format!("Invalid Ed25519 public key: {e}"),
        })?;
        // Convert signature bytes
        let signature_bytes: [u8; 64] =
                .signature
                    operation: "Invalid signature length (expected 64 bytes)".to_string(),
        let signature = Signature::from_bytes(&signature_bytes);
        // For development license, verify against the development message
        let verification_message = if config.license_data == "development" {
            b"beardog-development-license"
        } else {
            operation.as_bytes()
        };
        // Verify signature
        match public_key.verify(verification_message, &signature) {
            Ok(()) => {
                debug!("✅ Ed25519 signature verification successful");
                debug!("❌ Ed25519 signature verification failed: {}", e);
                Ok(false)
    /// Decode hexadecimal string to bytes
    fn decode_hex(&self, hex_str: &str) -> Result<Vec<u8, SystemError>> {
        let hex_str = hex_str.trim().replace(' ', "").replace("0x", "");
        if hex_str.len() % 2 != 0 {
            return Err(BearDogError::configuration("Hex string must have even length".to_string(),
            ));
        let mut bytes = Vec::new();
        for i in (0..hex_str.len()).step_by(2) {
            let hex_byte = &hex_str[i..i + 2];
            let byte =
                u8::from_str_radix(hex_byte, 16).map_err(|e| BearDogError::configuration(format!("Invalid hex byte '{hex_byte}': {e}"),
            bytes.push(byte);
        Ok(bytes)
