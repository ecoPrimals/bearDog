//! # Native Android Keystore Operations
//!
//! This module provides real Android Keystore operations using NDK APIs
//! for hardware-backed cryptographic operations with StrongBox integration.

use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{info, warn};

#[cfg(target_os = "android")]
use {
    ndk_sys::{
        AKeyStore_KeyGenParameterSpec, AKeyStore_KeyProperties_DIGEST_SHA256,
        AKeyStore_KeyProperties_PADDING_RSA_PKCS1, AKeyStore_KeyProperties_PADDING_RSA_PSS,
        AKeyStore_KeyProperties_PURPOSE_DECRYPT, AKeyStore_KeyProperties_PURPOSE_ENCRYPT,
        AKeyStore_KeyProperties_PURPOSE_SIGN, AKeyStore_KeyProperties_PURPOSE_VERIFY,
        AKeyStore_decrypt, AKeyStore_delete, AKeyStore_encrypt, AKeyStore_generateKey,
        AKeyStore_getKeyCharacteristics, AKeyStore_isSecurityLevelSupported, AKeyStore_sign,
        AKeyStore_verify, AKEYSTORE_ALGORITHM_AES, AKEYSTORE_ALGORITHM_EC, AKEYSTORE_ALGORITHM_RSA,
        AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_DIGEST_SHA256, AKEYSTORE_PADDING_NONE,
        AKEYSTORE_PADDING_RSA_OAEP, AKEYSTORE_PADDING_RSA_PKCS1_1_5, AKEYSTORE_PADDING_RSA_PSS,
        AKEYSTORE_PARAMETER_ALGORITHM, AKEYSTORE_SECURITY_LEVEL_STRONGBOX,
        AKEYSTORE_SECURITY_LEVEL_TRUSTED_ENVIRONMENT,
    },
    std::ffi::{CStr, CString},
    std::ptr,
};

/// Native Android Keystore operations
pub struct NativeKeystoreOperations;

impl NativeKeystoreOperations {
    /// Generate a hardware-backed key in Android Keystore
    pub async fn generate_strongbox_key(
        key_id: &str,
        key_type: &KeyType,
        strongbox_required: bool,
    ) -> BearDogResult<()> {
        info!("🔐 Generating native StrongBox key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_generate_key(key_id, key_type, strongbox_required).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_generate_key(key_id, key_type, strongbox_required).await
        }
    }

    /// Sign data using native Android Keystore
    pub async fn sign_with_keystore(
        key_id: &str,
        data: &[u8],
        algorithm: SigningAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        info!("✍️ Native signing with key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_sign(key_id, data, algorithm).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_sign(key_id, data, algorithm).await
        }
    }

    /// Verify signature using native Android Keystore
    pub async fn verify_with_keystore(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        algorithm: SigningAlgorithm,
    ) -> BearDogResult<bool> {
        info!("🔍 Native verification with key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_verify(key_id, data, signature, algorithm).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_verify(key_id, data, signature, algorithm).await
        }
    }

    /// Encrypt data using native Android Keystore
    pub async fn encrypt_with_keystore(
        key_id: &str,
        plaintext: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        info!("🔐 Native encryption with key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_encrypt(key_id, plaintext, algorithm).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_encrypt(key_id, plaintext, algorithm).await
        }
    }

    /// Decrypt data using native Android Keystore
    pub async fn decrypt_with_keystore(
        key_id: &str,
        ciphertext: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        info!("🔓 Native decryption with key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_decrypt(key_id, ciphertext, algorithm).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_decrypt(key_id, ciphertext, algorithm).await
        }
    }

    /// Delete key from native Android Keystore
    pub async fn delete_keystore_key(key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting native key: {}", key_id);

        #[cfg(target_os = "android")]
        {
            Self::native_delete_key(key_id).await
        }

        #[cfg(not(target_os = "android"))]
        {
            Self::mock_delete_key(key_id).await
        }
    }
}

#[cfg(target_os = "android")]
impl NativeKeystoreOperations {
    /// Native Android key generation using real Android Keystore API
    async fn native_generate_key(
        key_id: &str,
        key_type: &KeyType,
        strongbox_required: bool,
    ) -> BearDogResult<()> {
        use ndk_sys::{
            AKeyStore_generateKey, AKeyStore_isSecurityLevelSupported, AKEYSTORE_ALGORITHM_AES,
            AKEYSTORE_ALGORITHM_EC, AKEYSTORE_ALGORITHM_RSA, AKEYSTORE_PARAMETER_ALGORITHM,
            AKEYSTORE_SECURITY_LEVEL_STRONGBOX,
        };
        use std::ffi::CString;

        info!("🔐 Real Android: Generating StrongBox key: {}", key_id);

        unsafe {
            // Convert key ID to C string
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            // Check if StrongBox is available and required
            if strongbox_required {
                let strongbox_supported =
                    AKeyStore_isSecurityLevelSupported(AKEYSTORE_SECURITY_LEVEL_STRONGBOX);
                if strongbox_supported != 0 {
                    return Err(BearDogError::Unavailable {
                        message: "StrongBox required but not available".to_string(),
                    });
                }
            }

            // Set up key parameters based on key type
            let algorithm = match key_type {
                KeyType::Rsa { key_size } => {
                    if *key_size < 2048 {
                        return Err(BearDogError::InvalidInput {
                            message: "RSA key size must be at least 2048 bits".to_string(),
                        });
                    }
                    AKEYSTORE_ALGORITHM_RSA
                }
                KeyType::EcdsaP256 => AKEYSTORE_ALGORITHM_EC,
                KeyType::EcdsaP384 => AKEYSTORE_ALGORITHM_EC,
                KeyType::Ed25519 => {
                    return Err(BearDogError::UnsupportedOperation {
                        operation: "Ed25519 not supported in Android Keystore".to_string(),
                    });
                }
                KeyType::Aes { key_size } => {
                    if ![128, 192, 256].contains(key_size) {
                        return Err(BearDogError::InvalidInput {
                            message: "AES key size must be 128, 192, or 256 bits".to_string(),
                        });
                    }
                    AKEYSTORE_ALGORITHM_AES
                }
            };

            // Generate the key using Android Keystore API
            let result = AKeyStore_generateKey(
                key_alias.as_ptr(),
                algorithm,
                if strongbox_required {
                    AKEYSTORE_SECURITY_LEVEL_STRONGBOX
                } else {
                    0
                },
            );

            if result != 0 {
                return Err(BearDogError::Hsm {
                    message: format!("Android Keystore key generation failed: {}", result),
                });
            }

            info!("✅ Real Android: StrongBox key generated: {}", key_id);
            Ok(())
        }
    }

    /// Native Android signing using real Android Keystore API
    async fn native_sign(
        key_id: &str,
        data: &[u8],
        algorithm: SigningAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        use ndk_sys::{AKeyStore_sign, AKEYSTORE_DIGEST_SHA256};
        use std::ffi::CString;

        info!("✍️ Real Android: Signing with StrongBox key: {}", key_id);

        unsafe {
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            // Set digest algorithm based on signing algorithm
            let digest = match algorithm {
                SigningAlgorithm::EcdsaSha256
                | SigningAlgorithm::RsaPssSha256
                | SigningAlgorithm::RsaPkcs1Sha256 => AKEYSTORE_DIGEST_SHA256,
            };

            // Allocate signature buffer (max signature size)
            let mut signature_buffer = vec![0u8; 1024];
            let mut signature_length = signature_buffer.len();

            // Perform the signing operation
            let result = AKeyStore_sign(
                key_alias.as_ptr(),
                digest,
                data.as_ptr(),
                data.len(),
                signature_buffer.as_mut_ptr(),
                &mut signature_length,
            );

            if result != 0 {
                return Err(BearDogError::Hsm {
                    message: format!("Android Keystore signing failed: {}", result),
                });
            }

            // Resize to actual signature length
            signature_buffer.truncate(signature_length);

            info!(
                "✅ Real Android: Data signed ({} bytes)",
                signature_buffer.len()
            );
            Ok(signature_buffer)
        }
    }

    /// Native Android signature verification using real Android Keystore API
    async fn native_verify(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        algorithm: SigningAlgorithm,
    ) -> BearDogResult<bool> {
        use ndk_sys::{AKeyStore_verify, AKEYSTORE_DIGEST_SHA256};
        use std::ffi::CString;

        info!("🔍 Real Android: Verifying with StrongBox key: {}", key_id);

        unsafe {
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            let digest = match algorithm {
                SigningAlgorithm::EcdsaSha256
                | SigningAlgorithm::RsaPssSha256
                | SigningAlgorithm::RsaPkcs1Sha256 => AKEYSTORE_DIGEST_SHA256,
            };

            // Perform verification
            let result = AKeyStore_verify(
                key_alias.as_ptr(),
                digest,
                data.as_ptr(),
                data.len(),
                signature.as_ptr(),
                signature.len(),
            );

            let is_valid = result == 0;
            info!(
                "✅ Real Android: Signature verification result: {}",
                is_valid
            );
            Ok(is_valid)
        }
    }

    /// Native Android encryption using real Android Keystore API
    async fn native_encrypt(
        key_id: &str,
        plaintext: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        use ndk_sys::{AKeyStore_encrypt, AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_PADDING_NONE};
        use std::ffi::CString;

        info!("🔐 Real Android: Encrypting with StrongBox key: {}", key_id);

        unsafe {
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            // Set encryption parameters based on algorithm
            let (block_mode, padding) = match algorithm {
                EncryptionAlgorithm::AesGcm => (AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_PADDING_NONE),
                EncryptionAlgorithm::RsaOaep => {
                    return Err(BearDogError::UnsupportedOperation {
                        operation: "RSA-OAEP encryption not implemented yet".to_string(),
                    });
                }
            };

            // Allocate ciphertext buffer
            let mut ciphertext_buffer = vec![0u8; plaintext.len() + 32]; // Extra space for tag/IV
            let mut ciphertext_length = ciphertext_buffer.len();

            // Perform encryption
            let result = AKeyStore_encrypt(
                key_alias.as_ptr(),
                block_mode,
                padding,
                plaintext.as_ptr(),
                plaintext.len(),
                ciphertext_buffer.as_mut_ptr(),
                &mut ciphertext_length,
            );

            if result != 0 {
                return Err(BearDogError::Hsm {
                    message: format!("Android Keystore encryption failed: {}", result),
                });
            }

            ciphertext_buffer.truncate(ciphertext_length);

            info!(
                "✅ Real Android: Encryption completed: {} bytes",
                ciphertext_buffer.len()
            );
            Ok(ciphertext_buffer)
        }
    }

    /// Native Android decryption using real Android Keystore API
    async fn native_decrypt(
        key_id: &str,
        ciphertext: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        use ndk_sys::{AKeyStore_decrypt, AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_PADDING_NONE};
        use std::ffi::CString;

        info!("🔓 Real Android: Decrypting with StrongBox key: {}", key_id);

        unsafe {
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            let (block_mode, padding) = match algorithm {
                EncryptionAlgorithm::AesGcm => (AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_PADDING_NONE),
                EncryptionAlgorithm::RsaOaep => {
                    return Err(BearDogError::UnsupportedOperation {
                        operation: "RSA-OAEP decryption not implemented yet".to_string(),
                    });
                }
            };

            // Allocate plaintext buffer
            let mut plaintext_buffer = vec![0u8; ciphertext.len()];
            let mut plaintext_length = plaintext_buffer.len();

            // Perform decryption
            let result = AKeyStore_decrypt(
                key_alias.as_ptr(),
                block_mode,
                padding,
                ciphertext.as_ptr(),
                ciphertext.len(),
                plaintext_buffer.as_mut_ptr(),
                &mut plaintext_length,
            );

            if result != 0 {
                return Err(BearDogError::Hsm {
                    message: format!("Android Keystore decryption failed: {}", result),
                });
            }

            plaintext_buffer.truncate(plaintext_length);

            info!(
                "✅ Real Android: Decryption completed: {} bytes",
                plaintext_buffer.len()
            );
            Ok(plaintext_buffer)
        }
    }

    /// Native Android key deletion using real Android Keystore API
    async fn native_delete_key(key_id: &str) -> BearDogResult<()> {
        use ndk_sys::AKeyStore_delete;
        use std::ffi::CString;

        info!("🗑️ Real Android: Deleting StrongBox key: {}", key_id);

        unsafe {
            let key_alias = CString::new(key_id).map_err(|_| BearDogError::InvalidInput {
                message: "Invalid key ID format".to_string(),
            })?;

            let result = AKeyStore_delete(key_alias.as_ptr());

            if result != 0 {
                return Err(BearDogError::Hsm {
                    message: format!("Android Keystore key deletion failed: {}", result),
                });
            }

            info!("✅ Real Android: Key deleted: {}", key_id);
            Ok(())
        }
    }

    /// Convert SigningAlgorithm to native Android constant
    fn algorithm_to_native(algorithm: SigningAlgorithm) -> i32 {
        use ndk_sys::{
            AKEYSTORE_DIGEST_SHA256, AKEYSTORE_PADDING_RSA_PKCS1_1_5, AKEYSTORE_PADDING_RSA_PSS,
        };

        match algorithm {
            SigningAlgorithm::EcdsaSha256 => AKEYSTORE_DIGEST_SHA256,
            SigningAlgorithm::RsaPssSha256 => AKEYSTORE_PADDING_RSA_PSS,
            SigningAlgorithm::RsaPkcs1Sha256 => AKEYSTORE_PADDING_RSA_PKCS1_1_5,
        }
    }

    /// Convert EncryptionAlgorithm to native Android constant
    fn encryption_algorithm_to_native(algorithm: EncryptionAlgorithm) -> i32 {
        use ndk_sys::{AKEYSTORE_BLOCK_MODE_GCM, AKEYSTORE_PADDING_RSA_OAEP};

        match algorithm {
            EncryptionAlgorithm::AesGcm => AKEYSTORE_BLOCK_MODE_GCM,
            EncryptionAlgorithm::RsaOaep => AKEYSTORE_PADDING_RSA_OAEP,
        }
    }
}

#[cfg(not(target_os = "android"))]
impl NativeKeystoreOperations {
    /// Mock key generation for development on non-Android platforms
    async fn mock_generate_key(
        key_id: &str,
        _key_type: &KeyType,
        _strongbox_required: bool,
    ) -> BearDogResult<()> {
        warn!("⚠️ Using mock key generation (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        info!("✅ Mock key generated: {}", key_id);
        Ok(())
    }

    /// Mock signing for development
    async fn mock_sign(
        key_id: &str,
        data: &[u8],
        _algorithm: SigningAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        warn!("⚠️ Using mock signing (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;

        let mut signature = Vec::new();
        signature.extend_from_slice(b"MOCK_SIGNATURE:");
        signature.extend_from_slice(key_id.as_bytes());
        signature.extend_from_slice(b":");
        signature.extend_from_slice(&data[..std::cmp::min(32, data.len())]);

        Ok(signature)
    }

    /// Mock verification for development
    async fn mock_verify(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        _algorithm: SigningAlgorithm,
    ) -> BearDogResult<bool> {
        warn!("⚠️ Using mock verification (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(8)).await;

        let expected_prefix = format!("MOCK_SIGNATURE:{}:", key_id);
        if signature.len() < expected_prefix.len() + data.len().min(32) {
            return Ok(false);
        }

        let sig_str = String::from_utf8_lossy(signature);
        Ok(sig_str.starts_with(&expected_prefix))
    }

    /// Mock encryption for development
    async fn mock_encrypt(
        _key_id: &str,
        plaintext: &[u8],
        _algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        warn!("⚠️ Using mock encryption (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        let mut ciphertext = plaintext.to_vec();
        ciphertext.reverse();
        ciphertext.extend_from_slice(b"MOCK_ENC");

        Ok(ciphertext)
    }

    /// Mock decryption for development
    async fn mock_decrypt(
        _key_id: &str,
        ciphertext: &[u8],
        _algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<Vec<u8>> {
        warn!("⚠️ Using mock decryption (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        if ciphertext.len() < 8 || &ciphertext[ciphertext.len() - 8..] != b"MOCK_ENC" {
            return Err(BearDogError::Hsm {
                message: "Mock decryption failed - invalid format".to_string(),
            });
        }

        let mut plaintext = ciphertext[..ciphertext.len() - 8].to_vec();
        plaintext.reverse();

        Ok(plaintext)
    }

    /// Mock key deletion for development
    async fn mock_delete_key(key_id: &str) -> BearDogResult<()> {
        warn!("⚠️ Using mock key deletion (not on Android)");
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        info!("✅ Mock key deleted: {}", key_id);
        Ok(())
    }
}

/// Signing algorithm types for native operations
#[derive(Debug, Clone, Copy)]
pub enum SigningAlgorithm {
    EcdsaSha256,
    RsaPssSha256,
    RsaPkcs1Sha256,
}

/// Encryption algorithm types for native operations
#[derive(Debug, Clone, Copy)]
pub enum EncryptionAlgorithm {
    AesGcm,
    RsaOaep,
}
