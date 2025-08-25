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


/// Authorization verification and proof handling
///
/// Handles verification of authorization proofs and revocation logic.

use chrono::Utc;
use beardog_errors::{BearDogError, BearDogResult};
use super::types::*;
impl CrossNodeAuthEngine {
    /// Verify an authorization proof
    pub async fn verify_authorization_proof(
        &self,
        proof: &AuthorizationProof,
    ) -> BearDogResult<bool> {
        // Implement actual proof verification with cryptographic validation
        
        // First check if the authorization exists and is active
        let auth = match self.active_authorizations.get(&proof.authorization_id) {
            Some(auth) => auth,
            None => return Ok(false),
        };
        // Check if active
        if !auth.is_active {
            return Ok(false);
        }
        // Check if not expired
        if auth.expires_at < Utc::now() {
        // Verify cryptographic proof using Ed25519 signature verification
        let message = format!("{}:{}:{}", 
            proof.authorization_id,
            proof.timestamp.timestamp(),
            proof.resource_id
        );
        self.verify_proof(&proof.signature, &proof.public_key, message.as_bytes()).await
    }
    /// Revoke an active authorization
    pub async fn revoke_authorization(&mut self, auth_id: &str) -> BearDogResult<()> {
        if let Some(auth) = self.active_authorizations.get_mut(auth_id) {
            auth.is_active = false;
            Ok(())
        } else {
            Err(BearDogError::Authorization(format!("Authorization not found: {auth_id)"},
            })
    // Implement actual proof verification using cryptographic methods
    pub async fn verify_proof(&self, proof: &str, public_key: &[u8], message: &[u8]) -> BearDogResult<bool> {
        use beardog_security::crypto_utils::BearDogCrypto;
        // Decode the proof as a signature
        let signature = hex::decode(proof).map_err(|e| BearDogError::invalid_input(format!("Invalid proof format: {e}"),
        })?;
        // Verify using Ed25519 signature verification
        BearDogCrypto::verify_ed25519_signature(public_key, message, &signature)
}
