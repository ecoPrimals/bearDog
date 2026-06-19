// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cross-node authorization proof verification.
//!
//! [`Ed25519ProofVerifier`] is the production [`ProofVerifier`] implementation. It signs and
//! verifies canonical payloads with an Ed25519 key supplied at construction time (from an HSM or
//! software keystore).
//!
//! [`PlaceholderProofVerifier`] is available only under `cfg(test)` or the `test-utils` feature.
//! It performs structural and freshness checks without cryptography and must not be used in
//! production deployments.

use crate::auth::types::{
    AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation, ProofVerifier,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::Serialize;
use std::collections::BTreeMap;

/// Production [`ProofVerifier`] backed by Ed25519 signatures.
///
/// Construct with a [`SigningKey`] obtained from your node's HSM or software keystore. The
/// corresponding [`VerifyingKey`] is derived automatically for proof verification.
pub struct Ed25519ProofVerifier {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519ProofVerifier {
    /// Creates a verifier that signs proofs with `signing_key`.
    #[must_use]
    pub fn new(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Returns the verifying key peers use to validate proofs from this node.
    #[must_use]
    pub const fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
}

impl ProofVerifier for Ed25519ProofVerifier {
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError> {
        if proof.authorization_id.is_empty() || proof.proof_signature.is_empty() {
            return Ok(false);
        }

        if proof_is_stale(proof.timestamp) {
            return Ok(false);
        }

        let payload =
            canonical_proof_bytes(&proof.authorization_id, &proof.operation, proof.timestamp)?;

        let signature = decode_ed25519_signature(&proof.proof_signature)?;

        Ok(self.verifying_key.verify(&payload, &signature).is_ok())
    }

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError> {
        let timestamp = Utc::now();
        let payload = canonical_proof_bytes(&authorization.request_id, operation, timestamp)?;

        let signature = self.signing_key.sign(&payload);
        let proof_signature = BASE64_STANDARD.encode(signature.to_bytes());

        Ok(AuthorizationProof {
            authorization_id: authorization.request_id.clone(),
            operation: operation.clone(),
            proof_signature,
            timestamp,
        })
    }
}

/// Deterministic JSON payload signed by [`Ed25519ProofVerifier`].
#[derive(Serialize)]
struct CanonicalProofPayload<'a> {
    authorization_id: &'a str,
    operation: CanonicalOperation<'a>,
    timestamp: String,
}

/// Operation view with sorted parameters for stable serialization.
#[derive(Serialize)]
struct CanonicalOperation<'a> {
    operation_type: &'a crate::auth::types::OperationType,
    target_resource: &'a str,
    parameters: BTreeMap<&'a str, &'a str>,
    requester_signature: &'a str,
}

fn canonical_proof_bytes(
    authorization_id: &str,
    operation: &CrossNodeOperation,
    timestamp: DateTime<Utc>,
) -> Result<Vec<u8>, BearDogError> {
    let parameters = operation
        .parameters
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();

    let payload = CanonicalProofPayload {
        authorization_id,
        operation: CanonicalOperation {
            operation_type: &operation.operation_type,
            target_resource: &operation.target_resource,
            parameters,
            requester_signature: &operation.requester_signature,
        },
        timestamp: timestamp.to_rfc3339(),
    };

    serde_json::to_vec(&payload).map_err(|error| {
        BearDogError::crypto_error(format!(
            "failed to serialize authorization proof payload: {error}"
        ))
    })
}

fn decode_ed25519_signature(encoded: &str) -> Result<ed25519_dalek::Signature, BearDogError> {
    let bytes = BASE64_STANDARD.decode(encoded).map_err(|error| {
        BearDogError::crypto_error(format!("invalid base64 proof signature: {error}"))
    })?;

    if bytes.len() != 64 {
        return Err(BearDogError::crypto_error(format!(
            "invalid Ed25519 signature length: {} (expected 64)",
            bytes.len()
        )));
    }

    let mut signature_bytes = [0u8; 64];
    signature_bytes.copy_from_slice(&bytes);
    Ok(ed25519_dalek::Signature::from_bytes(&signature_bytes))
}

fn proof_is_stale(timestamp: DateTime<Utc>) -> bool {
    let now = Utc::now();
    now.signed_duration_since(timestamp).num_hours() > 1
}

/// Test-only [`ProofVerifier`] that checks proof shape and freshness without cryptography.
#[cfg(any(test, feature = "test-utils"))]
pub struct PlaceholderProofVerifier;

#[cfg(any(test, feature = "test-utils"))]
impl Default for PlaceholderProofVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl PlaceholderProofVerifier {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl ProofVerifier for PlaceholderProofVerifier {
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError> {
        if proof.authorization_id.is_empty() || proof.proof_signature.is_empty() {
            return Ok(false);
        }

        if proof_is_stale(proof.timestamp) {
            return Ok(false);
        }

        Ok(!proof.proof_signature.is_empty())
    }

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError> {
        let proof_signature = format!(
            "proof_{}_{}_{}_{}",
            authorization.request_id,
            operation.operation_type.clone() as u8,
            Utc::now().timestamp_nanos_opt().unwrap_or_default(),
            uuid::Uuid::new_v4().as_simple(),
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
    use crate::auth::types::OperationType;
    use chrono::Duration;
    use ed25519_dalek::SigningKey;

    fn test_signing_key(seed: u8) -> SigningKey {
        SigningKey::from_bytes(&[seed; 32])
    }

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

    mod ed25519 {
        use super::*;

        #[test]
        fn generate_and_verify_roundtrip() {
            let verifier = Ed25519ProofVerifier::new(test_signing_key(7));
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let proof = verifier
                .generate_proof(&authorization, &operation)
                .expect("generate_proof should succeed");

            let verified = verifier
                .verify_authorization_proof(&proof)
                .expect("verify_authorization_proof should succeed");
            assert!(verified, "generated proof should verify");
        }

        #[test]
        fn verify_rejects_tampered_signature() {
            let verifier = Ed25519ProofVerifier::new(test_signing_key(8));
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let mut proof = verifier
                .generate_proof(&authorization, &operation)
                .expect("generate_proof should succeed");
            proof.proof_signature = BASE64_STANDARD.encode([0u8; 64]);

            let verified = verifier
                .verify_authorization_proof(&proof)
                .expect("verify_authorization_proof should succeed");
            assert!(!verified, "tampered signature should fail verification");
        }

        #[test]
        fn verify_rejects_stale_proof() {
            let verifier = Ed25519ProofVerifier::new(test_signing_key(9));
            let proof = AuthorizationProof {
                authorization_id: "auth-123".to_string(),
                operation: create_test_operation(),
                timestamp: Utc::now() - Duration::hours(2),
                proof_signature: BASE64_STANDARD.encode([1u8; 64]),
            };

            let verified = verifier
                .verify_authorization_proof(&proof)
                .expect("verify_authorization_proof should succeed");
            assert!(!verified, "stale proof should fail verification");
        }

        #[test]
        fn verify_rejects_wrong_signing_key() {
            let signer = Ed25519ProofVerifier::new(test_signing_key(10));
            let other = Ed25519ProofVerifier::new(test_signing_key(11));
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let proof = signer
                .generate_proof(&authorization, &operation)
                .expect("generate_proof should succeed");

            let verified = other
                .verify_authorization_proof(&proof)
                .expect("verify_authorization_proof should succeed");
            assert!(!verified, "proof from another key should not verify");
        }

        #[test]
        fn signatures_are_base64_encoded() {
            let verifier = Ed25519ProofVerifier::new(test_signing_key(12));
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let proof = verifier
                .generate_proof(&authorization, &operation)
                .expect("generate_proof should succeed");

            assert!(
                BASE64_STANDARD.decode(&proof.proof_signature).is_ok(),
                "signature should be valid base64"
            );
        }
    }

    mod placeholder {
        use super::*;

        #[test]
        fn test_verify_authorization_proof_valid() {
            let verifier = PlaceholderProofVerifier::new();
            let proof = AuthorizationProof {
                authorization_id: "auth-123".to_string(),
                operation: create_test_operation(),
                timestamp: Utc::now(),
                proof_signature: "valid-signature".to_string(),
            };

            let result = verifier.verify_authorization_proof(&proof);
            assert!(result.is_ok());
            assert!(
                result.expect("verify_authorization_proof result in test"),
                "Valid proof should verify"
            );
        }

        #[test]
        fn test_verify_authorization_proof_empty_auth_id() {
            let verifier = PlaceholderProofVerifier::new();
            let proof = AuthorizationProof {
                authorization_id: String::new(),
                operation: create_test_operation(),
                timestamp: Utc::now(),
                proof_signature: "signature".to_string(),
            };

            let result = verifier.verify_authorization_proof(&proof);
            assert!(result.is_ok());
            assert!(
                !result.expect("verify_authorization_proof result in test"),
                "Empty auth ID should fail verification"
            );
        }

        #[test]
        fn test_verify_authorization_proof_empty_signature() {
            let verifier = PlaceholderProofVerifier::new();
            let proof = AuthorizationProof {
                authorization_id: "auth-123".to_string(),
                operation: create_test_operation(),
                timestamp: Utc::now(),
                proof_signature: String::new(),
            };

            let result = verifier.verify_authorization_proof(&proof);
            assert!(result.is_ok());
            assert!(
                !result.expect("verify_authorization_proof result in test"),
                "Empty signature should fail verification"
            );
        }

        #[test]
        fn test_verify_authorization_proof_expired() {
            let verifier = PlaceholderProofVerifier::new();
            let proof = AuthorizationProof {
                authorization_id: "auth-123".to_string(),
                operation: create_test_operation(),
                timestamp: Utc::now() - Duration::hours(2),
                proof_signature: "signature".to_string(),
            };

            let result = verifier.verify_authorization_proof(&proof);
            assert!(result.is_ok());
            assert!(
                !result.expect("verify_authorization_proof result in test"),
                "Expired proof (>1 hour) should fail verification"
            );
        }

        #[test]
        fn test_generate_proof_success() {
            let verifier = PlaceholderProofVerifier::new();
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let result = verifier.generate_proof(&authorization, &operation);
            assert!(result.is_ok(), "Proof generation should succeed");

            let proof = result.expect("generate_proof in test");
            assert_eq!(proof.authorization_id, authorization.request_id);
            assert!(
                !proof.proof_signature.is_empty(),
                "Generated proof should have signature"
            );
        }

        #[test]
        fn test_generate_proof_signature_format() {
            let verifier = PlaceholderProofVerifier::new();
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let result = verifier.generate_proof(&authorization, &operation);
            let proof = result.expect("generate_proof in test");

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
            let verifier = PlaceholderProofVerifier::new();
            let authorization = create_test_authorization();
            let operation = create_test_operation();

            let proof1 = verifier
                .generate_proof(&authorization, &operation)
                .expect("generate_proof in test");
            let proof2 = verifier
                .generate_proof(&authorization, &operation)
                .expect("generate_proof in test");

            assert_ne!(
                proof1.proof_signature, proof2.proof_signature,
                "Generated proofs should have unique signatures"
            );
        }

        #[test]
        fn test_placeholder_proof_verifier_construction() {
            let verifier1 = PlaceholderProofVerifier::new();
            let verifier2 = PlaceholderProofVerifier;

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

    #[test]
    fn ed25519_verify_rejects_modified_operation() {
        let verifier = Ed25519ProofVerifier::new(test_signing_key(13));
        let authorization = create_test_authorization();
        let operation = CrossNodeOperation {
            operation_type: OperationType::Write,
            target_resource: "resource-a".to_string(),
            ..create_test_operation()
        };

        let mut proof = verifier
            .generate_proof(&authorization, &operation)
            .expect("generate_proof should succeed");
        proof.operation.target_resource = "resource-b".to_string();

        let verified = verifier
            .verify_authorization_proof(&proof)
            .expect("verify_authorization_proof should succeed");
        assert!(!verified, "modified operation should fail verification");
    }
}
