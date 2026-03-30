// SPDX-License-Identifier: AGPL-3.0-only

//! Authorization verification implementation

use super::types::*;
use beardog_errors::BearDogError;
use beardog_types::genetics_constraints::KeyOperation;
use chrono::Utc;
use tracing::{debug, info};

impl CrossNodeAuthEngine {
    /// Verify an operation against a genetic key's constraints
    ///
    /// This is a convenience method that:
    /// 1. Checks if the key has expired
    /// 2. Verifies constraint integrity (detects tampering)
    /// 3. Checks if the operation is allowed by the key's constraints
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let delete_op = KeyOperation::Delete {
    ///     path: "raw_data/temperature.nc".to_string(),
    /// };
    ///
    /// if let Err(e) = auth_engine.verify_genetic_operation(&key, &delete_op) {
    ///     return Err(e); // Operation blocked by constraints
    /// }
    ///
    /// // Proceed with operation
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Key has expired
    /// - Constraints have been tampered with
    /// - Operation violates constraints
    pub fn verify_genetic_operation(
        &self,
        key: &BearDogGenetics,
        operation: &KeyOperation,
    ) -> Result<(), BearDogError> {
        // Check expiration first
        if key.is_expired() {
            info!("Operation blocked: Key {} has expired", key.id);
            return Err(BearDogError::unauthorized(format!(
                "Key {} has expired and cannot be used",
                key.id
            )));
        }

        // Verify operation against constraints
        key.verify_operation(operation)?;

        debug!(
            "Operation {:?} verified for key {} with constraints: {}",
            operation,
            key.id,
            key.constraint_description()
        );

        Ok(())
    }

    /// Verify authorization proof
    ///
    /// # Errors
    ///
    /// Currently returns [`Ok`] with `true`/`false`; the `Result` type is reserved for future crypto failures.
    pub fn verify_authorization_proof(
        &self,
        proof: &AuthorizationProof,
    ) -> Result<bool, BearDogError> {
        let Some(auth) = self.active_authorizations.get(&proof.authorization_id) else {
            return Ok(false);
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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError::not_found`] when `auth_id` is unknown.
    pub fn revoke_authorization(&mut self, auth_id: &str) -> Result<(), BearDogError> {
        if let Some(auth) = self.active_authorizations.get_mut(auth_id) {
            auth.is_active = false;
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Authorization not found: {auth_id}"
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
        assert!(
            result.expect("verify_authorization_proof Ok"),
            "Valid proof should verify"
        );
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
        assert!(
            !result.expect("verify_authorization_proof Ok"),
            "Non-existent auth should fail"
        );
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
        assert!(
            !result.expect("verify_authorization_proof Ok"),
            "Inactive auth should fail"
        );
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
        assert!(
            !result.expect("verify_authorization_proof Ok"),
            "Expired auth should fail"
        );
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
        assert!(
            !result.expect("verify_authorization_proof Ok"),
            "Empty signature should fail"
        );
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

        let auth = engine
            .active_authorizations
            .get(&auth_id)
            .expect("auth present after insert");
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

    // ========================================================================
    // GENETIC KEY CONSTRAINT VERIFICATION TESTS
    // ========================================================================

    #[test]
    fn test_verify_genetic_operation_allows_compliant() {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = CrossNodeAuthEngine::default();

        // Create key with constraints
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["protected/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![])
            .expect("generate_with_constraints for compliant op test");

        // Allowed operation
        let delete_unprotected = KeyOperation::Delete {
            path: "temp/cache.dat".to_string(),
        };

        let result = engine.verify_genetic_operation(&key, &delete_unprotected);
        assert!(result.is_ok(), "Compliant operation should be allowed");
    }

    #[test]
    fn test_verify_genetic_operation_blocks_violation() {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = CrossNodeAuthEngine::default();

        // Create key with constraints
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["protected/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![])
            .expect("generate_with_constraints for violation test");

        // Violating operation
        let delete_protected = KeyOperation::Delete {
            path: "protected/data.csv".to_string(),
        };

        let result = engine.verify_genetic_operation(&key, &delete_protected);
        assert!(result.is_err(), "Constraint violation should be blocked");
        assert!(
            result
                .expect_err("constraint violation expected")
                .to_string()
                .contains("protected"),
            "Error should mention protected path"
        );
    }

    #[test]
    fn test_verify_genetic_operation_blocks_expired() {
        use beardog_types::genetics_constraints::{KeyConstraints, LifetimeConstraint};

        let engine = CrossNodeAuthEngine::default();

        // Create expired key
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint {
                expires_at: Utc::now() - Duration::hours(1), // Expired 1 hour ago
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![])
            .expect("generate_with_constraints for expired key test");

        // Any operation should fail
        let read_op = KeyOperation::Read {
            path: "data.csv".to_string(),
            project: None,
        };

        let result = engine.verify_genetic_operation(&key, &read_op);
        assert!(result.is_err(), "Expired key should be rejected");
        assert!(
            result
                .expect_err("expired key should error")
                .to_string()
                .contains("expired"),
            "Error should mention expiration"
        );
    }

    #[test]
    fn test_verify_genetic_operation_detects_tampering() {
        use beardog_types::genetics_constraints::{DataAccessConstraint, KeyConstraints};

        let engine = CrossNodeAuthEngine::default();

        // Create key with constraints
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["protected/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        let entropy = vec![1u8; 32];
        let mut key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![])
            .expect("generate_with_constraints for tamper test");

        // Tamper with constraints
        if let Some(ref mut constraints) = key.constraints {
            constraints.data_access.immutable_paths.clear(); // Remove protection
        }

        // Any operation should fail due to tampering
        let delete_op = KeyOperation::Delete {
            path: "protected/data.csv".to_string(),
        };

        let result = engine.verify_genetic_operation(&key, &delete_op);
        assert!(result.is_err(), "Tampered key should be rejected");
        assert!(
            result
                .expect_err("tampered key should error")
                .to_string()
                .contains("tampered"),
            "Error should mention tampering"
        );
    }

    #[test]
    fn test_verify_genetic_operation_with_no_constraints() {
        let engine = CrossNodeAuthEngine::default();

        // Key without constraints (backward compatibility)
        let key = BearDogGenetics::default();

        // Any operation should be allowed
        let delete_op = KeyOperation::Delete {
            path: "any/file.dat".to_string(),
        };

        let result = engine.verify_genetic_operation(&key, &delete_op);
        assert!(
            result.is_ok(),
            "Operations should be allowed for keys without constraints"
        );
    }
}
