// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Shared imports and re-exports for Phase 6 comprehensive crypto integration tests.

pub use base64::Engine;
pub use base64::engine::general_purpose::STANDARD as BASE64;
pub use serde_json::json;

pub use beardog_tunnel::unix_socket_ipc::crypto_handlers_aes_gcm::{
    handle_aes128_gcm_encrypt, handle_aes256_gcm_decrypt, handle_aes256_gcm_encrypt,
};
pub use beardog_tunnel::unix_socket_ipc::crypto_handlers_ecdh::{
    handle_ecdh_p256_derive, handle_ecdh_p256_generate, handle_ecdh_p384_generate,
};
pub use beardog_tunnel::unix_socket_ipc::crypto_handlers_hashing::{
    handle_sha256, handle_sha384, handle_sha512,
};
pub use beardog_tunnel::unix_socket_ipc::crypto_handlers_passwords::{
    handle_argon2id_hash, handle_argon2id_verify, handle_pbkdf2_sha256,
};
