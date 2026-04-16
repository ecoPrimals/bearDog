// SPDX-License-Identifier: AGPL-3.0-or-later

//! Enum dispatch for [`super::provider::UniversalCryptoProvider`].

use super::algorithms::{
    AsymmetricAlgorithm, CryptoAlgorithm, DecryptionOptions, EncryptedData, EncryptionOptions,
    HashAlgorithm, KdfAlgorithm, Signature, SignatureAlgorithm, SigningOptions, SymmetricAlgorithm,
    VerificationOptions,
};
use super::capabilities::CryptoCapabilities;
use super::provider::UniversalCryptoProvider;
use super::providers::RustCryptoProvider;
use beardog_errors::BearDogError;
use std::future::Future;

/// Runtime wrapper for universal crypto providers in the tunnel crate.
#[derive(Debug, Clone)]
pub enum UniversalCryptoBackend {
    /// Pure Rust crypto stack.
    RustCrypto(RustCryptoProvider),
}

impl UniversalCryptoProvider for UniversalCryptoBackend {
    fn provider_name(&self) -> &str {
        match self {
            Self::RustCrypto(p) => p.provider_name(),
        }
    }

    fn provider_version(&self) -> &str {
        match self {
            Self::RustCrypto(p) => p.provider_version(),
        }
    }

    fn discover_capabilities(
        &self,
    ) -> impl Future<Output = Result<CryptoCapabilities, BearDogError>> + Send {
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.discover_capabilities().await,
            }
        }
    }

    fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> impl Future<Output = bool> + Send {
        let algorithm = algorithm.clone();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.supports_algorithm(&algorithm).await,
            }
        }
    }

    fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> impl Future<Output = Result<EncryptedData, BearDogError>> + Send {
        let key = key.to_vec();
        let plaintext = plaintext.to_vec();
        let options = options.clone();
        async move {
            match self {
                Self::RustCrypto(p) => {
                    p.encrypt_symmetric(algorithm, &key, &plaintext, &options)
                        .await
                }
            }
        }
    }

    fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key = key.to_vec();
        let ciphertext = ciphertext.clone();
        let options = options.clone();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => {
                    p.decrypt_symmetric(algorithm, &key, &ciphertext, &options)
                        .await
                }
            }
        }
    }

    async fn encrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        public_key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        let public_key = public_key.to_vec();
        let plaintext = plaintext.to_vec();
        let options = options.clone();
        match self {
            Self::RustCrypto(p) => {
                p.encrypt_asymmetric(algorithm, &public_key, &plaintext, &options)
                    .await
            }
        }
    }

    async fn decrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        private_key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        let private_key = private_key.to_vec();
        let ciphertext = ciphertext.clone();
        let options = options.clone();
        match self {
            Self::RustCrypto(p) => {
                p.decrypt_asymmetric(algorithm, &private_key, &ciphertext, &options)
                    .await
            }
        }
    }

    fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        options: &SigningOptions,
    ) -> impl Future<Output = Result<Signature, BearDogError>> + Send {
        let private_key = private_key.to_vec();
        let message = message.to_vec();
        let options = options.clone();
        async move {
            match self {
                Self::RustCrypto(p) => p.sign(algorithm, &private_key, &message, &options).await,
            }
        }
    }

    fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        options: &VerificationOptions,
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let public_key = public_key.to_vec();
        let message = message.to_vec();
        let signature = signature.clone();
        let options = options.clone();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => {
                    p.verify(algorithm, &public_key, &message, &signature, &options)
                        .await
                }
            }
        }
    }

    fn hash(
        &self,
        algorithm: HashAlgorithm,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let data = data.to_vec();
        async move {
            match self {
                Self::RustCrypto(p) => p.hash(algorithm, &data).await,
            }
        }
    }

    fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let input_key = input_key.to_vec();
        let salt = salt.to_vec();
        let info = info.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => {
                    p.derive_key(algorithm, &input_key, &salt, &info, output_length)
                        .await
                }
            }
        }
    }
}
