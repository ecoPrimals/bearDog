// SPDX-License-Identifier: AGPL-3.0-only

//! Password hashing (Argon2id, PBKDF2) and additional KDFs (bcrypt, scrypt).

use crate::unix_socket_ipc::crypto_handlers_kdf::{
    handle_bcrypt_hash, handle_bcrypt_verify, handle_scrypt,
};
use crate::unix_socket_ipc::crypto_handlers_passwords::{
    handle_argon2id_hash, handle_argon2id_verify, handle_pbkdf2_sha256,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.argon2id_hash" => {
            info!("🔒 Crypto: argon2id_hash (OWASP password hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_argon2id_hash(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.argon2id_verify" => {
            info!("🔓 Crypto: argon2id_verify (password verification)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_argon2id_verify(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.pbkdf2_sha256" => {
            info!("🔑 Crypto: pbkdf2_sha256 (legacy password derivation)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_pbkdf2_sha256(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.bcrypt_hash" => {
            info!("🔐 Crypto: bcrypt_hash (legacy password hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_bcrypt_hash(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.bcrypt_verify" => {
            info!("🔐 Crypto: bcrypt_verify (legacy password verification)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_bcrypt_verify(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.scrypt" => {
            info!("🔑 Crypto: scrypt (memory-hard KDF)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(handle_scrypt(params_ref).map_err(|e| e.to_string())?))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn route_argon2id_hash_returns_hash() {
        let params = json!({ "password": "test-password-kdf-argon2" });
        let out = route("crypto.argon2id_hash", Some(&params))
            .await
            .expect("route");
        let v = out.expect("some result");
        assert!(v.get("hash").and_then(|x| x.as_str()).is_some());
        assert_eq!(
            v.get("algorithm").and_then(|x| x.as_str()),
            Some("argon2id")
        );
    }

    #[tokio::test]
    async fn route_argon2id_verify_roundtrip() {
        let hash_params = json!({ "password": "verify-me-password" });
        let hashed = route("crypto.argon2id_hash", Some(&hash_params))
            .await
            .expect("route")
            .expect("hash result");
        let hash_str = hashed
            .get("hash")
            .and_then(|x| x.as_str())
            .expect("hash string")
            .to_string();

        let verify_params = json!({
            "password": "verify-me-password",
            "hash": hash_str,
        });
        let verified = route("crypto.argon2id_verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify result");
        assert_eq!(verified.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_pbkdf2_sha256() {
        let salt = BASE64.encode(b"salt-value-12345678");
        let params = json!({
            "password": "pbkdf2-secret",
            "salt": salt,
            "iterations": 100000,
            "output_length": 32
        });
        let out = route("crypto.pbkdf2_sha256", Some(&params))
            .await
            .expect("route")
            .expect("pbkdf2");
        assert!(out.get("derived_key").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_bcrypt_hash_and_verify() {
        let pw = BASE64.encode(b"bcrypt-test-password");
        let hash_params = json!({ "password": pw, "cost": 4 });
        let hashed = route("crypto.bcrypt_hash", Some(&hash_params))
            .await
            .expect("route")
            .expect("bcrypt hash");
        let hash_str = hashed
            .get("hash")
            .and_then(|x| x.as_str())
            .expect("bcrypt hash string")
            .to_string();

        let verify_params = json!({ "password": pw, "hash": hash_str });
        let ok = route("crypto.bcrypt_verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("bcrypt verify");
        assert_eq!(ok.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_scrypt() {
        let params = json!({
            "password": BASE64.encode(b"scrypt-pw"),
            "salt": BASE64.encode(b"salt12345678"),
            "log_n": 10,
            "r": 8,
            "p": 1,
            "key_length": 32
        });
        let out = route("crypto.scrypt", Some(&params))
            .await
            .expect("route")
            .expect("scrypt");
        assert!(out.get("derived_key").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_missing_params_errors() {
        let err = route("crypto.argon2id_hash", None)
            .await
            .expect_err("missing params");
        assert!(err.contains("Missing parameters"));
    }

    #[tokio::test]
    async fn route_unknown_method_returns_none() {
        let params = json!({ "password": "x" });
        assert!(
            route("crypto.not_a_real_method", Some(&params))
                .await
                .expect("route")
                .is_none()
        );
    }

    #[tokio::test]
    async fn route_argon2id_verify_rejects_wrong_password() {
        let hash_params = json!({ "password": "correct-horse-battery" });
        let hashed = route("crypto.argon2id_hash", Some(&hash_params))
            .await
            .expect("route")
            .expect("hash");
        let hash_str = hashed
            .get("hash")
            .and_then(|x| x.as_str())
            .expect("hash")
            .to_string();
        let verify_params = json!({
            "password": "wrong-password",
            "hash": hash_str,
        });
        let verified = route("crypto.argon2id_verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(verified.get("valid").and_then(|x| x.as_bool()), Some(false));
    }

    #[tokio::test]
    async fn route_pbkdf2_invalid_salt_base64_errors() {
        let params = json!({
            "password": "x",
            "salt": "not-valid-base64!!!",
            "iterations": 1000,
            "output_length": 16
        });
        let err = route("crypto.pbkdf2_sha256", Some(&params))
            .await
            .expect_err("invalid salt");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn route_bcrypt_verify_rejects_wrong_password() {
        let pw = BASE64.encode(b"bcrypt-route-wrong-pw");
        let hash_params = json!({ "password": pw, "cost": 4 });
        let hashed = route("crypto.bcrypt_hash", Some(&hash_params))
            .await
            .expect("route")
            .expect("bcrypt hash");
        let hash_str = hashed
            .get("hash")
            .and_then(|x| x.as_str())
            .expect("hash string")
            .to_string();
        let verify_params = json!({
            "password": BASE64.encode(b"different-password"),
            "hash": hash_str,
        });
        let out = route("crypto.bcrypt_verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(out.get("valid").and_then(|x| x.as_bool()), Some(false));
    }

    #[tokio::test]
    async fn route_scrypt_missing_required_fields_errors() {
        let params = json!({ "password": BASE64.encode(b"x") });
        let err = route("crypto.scrypt", Some(&params))
            .await
            .expect_err("scrypt params");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn route_bcrypt_hash_rejects_cost_too_low() {
        let params = json!({
            "password": BASE64.encode(b"pw"),
            "cost": 3
        });
        let err = route("crypto.bcrypt_hash", Some(&params))
            .await
            .expect_err("bcrypt cost");
        assert!(!err.is_empty());
    }
}
