

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(AesMode, key_size: u16 },

    Rsa { key_size: u16, padding: RsaPadding },

    EllipticCurve { curve: EcCurve },


    Ed25519,


    X25519,


    ChaCha20,

    Hmac { hash: HashAlgorithm },

    Hash { algorithm: HashAlgorithm },
}

pub enum AesMode {

    /// Gcm variant
    Gcm,

    /// Cbc variant
    Cbc,

    /// Ctr variant
    Ctr,

    /// Ecb variant
    Ecb,

pub enum RsaPadding {


    /// Represents pkcs1v15 variant
    Pkcs1v15,

    /// Oaep variant
    Oaep,

    /// Pss variant
    Pss,

pub enum HashAlgorithm {


    /// Represents sha256 variant
    Sha256,


    /// Represents sha384 variant
    Sha384,


    /// Represents sha512 variant
    Sha512,


    /// Represents blake3 variant
    Blake3,


    /// Represents sha3_256 variant
    Sha3_256,

pub use crate::hsm::EcCurve;
