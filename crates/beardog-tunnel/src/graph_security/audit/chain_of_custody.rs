// SPDX-License-Identifier: AGPL-3.0-or-later

//! Chain-of-custody verification for template lineage.
//!
//! Verifies Ed25519 signatures on each lineage version to ensure the
//! modification chain has not been tampered with.

use crate::graph_security::types::{LineageVersion, VerificationStatus};
use beardog_errors::BearDogError;
use ed25519_dalek::{Signature as DalekSignature, Verifier, VerifyingKey};

/// Result of chain-of-custody verification.
#[derive(Debug)]
pub(super) struct ChainVerification {
    pub chain_valid: bool,
    pub verification_status: VerificationStatus,
}

/// Verify chain of custody for lineage.
///
/// # Current Limitations
///
/// Full verification requires public keys for all signers, retrieved via
/// `CollaborationService`. When public keys are unavailable, verification returns
/// `Unverified` with `chain_valid: false` rather than treating format checks as success.
///
/// # Errors
///
/// Returns an error if signature data has invalid encoding or serialization fails.
pub(super) async fn verify_chain_of_custody(
    lineage: &[LineageVersion],
) -> Result<ChainVerification, BearDogError> {
    use base64::Engine;

    if lineage.is_empty() {
        return Ok(ChainVerification {
            chain_valid: false,
            verification_status: VerificationStatus::Failed,
        });
    }

    let mut verification_status = VerificationStatus::Verified;

    for (idx, version) in lineage.iter().enumerate() {
        if let Some(signature_b64) = &version.signature {
            let signature = base64::engine::general_purpose::STANDARD
                .decode(signature_b64)
                .map_err(|e| {
                    BearDogError::validation(&format!(
                        "Invalid base64 signature in lineage version {}: {e}",
                        version.version
                    ))
                })?;

            if signature.len() != 64 {
                tracing::warn!(
                    "Lineage version {} has invalid signature length: {} bytes (expected 64)",
                    version.version,
                    signature.len()
                );
                return Ok(ChainVerification {
                    chain_valid: false,
                    verification_status: VerificationStatus::Failed,
                });
            }

            // Public key lookup pending CollaborationService integration.
            let public_key_b64: Option<String> = None;

            let Some(public_key_b64) = public_key_b64 else {
                tracing::debug!(
                    "Lineage version {} signature present but public key unavailable — verification cannot succeed",
                    version.version
                );
                return Ok(ChainVerification {
                    chain_valid: false,
                    verification_status: VerificationStatus::Unverified,
                });
            };

            let mut canonical_version = version.clone();
            canonical_version.signature = None;
            let canonical_json = serde_json::to_vec(&canonical_version).map_err(|e| {
                BearDogError::validation(&format!(
                    "Failed to serialize lineage version {} for signature verification: {e}",
                    version.version
                ))
            })?;

            let public_key_bytes = base64::engine::general_purpose::STANDARD
                .decode(public_key_b64)
                .map_err(|e| {
                    BearDogError::validation(&format!(
                        "Invalid base64 public key for lineage version {}: {e}",
                        version.version
                    ))
                })?;

            if public_key_bytes.len() != 32 {
                tracing::warn!(
                    "Lineage version {} has invalid public key length: {} bytes (expected 32)",
                    version.version,
                    public_key_bytes.len()
                );
                return Ok(ChainVerification {
                    chain_valid: false,
                    verification_status: VerificationStatus::Failed,
                });
            }

            let verifying_key =
                VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into().map_err(|_| {
                    BearDogError::validation(&format!(
                        "Invalid public key format for lineage version {}",
                        version.version
                    ))
                })?)
                .map_err(|e| {
                    BearDogError::validation(&format!(
                        "Invalid Ed25519 public key for lineage version {}: {e}",
                        version.version
                    ))
                })?;

            let sig =
                DalekSignature::from_bytes(signature.as_slice().try_into().map_err(|_| {
                    BearDogError::validation(&format!(
                        "Invalid signature format for lineage version {}",
                        version.version
                    ))
                })?);

            if let Err(e) = verifying_key.verify(&canonical_json, &sig) {
                tracing::warn!(
                    "Lineage version {} has invalid Ed25519 signature: {e}",
                    version.version
                );
                return Ok(ChainVerification {
                    chain_valid: false,
                    verification_status: VerificationStatus::Failed,
                });
            }

            tracing::debug!(
                "Lineage version {} Ed25519 signature verified successfully",
                version.version
            );
        } else if idx > 0 {
            tracing::warn!(
                "Lineage version {} is not signed (chain of custody incomplete)",
                version.version
            );
            verification_status = VerificationStatus::Unverified;
        }
    }

    let chain_valid = verification_status == VerificationStatus::Verified;

    Ok(ChainVerification {
        chain_valid,
        verification_status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[tokio::test]
    async fn empty_lineage_is_failed() {
        let result = verify_chain_of_custody(&[]).await.expect("query");
        assert!(!result.chain_valid);
        assert_eq!(result.verification_status, VerificationStatus::Failed);
    }

    #[tokio::test]
    async fn invalid_base64_signature_is_error() {
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some("not-valid-base64!!!".to_string()),
        }];
        let err = verify_chain_of_custody(&lineage).await.unwrap_err();
        assert!(
            format!("{err}").contains("base64") || format!("{err}").contains("Invalid"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn wrong_signature_length_is_failed() {
        let short_sig = base64::engine::general_purpose::STANDARD.encode([1u8, 2u8, 3u8]);
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some(short_sig),
        }];
        let result = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(!result.chain_valid);
        assert_eq!(result.verification_status, VerificationStatus::Failed);
    }

    #[tokio::test]
    async fn well_formed_ed25519_without_pubkey_is_unverified() {
        let sig64 = base64::engine::general_purpose::STANDARD.encode([0u8; 64]);
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some(sig64),
        }];
        let result = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(!result.chain_valid);
        assert_eq!(result.verification_status, VerificationStatus::Unverified);
    }

    #[tokio::test]
    async fn unsigned_first_version_is_verified() {
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: None,
        }];
        let result = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(result.chain_valid);
        assert_eq!(result.verification_status, VerificationStatus::Verified);
    }

    #[tokio::test]
    async fn second_version_unsigned_is_unverified() {
        let lineage = vec![
            LineageVersion {
                version: "v1".to_string(),
                created_at: None,
                modified_at: None,
                created_by: None,
                modified_by: None,
                change_type: "create".to_string(),
                changes: None,
                signature: None,
            },
            LineageVersion {
                version: "v2".to_string(),
                created_at: None,
                modified_at: None,
                created_by: None,
                modified_by: None,
                change_type: "edit".to_string(),
                changes: None,
                signature: None,
            },
        ];
        let result = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(!result.chain_valid);
        assert_eq!(result.verification_status, VerificationStatus::Unverified);
    }
}
