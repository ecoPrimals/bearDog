//! API Endpoint Modules
//!
//! Capability-based endpoints for BearDog operations.
//! Each module exposes operations based on capabilities, not hardcoded names.

pub mod capabilities;
pub mod crypto;
pub mod generic_crypto;
pub mod health;
pub mod key_management;
pub mod protocols;

pub use capabilities::*;
pub use health::*;

// Re-export specific types to avoid ambiguity
pub use crypto::{aes_gcm_decrypt, aes_gcm_encrypt, ed25519_sign, ed25519_verify};
pub use generic_crypto::{
    decrypt, encrypt, DecryptRequest as GenericDecryptRequest,
    DecryptResponse as GenericDecryptResponse, EncryptRequest, EncryptResponse,
};
pub use key_management::{
    delete_key, generate_key, get_key_info, DeleteKeyRequest, DeleteKeyResponse,
    GenerateKeyRequest, GenerateKeyResponse, GetKeyInfoRequest, GetKeyInfoResponse,
};
