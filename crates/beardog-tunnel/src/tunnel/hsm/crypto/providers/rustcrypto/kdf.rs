// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hashing and key derivation operations (SHA-2, BLAKE3, HKDF, PBKDF2, Argon2, scrypt).

use super::RustCryptoProvider;
use crate::tunnel::hsm::crypto::algorithms::{Argon2Variant, HashAlgorithm};
use beardog_errors::BearDogError;

impl RustCryptoProvider {
    pub(super) fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        Sha256::digest(data).to_vec()
    }

    pub(super) fn hash_sha384(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha384};
        Sha384::digest(data).to_vec()
    }

    pub(super) fn hash_sha512(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha512};
        Sha512::digest(data).to_vec()
    }

    pub(super) fn hash_blake3(&self, data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    pub(super) fn derive_hkdf_sha256(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let hkdf = Hkdf::<Sha256>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output).map_err(|e| {
            BearDogError::crypto_error(format!("HKDF-SHA256 expansion failed: {e}"))
        })?;
        Ok(output)
    }

    pub(super) fn derive_hkdf_sha384(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha384;

        let hkdf = Hkdf::<Sha384>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output).map_err(|e| {
            BearDogError::crypto_error(format!("HKDF-SHA384 expansion failed: {e}"))
        })?;
        Ok(output)
    }

    pub(super) fn derive_hkdf_sha512(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha512;

        let hkdf = Hkdf::<Sha512>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output).map_err(|e| {
            BearDogError::crypto_error(format!("HKDF-SHA512 expansion failed: {e}"))
        })?;
        Ok(output)
    }

    pub(super) fn derive_pbkdf2(
        &self,
        hash: &HashAlgorithm,
        iterations: u32,
        password: &[u8],
        salt: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut output = vec![0u8; output_length];
        match hash {
            HashAlgorithm::Sha256 => {
                pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password, salt, iterations, &mut output);
            }
            HashAlgorithm::Sha384 => {
                pbkdf2::pbkdf2_hmac::<sha2::Sha384>(password, salt, iterations, &mut output);
            }
            HashAlgorithm::Sha512 => {
                pbkdf2::pbkdf2_hmac::<sha2::Sha512>(password, salt, iterations, &mut output);
            }
            other => {
                return Err(BearDogError::unsupported_operation(format!(
                    "PBKDF2 with {other} not supported"
                )));
            }
        }
        Ok(output)
    }

    pub(super) fn derive_argon2(
        &self,
        variant: &Argon2Variant,
        password: &[u8],
        salt: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use argon2::Argon2;

        let algorithm = match variant {
            Argon2Variant::Argon2d => argon2::Algorithm::Argon2d,
            Argon2Variant::Argon2i => argon2::Algorithm::Argon2i,
            Argon2Variant::Argon2id => argon2::Algorithm::Argon2id,
        };

        let params = argon2::Params::new(19456, 2, 1, Some(output_length))
            .map_err(|e| BearDogError::crypto_error(format!("Argon2 params: {e}")))?;

        let argon2 = Argon2::new(algorithm, argon2::Version::V0x13, params);
        let mut output = vec![0u8; output_length];
        argon2
            .hash_password_into(password, salt, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("Argon2 derivation failed: {e}")))?;

        Ok(output)
    }

    pub(super) fn derive_scrypt(
        &self,
        n: u64,
        r: u32,
        p: u32,
        password: &[u8],
        salt: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        let log_n = u8::try_from(
            n.checked_ilog2()
                .ok_or_else(|| BearDogError::crypto_error("scrypt n must be a power of 2 > 0"))?,
        )
        .map_err(|_| BearDogError::crypto_error("scrypt log_n exceeds u8 range"))?;
        let params = scrypt::Params::new(log_n, r, p, output_length)
            .map_err(|e| BearDogError::crypto_error(format!("scrypt params: {e}")))?;

        let mut output = vec![0u8; output_length];
        scrypt::scrypt(password, salt, &params, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("scrypt derivation failed: {e}")))?;

        Ok(output)
    }
}
