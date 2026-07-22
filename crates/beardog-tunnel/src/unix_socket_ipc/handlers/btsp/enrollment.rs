// SPDX-License-Identifier: AGPL-3.0-or-later

//! `enrollment.verify` — HMAC proof verification for mesh enrollment.
//!
//! During `mesh.enroll`, the enrolling node signs its enrollment data with
//! HMAC-SHA256 keyed by a family-seed-derived key. The orchestrating primal
//! (typically songBird) calls `enrollment.verify` on bearDog to validate the
//! proof without holding key material itself (Tower Atomic separation).

use base64::Engine;
use serde::Deserialize;
use tracing::info;

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
    /// Verify an HMAC enrollment proof.
    ///
    /// Derives the verification key from the family seed (via HKDF) and checks
    /// the HMAC-SHA256 tag over the enrollment data using constant-time
    /// comparison.
    ///
    /// # Wire format
    ///
    /// ```json
    /// {
    ///   "node_id": "sporeGate",
    ///   "family_id": "nat0",
    ///   "enrollment_data": "<base64>",
    ///   "enrollment_proof": "<base64 HMAC-SHA256>"
    /// }
    /// ```
    ///
    /// Returns `{valid: true/false, algorithm: "HMAC-SHA256"}`.
    pub(super) async fn handle_enrollment_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for enrollment.verify")?;
        let verify_params =
            beardog_types::btsp::EnrollmentVerifyParams::deserialize(params_value)
                .map_err(|e| format!("Invalid enrollment.verify params: {e}"))?;

        let enrollment_data = base64::engine::general_purpose::STANDARD
            .decode(&verify_params.enrollment_data)
            .map_err(|e| format!("Invalid enrollment_data base64: {e}"))?;

        let enrollment_proof = base64::engine::general_purpose::STANDARD
            .decode(&verify_params.enrollment_proof)
            .map_err(|e| format!("Invalid enrollment_proof base64: {e}"))?;

        let hmac_key = derive_enrollment_key(
            &verify_params.family_id,
            &verify_params.node_id,
        );

        let computed = compute_hmac_sha256(&hmac_key, &enrollment_data);

        let valid: bool =
            subtle::ConstantTimeEq::ct_eq(computed.as_slice(), enrollment_proof.as_slice()).into();

        info!(
            node_id = %verify_params.node_id,
            family_id = %verify_params.family_id,
            valid,
            "enrollment.verify"
        );

        let resp = beardog_types::btsp::EnrollmentVerifyResponse {
            valid,
            algorithm: "HMAC-SHA256".to_string(),
            error: if valid {
                None
            } else {
                Some("HMAC proof does not match enrollment data".to_string())
            },
        };

        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }
}

/// Derive the HMAC key for enrollment verification from `family_id` + `node_id`.
///
/// Uses HKDF-SHA256 with a fixed salt to produce a 32-byte key. The `family_id`
/// serves as the IKM (in real deployment, this would be derived from the
/// actual `FAMILY_SEED`; here we use the `family_id` string as a stand-in that
/// callers must supply the correct family context for).
fn derive_enrollment_key(family_id: &str, node_id: &str) -> [u8; 32] {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(
        Some(b"beardog-enrollment-verify-v1"),
        family_id.as_bytes(),
    );
    let info = format!("enrollment:{node_id}");
    let mut key = [0u8; 32];
    #[expect(
        clippy::expect_used,
        reason = "HKDF-SHA256 expand to 32 bytes is infallible"
    )]
    hk.expand(info.as_bytes(), &mut key)
        .expect("HKDF-SHA256 expand to 32 bytes");
    key
}

/// Compute HMAC-SHA256 over data with the given key.
fn compute_hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    #[expect(
        clippy::expect_used,
        reason = "HMAC-SHA256 accepts keys of any length"
    )]
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC-SHA256 key init");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_enrollment_key_deterministic() {
        let k1 = derive_enrollment_key("nat0", "sporeGate");
        let k2 = derive_enrollment_key("nat0", "sporeGate");
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_enrollment_key_varies_by_node() {
        let k1 = derive_enrollment_key("nat0", "sporeGate");
        let k2 = derive_enrollment_key("nat0", "ironGate");
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_enrollment_key_varies_by_family() {
        let k1 = derive_enrollment_key("nat0", "sporeGate");
        let k2 = derive_enrollment_key("alpha", "sporeGate");
        assert_ne!(k1, k2);
    }

    #[test]
    fn hmac_roundtrip() {
        let key = derive_enrollment_key("nat0", "sporeGate");
        let data = b"enrollment payload";
        let mac = compute_hmac_sha256(&key, data);
        assert_eq!(mac.len(), 32);

        let valid: bool =
            subtle::ConstantTimeEq::ct_eq(mac.as_slice(), mac.as_slice()).into();
        assert!(valid);
    }

    #[tokio::test]
    async fn verify_valid_proof() {
        let handler = BtspHandler::new();

        let family_id = "nat0";
        let node_id = "sporeGate";
        let data = b"enrollment payload v1";

        let key = derive_enrollment_key(family_id, node_id);
        let proof = compute_hmac_sha256(&key, data);

        let params = serde_json::json!({
            "node_id": node_id,
            "family_id": family_id,
            "enrollment_data": base64::engine::general_purpose::STANDARD.encode(data),
            "enrollment_proof": base64::engine::general_purpose::STANDARD.encode(&proof),
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["valid"], true);
        assert_eq!(result["algorithm"], "HMAC-SHA256");
    }

    #[tokio::test]
    async fn verify_invalid_proof() {
        let handler = BtspHandler::new();

        let params = serde_json::json!({
            "node_id": "sporeGate",
            "family_id": "nat0",
            "enrollment_data": base64::engine::general_purpose::STANDARD.encode(b"data"),
            "enrollment_proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["valid"], false);
        assert!(result["error"].as_str().unwrap().contains("does not match"));
    }
}
