

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {

    Aes { mode: AesMode, key_size: u16 },

    Rsa { key_size: u16, padding: RsaPadding },

    EllipticCurve { curve: EcCurve },

    Ed25519,

    X25519,

    ChaCha20,

    Hmac { hash: HashAlgorithm },

    Hash { algorithm: HashAlgorithm },
}

pub enum AesMode {

    Gcm,

    Cbc,

    Ctr,

    Ecb,

pub enum RsaPadding {

    Pkcs1v15,

    Oaep,

    Pss,

pub enum HashAlgorithm {

    Sha256,

    Sha384,

    Sha512,

    Blake3,

    Sha3_256,

pub use crate::hsm::EcCurve;
