// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::auth::types::{
    AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation, ProofVerifier,
};
use beardog_errors::BearDogError;
use chrono::Utc;

pub struct DefaultProofVerifier {
    // In a real implementation, this would contain cryptographic keys and verification logic
}

impl Default for DefaultProofVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultProofVerifier {
    /// Creates a new instance
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl ProofVerifier for DefaultProofVerifier {
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError> {
        // Basic validation checks
        if proof.authorization_id.is_empty() {
            return Ok(false);
        }

        if proof.proof_signature.is_empty() {
            return Ok(false);
        }

        // Check if proof is not too old (within 1 hour)
        let now = Utc::now();
        let proof_age = now.signed_duration_since(proof.timestamp);
        if proof_age.num_hours() > 1 {
            return Ok(false);
        }

        // In a real implementation, this would do cryptographic signature verification
        // For now, we'll do basic validation and return true for non-empty signatures
        Ok(!proof.proof_signature.is_empty())
    }

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError> {
        // Generate a proof for the given authorization and operation
        // In a real implementation, this would create cryptographic signatures

        let proof_signature = format!(
            "proof_{}_{}_{}",
            authorization.request_id,
            operation.operation_type.clone() as u8,
            Utc::now().timestamp()
        );

        Ok(AuthorizationProof {
            authorization_id: authorization.request_id.clone(),
            operation: operation.clone(),
            proof_signature,
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_authorization() -> CrossNodeAuthorization {
        CrossNodeAuthorization {
            request_id: "test-request".to_string(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "resource-1".to_string(),
            permissions: vec![],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "test-signature".to_string(),
            is_active: true,
        }
    }

    fn create_test_operation() -> CrossNodeOperation {
        CrossNodeOperation::default()
    }

    #[test]
    fn test_verify_authorization_proof_valid() {
        let verifier = DefaultProofVerifier::new();
        let proof = AuthorizationProof {
            authorization_id: "auth-123".to_string(),
            operation: create_test_operation(),
            timestamp: Utc::now(),
            proof_signature: "valid-signature".to_string(),
        };

        let result = verifier.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(result.unwrap(), "Valid proof should verify");
    }

    #[test]
    fn test_verify_authorization_proof_empty_auth_id() {
        let verifier = DefaultProofVerifier::new();
        let proof = AuthorizationProof {
            authorization_id: String::new(),
            operation: create_test_operation(),
            timestamp: Utc::now(),
            proof_signature: "signature".to_string(),
        };

        let result = verifier.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Empty auth ID should fail verification");
    }

    #[test]
    fn test_verify_authorization_proof_empty_signature() {
        let verifier = DefaultProofVerifier::new();
        let proof = AuthorizationProof {
            authorization_id: "auth-123".to_string(),
            operation: create_test_operation(),
            timestamp: Utc::now(),
            proof_signature: String::new(),
        };

        let result = verifier.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Empty signature should fail verification");
    }

    #[test]
    fn test_verify_authorization_proof_expired() {
        let verifier = DefaultProofVerifier::new();
        let proof = AuthorizationProof {
            authorization_id: "auth-123".to_string(),
            operation: create_test_operation(),
            timestamp: Utc::now() - Duration::hours(2),
            proof_signature: "signature".to_string(),
        };

        let result = verifier.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(
            !result.unwrap(),
            "Expired proof (>1 hour) should fail verification"
        );
    }

    #[test]
    fn test_generate_proof_success() {
        let verifier = DefaultProofVerifier::new();
        let authorization = create_test_authorization();
        let operation = create_test_operation();

        let result = verifier.generate_proof(&authorization, &operation);
        assert!(result.is_ok(), "Proof generation should succeed");

        let proof = result.unwrap();
        assert_eq!(proof.authorization_id, authorization.request_id);
        assert!(
            !proof.proof_signature.is_empty(),
            "Generated proof should have signature"
        );
    }

    #[test]
    fn test_generate_proof_signature_format() {
        let verifier = DefaultProofVerifier::new();
        let authorization = create_test_authorization();
        let operation = create_test_operation();

        let result = verifier.generate_proof(&authorization, &operation);
        let proof = result.unwrap();

        assert!(
            proof.proof_signature.starts_with("proof_"),
            "Signature should start with 'proof_'"
        );
        assert!(
            proof.proof_signature.contains(&authorization.request_id),
            "Signature should contain request ID"
        );
    }

    #[test]
    fn test_generate_proof_unique_signatures() {
        let verifier = DefaultProofVerifier::new();
        let authorization = create_test_authorization();
        let operation = create_test_operation();

        let proof1 = verifier.generate_proof(&authorization, &operation).unwrap();
        // Sleep for 1 second since signatures use timestamp in seconds
        std::thread::sleep(std::time::Duration::from_secs(1));
        let proof2 = verifier.generate_proof(&authorization, &operation).unwrap();

        assert_ne!(
            proof1.proof_signature, proof2.proof_signature,
            "Generated proofs should have unique signatures"
        );
    }

    #[test]
    fn test_default_proof_verifier_construction() {
        let verifier1 = DefaultProofVerifier::new();
        let verifier2 = DefaultProofVerifier::default();

        // Both constructors should work
        let proof = AuthorizationProof {
            authorization_id: "test".to_string(),
            operation: create_test_operation(),
            timestamp: Utc::now(),
            proof_signature: "sig".to_string(),
        };

        assert!(verifier1.verify_authorization_proof(&proof).is_ok());
        assert!(verifier2.verify_authorization_proof(&proof).is_ok());
    }
}
