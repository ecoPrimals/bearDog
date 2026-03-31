// SPDX-License-Identifier: AGPL-3.0-only

//! Witness signing message and Ed25519 signature verification.

use super::core::GenesisWitness;

impl GenesisWitness {
    /// Create message to be signed by witness
    ///
    /// Format: `new_node_id` || timestamp (8 bytes, big-endian) || `witness_public_key`
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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the public key length is wrong, the key material is invalid,
    /// or the signature buffer cannot be parsed as Ed25519.
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
                beardog_errors::BearDogError::security(format!("Invalid witness pubkey: {e}"))
            })?;

        let signature = Signature::try_from(&self.signature.as_slice()[..64]).map_err(|e| {
            beardog_errors::BearDogError::security(format!("Invalid signature: {e}"))
        })?;

        pubkey
            .verify(&message, &signature)
            .map(|()| true)
            .or(Ok(false))
    }
}
