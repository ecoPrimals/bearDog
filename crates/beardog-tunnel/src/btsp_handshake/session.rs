// SPDX-License-Identifier: AGPL-3.0-or-later

//! Per-connection BTSP session holding cipher state.

use beardog_errors::BearDogError;
use chacha20poly1305::{ChaCha20Poly1305, Nonce, aead::Aead};
use zeroize::Zeroize;

/// Negotiated cipher suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtspCipher {
    /// ChaCha20-Poly1305 AEAD — confidentiality + integrity + auth.
    ChaCha20Poly1305,
    /// HMAC-SHA256 tag per frame — integrity + auth, no confidentiality.
    HmacPlain,
    /// No per-frame protection — handshake-only authentication.
    Null,
}

impl BtspCipher {
    /// Wire name used in handshake negotiation.
    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::ChaCha20Poly1305 => "chacha20_poly1305",
            Self::HmacPlain => "hmac_plain",
            Self::Null => "null",
        }
    }

    /// Parse a wire name into a cipher variant.
    ///
    /// # Errors
    ///
    /// Returns an error for unrecognized cipher names.
    pub fn from_wire_name(name: &str) -> Result<Self, BearDogError> {
        match name {
            "chacha20_poly1305" | "chacha20" => Ok(Self::ChaCha20Poly1305),
            "hmac_plain" => Ok(Self::HmacPlain),
            "null" => Ok(Self::Null),
            other => Err(BearDogError::invalid_input(&format!(
                "Unknown BTSP cipher: {other}"
            ))),
        }
    }
}

/// Per-connection BTSP session established after a successful handshake.
pub struct BtspSession {
    /// Hex-encoded session ID (16 random bytes → 32 hex chars).
    pub session_id: String,
    /// Negotiated cipher suite.
    pub cipher: BtspCipher,
    /// Key for server → client direction.
    encrypt_key: [u8; 32],
    /// Key for client → server direction.
    decrypt_key: [u8; 32],
    /// Monotonic nonce counter for encrypt (server → client).
    encrypt_counter: u64,
    /// Monotonic nonce counter for decrypt (client → server).
    decrypt_counter: u64,
}

impl BtspSession {
    /// Create a new session after handshake completion (server side).
    ///
    /// `encrypt_key` is used for server → client, `decrypt_key` for client → server.
    #[must_use]
    pub fn new_server(
        session_id: String,
        cipher: BtspCipher,
        server_to_client_key: [u8; 32],
        client_to_server_key: [u8; 32],
    ) -> Self {
        Self {
            session_id,
            cipher,
            encrypt_key: server_to_client_key,
            decrypt_key: client_to_server_key,
            encrypt_counter: 0,
            decrypt_counter: 0,
        }
    }

    /// Encrypt a plaintext JSON-RPC frame for sending to the client.
    ///
    /// For `ChaCha20Poly1305`: returns `nonce(12) || ciphertext || tag(16)`.
    /// For `HmacPlain`: returns `plaintext || hmac(32)`.
    /// For `Null`: returns plaintext unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the AEAD operation fails.
    pub fn encrypt_frame(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        match self.cipher {
            BtspCipher::ChaCha20Poly1305 => {
                use chacha20poly1305::KeyInit;

                let cipher = ChaCha20Poly1305::new_from_slice(&self.encrypt_key)
                    .map_err(|e| BearDogError::system(format!("BTSP cipher init: {e}")))?;
                let nonce_bytes = build_nonce(self.encrypt_counter);
                self.encrypt_counter += 1;
                let nonce = Nonce::from_slice(&nonce_bytes);

                let ciphertext = cipher
                    .encrypt(nonce, plaintext)
                    .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;

                let mut frame = Vec::with_capacity(12 + ciphertext.len());
                frame.extend_from_slice(&nonce_bytes);
                frame.extend_from_slice(&ciphertext);
                Ok(frame)
            }
            BtspCipher::HmacPlain => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                type HmacSha256 = Hmac<Sha256>;

                let mut mac = HmacSha256::new_from_slice(&self.encrypt_key)
                    .map_err(|e| BearDogError::system(format!("BTSP HMAC init: {e}")))?;
                mac.update(plaintext);
                let tag = mac.finalize().into_bytes();

                let mut frame = Vec::with_capacity(plaintext.len() + 32);
                frame.extend_from_slice(plaintext);
                frame.extend_from_slice(&tag);
                Ok(frame)
            }
            BtspCipher::Null => Ok(plaintext.to_vec()),
        }
    }

    /// Decrypt a frame received from the client.
    ///
    /// # Errors
    ///
    /// Returns an error if decryption/verification fails.
    pub fn decrypt_frame(&mut self, frame: &[u8]) -> Result<Vec<u8>, BearDogError> {
        match self.cipher {
            BtspCipher::ChaCha20Poly1305 => {
                if frame.len() < 12 + 16 {
                    return Err(BearDogError::system(
                        "BTSP encrypted frame too short (need nonce + tag)".to_string(),
                    ));
                }
                let nonce_bytes = &frame[..12];
                let ciphertext = &frame[12..];

                let expected_nonce = build_nonce(self.decrypt_counter);
                if nonce_bytes != expected_nonce {
                    return Err(BearDogError::system(
                        "BTSP nonce mismatch (replay?)".to_string(),
                    ));
                }
                self.decrypt_counter += 1;

                use chacha20poly1305::KeyInit;

                let cipher = ChaCha20Poly1305::new_from_slice(&self.decrypt_key)
                    .map_err(|e| BearDogError::system(format!("BTSP cipher init: {e}")))?;
                let nonce = Nonce::from_slice(nonce_bytes);

                cipher
                    .decrypt(nonce, ciphertext)
                    .map_err(|e| BearDogError::system(format!("BTSP decrypt: {e}")))
            }
            BtspCipher::HmacPlain => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                type HmacSha256 = Hmac<Sha256>;

                if frame.len() < 32 {
                    return Err(BearDogError::system(
                        "BTSP HMAC frame too short".to_string(),
                    ));
                }
                let payload = &frame[..frame.len() - 32];
                let tag = &frame[frame.len() - 32..];

                let mut mac = HmacSha256::new_from_slice(&self.decrypt_key)
                    .map_err(|e| BearDogError::system(format!("BTSP HMAC init: {e}")))?;
                mac.update(payload);
                mac.verify_slice(tag).map_err(|_| {
                    BearDogError::system("BTSP HMAC verification failed".to_string())
                })?;

                Ok(payload.to_vec())
            }
            BtspCipher::Null => Ok(frame.to_vec()),
        }
    }
}

impl Drop for BtspSession {
    fn drop(&mut self) {
        self.encrypt_key.zeroize();
        self.decrypt_key.zeroize();
    }
}

/// Build a 12-byte nonce from a 64-bit counter (zero-padded big-endian).
fn build_nonce(counter: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[4..12].copy_from_slice(&counter.to_be_bytes());
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_session(cipher: BtspCipher) -> (BtspSession, BtspSession) {
        let key_s2c = [0x11; 32];
        let key_c2s = [0x22; 32];
        let server = BtspSession::new_server("abc123".into(), cipher, key_s2c, key_c2s);
        let client = BtspSession {
            session_id: "abc123".into(),
            cipher,
            encrypt_key: key_c2s,
            decrypt_key: key_s2c,
            encrypt_counter: 0,
            decrypt_counter: 0,
        };
        (server, client)
    }

    #[test]
    fn chacha20_roundtrip() {
        let (mut server, mut client) = test_session(BtspCipher::ChaCha20Poly1305);
        let msg = b"hello encrypted world";

        let encrypted = server.encrypt_frame(msg).expect("encrypt");
        assert_ne!(&encrypted[12..], msg);

        let decrypted = client.decrypt_frame(&encrypted).expect("decrypt");
        assert_eq!(decrypted, msg);
    }

    #[test]
    fn chacha20_nonce_increments() {
        let (mut server, mut client) = test_session(BtspCipher::ChaCha20Poly1305);

        let e1 = server.encrypt_frame(b"msg1").expect("e1");
        let e2 = server.encrypt_frame(b"msg2").expect("e2");
        assert_ne!(e1[..12], e2[..12]);

        let d1 = client.decrypt_frame(&e1).expect("d1");
        let d2 = client.decrypt_frame(&e2).expect("d2");
        assert_eq!(d1, b"msg1");
        assert_eq!(d2, b"msg2");
    }

    #[test]
    fn hmac_plain_roundtrip() {
        let (mut server, mut client) = test_session(BtspCipher::HmacPlain);
        let msg = b"integrity checked";

        let frame = server.encrypt_frame(msg).expect("encrypt");
        assert_eq!(&frame[..msg.len()], msg);
        assert_eq!(frame.len(), msg.len() + 32);

        let decrypted = client.decrypt_frame(&frame).expect("decrypt");
        assert_eq!(decrypted, msg);
    }

    #[test]
    fn null_passthrough() {
        let (mut server, mut client) = test_session(BtspCipher::Null);
        let msg = b"raw plaintext";

        let frame = server.encrypt_frame(msg).expect("encrypt");
        assert_eq!(frame, msg);

        let decrypted = client.decrypt_frame(&frame).expect("decrypt");
        assert_eq!(decrypted, msg);
    }

    #[test]
    fn chacha20_rejects_tampered_frame() {
        let (mut server, mut client) = test_session(BtspCipher::ChaCha20Poly1305);
        let mut frame = server.encrypt_frame(b"secret").expect("encrypt");
        frame[15] ^= 0xFF;
        assert!(client.decrypt_frame(&frame).is_err());
    }

    #[test]
    fn cipher_wire_names() {
        assert_eq!(
            BtspCipher::ChaCha20Poly1305.wire_name(),
            "chacha20_poly1305"
        );
        assert_eq!(
            BtspCipher::from_wire_name("chacha20")
                .expect("parse")
                .wire_name(),
            "chacha20_poly1305"
        );
        assert!(BtspCipher::from_wire_name("unknown").is_err());
    }
}
