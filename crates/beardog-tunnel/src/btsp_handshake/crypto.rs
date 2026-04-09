// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP handshake cryptographic primitives.
//!
//! Key derivation follows `BTSP_PROTOCOL_STANDARD.md §Key Derivation`:
//!
//! ```text
//! handshake_key  = HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")
//! shared_secret  = X25519(our_ephemeral, their_pub)
//! session_keys   = HKDF(shared_secret, "btsp-session-v1", session_id)
//! ```

use beardog_errors::BearDogError;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use subtle::ConstantTimeEq;
use x25519_dalek::{PublicKey, StaticSecret};

type HmacSha256 = Hmac<Sha256>;

fn new_hmac(key: &[u8]) -> Result<HmacSha256, BearDogError> {
    HmacSha256::new_from_slice(key)
        .map_err(|e| BearDogError::system(format!("BTSP HMAC key error: {e}")))
}

/// Derive the handshake key from the family seed.
///
/// `HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")` → 32 bytes.
///
/// # Errors
///
/// Returns an error if HKDF expansion fails (should not happen with valid inputs).
pub fn derive_handshake_key(family_seed: &[u8]) -> Result<[u8; 32], BearDogError> {
    let hk = Hkdf::<Sha256>::new(Some(b"btsp-v1"), family_seed);
    let mut okm = [0u8; 32];
    hk.expand(b"handshake", &mut okm).map_err(|e| {
        BearDogError::system(format!("BTSP HKDF handshake key derivation failed: {e}"))
    })?;
    Ok(okm)
}

/// Compute the challenge response HMAC.
///
/// `HMAC-SHA256(key=handshake_key, data=challenge || client_pub || server_pub)`
///
/// # Errors
///
/// Returns an error if the HMAC key length is invalid.
pub fn compute_challenge_hmac(
    handshake_key: &[u8; 32],
    challenge: &[u8],
    client_pub: &[u8],
    server_pub: &[u8],
) -> Result<[u8; 32], BearDogError> {
    let mut mac = new_hmac(handshake_key)?;
    mac.update(challenge);
    mac.update(client_pub);
    mac.update(server_pub);
    let result = mac.finalize();
    Ok(result.into_bytes().into())
}

/// Verify a client's challenge response in constant time.
///
/// # Errors
///
/// Returns an error if the HMAC computation itself fails or if
/// the response does not match (family membership verification failure).
pub fn verify_challenge_response(
    handshake_key: &[u8; 32],
    challenge: &[u8],
    client_pub: &[u8],
    server_pub: &[u8],
    client_response: &[u8],
) -> Result<(), BearDogError> {
    let expected = compute_challenge_hmac(handshake_key, challenge, client_pub, server_pub)?;

    if client_response.len() != 32 || expected.ct_eq(client_response).unwrap_u8() != 1 {
        return Err(BearDogError::security(
            "BTSP handshake failed: family_verification".to_string(),
        ));
    }
    Ok(())
}

/// Generate an X25519 ephemeral keypair for forward secrecy.
pub fn generate_ephemeral_keypair() -> (StaticSecret, PublicKey) {
    let mut secret_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut secret_bytes);
    let secret = StaticSecret::from(secret_bytes);
    let public = PublicKey::from(&secret);
    (secret, public)
}

/// Compute the X25519 shared secret.
pub fn x25519_shared_secret(our_secret: &StaticSecret, their_public: &PublicKey) -> [u8; 32] {
    *our_secret.diffie_hellman(their_public).as_bytes()
}

/// Derive directional session keys from the X25519 shared secret.
///
/// ```text
/// HKDF-SHA256(ikm=shared_secret, salt="btsp-session-v1", info=session_id)
///   → 64 bytes split into (server_to_client_key[32], client_to_server_key[32])
/// ```
///
/// # Errors
///
/// Returns an error if HKDF expansion fails.
pub fn derive_session_keys(
    shared_secret: &[u8; 32],
    session_id: &[u8],
) -> Result<SessionKeys, BearDogError> {
    let hk = Hkdf::<Sha256>::new(Some(b"btsp-session-v1"), shared_secret);
    let mut okm = [0u8; 64];
    hk.expand(session_id, &mut okm)
        .map_err(|e| BearDogError::system(format!("BTSP session key derivation failed: {e}")))?;

    let mut server_to_client = [0u8; 32];
    let mut client_to_server = [0u8; 32];
    server_to_client.copy_from_slice(&okm[..32]);
    client_to_server.copy_from_slice(&okm[32..]);

    Ok(SessionKeys {
        server_to_client,
        client_to_server,
    })
}

/// Pair of directional session keys.
pub struct SessionKeys {
    /// Key for server-to-client traffic.
    pub server_to_client: [u8; 32],
    /// Key for client-to-server traffic.
    pub client_to_server: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_key_is_deterministic() {
        let seed = b"test-family-seed-32-bytes-long!!";
        let k1 = derive_handshake_key(seed).expect("derive 1");
        let k2 = derive_handshake_key(seed).expect("derive 2");
        assert_eq!(k1, k2);
    }

    #[test]
    fn different_seeds_produce_different_keys() {
        let k1 = derive_handshake_key(b"seed-alpha").expect("alpha");
        let k2 = derive_handshake_key(b"seed-bravo").expect("bravo");
        assert_ne!(k1, k2);
    }

    #[test]
    fn hmac_roundtrip_verify() {
        let hk = [0xAA; 32];
        let challenge = b"random-challenge-bytes";
        let client_pub = b"client-pub-placeholder-32-bytes!";
        let server_pub = b"server-pub-placeholder-32-bytes!";

        let mac = compute_challenge_hmac(&hk, challenge, client_pub, server_pub).expect("compute");
        verify_challenge_response(&hk, challenge, client_pub, server_pub, &mac).expect("verify");
    }

    #[test]
    fn hmac_rejects_wrong_response() {
        let hk = [0xAA; 32];
        let challenge = b"challenge";
        let client_pub = b"cpub";
        let server_pub = b"spub";
        let bad = [0xBB; 32];

        let result = verify_challenge_response(&hk, challenge, client_pub, server_pub, &bad);
        assert!(result.is_err());
    }

    #[test]
    fn x25519_shared_secret_agreement() {
        let (secret_a, pub_a) = generate_ephemeral_keypair();
        let (secret_b, pub_b) = generate_ephemeral_keypair();

        let shared_ab = x25519_shared_secret(&secret_a, &pub_b);
        let shared_ba = x25519_shared_secret(&secret_b, &pub_a);
        assert_eq!(shared_ab, shared_ba);
    }

    #[test]
    fn session_keys_are_deterministic_for_same_inputs() {
        let shared = [0x42; 32];
        let sid = b"session-id-hex";
        let k1 = derive_session_keys(&shared, sid).expect("k1");
        let k2 = derive_session_keys(&shared, sid).expect("k2");
        assert_eq!(k1.server_to_client, k2.server_to_client);
        assert_eq!(k1.client_to_server, k2.client_to_server);
    }

    #[test]
    fn session_keys_directions_differ() {
        let shared = [0x42; 32];
        let sid = b"sid";
        let keys = derive_session_keys(&shared, sid).expect("keys");
        assert_ne!(keys.server_to_client, keys.client_to_server);
    }

    #[test]
    fn verify_challenge_rejects_non_32_byte_response() {
        let hk = [0x55; 32];
        let challenge = b"chal";
        let cp = [0u8; 32];
        let sp = [0u8; 32];
        let short = [0u8; 31];
        let err = verify_challenge_response(&hk, challenge, &cp, &sp, &short).expect_err("short");
        assert!(err.to_string().contains("family") || err.to_string().contains("verification"));
    }
}
