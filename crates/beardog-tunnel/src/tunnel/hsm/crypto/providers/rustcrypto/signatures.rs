// SPDX-License-Identifier: AGPL-3.0-or-later

//! Digital signature operations (Ed25519, ECDSA P-256, ECDSA P-384).

use super::RustCryptoProvider;
use crate::tunnel::hsm::crypto::algorithms::Signature;
use beardog_errors::BearDogError;

impl RustCryptoProvider {
    pub(super) fn sign_ed25519(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Signature, BearDogError> {
        use ed25519_dalek::{Signer, SigningKey};

        let signing_key = SigningKey::from_bytes(
            private_key
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 private key length"))?,
        );

        let signature = signing_key.sign(message);

        Ok(Signature {
            algorithm: "Ed25519".to_string(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    pub(super) fn verify_ed25519(
        &self,
        key_material: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, BearDogError> {
        use ed25519_dalek::{Signature as Ed25519Signature, SigningKey, Verifier, VerifyingKey};

        let verifying_key = if key_material.len() == 32 {
            let signing_key =
                SigningKey::from_bytes(key_material.try_into().map_err(|_| {
                    BearDogError::crypto_error("Invalid Ed25519 private key length")
                })?);
            signing_key.verifying_key()
        } else {
            VerifyingKey::from_bytes(
                key_material
                    .try_into()
                    .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 public key length"))?,
            )
            .map_err(|e| BearDogError::crypto_error(format!("Invalid Ed25519 public key: {e}")))?
        };

        let sig = Ed25519Signature::from_bytes(
            signature
                .signature
                .as_slice()
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 signature length"))?,
        );

        Ok(verifying_key.verify(message, &sig).is_ok())
    }

    pub(super) fn sign_ecdsa_p256(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Signature, BearDogError> {
        use p256::ecdsa::{SigningKey, signature::Signer};

        let signing_key = SigningKey::from_bytes(private_key.into()).map_err(|e| {
            beardog_errors::crypto_error(
                "sign_ecdsa_p256",
                &format!("Invalid P-256 private key format: {e}"),
            )
        })?;

        let signature: p256::ecdsa::Signature = signing_key.sign(message);

        Ok(Signature {
            algorithm: "ECDSA-P256-SHA256".to_string(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    pub(super) fn verify_ecdsa_p256(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, BearDogError> {
        use p256::ecdsa::{VerifyingKey, signature::Verifier};

        let verifying_key = VerifyingKey::from_sec1_bytes(public_key).map_err(|e| {
            beardog_errors::crypto_error(
                "verify_ecdsa_p256",
                &format!("Invalid P-256 public key format: {e}"),
            )
        })?;

        let sig = p256::ecdsa::Signature::from_bytes(signature.signature.as_slice().into())
            .map_err(|e| {
                beardog_errors::crypto_error(
                    "verify_ecdsa_p256",
                    &format!("Invalid signature format: {e}"),
                )
            })?;

        Ok(verifying_key.verify(message, &sig).is_ok())
    }

    pub(super) fn sign_ecdsa_p384(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Signature, BearDogError> {
        use p384::ecdsa::{SigningKey, signature::Signer};

        let signing_key = SigningKey::from_bytes(private_key.into())
            .map_err(|e| BearDogError::crypto_error(format!("Invalid P-384 private key: {e}")))?;

        let signature: p384::ecdsa::Signature = signing_key.sign(message);

        Ok(Signature {
            algorithm: "ECDSA-P384-SHA384".to_string(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    pub(super) fn verify_ecdsa_p384(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, BearDogError> {
        use p384::ecdsa::{VerifyingKey, signature::Verifier};

        let verifying_key = VerifyingKey::from_sec1_bytes(public_key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid P-384 public key: {e}")))?;

        let sig = p384::ecdsa::Signature::from_bytes(signature.signature.as_slice().into())
            .map_err(|e| BearDogError::crypto_error(format!("Invalid P-384 signature: {e}")))?;

        Ok(verifying_key.verify(message, &sig).is_ok())
    }
}
