// SPDX-License-Identifier: AGPL-3.0-only

//! Shared safe software crypto for SIMD-style accelerator modules (`sha2`, `aes`, `ctr`).
//! Hand-written intrinsics are unnecessary; RustCrypto backends are auto-vectorized by LLVM where supported.

use aes::Aes128;
use aes::cipher::{KeyIvInit, StreamCipher, generic_array::GenericArray};
use beardog_errors::BearDogError;
use ctr::Ctr128BE;
use sha2::{Digest, Sha256};

/// Iterations for micro-benchmark helpers in accelerator modules.
pub(crate) const CRYPTO_BENCHMARK_ITERATIONS: u64 = 1000;

pub(crate) fn safe_sha256_digest(input: &[u8]) -> Result<[u8; 32], BearDogError> {
    if input.is_empty() {
        return Err(BearDogError::validation("Input data cannot be empty"));
    }
    let mut hasher = Sha256::new();
    hasher.update(input);
    Ok(hasher.finalize().into())
}

fn derive_aes128_key(key_material: &[u8]) -> [u8; 16] {
    let mut out = [0u8; 16];
    if key_material.len() == 16 {
        out.copy_from_slice(key_material);
    } else {
        let h = Sha256::digest(key_material);
        out.copy_from_slice(&h[..16]);
    }
    out
}

/// AES-128-CTR keystream XOR (length-preserving).
///
/// Uses an all-zero nonce so this helper stays deterministic for tests and benchmarks.
/// Callers must not rely on this alone for message secrecy across multiple messages.
pub(crate) fn aes128_ctr_apply(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if plaintext.is_empty() {
        return Err(BearDogError::validation("Plaintext cannot be empty"));
    }
    if key.is_empty() {
        return Err(BearDogError::validation("Key cannot be empty"));
    }
    let key_bytes = derive_aes128_key(key);
    let key = GenericArray::from_slice(&key_bytes);
    let nonce = GenericArray::default();
    let mut cipher = Ctr128BE::<Aes128>::new(key, &nonce);
    let mut buf = plaintext.to_vec();
    cipher.apply_keystream(&mut buf);
    Ok(buf)
}
