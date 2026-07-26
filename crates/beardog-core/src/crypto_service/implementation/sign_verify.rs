// SPDX-License-Identifier: AGPL-3.0-or-later

//! Digital signature creation and verification.

use super::BearDogCryptoService;
use crate::crypto_service::Result;
use crate::crypto_service::algorithms::asymmetric;
use beardog_errors::BearDogError;
use beardog_types::crypto_service::{
    SignOptions, Signature, SignatureAlgorithm, SignatureMetadata, VerifyOptions,
};
use std::time::SystemTime;

impl BearDogCryptoService {
    pub(crate) fn sign_impl(
        &self,
        data: &[u8],
        algorithm: SignatureAlgorithm,
        options: SignOptions,
    ) -> Result<Signature> {
        let _op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        self.validate_data_size(data)?;

        let key_id = &options.key_id;

        if !self.algorithms.supports_signature(&algorithm) {
            return Err(BearDogError::business(format!(
                "Signature algorithm {algorithm:?} not supported"
            )));
        }

        let signature_bytes = match algorithm {
            SignatureAlgorithm::Ed25519 => {
                let key = self.derive_signing_key(key_id)?;
                let (secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&key)?;

                self.store_public_key(key_id, public_key.to_vec());

                asymmetric::sign_ed25519(data, &secret_key)?
            }
            SignatureAlgorithm::EcdsaP256 => {
                let key = self.derive_signing_key(key_id)?;
                asymmetric::sign_ecdsa_p256(data, &key)?
            }
            SignatureAlgorithm::RsaPss => {
                let private_key_der = self.get_or_generate_rsa_key(key_id)?;
                asymmetric::sign_rsa_pss(data, &private_key_der)?
            }
        };

        self.audit_log("sign", Some(key_id), true);

        Ok(Signature {
            signature: signature_bytes,
            algorithm,
            metadata: SignatureMetadata {
                timestamp: start_time,
                key_id: Some(key_id.clone()),
                context: options.context.clone(),
            },
        })
    }

    pub(crate) fn verify_impl(
        &self,
        data: &[u8],
        signature: &Signature,
        options: VerifyOptions,
    ) -> Result<bool> {
        let _op_id = self.next_operation_id();

        let public_key = &options.public_key;

        let valid = match signature.algorithm {
            SignatureAlgorithm::Ed25519 => {
                asymmetric::verify_ed25519(data, &signature.signature, public_key)?
            }
            SignatureAlgorithm::EcdsaP256 => {
                asymmetric::verify_ecdsa_p256(data, &signature.signature, public_key)?
            }
            SignatureAlgorithm::RsaPss => {
                asymmetric::verify_rsa_pss(data, &signature.signature, public_key)?
            }
        };

        self.audit_log("verify", None, valid);

        Ok(valid)
    }
}
