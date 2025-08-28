

use chrono::Utc;
use beardog_errors::BearDogError;
use super::types::*;
impl CrossNodeAuthEngine {

    pub async fn verify_authorization_proof(
        &self,
        proof: &AuthorizationProof,
    ) -> Result<bool, BearDogError> {

        let auth = match self.active_authorizations.get(&proof.authorization_id) {
            Some(auth) => auth,
            None => return Ok(false),
        };

        if !auth.is_active {
            return Ok(false);
        }

        if auth.expires_at < Utc::now() {

        let message = format_args!("{}:{}:{}", 
            proof.authorization_id,
            proof.timestamp.timestamp().to_string(),
            proof.resource_id
        );
        self.verify_proof(&proof.signature, &proof.public_key, message.as_bytes()).await
    }

    pub async fn revoke_authorization(&mut self, auth_id: &str) -> Result<(), BearDogError> {
        if let Some(auth) = self.active_authorizations.get_mut(auth_id) {
            auth.is_active = false;
            Ok(())
        } else {
            Err(BearDogError::Authorization(format!("Authorization not found: {auth_id)"},
            })

    pub async fn verify_proof(&self, proof: &str, public_key: &[u8], message: &[u8]) -> Result<bool, BearDogError> {
        use beardog_security::crypto_utils::BearDogCrypto;

        let signature = hex::decode(proof).map_err(|e| BearDogError::invalid_input(format!("Invalid proof format: {e}"),
        })?;

        BearDogCrypto::verify_ed25519_signature(public_key, message, &signature)
}
