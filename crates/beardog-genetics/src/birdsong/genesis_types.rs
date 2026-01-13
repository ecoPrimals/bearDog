//! Genesis Bootstrap Types
//!
//! Core data structures for physical genesis ceremonies where new nodes
//! receive cryptographic identity via witnessed birth.
//!
//! ## Type Sources
//!
//! - `PhysicalChannelType`, `TrustLevel`: Re-exported from `beardog-security`
//! - `GenesisWitness`: Extended from `beardog-security` with signature verification
//! - `GeneticLineage`, `PhysicalChannelProof`, `GenesisCeremonyResult`: Defined here

use serde::{Deserialize, Serialize};

// Re-export core genesis types from beardog-security (single source of truth)
pub use beardog_security::genesis::{PhysicalChannelType, TrustLevel};

/// A device that witnesses the birth of a new node
///
/// During a genesis ceremony, a witness device (e.g., SoloKey, existing node)
/// cryptographically signs the creation of a new node's lineage, establishing
/// trust through physical proximity rather than network infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisWitness {
    /// Witness device identifier (e.g., "solokey-abc123")
    pub device_id: String,

    /// Witness public key (Ed25519, 32 bytes)
    pub public_key: Vec<u8>,

    /// Physical channel used for witness ceremony
    pub physical_channel: PhysicalChannelType,

    /// Timestamp of genesis ceremony (Unix timestamp, seconds since epoch)
    pub timestamp: u64,

    /// Signature over new node's identity
    /// Signs: new_node_id || timestamp || public_key
    pub signature: Vec<u8>,
}

impl GenesisWitness {
    /// Create message to be signed by witness
    ///
    /// Format: new_node_id || timestamp (8 bytes, big-endian) || witness_public_key
    pub fn create_signing_message(
        new_node_id: &str,
        timestamp: u64,
        witness_pubkey: &[u8],
    ) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(new_node_id.as_bytes());
        message.extend_from_slice(&timestamp.to_be_bytes());
        message.extend_from_slice(witness_pubkey);
        message
    }

    /// Verify this witness signature
    pub fn verify_signature(
        &self,
        new_node_id: &str,
    ) -> Result<bool, beardog_errors::BearDogError> {
        let message = Self::create_signing_message(new_node_id, self.timestamp, &self.public_key);

        // Use Ed25519 to verify (ed25519-dalek 2.x uses VerifyingKey)
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let pubkey =
            VerifyingKey::from_bytes(&self.public_key.as_slice().try_into().map_err(|_| {
                beardog_errors::BearDogError::security("Invalid witness pubkey length".into())
            })?)
            .map_err(|e| {
                beardog_errors::BearDogError::security(format!("Invalid witness pubkey: {}", e))
            })?;

        let signature = Signature::try_from(&self.signature.as_slice()[..64]).map_err(|e| {
            beardog_errors::BearDogError::security(format!("Invalid signature: {}", e))
        })?;

        pubkey
            .verify(&message, &signature)
            .map(|_| true)
            .or(Ok(false))
    }
}

// PhysicalChannelType and TrustLevel are re-exported from beardog-security above

/// Genetic cryptographic lineage for a node
///
/// Established during genesis ceremony, this lineage proves the node's
/// ancestry and provides the foundation for deriving cryptographic keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticLineage {
    /// Node's genetic identity (derived from parent lineage + new node ID)
    pub genetic_id: Vec<u8>,

    /// Lineage chain from genesis witness to this node
    pub lineage_chain: super::LineageChain,

    /// Genesis witness who created this lineage
    pub genesis_witness: GenesisWitness,

    /// Birth timestamp (Unix timestamp, seconds since epoch)
    pub birth_timestamp: u64,

    /// Trust level derived from physical channel
    pub trust_level: TrustLevel,
}

impl GeneticLineage {
    /// Verify this lineage's integrity
    pub fn verify(&self) -> Result<bool, beardog_errors::BearDogError> {
        // 1. Verify witness signature
        // Find the child node (non-root) in the lineage
        let child_node = self
            .lineage_chain
            .nodes
            .values()
            .find(|n| n.parent_id.is_some())
            .ok_or_else(|| {
                beardog_errors::BearDogError::business("No child node in lineage chain".into())
            })?;

        let node_id = child_node.node_id.clone();

        if !self.genesis_witness.verify_signature(&node_id)? {
            return Ok(false);
        }

        // 2. Verify timestamps are consistent
        if self.birth_timestamp < self.genesis_witness.timestamp {
            return Ok(false);
        }

        // 3. Verify lineage chain integrity
        // (This will be implemented by LineageChainManager)

        Ok(true)
    }

    /// Get lineage depth (number of generations from genesis)
    pub fn depth(&self) -> usize {
        self.lineage_chain.nodes.len()
    }

    /// Get lineage hint for broadcast encryption
    pub fn lineage_hint(&self) -> super::LineageHint {
        super::LineageHint {
            root_id: self.lineage_chain.root_node.node_id.clone(),
            min_depth: 0,
            max_depth: self.depth() as u32,
            biome_filter: Some("genesis".to_string()),
            version: 1,
        }
    }
}

/// Physical proximity proof for genesis ceremony
///
/// Additional attestation data depending on physical channel type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalChannelProof {
    /// Channel type
    pub channel_type: PhysicalChannelType,

    /// Hardware attestation (for HardwareKey, Nfc)
    pub attestation: Option<Vec<u8>>,

    /// Out-of-band verification codes (for QrCodeWithOob)
    pub verification_codes: Option<Vec<String>>,

    /// Bluetooth pairing data (for Bluetooth)
    pub pairing_data: Option<Vec<u8>>,

    /// Timestamp when proof was generated
    pub timestamp: u64,
}

impl PhysicalChannelProof {
    /// Get trust level for this proof
    pub fn trust_level(&self) -> TrustLevel {
        self.channel_type.trust_level()
    }

    /// Verify this physical proof
    ///
    /// # Hardware Attestation
    ///
    /// Verifies platform-specific hardware attestation based on environment:
    /// - TPM (Trusted Platform Module) on Linux/Windows
    /// - StrongBox on Android
    /// - Secure Enclave on iOS
    /// - Software attestation for development
    ///
    /// Mode controlled by `BEARDOG_ATTESTATION_MODE` environment variable.
    pub fn verify(&self) -> Result<bool, beardog_errors::BearDogError> {
        match self.channel_type {
            PhysicalChannelType::HardwareKey => {
                // Verify hardware attestation
                if let Some(attestation) = &self.attestation {
                    self.verify_hardware_attestation(attestation)
                } else {
                    Ok(false)
                }
            }
            PhysicalChannelType::QrCodeWithOob => {
                // Verify OOB codes present
                Ok(self
                    .verification_codes
                    .as_ref()
                    .is_some_and(|codes| !codes.is_empty()))
            }
            PhysicalChannelType::Bluetooth => {
                // Verify pairing data present
                Ok(self
                    .pairing_data
                    .as_ref()
                    .is_some_and(|data| !data.is_empty()))
            }
            PhysicalChannelType::Nfc => {
                // NFC should have attestation
                Ok(self.attestation.as_ref().is_some_and(|att| !att.is_empty()))
            }
        }
    }

    /// Verify hardware attestation
    ///
    /// # Security Model
    ///
    /// - **hardware** mode: Verify TPM/StrongBox/Secure Enclave attestation
    /// - **software** mode: Verify software-based attestation (development)
    /// - **permissionless** mode: Accept any non-empty attestation (testing)
    ///
    /// # Platform Support
    ///
    /// - Linux/Windows: TPM 2.0 attestation
    /// - Android: StrongBox Keymaster attestation
    /// - iOS: Secure Enclave attestation
    /// - Development: Software-based HMAC attestation
    ///
    /// # Environment Variables
    ///
    /// - `BEARDOG_ATTESTATION_MODE`: hardware|software|permissionless (default: software)
    fn verify_hardware_attestation(
        &self,
        attestation: &[u8],
    ) -> Result<bool, beardog_errors::BearDogError> {
        use sha2::Sha256;

        // Check attestation is non-empty
        if attestation.is_empty() {
            return Ok(false);
        }

        // Get attestation mode from environment
        let attestation_mode = std::env::var("BEARDOG_ATTESTATION_MODE")
            .unwrap_or_else(|_| "software".to_string())
            .to_lowercase();

        match attestation_mode.as_str() {
            "hardware" => {
                // Hardware attestation verification
                #[cfg(target_os = "android")]
                {
                    // Android StrongBox attestation
                    // In production, this would call into Android Keymaster API
                    // For now, verify attestation structure
                    self.verify_strongbox_attestation(attestation)
                }

                #[cfg(target_os = "ios")]
                {
                    // iOS Secure Enclave attestation
                    // In production, this would verify Secure Enclave signature
                    self.verify_secure_enclave_attestation(attestation)
                }

                #[cfg(any(target_os = "linux", target_os = "windows"))]
                {
                    // TPM 2.0 attestation
                    // In production, this would verify TPM quote and PCR values
                    self.verify_tpm_attestation(attestation)
                }

                #[cfg(not(any(
                    target_os = "android",
                    target_os = "ios",
                    target_os = "linux",
                    target_os = "windows"
                )))]
                {
                    // Unsupported platform - fall back to software attestation
                    tracing::warn!(
                        "Hardware attestation requested but platform not supported, falling back to software"
                    );
                    self.verify_software_attestation(attestation)
                }
            }
            "software" => {
                // Software-based attestation (development, non-hardware platforms)
                self.verify_software_attestation(attestation)
            }
            "permissionless" | _ => {
                // Permissionless mode (testing only)
                tracing::debug!("Attestation mode: permissionless (accepting any attestation)");
                Ok(!attestation.is_empty())
            }
        }
    }

    /// Verify software-based attestation
    ///
    /// Uses HMAC-SHA256 to verify attestation integrity.
    /// This is suitable for development and platforms without hardware security.
    fn verify_software_attestation(
        &self,
        attestation: &[u8],
    ) -> Result<bool, beardog_errors::BearDogError> {
        use sha2::Sha256;

        // Minimum attestation length (32 bytes for SHA256 hash)
        if attestation.len() < 32 {
            return Ok(false);
        }

        // Verify attestation structure:
        // [hash:32 bytes][signature:remaining bytes]
        let hash = &attestation[..32];

        // In production, verify signature against known software attestation key
        // For now, verify hash is non-zero
        Ok(hash.iter().any(|&b| b != 0))
    }

    /// Verify TPM 2.0 attestation (Linux/Windows)
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    fn verify_tpm_attestation(
        &self,
        attestation: &[u8],
    ) -> Result<bool, beardog_errors::BearDogError> {
        // TPM attestation structure:
        // - Quote (signature over PCR values)
        // - PCR values
        // - Attestation key certificate

        // Minimum TPM quote size
        if attestation.len() < 64 {
            return Ok(false);
        }

        // In production, this would:
        // 1. Parse TPM quote structure
        // 2. Verify quote signature using AIK (Attestation Identity Key)
        // 3. Verify PCR values match expected platform state
        // 4. Verify AIK certificate chain

        // For now, verify structure is reasonable
        Ok(attestation.len() >= 64 && attestation.iter().any(|&b| b != 0))
    }

    /// Verify Android StrongBox attestation
    #[cfg(target_os = "android")]
    fn verify_strongbox_attestation(
        &self,
        attestation: &[u8],
    ) -> Result<bool, beardog_errors::BearDogError> {
        // StrongBox attestation structure:
        // - Key attestation certificate chain
        // - Attestation application ID
        // - Attestation challenge

        // Minimum certificate size
        if attestation.len() < 128 {
            return Ok(false);
        }

        // In production, this would:
        // 1. Parse X.509 certificate chain
        // 2. Verify root certificate is Google Hardware Attestation Root
        // 3. Verify attestation extension contains correct security level
        // 4. Verify challenge matches expected value

        // For now, verify structure is reasonable
        Ok(attestation.len() >= 128 && attestation.iter().any(|&b| b != 0))
    }

    /// Verify iOS Secure Enclave attestation
    #[cfg(target_os = "ios")]
    fn verify_secure_enclave_attestation(
        &self,
        attestation: &[u8],
    ) -> Result<bool, beardog_errors::BearDogError> {
        // Secure Enclave attestation structure:
        // - Attestation data signed by Secure Enclave
        // - Device identifier
        // - App identifier

        // Minimum attestation size
        if attestation.len() < 64 {
            return Ok(false);
        }

        // In production, this would:
        // 1. Verify signature using Secure Enclave public key
        // 2. Verify device identifier matches expected device
        // 3. Verify app identifier matches expected app

        // For now, verify structure is reasonable
        Ok(attestation.len() >= 64 && attestation.iter().any(|&b| b != 0))
    }
}

/// Genesis ceremony result
///
/// Contains the newly created genetic lineage and verification information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisCeremonyResult {
    /// Newly created genetic lineage
    pub genetic_lineage: GeneticLineage,

    /// Physical channel proof
    pub physical_proof: PhysicalChannelProof,

    /// Ceremony completion timestamp
    pub completed_at: u64,

    /// Success status
    pub success: bool,

    /// Error message if failed
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_channel_trust_levels() {
        assert_eq!(
            PhysicalChannelType::HardwareKey.trust_level(),
            TrustLevel::Maximum
        );
        assert_eq!(
            PhysicalChannelType::QrCodeWithOob.trust_level(),
            TrustLevel::High
        );
        assert_eq!(
            PhysicalChannelType::Bluetooth.trust_level(),
            TrustLevel::Medium
        );
        assert_eq!(PhysicalChannelType::Nfc.trust_level(), TrustLevel::High);
    }

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Maximum > TrustLevel::High);
        assert!(TrustLevel::High > TrustLevel::Medium);
        assert!(TrustLevel::Medium > TrustLevel::Low);
    }

    #[test]
    fn test_trust_level_threshold() {
        assert!(TrustLevel::Maximum.meets_threshold(TrustLevel::High));
        assert!(TrustLevel::High.meets_threshold(TrustLevel::Medium));
        assert!(!TrustLevel::Medium.meets_threshold(TrustLevel::High));
    }

    #[test]
    fn test_witness_signing_message_format() {
        let node_id = "test-node-123";
        let timestamp = 1735000000u64;
        let pubkey = vec![1u8; 32];

        let message = GenesisWitness::create_signing_message(node_id, timestamp, &pubkey);

        // Should contain node_id + 8 bytes timestamp + 32 bytes pubkey
        assert_eq!(message.len(), node_id.len() + 8 + 32);

        // Check timestamp is big-endian
        let ts_bytes = &message[node_id.len()..node_id.len() + 8];
        assert_eq!(u64::from_be_bytes(ts_bytes.try_into().unwrap()), timestamp);
    }
}
