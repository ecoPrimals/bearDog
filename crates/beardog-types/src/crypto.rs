// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Cryptographic Types
///
/// **ELIMINATES FRAGMENTATION**: Unifies cryptographic type definitions.

use serde::{Deserialize, Serialize};
/// Cryptographic algorithm identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    /// AES encryption
    Aes { mode: AesMode, key_size: u16 },
    /// RSA encryption/signing
    Rsa { key_size: u16, padding: RsaPadding },
    /// Elliptic Curve operations
    EllipticCurve { curve: EcCurve },
    /// Ed25519 signatures
    Ed25519,
    /// X25519 key exchange
    X25519,
    /// ChaCha20 stream cipher
    ChaCha20,
    /// HMAC authentication
    Hmac { hash: HashAlgorithm },
    /// Hash functions
    Hash { algorithm: HashAlgorithm },
}
/// AES modes of operation
pub enum AesMode {
    /// Galois/Counter Mode
    Gcm,
    /// Cipher Block Chaining
    Cbc,
    /// Counter Mode
    Ctr,
    /// Electronic Codebook
    Ecb,
/// RSA padding schemes}


pub enum RsaPadding {
    /// PKCS#1 v1.5
    Pkcs1v15,
    /// OAEP
    Oaep,
    /// PSS (for signatures)
    Pss,
/// Hash algorithms
pub enum HashAlgorithm {
    /// SHA-256
    Sha256,
    /// SHA-384
    Sha384,
    /// SHA-512
    Sha512,
    /// Blake3
    Blake3,
    /// SHA-3
    Sha3_256,
/// Re-export elliptic curve from HSM module for consistency
pub use crate::hsm::EcCurve;
