// SPDX-License-Identifier: AGPL-3.0-only

//! `CryptoService` trait implementation (delegates to domain helpers).

use super::BearDogCryptoService;
use crate::crypto_service::Result;
use crate::crypto_service::r#trait::CryptoService;
use async_trait::async_trait;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, EncryptedData, HealthStatus, KeyAlgorithm,
    KeyGenOptions, KeyInfo, ServiceCapabilities, SignOptions, Signature, SignatureAlgorithm,
    VerifyOptions,
};

#[async_trait]
impl CryptoService for BearDogCryptoService {
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: CryptoAlgorithm,
        options: EncryptOptions,
    ) -> Result<EncryptedData> {
        self.encrypt_impl(data, algorithm, options).await
    }

    async fn decrypt(&self, encrypted: &EncryptedData, options: DecryptOptions) -> Result<Vec<u8>> {
        self.decrypt_impl(encrypted, options).await
    }

    async fn sign(
        &self,
        data: &[u8],
        algorithm: SignatureAlgorithm,
        options: SignOptions,
    ) -> Result<Signature> {
        self.sign_impl(data, algorithm, options).await
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &Signature,
        options: VerifyOptions,
    ) -> Result<bool> {
        self.verify_impl(data, signature, options).await
    }

    async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        options: KeyGenOptions,
    ) -> Result<KeyInfo> {
        self.generate_key_impl(algorithm, options).await
    }

    async fn get_capabilities(&self) -> Result<ServiceCapabilities> {
        self.get_capabilities_impl().await
    }

    async fn get_health(&self) -> Result<HealthStatus> {
        self.get_health_impl().await
    }
}
