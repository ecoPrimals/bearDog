// SPDX-License-Identifier: AGPL-3.0-or-later

//! JWS (JSON Web Signature) encoding for ACME requests.
//!
//! ACME requires all requests to be signed with the account key in a
//! JWS Flattened JSON Serialization format (RFC 7515).

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::{SigningKey, Verifier, VerifyingKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Encode bytes as base64url (no padding).
pub fn base64url(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

/// Compute the JWK thumbprint (RFC 7638) for an Ed25519 public key.
///
/// Used as the account key identifier in ACME.
pub fn jwk_thumbprint(verifying_key: &VerifyingKey) -> String {
    let jwk_json = format!(
        r#"{{"crv":"Ed25519","kty":"OKP","x":"{}"}}"#,
        base64url(verifying_key.as_bytes())
    );
    let hash = Sha256::digest(jwk_json.as_bytes());
    base64url(&hash)
}

/// Build the JWK representation of an Ed25519 public key for ACME.
pub fn ed25519_jwk(verifying_key: &VerifyingKey) -> Value {
    json!({
        "kty": "OKP",
        "crv": "Ed25519",
        "x": base64url(verifying_key.as_bytes()),
    })
}

/// Sign a payload with the account key, producing a JWS Flattened JSON object.
///
/// Per RFC 8555 §6.2, the protected header includes:
/// - `alg`: `EdDSA`
/// - `nonce`: replay-protection nonce from server
/// - `url`: the request URL
/// - `kid` or `jwk`: account identifier
#[expect(
    clippy::expect_used,
    reason = "serde_json::to_string on Value is infallible"
)]
pub fn sign_request(
    signing_key: &SigningKey,
    url: &str,
    nonce: &str,
    payload: &Value,
    kid: Option<&str>,
) -> Value {
    use ed25519_dalek::Signer;

    let verifying_key = signing_key.verifying_key();

    let protected = if let Some(kid) = kid {
        json!({
            "alg": "EdDSA",
            "nonce": nonce,
            "url": url,
            "kid": kid,
        })
    } else {
        json!({
            "alg": "EdDSA",
            "nonce": nonce,
            "url": url,
            "jwk": ed25519_jwk(&verifying_key),
        })
    };

    let protected_b64 = base64url(
        serde_json::to_string(&protected)
            .expect("Value→JSON is infallible")
            .as_bytes(),
    );

    let payload_b64 = if payload.is_null() {
        String::new()
    } else {
        base64url(
            serde_json::to_string(payload)
                .expect("Value→JSON is infallible")
                .as_bytes(),
        )
    };

    let signing_input = format!("{protected_b64}.{payload_b64}");
    let signature = signing_key.sign(signing_input.as_bytes());
    let signature_b64 = base64url(&signature.to_bytes());

    // Verify our own signature as defense-in-depth
    debug_assert!(
        verifying_key
            .verify(signing_input.as_bytes(), &signature)
            .is_ok(),
        "self-verification of JWS signature failed"
    );

    json!({
        "protected": protected_b64,
        "payload": payload_b64,
        "signature": signature_b64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    fn test_key() -> SigningKey {
        let secret: [u8; 32] = rand::random();
        SigningKey::from_bytes(&secret)
    }

    #[test]
    fn jwk_thumbprint_is_deterministic() {
        let key = test_key();
        let vk = key.verifying_key();
        let t1 = jwk_thumbprint(&vk);
        let t2 = jwk_thumbprint(&vk);
        assert_eq!(t1, t2);
        assert!(!t1.is_empty());
    }

    #[test]
    fn sign_request_produces_valid_jws_structure() {
        let key = test_key();
        let payload = json!({"foo": "bar"});
        let jws = sign_request(&key, "https://example.com/acme", "nonce123", &payload, None);

        assert!(jws.get("protected").is_some());
        assert!(jws.get("payload").is_some());
        assert!(jws.get("signature").is_some());
    }

    #[test]
    fn sign_request_with_kid_omits_jwk() {
        let key = test_key();
        let payload = json!({});
        let jws = sign_request(
            &key,
            "https://example.com/order",
            "n1",
            &payload,
            Some("https://example.com/acct/1"),
        );

        let protected_b64 = jws["protected"].as_str().unwrap();
        let protected_bytes = URL_SAFE_NO_PAD.decode(protected_b64).unwrap();
        let protected: Value = serde_json::from_slice(&protected_bytes).unwrap();

        assert_eq!(
            protected["kid"].as_str().unwrap(),
            "https://example.com/acct/1"
        );
        assert!(protected.get("jwk").is_none());
    }

    #[test]
    fn empty_payload_produces_empty_string() {
        let key = test_key();
        let jws = sign_request(&key, "https://example.com", "n", &Value::Null, Some("kid"));
        assert_eq!(jws["payload"].as_str().unwrap(), "");
    }

    #[test]
    fn ed25519_jwk_has_correct_fields() {
        let key = test_key();
        let jwk = ed25519_jwk(&key.verifying_key());
        assert_eq!(jwk["kty"], "OKP");
        assert_eq!(jwk["crv"], "Ed25519");
        assert!(jwk["x"].as_str().unwrap().len() > 10);
    }
}
