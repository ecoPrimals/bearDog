// SPDX-License-Identifier: AGPL-3.0-or-later

//! JWS (JSON Web Signature) encoding for ACME requests.
//!
//! ACME requires all requests to be signed with the account key in a
//! JWS Flattened JSON Serialization format (RFC 7515).
//!
//! Uses ECDSA P-256 (ES256) — required by Let's Encrypt and all major
//! ACME providers. EdDSA is not supported by Let's Encrypt as of 2026.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use p256::ecdsa::{SigningKey, VerifyingKey, signature::Signer};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Encode bytes as base64url (no padding).
pub fn base64url(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

/// Compute the JWK thumbprint (RFC 7638) for an ECDSA P-256 public key.
///
/// Per RFC 7638, the thumbprint is computed over the lexicographically
/// sorted JSON members: `crv`, `kty`, `x`, `y`.
pub fn jwk_thumbprint(verifying_key: &VerifyingKey) -> String {
    let point = verifying_key.to_encoded_point(false);
    let x = base64url(point.x().expect("uncompressed point has x").as_slice());
    let y = base64url(point.y().expect("uncompressed point has y").as_slice());
    let jwk_json = format!(r#"{{"crv":"P-256","kty":"EC","x":"{x}","y":"{y}"}}"#);
    let hash = Sha256::digest(jwk_json.as_bytes());
    base64url(&hash)
}

/// Build the JWK representation of an ECDSA P-256 public key for ACME.
pub fn es256_jwk(verifying_key: &VerifyingKey) -> Value {
    let point = verifying_key.to_encoded_point(false);
    json!({
        "kty": "EC",
        "crv": "P-256",
        "x": base64url(point.x().expect("uncompressed point has x").as_slice()),
        "y": base64url(point.y().expect("uncompressed point has y").as_slice()),
    })
}

/// Sign a payload with the account key, producing a JWS Flattened JSON object.
///
/// Per RFC 8555 §6.2, the protected header includes:
/// - `alg`: `ES256`
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
    let verifying_key = VerifyingKey::from(signing_key);

    let protected = if let Some(kid) = kid {
        json!({
            "alg": "ES256",
            "nonce": nonce,
            "url": url,
            "kid": kid,
        })
    } else {
        json!({
            "alg": "ES256",
            "nonce": nonce,
            "url": url,
            "jwk": es256_jwk(&verifying_key),
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
    let signature: p256::ecdsa::Signature = signing_key.sign(signing_input.as_bytes());
    let signature_b64 = base64url(&signature.to_bytes());

    json!({
        "protected": protected_b64,
        "payload": payload_b64,
        "signature": signature_b64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> SigningKey {
        SigningKey::random(&mut p256::elliptic_curve::rand_core::OsRng)
    }

    #[test]
    fn jwk_thumbprint_is_deterministic() {
        let key = test_key();
        let vk = VerifyingKey::from(&key);
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
        assert_eq!(protected["alg"], "ES256");
    }

    #[test]
    fn empty_payload_produces_empty_string() {
        let key = test_key();
        let jws = sign_request(&key, "https://example.com", "n", &Value::Null, Some("kid"));
        assert_eq!(jws["payload"].as_str().unwrap(), "");
    }

    #[test]
    fn es256_jwk_has_correct_fields() {
        let key = test_key();
        let jwk = es256_jwk(&VerifyingKey::from(&key));
        assert_eq!(jwk["kty"], "EC");
        assert_eq!(jwk["crv"], "P-256");
        assert!(jwk["x"].as_str().unwrap().len() > 10);
        assert!(jwk["y"].as_str().unwrap().len() > 10);
    }
}
