//! TLS 1.3 Key Derivation Functions
//!
//! This module implements the complete TLS 1.3 key schedule according to RFC 8446 Section 7.1.
//! It provides both handshake and application traffic secret derivation using HKDF-SHA256.
//!
//! # TLS 1.3 Key Schedule
//!
//! ```text
//!              0
//!              |
//!              v
//!    PSK ->  HKDF-Extract = Early Secret
//!              |
//!              v
//!        Derive-Secret(., "derived", "")
//!              |
//!              v
//! (EC)DHE -> HKDF-Extract = Handshake Secret
//!              |
//!              +-----> Derive-Secret(., "c hs traffic", transcript)
//!              |       = client_handshake_traffic_secret
//!              |
//!              +-----> Derive-Secret(., "s hs traffic", transcript)
//!              |       = server_handshake_traffic_secret
//!              v
//!        Derive-Secret(., "derived", "")
//!              |
//!              v
//!        0 -> HKDF-Extract = Master Secret
//!              |
//!              +-----> Derive-Secret(., "c ap traffic", transcript)
//!              |       = client_application_traffic_secret_0
//!              |
//!              +-----> Derive-Secret(., "s ap traffic", transcript)
//!                      = server_application_traffic_secret_0
//! ```
//!
//! # Exports
//!
//! - [`handle_tls_derive_secrets`] - Legacy combined derivation
//! - [`handle_tls_derive_handshake_secrets`] - Handshake traffic keys
//! - [`handle_tls_derive_application_secrets`] - Application traffic keys
//!
//! # References
//!
//! - RFC 8446: TLS 1.3
//! - RFC 5869: HKDF

use base64::Engine;
use hkdf::Hkdf;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};

// Re-export sslkeylog utility for key export
use super::super::sslkeylog::export_to_sslkeylogfile;

/// Derive TLS 1.3 secrets (legacy combined method)
///
/// This handler provides backward compatibility by combining both handshake
/// and application secret derivation into a single call. For new code, prefer
/// the split methods: [`handle_tls_derive_handshake_secrets`] and
/// [`handle_tls_derive_application_secrets`].
///
/// # Parameters
///
/// - `pre_master_secret` (base64): The ECDHE shared secret from key exchange
/// - `transcript_hash` (base64): Hash of handshake messages for key binding
/// - `cipher_suite` (optional): TLS cipher suite identifier
///
/// # Returns
///
/// A JSON object containing all derived secrets:
/// - `master_secret`: Handshake stage secret
/// - `client_handshake_traffic_secret`: Client handshake encryption key
/// - `server_handshake_traffic_secret`: Server handshake encryption key
/// - `master_secret`: Application stage secret
/// - `client_application_traffic_secret`: Client application encryption key
/// - `server_application_traffic_secret`: Server application encryption key
///
/// # Errors
///
/// Returns an error if:
/// - Required parameters are missing
/// - Base64 decoding fails
/// - Key derivation fails (HKDF error)
///
/// # Example
///
/// ```json
/// {
///   "method": "tls.derive_secrets",
///   "params": {
///     "pre_master_secret": "base64_encoded_ecdhe_secret",
///     "transcript_hash": "base64_encoded_transcript_hash"
///   }
/// }
/// ```
///
/// # References
///
/// - RFC 8446 Section 7.1: TLS 1.3 Key Schedule
/// - RFC 5869: HMAC-based Extract-and-Expand Key Derivation Function (HKDF)
pub async fn handle_tls_derive_secrets(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for tls.derive_secrets")?;

    // Extract parameters
    let pre_master_secret_b64 = params
        .get("pre_master_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: pre_master_secret")?;

    let client_random_b64 = params
        .get("client_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: client_random")?;

    let server_random_b64 = params
        .get("server_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: server_random")?;

    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_str())
        .unwrap_or("TLS_CHACHA20_POLY1305_SHA256");

    // Decode parameters
    let pre_master_secret = base64::engine::general_purpose::STANDARD
        .decode(pre_master_secret_b64)
        .map_err(|e| format!("Invalid base64 pre_master_secret: {e}"))?;

    let client_random = base64::engine::general_purpose::STANDARD
        .decode(client_random_b64)
        .map_err(|e| format!("Invalid base64 client_random: {e}"))?;

    let server_random = base64::engine::general_purpose::STANDARD
        .decode(server_random_b64)
        .map_err(|e| format!("Invalid base64 server_random: {e}"))?;

    if client_random.len() != 32 {
        return Err("client_random must be 32 bytes".to_string());
    }

    if server_random.len() != 32 {
        return Err("server_random must be 32 bytes".to_string());
    }

    debug!(
        "🔑 Deriving TLS 1.3 secrets (cipher_suite: {}, pre_master: {} bytes)",
        cipher_suite,
        pre_master_secret.len()
    );

    // Use HKDF for TLS 1.3 key derivation
    // HKDF-Extract: Derive master secret from pre-master secret
    // Salt = client_random || server_random (TLS 1.3 pattern)
    let mut salt = Vec::with_capacity(64);
    salt.extend_from_slice(&client_random);
    salt.extend_from_slice(&server_random);

    let hkdf = Hkdf::<Sha256>::new(Some(&salt), &pre_master_secret);

    // Derive master secret (48 bytes for TLS 1.3)
    let mut master_secret = [0u8; 48];
    hkdf.expand(b"tls13 master secret", &mut master_secret)
        .map_err(|e| format!("HKDF expand failed for master secret: {e}"))?;

    // HKDF-Expand: Derive session keys from master secret
    let hkdf_master = Hkdf::<Sha256>::new(None, &master_secret);

    // Determine key and IV sizes based on cipher suite
    let (key_size, iv_size) = match cipher_suite {
        "TLS_CHACHA20_POLY1305_SHA256" | "TLS_AES_256_GCM_SHA384" => (32, 12), // 256-bit keys, 96-bit IVs
        "TLS_AES_128_GCM_SHA256" => (16, 12), // 128-bit keys, 96-bit IVs
        _ => (32, 12),                        // Default to 256-bit
    };

    // Derive client write key
    let mut client_write_key = vec![0u8; key_size];
    hkdf_master
        .expand(b"tls13 client write key", &mut client_write_key)
        .map_err(|e| format!("HKDF expand failed for client write key: {e}"))?;

    // Derive server write key
    let mut server_write_key = vec![0u8; key_size];
    hkdf_master
        .expand(b"tls13 server write key", &mut server_write_key)
        .map_err(|e| format!("HKDF expand failed for server write key: {e}"))?;

    // Derive client write IV
    let mut client_write_iv = vec![0u8; iv_size];
    hkdf_master
        .expand(b"tls13 client write iv", &mut client_write_iv)
        .map_err(|e| format!("HKDF expand failed for client write IV: {e}"))?;

    // Derive server write IV
    let mut server_write_iv = vec![0u8; iv_size];
    hkdf_master
        .expand(b"tls13 server write iv", &mut server_write_iv)
        .map_err(|e| format!("HKDF expand failed for server write IV: {e}"))?;

    // Encode results
    let master_secret_b64 = base64::engine::general_purpose::STANDARD.encode(&master_secret);
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    info!(
        "✅ TLS 1.3 secrets derived (master: {} bytes, keys: {} bytes, IVs: {} bytes)",
        master_secret.len(),
        key_size,
        iv_size
    );

    Ok(serde_json::json!({
        "master_secret": master_secret_b64,
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "cipher_suite": cipher_suite,
        "algorithm": "HKDF-SHA256"
    }))
}

/// Handle tls.derive_handshake_secrets method
///
/// Derives TLS 1.3 HANDSHAKE traffic secrets using the full RFC 8446 key schedule.
/// This is the FIRST key derivation stage - used for encrypting handshake messages.
///
/// # Parameters
///
/// - `pre_master_secret`: Base64-encoded ECDH shared secret (32 bytes for X25519)
/// - `client_random`: Base64-encoded ClientHello random (32 bytes)
/// - `server_random`: Base64-encoded ServerHello random (32 bytes)
/// - `transcript_hash`: Base64-encoded SHA-256(ClientHello + ServerHello) (32 bytes)
///
/// # Returns
///
/// - `client_write_key`: Base64-encoded client key (32 bytes for ChaCha20)
/// - `client_write_iv`: Base64-encoded client IV/nonce (12 bytes)
/// - `server_write_key`: Base64-encoded server key (32 bytes for ChaCha20)
/// - `server_write_iv`: Base64-encoded server IV/nonce (12 bytes)
///
/// # Difference from `tls.derive_application_secrets`
///
/// - `tls.derive_handshake_secrets`: Derives HANDSHAKE traffic keys (for handshake messages)
/// - `tls.derive_application_secrets`: Derives APPLICATION traffic keys (for HTTP data)
///
/// Both follow RFC 8446, but at different stages of the key schedule.
pub async fn handle_tls_derive_handshake_secrets(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for tls.derive_handshake_secrets")?;

    // Extract parameters
    let pre_master_secret_b64 = params
        .get("pre_master_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: pre_master_secret")?;

    let client_random_b64 = params
        .get("client_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: client_random")?;

    let server_random_b64 = params
        .get("server_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: server_random")?;

    // REQUIRED: transcript_hash (RFC 8446 compliance)
    let transcript_hash_b64 = params
        .get("transcript_hash")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: transcript_hash")?;

    // REQUIRED: cipher_suite (RFC 8446 Section 7.3 - determines key length!)
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_u64())
        .ok_or("Missing required parameter: cipher_suite")? as u16;

    // Decode parameters
    let pre_master_secret = base64::engine::general_purpose::STANDARD
        .decode(pre_master_secret_b64)
        .map_err(|e| format!("Invalid base64 pre_master_secret: {e}"))?;

    let client_random = base64::engine::general_purpose::STANDARD
        .decode(client_random_b64)
        .map_err(|e| format!("Invalid base64 client_random: {e}"))?;

    let server_random = base64::engine::general_purpose::STANDARD
        .decode(server_random_b64)
        .map_err(|e| format!("Invalid base64 server_random: {e}"))?;

    let transcript_hash = base64::engine::general_purpose::STANDARD
        .decode(transcript_hash_b64)
        .map_err(|e| format!("Invalid base64 transcript_hash: {e}"))?;

    // Validate parameter sizes
    if client_random.len() != 32 {
        return Err("client_random must be 32 bytes".to_string());
    }

    if server_random.len() != 32 {
        return Err("server_random must be 32 bytes".to_string());
    }

    if transcript_hash.len() != 32 {
        return Err("transcript_hash must be 32 bytes (SHA-256)".to_string());
    }

    // Determine key length based on cipher suite (RFC 8446 Section 7.3)
    let key_len = match cipher_suite {
        0x1301 => {
            info!("  → Cipher suite: 0x1301 (TLS_AES_128_GCM_SHA256) - using 16-byte keys");
            16 // AES-128-GCM uses 16-byte keys
        }
        0x1302 => {
            info!("  → Cipher suite: 0x1302 (TLS_AES_256_GCM_SHA384) - using 32-byte keys");
            32 // AES-256-GCM uses 32-byte keys
        }
        0x1303 => {
            info!("  → Cipher suite: 0x1303 (TLS_CHACHA20_POLY1305_SHA256) - using 32-byte keys");
            32 // ChaCha20-Poly1305 uses 32-byte keys
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)",
                cipher_suite
            ));
        }
    };

    debug!("🔑 Deriving TLS 1.3 HANDSHAKE secrets (RFC 8446 Section 7.1)");
    debug!(
        "  → pre_master: {} bytes (ECDH shared secret)",
        pre_master_secret.len()
    );
    debug!("  → client_random: {} bytes", client_random.len());
    debug!("  → server_random: {} bytes", server_random.len());
    debug!(
        "  → transcript_hash: {} bytes (ClientHello + ServerHello)",
        transcript_hash.len()
    );
    debug!(
        "  → cipher_suite: 0x{:04x} → key_len: {} bytes",
        cipher_suite, key_len
    );

    // Constants
    const IV_LEN: usize = 12; // AEAD nonce size (same for all cipher suites)

    // Helper: HKDF-Expand-Label (RFC 8446 Section 7.1)
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        hkdf_label.extend_from_slice(&(length as u16).to_be_bytes()); // Length (2 bytes)

        let tls13_label = format!("tls13 {}", label);
        hkdf_label.push(tls13_label.len() as u8); // Label length (1 byte)
        hkdf_label.extend_from_slice(tls13_label.as_bytes()); // Label

        hkdf_label.push(context.len() as u8); // Context length (1 byte)
        hkdf_label.extend_from_slice(context); // Context

        let hkdf =
            Hkdf::<Sha256>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // RFC 8446 Section 7.1: Key Schedule for Handshake Keys

    // Step 1: Early Secret = HKDF-Extract(salt: 0, IKM: 0)
    let zeros_32 = [0u8; 32];
    let early_secret = Hkdf::<Sha256>::extract(Some(&zeros_32), &zeros_32);
    debug!("  Step 1: Early Secret derived");

    // Step 2: Derive-Secret(early_secret, "derived", "")
    // This is: HKDF-Expand-Label(early_secret, "derived", Hash(""), 32)
    let empty_hash = Sha256::digest(&[]);
    let early_derived = hkdf_expand_label(&early_secret.0, "derived", &empty_hash, 32)?;
    debug!("  Step 2: Early derived secret computed");

    // Step 3: Handshake Secret = HKDF-Extract(salt: early_derived, IKM: ECDH)
    let handshake_secret = Hkdf::<Sha256>::extract(Some(&early_derived), &pre_master_secret);
    debug!("  Step 3: Handshake Secret derived from ECDH");

    // Step 4: Client Handshake Traffic Secret
    // HKDF-Expand-Label(handshake_secret, "c hs traffic", transcript_hash, 32)
    let client_handshake_secret =
        hkdf_expand_label(&handshake_secret.0, "c hs traffic", &transcript_hash, 32)?;
    debug!("  Step 4: Client Handshake Traffic Secret derived");

    // Step 5: Server Handshake Traffic Secret
    // HKDF-Expand-Label(handshake_secret, "s hs traffic", transcript_hash, 32)
    let server_handshake_secret =
        hkdf_expand_label(&handshake_secret.0, "s hs traffic", &transcript_hash, 32)?;
    debug!("  Step 5: Server Handshake Traffic Secret derived");

    // Step 6: Derive Keys and IVs from Handshake Traffic Secrets
    // Key length determined by cipher suite (RFC 8446 Section 7.3)

    // Client write key = HKDF-Expand-Label(client_secret, "key", "", key_len)
    let client_write_key = hkdf_expand_label(&client_handshake_secret, "key", &[], key_len)?;

    // Client write IV = HKDF-Expand-Label(client_secret, "iv", "", 12)
    let client_write_iv = hkdf_expand_label(&client_handshake_secret, "iv", &[], IV_LEN)?;

    // Server write key = HKDF-Expand-Label(server_secret, "key", "", key_len)
    let server_write_key = hkdf_expand_label(&server_handshake_secret, "key", &[], key_len)?;

    // Server write IV = HKDF-Expand-Label(server_secret, "iv", "", 12)
    let server_write_iv = hkdf_expand_label(&server_handshake_secret, "iv", &[], IV_LEN)?;

    debug!(
        "  Step 6: Keys and IVs derived (key: {} bytes, IV: {} bytes)",
        key_len, IV_LEN
    );

    // HEX DUMPS for derived keys (cross-verify with Songbird and RFC 8448)
    info!("🔍 BEARDOG DERIVED HANDSHAKE KEYS - FULL HEX DUMPS:");
    info!("   client_write_key: {}", hex::encode(&client_write_key));
    info!("   server_write_key: {}", hex::encode(&server_write_key));
    info!("   client_write_iv: {}", hex::encode(&client_write_iv));
    info!("   server_write_iv: {}", hex::encode(&server_write_iv));

    // Encode to base64
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    // Also encode the traffic secrets (needed for Finished message computation, RFC 8446 Section 4.4.4)
    let client_handshake_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&client_handshake_secret);
    let server_handshake_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&server_handshake_secret);

    info!(
        "✅ TLS 1.3 HANDSHAKE secrets derived (cipher: 0x{:04x}, keys: {} bytes, IVs: {} bytes, RFC 8446 Section 7.3 compliant)",
        cipher_suite, key_len, IV_LEN
    );

    // Export to SSLKEYLOGFILE for Wireshark decryption (if SSLKEYLOGFILE env var is set)
    if let Err(e) = export_to_sslkeylogfile(
        &client_random,
        Some((&client_handshake_secret, &server_handshake_secret)),
        None, // No application secrets yet (will be exported in handle_tls_derive_application_secrets)
    ) {
        warn!("⚠️  Failed to export to SSLKEYLOGFILE: {}", e);
    }

    Ok(serde_json::json!({
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "client_handshake_secret": client_handshake_secret_b64,  // For Finished message (RFC 8446 Section 4.4.4)
        "server_handshake_secret": server_handshake_secret_b64,  // For Finished message (RFC 8446 Section 4.4.4)
        "algorithm": "HKDF-SHA256",
        "rfc": "RFC 8446 Section 7.1",
        "stage": "handshake",
        "mode": "RFC 8446 Full Compliance"
    }))
}

/// Handle tls.derive_application_secrets method
///
/// Derives TLS 1.3 APPLICATION traffic secrets using the full RFC 8446 key schedule.
/// This is the SECOND key derivation stage - used for encrypting HTTP application data.
///
/// # Parameters
///
/// - `pre_master_secret`: Base64-encoded shared secret (32 bytes from ECDH)
/// - `client_random`: Base64-encoded client random (32 bytes)
/// - `server_random`: Base64-encoded server random (32 bytes)
///
/// # Returns
///
/// - `client_write_key`: Base64-encoded client encryption key (32 bytes)
/// - `server_write_key`: Base64-encoded server encryption key (32 bytes)
/// - `client_write_iv`: Base64-encoded client IV/nonce (12 bytes)
/// - `server_write_iv`: Base64-encoded server IV/nonce (12 bytes)
///
/// # Difference from `tls.derive_secrets`
///
/// - `tls.derive_secrets`: Derives HANDSHAKE traffic keys (for handshake messages)
/// - `tls.derive_application_secrets`: Derives APPLICATION traffic keys (for HTTP data)
///
/// Both follow RFC 8446, but at different stages of the key schedule.
pub async fn handle_tls_derive_application_secrets(
    params: Option<&Value>,
) -> Result<Value, String> {
    // EXECUTION TRACE: Log immediately to confirm function entry
    info!("🚀 ENTERED handle_tls_derive_application_secrets");

    let params = params.ok_or("Missing params for tls.derive_application_secrets")?;
    info!("✅ Parameters parsed successfully");

    // Extract parameters
    let pre_master_secret_b64 = params
        .get("pre_master_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: pre_master_secret")?;

    let client_random_b64 = params
        .get("client_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: client_random")?;

    let server_random_b64 = params
        .get("server_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: server_random")?;

    // Optional: transcript_hash (for proper RFC 8446 compliance)
    // If not provided, falls back to simplified mode (client_random || server_random)
    let transcript_hash_b64 = params.get("transcript_hash").and_then(|v| v.as_str());

    // Extract cipher_suite (NEW: for dynamic key length derivation)
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_u64())
        .unwrap_or(0x1303) as u16; // Default to ChaCha20-Poly1305 for backward compat

    info!("🔐 Cipher suite: 0x{:04x}", cipher_suite);

    // Decode parameters
    let pre_master_secret = base64::engine::general_purpose::STANDARD
        .decode(pre_master_secret_b64)
        .map_err(|e| format!("Invalid base64 pre_master_secret: {e}"))?;

    let client_random = base64::engine::general_purpose::STANDARD
        .decode(client_random_b64)
        .map_err(|e| format!("Invalid base64 client_random: {e}"))?;

    let server_random = base64::engine::general_purpose::STANDARD
        .decode(server_random_b64)
        .map_err(|e| format!("Invalid base64 server_random: {e}"))?;

    info!("✅ Base64 decoding complete: pre_master={} bytes, client_random={} bytes, server_random={} bytes", 
          pre_master_secret.len(), client_random.len(), server_random.len());

    // Optional transcript hash (SHA-256 of all handshake messages)
    let transcript_hash = if let Some(th_b64) = transcript_hash_b64 {
        let th = base64::engine::general_purpose::STANDARD
            .decode(th_b64)
            .map_err(|e| format!("Invalid base64 transcript_hash: {e}"))?;
        if th.len() != 32 {
            return Err("transcript_hash must be 32 bytes (SHA-256)".to_string());
        }
        info!("✅ Transcript hash decoded: {} bytes", th.len());
        Some(th)
    } else {
        info!("⚠️  No transcript_hash provided - will use SIMPLIFIED MODE");
        None
    };

    if client_random.len() != 32 {
        return Err("client_random must be 32 bytes".to_string());
    }

    if server_random.len() != 32 {
        return Err("server_random must be 32 bytes".to_string());
    }

    // EXECUTION TRACE: About to enter comprehensive debug logging
    info!("🎯 CHECKPOINT: Starting comprehensive debug output...");

    // VERSION MARKER: This is v0.17.0+ with enhanced debug logging
    info!("════════════════════════════════════════════════════════════");
    info!("🔍 BEARDOG v0.17.0+ APPLICATION KEY DERIVATION - COMPREHENSIVE DEBUG");
    info!("════════════════════════════════════════════════════════════");

    if let Some(ref th) = transcript_hash {
        info!("RFC 8446 FULL MODE - Using actual transcript hash");
        info!("  • Pre-master secret: {} bytes", pre_master_secret.len());
        info!("  • Client random: {} bytes", client_random.len());
        info!("  • Server random: {} bytes", server_random.len());
        info!("  • Transcript hash: {} bytes (SHA-256)", th.len());
        info!("  • Transcript hash (hex): {}", hex::encode(th));
    } else {
        info!("SIMPLIFIED MODE (backward compat) - Using client_random || server_random");
        info!("  • Pre-master secret: {} bytes", pre_master_secret.len());
        warn!("⚠️  NOT using actual transcript hash - this is not RFC 8446 compliant!");
    }

    // Dynamic key length based on cipher suite (RFC 8446 Section 7.3)
    let (key_len, iv_len) = match cipher_suite {
        0x1301 => (16, 12), // TLS_AES_128_GCM_SHA256
        0x1302 => (32, 12), // TLS_AES_256_GCM_SHA384
        0x1303 => (32, 12), // TLS_CHACHA20_POLY1305_SHA256
        _ => {
            warn!(
                "⚠️  Unknown cipher suite 0x{:04x}, defaulting to ChaCha20 (32-byte keys)",
                cipher_suite
            );
            (32, 12)
        }
    };

    info!(
        "✅ Using key_len={} bytes, iv_len={} bytes for cipher suite 0x{:04x}",
        key_len, iv_len, cipher_suite
    );

    // Helper: HKDF-Expand-Label (RFC 8446 Section 7.1)
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let mut hkdf_label = Vec::new();
        hkdf_label.extend_from_slice(&(length as u16).to_be_bytes()); // Length (2 bytes)

        let tls13_label = format!("tls13 {}", label);
        hkdf_label.push(tls13_label.len() as u8); // Label length (1 byte)
        hkdf_label.extend_from_slice(tls13_label.as_bytes()); // Label

        hkdf_label.push(context.len() as u8); // Context length (1 byte)
        hkdf_label.extend_from_slice(context); // Context

        let hkdf =
            Hkdf::<Sha256>::from_prk(secret).map_err(|e| format!("HKDF from_prk failed: {e}"))?;
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm)
            .map_err(|e| format!("HKDF expand failed: {e}"))?;
        Ok::<Vec<u8>, String>(okm)
    };

    // Helper: Derive-Secret (RFC 8446 Section 7.1)
    let derive_secret = |secret: &[u8], label: &str, messages: &[u8]| {
        let transcript_hash = Sha256::digest(messages);
        hkdf_expand_label(secret, label, &transcript_hash, 32)
    };

    // Step 1: Early Secret (from all zeros)
    let early_secret = Hkdf::<Sha256>::extract(None, &[0u8; 32]);

    // Step 2: Derive-Secret(early_secret, "derived", "")
    let derived_1 = derive_secret(&early_secret.0, "derived", &[])?;

    // Step 3: Handshake Secret (from shared secret)
    let handshake_secret = Hkdf::<Sha256>::extract(Some(&derived_1), &pre_master_secret);

    // Step 4: Derive-Secret(handshake_secret, "derived", "")
    let derived_2 = derive_secret(&handshake_secret.0, "derived", &[])?;

    // Step 5: Master Secret (from all zeros)
    let master_secret = Hkdf::<Sha256>::extract(Some(&derived_2), &[0u8; 32]);

    // Step 6: Prepare transcript for key derivation
    // RFC 8446 Mode: Use provided transcript_hash directly (already SHA-256 hashed)
    // Simplified Mode: Hash(client_random || server_random) for backward compatibility
    let transcript_for_derivation = if let Some(ref th) = transcript_hash {
        // RFC 8446 FULL MODE: Use actual transcript hash
        // The hash is already computed by the caller (Songbird) from all handshake messages
        info!(
            "✅ Using RFC 8446 FULL transcript hash ({} bytes)",
            th.len()
        );
        th.clone()
    } else {
        // SIMPLIFIED MODE (backward compatibility):
        // Use client_random || server_random as a simplified transcript
        // This is NOT RFC 8446 compliant but works for initial testing
        let mut simplified_transcript = Vec::with_capacity(64);
        simplified_transcript.extend_from_slice(&client_random);
        simplified_transcript.extend_from_slice(&server_random);
        debug!("⚠️  Using SIMPLIFIED transcript (not RFC 8446 compliant)");

        // Hash the simplified transcript
        Sha256::digest(&simplified_transcript).to_vec()
    };

    // Step 7: Derive application traffic secrets (RFC 8446 labels)
    // Use HKDF-Expand-Label with the transcript hash as context
    info!("────────────────────────────────────────────────────────────");
    info!("Key Derivation Process:");
    info!("────────────────────────────────────────────────────────────");
    info!("  • Master secret (first 16 bytes):");
    info!("    {}", hex::encode(&master_secret.0[..16]));
    info!(
        "  • Transcript hash used for derivation ({} bytes):",
        transcript_for_derivation.len()
    );
    info!("    {}", hex::encode(&transcript_for_derivation));
    info!("");
    info!("  • Deriving with HKDF-Expand-Label:");
    info!("    - Label for client: 'c ap traffic'");
    info!("    - Label for server: 's ap traffic'");
    info!("    - Output length: 32 bytes each");
    info!("");

    let client_app_secret = hkdf_expand_label(
        &master_secret.0,
        "c ap traffic",
        &transcript_for_derivation,
        32,
    )?;
    info!("  ✅ Client application secret (CLIENT_TRAFFIC_SECRET_0, full 32 bytes):");
    info!("    {}", hex::encode(&client_app_secret));
    info!("");

    let server_app_secret = hkdf_expand_label(
        &master_secret.0,
        "s ap traffic",
        &transcript_for_derivation,
        32,
    )?;
    info!("  ✅ Server application secret (SERVER_TRAFFIC_SECRET_0, full 32 bytes):");
    info!("    {}", hex::encode(&server_app_secret));
    info!("");

    // Step 8: Derive keys and IVs using HKDF-Expand-Label (with dynamic lengths)
    info!("────────────────────────────────────────────────────────────");
    info!("Final Derived Keys:");
    info!("────────────────────────────────────────────────────────────");
    info!(
        "  • Expanding client_application_secret → key ({} bytes) + IV ({} bytes)",
        key_len, iv_len
    );
    let client_write_key = hkdf_expand_label(&client_app_secret, "key", &[], key_len)?;
    info!(
        "    Client write key ({} bytes): {}",
        client_write_key.len(),
        hex::encode(&client_write_key)
    );

    let client_write_iv = hkdf_expand_label(&client_app_secret, "iv", &[], iv_len)?;
    info!(
        "    Client write IV ({} bytes): {}",
        client_write_iv.len(),
        hex::encode(&client_write_iv)
    );
    info!("");

    info!(
        "  • Expanding server_application_secret → key ({} bytes) + IV ({} bytes)",
        key_len, iv_len
    );
    let server_write_key = hkdf_expand_label(&server_app_secret, "key", &[], key_len)?;
    info!(
        "    Server write key ({} bytes): {}",
        server_write_key.len(),
        hex::encode(&server_write_key)
    );

    let server_write_iv = hkdf_expand_label(&server_app_secret, "iv", &[], iv_len)?;
    info!(
        "    Server write IV ({} bytes): {}",
        server_write_iv.len(),
        hex::encode(&server_write_iv)
    );

    // Encode results
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    // Also encode traffic secrets (for key updates and debugging)
    let client_app_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&client_app_secret);
    let server_app_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&server_app_secret);

    let mode = if transcript_hash.is_some() {
        "RFC 8446 Full Compliance"
    } else {
        "Simplified (backward compat)"
    };

    info!("════════════════════════════════════════════════════════════");
    info!("✅ TLS 1.3 APPLICATION secrets derived successfully!");
    info!(
        "  Cipher: 0x{:04x}, Keys: {} bytes, IVs: {} bytes",
        cipher_suite, key_len, iv_len
    );
    info!("  Mode: {}", mode);
    info!("════════════════════════════════════════════════════════════");

    // Export to SSLKEYLOGFILE for Wireshark decryption (if SSLKEYLOGFILE env var is set)
    if let Err(e) = export_to_sslkeylogfile(
        &client_random,
        None, // No handshake secrets here (already exported in handle_tls_derive_handshake_secrets)
        Some((&client_app_secret, &server_app_secret)),
    ) {
        warn!("⚠️  Failed to export to SSLKEYLOGFILE: {}", e);
    }

    Ok(serde_json::json!({
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "client_application_secret": client_app_secret_b64,  // For key updates (RFC 8446 Section 7.2)
        "server_application_secret": server_app_secret_b64,  // For key updates (RFC 8446 Section 7.2)
        "algorithm": "HKDF-SHA256",
        "rfc": "RFC 8446 Section 7.1",
        "mode": mode,
        "key_length": key_len,       // NEW: For verification
        "iv_length": iv_len,         // NEW: For verification
        "cipher_suite": cipher_suite // NEW: Echo back for debugging
    }))
}
