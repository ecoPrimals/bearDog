

use chrono::Utc;
use beardog_errors::BearDogError;
use super::types::*;
impl CrossNodeAuthEngine {

/// Verify Authorization Proof operation.
    pub fn verify_authorization_proof(&AuthorizationProof,
    ) -> Result<bool, BearDogError> {

        let auth = match self.active_authorizations.get(&proof.authorization_id) {
            Some(auth) => auth,
            None => return Ok(false),
        };

        if !auth.is_active {
            return Ok(false);
        }

        if auth.expires_at < Utc::now() {

        let message = format!("{}:{}:{}", 
            proof.authorization_id,
            proof.timestamp.timestamp(),
            proof.resource_id
        );
        self.verify_proof(&proof.signature, &proof.public_key, message.as_bytes())
    }

/// Revoke Authorization operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn revoke_authorization(&mut self, auth_id: &str) -> Result<(), BearDogError> {
        if let Some(auth) = self.active_authorizations.get_mut(auth_id) {
            auth.is_active = false;
            Ok(())
        } else {
            Err(BearDogError::Authorization({}auth_id"},
            })

/// Verify Proof operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn verify_proof(&str, public_key: &[u8], message: &[u8]) -> Result<bool, BearDogError> {
        use beardog_security::crypto_utils::BearDogCrypto;

        let signature = hex::decode(proof).map_err(|e| BearDogError::invalid_input(format!("Invalid proof format: {e}"),
        })?;

        BearDogCrypto::verify_ed25519_signature(public_key, message, &signature)
}
