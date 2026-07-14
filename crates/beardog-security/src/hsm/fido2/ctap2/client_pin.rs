// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! CTAP2 ClientPIN Protocol (pinProtocol 1)
//!
//! Implements the authenticatorClientPIN command for:
//! - Getting the authenticator's key agreement public key
//! - Setting a new PIN on a fresh authenticator
//! - Getting a pinUvAuthToken for MakeCredential/GetAssertion
//!
//! Pin Protocol 1 uses: P-256 ECDH, SHA-256, AES-256-CBC (IV=0), HMAC-SHA-256.

use beardog_errors::BearDogError;
use beardog_hid::HidDevice;
use ciborium::Value as CborValue;
use hmac::{Hmac, Mac};
use p256::ecdh::EphemeralSecret;
use p256::elliptic_curve::rand_core::OsRng;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::PublicKey;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

use super::transport::{ctaphid_init, send_ctap2_command};
use super::types::Ctap2Command;

/// PIN protocol version used by this implementation.
const PIN_PROTOCOL: u64 = 1;

/// ClientPIN subcommands per CTAP2 spec.
#[repr(u8)]
enum SubCommand {
    GetRetries = 0x01,
    GetKeyAgreement = 0x02,
    SetPin = 0x03,
    ChangePin = 0x04,
    GetPinToken = 0x05,
}

/// Result of a successful PIN token retrieval.
pub struct PinToken {
    /// The decrypted pinToken from the authenticator (typically 16 or 32 bytes).
    pub token: Vec<u8>,
}

impl PinToken {
    /// Compute `pinUvAuthParam` for a given message (e.g. clientDataHash).
    /// Returns `left(HMAC-SHA-256(pinToken, message), 16)`.
    pub fn authenticate(&self, message: &[u8]) -> Vec<u8> {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.token).expect("HMAC accepts any key length");
        mac.update(message);
        let result = mac.finalize().into_bytes();
        result[..16].to_vec()
    }
}

/// Set a new PIN on a fresh authenticator (one that returns `CTAP2_ERR_PIN_NOT_SET`).
///
/// # Protocol Flow (pinProtocol 1)
///
/// 1. Get authenticator's platform key via `getKeyAgreement`
/// 2. Generate ephemeral P-256 key pair
/// 3. Compute shared secret = SHA-256(ECDH(ephemeral, authenticator_key).x)
/// 4. Pad PIN to 64 bytes, encrypt with AES-256-CBC(shared_secret, IV=0)
/// 5. Compute pinUvAuthParam = left(HMAC-SHA-256(shared_secret, newPinEnc), 16)
/// 6. Send `setPIN` command
///
/// # Errors
///
/// Returns `BearDogError` on transport failure, invalid authenticator response, or
/// if the authenticator rejects the PIN (too short, etc).
pub async fn set_pin<D: HidDevice + ?Sized>(
    device: &mut D,
    new_pin: &str,
) -> Result<(), BearDogError> {
    if new_pin.len() < 4 {
        return Err(BearDogError::validation(
            "PIN must be at least 4 characters",
        ));
    }
    if new_pin.len() > 63 {
        return Err(BearDogError::validation(
            "PIN must be at most 63 characters",
        ));
    }

    info!("Setting PIN on authenticator (pinProtocol 1)");

    let cid = ctaphid_init(device).await?;

    // Step 1: Get authenticator's key agreement public key
    let auth_pubkey = get_key_agreement(device, cid).await?;
    debug!("Got authenticator key agreement public key");

    // Step 2: Generate ephemeral key pair
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let ephemeral_public = PublicKey::from(&ephemeral_secret);

    // Step 3: Compute shared secret
    let shared_secret = compute_shared_secret(&ephemeral_secret, &auth_pubkey)?;

    // Step 4: Encrypt new PIN
    let new_pin_enc = encrypt_pin(&shared_secret, new_pin)?;

    // Step 5: Compute pinUvAuthParam
    let pin_uv_auth_param = left_hmac_sha256(&shared_secret, &new_pin_enc);

    // Step 6: Build and send setPIN command
    let platform_key_cbor = encode_public_key_cose(&ephemeral_public);

    let cmd_map = CborValue::Map(vec![
        // pinUvAuthProtocol (key 1)
        (
            CborValue::Integer(1.into()),
            CborValue::Integer(PIN_PROTOCOL.into()),
        ),
        // subCommand (key 2)
        (
            CborValue::Integer(2.into()),
            CborValue::Integer((SubCommand::SetPin as u64).into()),
        ),
        // keyAgreement (key 3) — platform's ephemeral public key in COSE format
        (CborValue::Integer(3.into()), platform_key_cbor),
        // pinUvAuthParam (key 4)
        (
            CborValue::Integer(4.into()),
            CborValue::Bytes(pin_uv_auth_param),
        ),
        // newPinEnc (key 5)
        (CborValue::Integer(5.into()), CborValue::Bytes(new_pin_enc)),
    ]);

    let mut body = Vec::new();
    ciborium::into_writer(&cmd_map, &mut body)
        .map_err(|e| BearDogError::system(format!("ClientPIN CBOR encode: {e}")))?;

    debug!("setPIN CBOR payload ({} bytes): {:02x?}", body.len(), &body);

    // send_ctap2_command returns Ok only if status == 0x00; payload is empty for setPIN.
    let _response = send_ctap2_command(device, cid, Ctap2Command::ClientPin, &body).await?;

    info!("PIN set successfully on authenticator");
    Ok(())
}

/// Get a `pinUvAuthToken` using a previously-set PIN.
///
/// # Protocol Flow (pinProtocol 1)
///
/// 1. Get authenticator's platform key via `getKeyAgreement`
/// 2. Generate ephemeral P-256 key pair
/// 3. Compute shared secret
/// 4. Compute pinHashEnc = AES-256-CBC(shared_secret, IV=0, left(SHA-256(pin), 16))
/// 5. Send `getPinToken` command
/// 6. Decrypt response to get pinToken
///
/// # Errors
///
/// Returns `BearDogError` on transport failure or incorrect PIN.
pub async fn get_pin_token<D: HidDevice + ?Sized>(
    device: &mut D,
    pin: &str,
) -> Result<PinToken, BearDogError> {
    info!("Getting pinUvAuthToken (pinProtocol 1)");

    let cid = ctaphid_init(device).await?;

    // Step 1: Get authenticator key
    let auth_pubkey = get_key_agreement(device, cid).await?;

    // Step 2: Ephemeral key pair
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let ephemeral_public = PublicKey::from(&ephemeral_secret);

    // Step 3: Shared secret
    let shared_secret = compute_shared_secret(&ephemeral_secret, &auth_pubkey)?;

    // Step 4: Encrypt PIN hash
    let pin_hash = Sha256::digest(pin.as_bytes());
    let pin_hash_left16 = &pin_hash[..16];
    let pin_hash_enc = aes256_cbc_encrypt_zero_iv(&shared_secret, pin_hash_left16)?;

    // Step 5: Build and send getPinToken command
    let platform_key_cbor = encode_public_key_cose(&ephemeral_public);

    let cmd_map = CborValue::Map(vec![
        (
            CborValue::Integer(1.into()),
            CborValue::Integer(PIN_PROTOCOL.into()),
        ),
        (
            CborValue::Integer(2.into()),
            CborValue::Integer((SubCommand::GetPinToken as u64).into()),
        ),
        (CborValue::Integer(3.into()), platform_key_cbor),
        // pinHashEnc (key 6)
        (
            CborValue::Integer(6.into()),
            CborValue::Bytes(pin_hash_enc),
        ),
    ]);

    let mut body = Vec::new();
    ciborium::into_writer(&cmd_map, &mut body)
        .map_err(|e| BearDogError::system(format!("ClientPIN CBOR encode: {e}")))?;

    debug!("getPinToken CBOR payload ({} bytes): {:02x?}", body.len(), &body);

    // send_ctap2_command returns Ok only if status == 0x00; response is raw CBOR payload
    let response = send_ctap2_command(device, cid, Ctap2Command::ClientPin, &body).await?;

    if response.is_empty() {
        return Err(BearDogError::system(
            "Empty CBOR response from getPinToken".to_string(),
        ));
    }

    // Parse CBOR response: map with key 2 = pinUvAuthToken (encrypted)
    let cbor: CborValue = ciborium::from_reader(response.as_slice())
        .map_err(|e| BearDogError::system(format!("getPinToken CBOR decode: {e}")))?;

    let encrypted_token = extract_bytes_from_map(&cbor, 2)?;

    // Decrypt pinToken with AES-256-CBC(shared_secret, IV=0, encrypted_token)
    let token = aes256_cbc_decrypt_zero_iv(&shared_secret, &encrypted_token)?;

    info!("pinUvAuthToken obtained ({} bytes)", token.len());
    Ok(PinToken { token })
}

/// Get PIN retry count from the authenticator.
///
/// # Errors
///
/// Returns `BearDogError` on transport failure or parse errors.
pub async fn get_retries<D: HidDevice + ?Sized>(
    device: &mut D,
) -> Result<u64, BearDogError> {
    let cid = ctaphid_init(device).await?;

    let cmd_map = CborValue::Map(vec![
        (
            CborValue::Integer(1.into()),
            CborValue::Integer(PIN_PROTOCOL.into()),
        ),
        (
            CborValue::Integer(2.into()),
            CborValue::Integer((SubCommand::GetRetries as u64).into()),
        ),
    ]);

    let mut body = Vec::new();
    ciborium::into_writer(&cmd_map, &mut body)
        .map_err(|e| BearDogError::system(format!("ClientPIN CBOR encode: {e}")))?;

    let response = send_ctap2_command(device, cid, Ctap2Command::ClientPin, &body).await?;

    // send_ctap2_command returns Ok only if status == 0x00; response is raw CBOR payload
    let cbor: CborValue = ciborium::from_reader(response.as_slice())
        .map_err(|e| BearDogError::system(format!("getRetries CBOR decode: {e}")))?;

    extract_integer_from_map(&cbor, 3)
}

// --- Internal helpers ---

/// Get the authenticator's key agreement public key (COSE_Key).
async fn get_key_agreement<D: HidDevice + ?Sized>(
    device: &mut D,
    cid: u32,
) -> Result<PublicKey, BearDogError> {
    let cmd_map = CborValue::Map(vec![
        (
            CborValue::Integer(1.into()),
            CborValue::Integer(PIN_PROTOCOL.into()),
        ),
        (
            CborValue::Integer(2.into()),
            CborValue::Integer((SubCommand::GetKeyAgreement as u64).into()),
        ),
    ]);

    let mut body = Vec::new();
    ciborium::into_writer(&cmd_map, &mut body)
        .map_err(|e| BearDogError::system(format!("ClientPIN CBOR encode: {e}")))?;

    let response = send_ctap2_command(device, cid, Ctap2Command::ClientPin, &body).await?;

    // send_ctap2_command already checks status; response is raw CBOR payload
    let cbor: CborValue = ciborium::from_reader(response.as_slice())
        .map_err(|e| BearDogError::system(format!("getKeyAgreement CBOR decode: {e}")))?;

    parse_cose_p256_pubkey(&cbor)
}

/// Compute the pinProtocol 1 shared secret: SHA-256(ECDH(a, B).x).
fn compute_shared_secret(
    ephemeral: &EphemeralSecret,
    authenticator_key: &PublicKey,
) -> Result<[u8; 32], BearDogError> {
    let shared_point = ephemeral.diffie_hellman(authenticator_key);
    let x_bytes = shared_point.raw_secret_bytes();
    let hash = Sha256::digest(x_bytes);
    Ok(hash.into())
}

/// Encrypt a PIN for `setPIN`: pad to 64 bytes, AES-256-CBC(key, IV=0, padded).
fn encrypt_pin(shared_secret: &[u8; 32], pin: &str) -> Result<Vec<u8>, BearDogError> {
    let mut padded = [0u8; 64];
    let pin_bytes = pin.as_bytes();
    padded[..pin_bytes.len()].copy_from_slice(pin_bytes);
    aes256_cbc_encrypt_zero_iv(shared_secret, &padded)
}

/// left(HMAC-SHA-256(key, message), 16)
fn left_hmac_sha256(key: &[u8; 32], message: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(message);
    let result = mac.finalize().into_bytes();
    result[..16].to_vec()
}

/// AES-256-CBC encryption with zero IV (pinProtocol 1).
/// Input must be a multiple of 16 bytes (no PKCS7 padding for CTAP2).
fn aes256_cbc_encrypt_zero_iv(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockEncrypt, KeyInit};

    if plaintext.len() % 16 != 0 {
        return Err(BearDogError::system(
            "AES-CBC plaintext must be a multiple of 16 bytes".to_string(),
        ));
    }

    let cipher = Aes256::new(key.into());
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut prev_block = [0u8; 16]; // IV = 0

    for chunk in plaintext.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ prev_block[i];
        }
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.encrypt_block(block_ref);
        prev_block = block;
        ciphertext.extend_from_slice(&block);
    }

    Ok(ciphertext)
}

/// AES-256-CBC decryption with zero IV (pinProtocol 1).
fn aes256_cbc_decrypt_zero_iv(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockDecrypt, KeyInit};

    if ciphertext.len() % 16 != 0 {
        return Err(BearDogError::system(
            "AES-CBC ciphertext must be a multiple of 16 bytes".to_string(),
        ));
    }

    let cipher = Aes256::new(key.into());
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    let mut prev_block = [0u8; 16]; // IV = 0

    for chunk in ciphertext.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.decrypt_block(block_ref);
        for (i, &p) in prev_block.iter().enumerate() {
            block[i] ^= p;
        }
        plaintext.extend_from_slice(&block);
        prev_block.copy_from_slice(chunk);
    }

    Ok(plaintext)
}

/// Encode a P-256 public key as a COSE_Key for ClientPIN keyAgreement.
/// Solo 2 requires `alg` = -25 (ECDH-ES+HKDF-256) for keyAgreement keys.
/// Keys in canonical CBOR order: positive ascending (1, 3), negative ascending (-1, -2, -3).
fn encode_public_key_cose(key: &PublicKey) -> CborValue {
    let point = key.to_encoded_point(false); // uncompressed
    let x = point.x().expect("valid P-256 point has x coordinate");
    let y = point.y().expect("valid P-256 point has y coordinate");

    CborValue::Map(vec![
        // kty: EC2 (2) — COSE key label 1
        (CborValue::Integer(1.into()), CborValue::Integer(2.into())),
        // alg: ECDH-ES+HKDF-256 (-25) — COSE key label 3
        (
            CborValue::Integer(3.into()),
            CborValue::Integer((-25_i64).into()),
        ),
        // crv: P-256 (1) — COSE key label -1
        (
            CborValue::Integer((-1_i64).into()),
            CborValue::Integer(1.into()),
        ),
        // x coordinate — COSE key label -2
        (
            CborValue::Integer((-2_i64).into()),
            CborValue::Bytes(x.to_vec()),
        ),
        // y coordinate — COSE key label -3
        (
            CborValue::Integer((-3_i64).into()),
            CborValue::Bytes(y.to_vec()),
        ),
    ])
}

/// Parse a COSE_Key (EC2, P-256) from a CTAP2 response map (key 1 = keyAgreement).
fn parse_cose_p256_pubkey(cbor: &CborValue) -> Result<PublicKey, BearDogError> {
    let map = match cbor {
        CborValue::Map(m) => m,
        _ => {
            return Err(BearDogError::system(
                "Expected CBOR map in getKeyAgreement response".to_string(),
            ))
        }
    };

    // Find key 1 (keyAgreement)
    let cose_key = map
        .iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == 1 => Some(v),
            _ => None,
        })
        .ok_or_else(|| {
            BearDogError::system("Missing keyAgreement (key 1) in response".to_string())
        })?;

    let cose_map = match cose_key {
        CborValue::Map(m) => m,
        _ => {
            return Err(BearDogError::system(
                "keyAgreement is not a CBOR map".to_string(),
            ))
        }
    };

    // Extract x (key -2) and y (key -3)
    let x = cose_map
        .iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == -2 => match v {
                CborValue::Bytes(b) => Some(b.as_slice()),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| BearDogError::system("Missing x coordinate in COSE key".to_string()))?;

    let y = cose_map
        .iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == -3 => match v {
                CborValue::Bytes(b) => Some(b.as_slice()),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| BearDogError::system("Missing y coordinate in COSE key".to_string()))?;

    if x.len() != 32 || y.len() != 32 {
        return Err(BearDogError::system(format!(
            "Invalid P-256 coordinate lengths: x={}, y={}",
            x.len(),
            y.len()
        )));
    }

    // Reconstruct uncompressed SEC1 point: 0x04 || x || y
    let mut sec1 = Vec::with_capacity(65);
    sec1.push(0x04);
    sec1.extend_from_slice(x);
    sec1.extend_from_slice(y);

    PublicKey::from_sec1_bytes(&sec1)
        .map_err(|e| BearDogError::system(format!("Invalid P-256 public key: {e}")))
}

/// Extract a bytes value from a CBOR map by integer key.
fn extract_bytes_from_map(cbor: &CborValue, key: i128) -> Result<Vec<u8>, BearDogError> {
    let map = match cbor {
        CborValue::Map(m) => m,
        _ => return Err(BearDogError::system("Expected CBOR map".to_string())),
    };

    map.iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == key => match v {
                CborValue::Bytes(b) => Some(b.clone()),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| {
            BearDogError::system(format!("Missing bytes at key {key} in CBOR map"))
        })
}

/// Extract an integer value from a CBOR map by integer key.
fn extract_integer_from_map(cbor: &CborValue, key: i128) -> Result<u64, BearDogError> {
    let map = match cbor {
        CborValue::Map(m) => m,
        _ => return Err(BearDogError::system("Expected CBOR map".to_string())),
    };

    map.iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == key => match v {
                CborValue::Integer(n) => u64::try_from(i128::from(*n)).ok(),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| {
            BearDogError::system(format!("Missing integer at key {key} in CBOR map"))
        })
}
