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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_verify_authorization_proof_valid() {
        let mut engine = CrossNodeAuthEngine::default();
        let auth_id = "test-auth-1".to_string();

        let auth = CrossNodeAuthorization {
            request_id: auth_id.clone(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "test-resource".to_string(),
            permissions: vec![ResourcePermission::Read],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "test-signature".to_string(),
            is_active: true,
        };
        engine.active_authorizations.insert(auth_id.clone(), auth);

        let proof = AuthorizationProof {
            authorization_id: auth_id,
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "valid-signature".to_string(),
        };

        let result = engine.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(result.unwrap(), "Valid proof should verify");
    }

    #[test]
    fn test_verify_authorization_proof_not_found() {
        let engine = CrossNodeAuthEngine::default();

        let proof = AuthorizationProof {
            authorization_id: "nonexistent".to_string(),
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "some-signature".to_string(),
        };

        let result = engine.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Non-existent auth should fail");
    }

    #[test]
    fn test_verify_authorization_proof_inactive() {
        let mut engine = CrossNodeAuthEngine::default();
        let auth_id = "test-auth-2".to_string();

        let auth = CrossNodeAuthorization {
            request_id: auth_id.clone(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "test-resource".to_string(),
            permissions: vec![ResourcePermission::Read],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "test-signature".to_string(),
            is_active: false, // Inactive
        };
        engine.active_authorizations.insert(auth_id.clone(), auth);

        let proof = AuthorizationProof {
            authorization_id: auth_id,
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "some-signature".to_string(),
        };

        let result = engine.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Inactive auth should fail");
    }

    #[test]
    fn test_verify_authorization_proof_expired() {
        let mut engine = CrossNodeAuthEngine::default();
        let auth_id = "test-auth-3".to_string();

        let auth = CrossNodeAuthorization {
            request_id: auth_id.clone(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "test-resource".to_string(),
            permissions: vec![ResourcePermission::Read],
            conditions: vec![],
            created_at: Utc::now() - Duration::hours(2),
            expires_at: Utc::now() - Duration::hours(1), // Expired
            signature: "test-signature".to_string(),
            is_active: true,
        };
        engine.active_authorizations.insert(auth_id.clone(), auth);

        let proof = AuthorizationProof {
            authorization_id: auth_id,
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "some-signature".to_string(),
        };

        let result = engine.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Expired auth should fail");
    }

    #[test]
    fn test_verify_authorization_proof_empty_signature() {
        let mut engine = CrossNodeAuthEngine::default();
        let auth_id = "test-auth-4".to_string();

        let auth = CrossNodeAuthorization {
            request_id: auth_id.clone(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "test-resource".to_string(),
            permissions: vec![ResourcePermission::Read],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "test-signature".to_string(),
            is_active: true,
        };
        engine.active_authorizations.insert(auth_id.clone(), auth);

        let proof = AuthorizationProof {
            authorization_id: auth_id,
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: String::new(), // Empty signature
        };

        let result = engine.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Empty signature should fail");
    }

    #[test]
    fn test_revoke_authorization_success() {
        let mut engine = CrossNodeAuthEngine::default();
        let auth_id = "test-auth-5".to_string();

        let auth = CrossNodeAuthorization {
            request_id: auth_id.clone(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "test-resource".to_string(),
            permissions: vec![ResourcePermission::Read],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "test-signature".to_string(),
            is_active: true,
        };
        engine.active_authorizations.insert(auth_id.clone(), auth);

        let result = engine.revoke_authorization(&auth_id);
        assert!(result.is_ok(), "Revocation should succeed");

        let auth = engine.active_authorizations.get(&auth_id).unwrap();
        assert!(
            !auth.is_active,
            "Authorization should be inactive after revocation"
        );
    }

    #[test]
    fn test_revoke_authorization_not_found() {
        let mut engine = CrossNodeAuthEngine::default();

        let result = engine.revoke_authorization("nonexistent");
        assert!(result.is_err(), "Revoking nonexistent auth should error");
    }
}
