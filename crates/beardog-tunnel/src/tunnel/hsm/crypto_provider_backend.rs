// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified [`beardog_types::hsm::CryptoProvider`] backend (enum dispatch; replaces `Arc<dyn CryptoProvider<..>>`).

use crate::tunnel::hsm::crypto_dispatch::CryptoProviderDispatch;
use crate::tunnel::hsm::software_hsm::crypto_providers::genetic_crypto::GeneticCryptoProvider;
use crate::tunnel::hsm::software_hsm::crypto_providers::rust_crypto::RustCryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use beardog_types::hsm::CryptoProvider;
use std::future::Future;

/// Enum dispatch over software HSM crypto provider implementations.
#[derive(Debug, Clone)]
pub enum CryptoProviderBackend {
    /// Pure Rust `RustCrypto` stack (software HSM).
    RustCrypto(RustCryptoProvider),
    /// Genetic lineage-aware provider.
    Genetic(GeneticCryptoProvider),
    /// Existing [`CryptoProviderDispatch`] wrapper.
    Dispatch(CryptoProviderDispatch),
}

impl CryptoProvider<KeyType> for CryptoProviderBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        match self {
            Self::RustCrypto(p) => p.initialize().await,
            Self::Genetic(p) => p.initialize().await,
            Self::Dispatch(p) => p.initialize().await,
        }
    }

    fn generate_key_material(
        &self,
        key_type: &KeyType,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_type = key_type.clone();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.generate_key_material(&key_type).await,
                Self::Genetic(p) => p.generate_key_material(&key_type).await,
                Self::Dispatch(p) => p.generate_key_material(&key_type).await,
            }
        }
    }

    fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let plaintext = plaintext.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.encrypt(&key_material, &plaintext).await,
                Self::Genetic(p) => p.encrypt(&key_material, &plaintext).await,
                Self::Dispatch(p) => p.encrypt(&key_material, &plaintext).await,
            }
        }
    }

    fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let ciphertext = ciphertext.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.decrypt(&key_material, &ciphertext).await,
                Self::Genetic(p) => p.decrypt(&key_material, &ciphertext).await,
                Self::Dispatch(p) => p.decrypt(&key_material, &ciphertext).await,
            }
        }
    }

    fn sign(
        &self,
        key_material: &[u8],
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.sign(&key_material, &data).await,
                Self::Genetic(p) => p.sign(&key_material, &data).await,
                Self::Dispatch(p) => p.sign(&key_material, &data).await,
            }
        }
    }

    fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        let signature = signature.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.verify(&key_material, &data, &signature).await,
                Self::Genetic(p) => p.verify(&key_material, &data, &signature).await,
                Self::Dispatch(p) => p.verify(&key_material, &data, &signature).await,
            }
        }
    }

    fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let root_key = root_key.to_vec();
        let derivation_data = derivation_data.to_vec();
        let slf = self.clone();
        async move {
            match slf {
                Self::RustCrypto(p) => p.derive_key(&root_key, &derivation_data).await,
                Self::Genetic(p) => p.derive_key(&root_key, &derivation_data).await,
                Self::Dispatch(p) => p.derive_key(&root_key, &derivation_data).await,
            }
        }
    }
}
