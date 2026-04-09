// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.3 Key Derivation Helper Functions
//!
//! Internal helper functions for deriving TLS 1.3 handshake and application
//! traffic secrets using SHA-256 and SHA-384 per RFC 8446 Section 7.1.

use hkdf::Hkdf;
use sha2::{Digest, Sha256, Sha384};

/// RFC 8446 HKDF-Expand-Label length-prefixed prefix (`length || label_len || label || ctx_len || ctx`).
pub fn append_tls13_hkdf_label(
    hkdf_label: &mut Vec<u8>,
    label: &str,
    context: &[u8],
    length: usize,
) {
    let tls13_label = format!("tls13 {label}");
    #[expect(
        clippy::cast_possible_truncation,
        reason = "HKDF expand output length fits TLS 1.3 label encoding"
    )]
    hkdf_label.extend_from_slice(&(length as u16).to_be_bytes());
    #[expect(
        clippy::cast_possible_truncation,
        reason = "formatted tls13 label length fits u8 in RFC 8446 labels"
    )]
    hkdf_label.push(tls13_label.len() as u8);
    hkdf_label.extend_from_slice(tls13_label.as_bytes());
    #[expect(
        clippy::cast_possible_truncation,
        reason = "HKDF label context length fits u8 in RFC 8446"
    )]
    hkdf_label.push(context.len() as u8);
    hkdf_label.extend_from_slice(context);
}

/// Derive TLS 1.3 application secrets using SHA-256
///
/// Used for cipher suites 0x1301 (AES-128-GCM-SHA256) and 0x1303 (ChaCha20-Poly1305-SHA256)
pub(super) fn derive_application_secrets_sha256(
    handshake_secret: &[u8],
    transcript_hash: &[u8],
    hash_len: usize,
    key_len: usize,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), String> {
    const IV_LEN: usize = 12;

    // Helper: HKDF-Expand-Label for SHA-256
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        append_tls13_hkdf_label(&mut hkdf_label, label, context, length);

        let hkdf =
            Hkdf::<Sha256>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // RFC 8446 Section 7.1: Key Schedule for Application Keys (SHA-256)
    let empty_hash = Sha256::digest([]);
    let handshake_derived = hkdf_expand_label(handshake_secret, "derived", &empty_hash, hash_len)?;

    let zeros = vec![0u8; hash_len];
    let master_secret = Hkdf::<Sha256>::extract(Some(&handshake_derived), &zeros);

    let client_app_secret =
        hkdf_expand_label(&master_secret.0, "c ap traffic", transcript_hash, hash_len)?;
    let server_app_secret =
        hkdf_expand_label(&master_secret.0, "s ap traffic", transcript_hash, hash_len)?;

    let client_write_key = hkdf_expand_label(&client_app_secret, "key", &[], key_len)?;
    let client_write_iv = hkdf_expand_label(&client_app_secret, "iv", &[], IV_LEN)?;
    let server_write_key = hkdf_expand_label(&server_app_secret, "key", &[], key_len)?;
    let server_write_iv = hkdf_expand_label(&server_app_secret, "iv", &[], IV_LEN)?;

    Ok((
        client_app_secret,
        server_app_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ))
}

/// Derive TLS 1.3 application secrets using SHA-384
///
/// Used for cipher suite 0x1302 (AES-256-GCM-SHA384)
pub(super) fn derive_application_secrets_sha384(
    handshake_secret: &[u8],
    transcript_hash: &[u8],
    hash_len: usize,
    key_len: usize,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), String> {
    const IV_LEN: usize = 12;

    // Helper: HKDF-Expand-Label for SHA-384
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        append_tls13_hkdf_label(&mut hkdf_label, label, context, length);

        let hkdf =
            Hkdf::<Sha384>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // RFC 8446 Section 7.1: Key Schedule for Application Keys (SHA-384)
    let empty_hash = Sha384::digest([]);
    let handshake_derived = hkdf_expand_label(handshake_secret, "derived", &empty_hash, hash_len)?;

    let zeros = vec![0u8; hash_len];
    let master_secret = Hkdf::<Sha384>::extract(Some(&handshake_derived), &zeros);

    let client_app_secret =
        hkdf_expand_label(&master_secret.0, "c ap traffic", transcript_hash, hash_len)?;
    let server_app_secret =
        hkdf_expand_label(&master_secret.0, "s ap traffic", transcript_hash, hash_len)?;

    let client_write_key = hkdf_expand_label(&client_app_secret, "key", &[], key_len)?;
    let client_write_iv = hkdf_expand_label(&client_app_secret, "iv", &[], IV_LEN)?;
    let server_write_key = hkdf_expand_label(&server_app_secret, "key", &[], key_len)?;
    let server_write_iv = hkdf_expand_label(&server_app_secret, "iv", &[], IV_LEN)?;

    Ok((
        client_app_secret,
        server_app_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ))
}

/// Derive TLS 1.3 handshake secrets using SHA-256
///
/// Used for cipher suites 0x1301 (AES-128-GCM-SHA256) and 0x1303 (ChaCha20-Poly1305-SHA256)
pub(super) fn derive_handshake_secrets_sha256(
    pre_master_secret: &[u8],
    transcript_hash: &[u8],
    hash_len: usize,
    key_len: usize,
) -> Result<
    (
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
    ),
    String,
> {
    const IV_LEN: usize = 12;

    // Helper: HKDF-Expand-Label for SHA-256
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        append_tls13_hkdf_label(&mut hkdf_label, label, context, length);

        let hkdf =
            Hkdf::<Sha256>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // RFC 8446 Section 7.1: Key Schedule for Handshake Keys (SHA-256)
    let zeros = vec![0u8; hash_len];
    let early_secret = Hkdf::<Sha256>::extract(Some(&zeros), &zeros);

    let empty_hash = Sha256::digest([]);
    let early_derived = hkdf_expand_label(&early_secret.0, "derived", &empty_hash, hash_len)?;

    let handshake_secret = Hkdf::<Sha256>::extract(Some(&early_derived), pre_master_secret);

    let client_handshake_secret = hkdf_expand_label(
        &handshake_secret.0,
        "c hs traffic",
        transcript_hash,
        hash_len,
    )?;
    let server_handshake_secret = hkdf_expand_label(
        &handshake_secret.0,
        "s hs traffic",
        transcript_hash,
        hash_len,
    )?;

    let client_write_key = hkdf_expand_label(&client_handshake_secret, "key", &[], key_len)?;
    let client_write_iv = hkdf_expand_label(&client_handshake_secret, "iv", &[], IV_LEN)?;
    let server_write_key = hkdf_expand_label(&server_handshake_secret, "key", &[], key_len)?;
    let server_write_iv = hkdf_expand_label(&server_handshake_secret, "iv", &[], IV_LEN)?;

    Ok((
        handshake_secret.0.to_vec(),
        client_handshake_secret,
        server_handshake_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ))
}

/// Derive TLS 1.3 handshake secrets using SHA-384
///
/// Used for cipher suite 0x1302 (AES-256-GCM-SHA384)
pub(super) fn derive_handshake_secrets_sha384(
    pre_master_secret: &[u8],
    transcript_hash: &[u8],
    hash_len: usize,
    key_len: usize,
) -> Result<
    (
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
    ),
    String,
> {
    const IV_LEN: usize = 12;

    // Helper: HKDF-Expand-Label for SHA-384
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        append_tls13_hkdf_label(&mut hkdf_label, label, context, length);

        let hkdf =
            Hkdf::<Sha384>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // RFC 8446 Section 7.1: Key Schedule for Handshake Keys (SHA-384)
    let zeros = vec![0u8; hash_len];
    let early_secret = Hkdf::<Sha384>::extract(Some(&zeros), &zeros);

    let empty_hash = Sha384::digest([]);
    let early_derived = hkdf_expand_label(&early_secret.0, "derived", &empty_hash, hash_len)?;

    let handshake_secret = Hkdf::<Sha384>::extract(Some(&early_derived), pre_master_secret);

    let client_handshake_secret = hkdf_expand_label(
        &handshake_secret.0,
        "c hs traffic",
        transcript_hash,
        hash_len,
    )?;
    let server_handshake_secret = hkdf_expand_label(
        &handshake_secret.0,
        "s hs traffic",
        transcript_hash,
        hash_len,
    )?;

    let client_write_key = hkdf_expand_label(&client_handshake_secret, "key", &[], key_len)?;
    let client_write_iv = hkdf_expand_label(&client_handshake_secret, "iv", &[], IV_LEN)?;
    let server_write_key = hkdf_expand_label(&server_handshake_secret, "key", &[], key_len)?;
    let server_write_iv = hkdf_expand_label(&server_handshake_secret, "iv", &[], IV_LEN)?;

    Ok((
        handshake_secret.0.to_vec(),
        client_handshake_secret,
        server_handshake_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ))
}
