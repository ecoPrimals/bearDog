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
