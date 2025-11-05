//! Authorization verification implementation

use super::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;

impl CrossNodeAuthEngine {
    /// Verify authorization proof
    pub fn verify_authorization_proof(
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
            return Ok(false);
        }

        // Simplified verification - just check if proof exists
        Ok(!proof.proof_signature.is_empty())
    }

    /// Revoke authorization
    pub fn revoke_authorization(&mut self, auth_id: &str) -> Result<(), BearDogError> {
        if let Some(auth) = self.active_authorizations.get_mut(auth_id) {
            auth.is_active = false;
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Authorization not found: {}",
                auth_id
            )))
        }
    }
}
