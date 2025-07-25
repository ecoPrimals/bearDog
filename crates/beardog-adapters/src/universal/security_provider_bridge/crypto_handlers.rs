//! Cryptographic operation handlers for the security provider bridge

use super::SecurityProviderBridge;

// Import the correct types from adapters::universal to match the trait
use crate::adapters::universal::{UniversalRequest, UniversalResponse};
use base64::{engine::general_purpose, Engine as _};
use beardog_security::crypto_utils::BearDogCrypto;
use serde_json::json;

impl SecurityProviderBridge {
    /// Handle cryptographic operations
    pub async fn handle_crypto_operation(&self, request: &UniversalRequest) -> UniversalResponse {
        match request.operation.as_str() {
            "ed25519_sign" => self.handle_ed25519_sign(request).await,
            "ed25519_verify" => self.handle_ed25519_verify(request).await,
            "aes_encrypt" => self.handle_aes_encrypt(request).await,
            "aes_decrypt" => self.handle_aes_decrypt(request).await,
            _ => self.error_response(
                request.system_id.clone(),
                "UNSUPPORTED_OPERATION",
                &format!("Unsupported crypto operation: {}", request.operation),
            ),
        }
    }

    /// Handle key management operations
    pub async fn handle_key_management_operation(
        &self,
        request: &UniversalRequest,
    ) -> UniversalResponse {
        match request.operation.as_str() {
            "generate_key" => self.handle_generate_key(request).await,
            "derive_key" => self.handle_derive_key(request).await,
            "generate_address" => self.handle_generate_address(request).await,
            _ => self.error_response(
                request.system_id.clone(),
                "unsupported_key_operation",
                &format!(
                    "Key management operation '{}' not supported",
                    request.operation
                ),
            ),
        }
    }

    /// Handle Ed25519 signing
    async fn handle_ed25519_sign(&self, request: &UniversalRequest) -> UniversalResponse {
        // Extract parameters from request payload
        let payload = &request.payload;

        let private_key_b64 = match payload.get("private_key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_PRIVATE_KEY",
                    "Private key is required for signing",
                )
            }
        };

        let message_b64 = match payload.get("message").and_then(|v| v.as_str()) {
            Some(msg) => msg,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_MESSAGE",
                    "Message is required for signing",
                )
            }
        };

        // Decode base64 inputs
        let private_key = match general_purpose::STANDARD.decode(private_key_b64) {
            Ok(key) => key,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_PRIVATE_KEY",
                    "Private key must be valid base64",
                )
            }
        };

        let message = match general_purpose::STANDARD.decode(message_b64) {
            Ok(msg) => msg,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_MESSAGE",
                    "Message must be valid base64",
                )
            }
        };

        // Perform signing
        match BearDogCrypto::sign_ed25519(&private_key, &message) {
            Ok(signature) => UniversalResponse {
                success: true,
                payload: json!({
                    "signature": general_purpose::STANDARD.encode(signature)
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "SIGNING_FAILED",
                &format!("Ed25519 signing failed: {}", e),
            ),
        }
    }

    /// Handle Ed25519 verification
    async fn handle_ed25519_verify(&self, request: &UniversalRequest) -> UniversalResponse {
        let payload = &request.payload;

        let public_key_b64 = match payload.get("public_key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_PUBLIC_KEY",
                    "Public key is required for verification",
                )
            }
        };

        let message_b64 = match payload.get("message").and_then(|v| v.as_str()) {
            Some(msg) => msg,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_MESSAGE",
                    "Message is required for verification",
                )
            }
        };

        let signature_b64 = match payload.get("signature").and_then(|v| v.as_str()) {
            Some(sig) => sig,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_SIGNATURE",
                    "Signature is required for verification",
                )
            }
        };

        // Decode base64 inputs
        let public_key = match general_purpose::STANDARD.decode(public_key_b64) {
            Ok(key) => key,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_PUBLIC_KEY",
                    "Public key must be valid base64",
                )
            }
        };

        let message = match general_purpose::STANDARD.decode(message_b64) {
            Ok(msg) => msg,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_MESSAGE",
                    "Message must be valid base64",
                )
            }
        };

        let signature = match general_purpose::STANDARD.decode(signature_b64) {
            Ok(sig) => sig,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_SIGNATURE",
                    "Signature must be valid base64",
                )
            }
        };

        // Perform verification
        match BearDogCrypto::verify_ed25519_signature(&public_key, &message, &signature) {
            Ok(is_valid) => UniversalResponse {
                success: true,
                payload: json!({
                    "verified": is_valid
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "VERIFICATION_FAILED",
                &format!("Ed25519 verification failed: {}", e),
            ),
        }
    }

    /// Handle AES encryption
    async fn handle_aes_encrypt(&self, request: &UniversalRequest) -> UniversalResponse {
        let payload = &request.payload;

        let key_b64 = match payload.get("key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_KEY",
                    "Key is required for encryption",
                )
            }
        };

        let plaintext_b64 = match payload.get("plaintext").and_then(|v| v.as_str()) {
            Some(text) => text,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_PLAINTEXT",
                    "Plaintext is required for encryption",
                )
            }
        };

        // Decode base64 inputs
        let key = match general_purpose::STANDARD.decode(key_b64) {
            Ok(k) => k,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_KEY",
                    "Key must be valid base64",
                )
            }
        };

        let plaintext = match general_purpose::STANDARD.decode(plaintext_b64) {
            Ok(text) => text,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_PLAINTEXT",
                    "Plaintext must be valid base64",
                )
            }
        };

        // Generate nonce (12 bytes for AES-GCM)
        let nonce = match BearDogCrypto::generate_secure_nonce(12) {
            Ok(n) => n,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "NONCE_GENERATION_FAILED",
                    "Failed to generate encryption nonce",
                )
            }
        };

        // Perform encryption
        match BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(&nonce)) {
            Ok((ciphertext, nonce_used)) => UniversalResponse {
                success: true,
                payload: json!({
                   "ciphertext": general_purpose::STANDARD.encode(ciphertext),
                    "nonce": general_purpose::STANDARD.encode(nonce_used)
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "ENCRYPTION_FAILED",
                &format!("AES encryption failed: {}", e),
            ),
        }
    }

    /// Handle AES decryption
    async fn handle_aes_decrypt(&self, request: &UniversalRequest) -> UniversalResponse {
        let payload = &request.payload;

        let key_b64 = match payload.get("key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_KEY",
                    "Key is required for decryption",
                )
            }
        };

        let ciphertext_b64 = match payload.get("ciphertext").and_then(|v| v.as_str()) {
            Some(text) => text,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_CIPHERTEXT",
                    "Ciphertext is required for decryption",
                )
            }
        };

        let nonce_b64 = match payload.get("nonce").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_NONCE",
                    "Nonce is required for decryption",
                )
            }
        };

        // Decode base64 inputs
        let key = match general_purpose::STANDARD.decode(key_b64) {
            Ok(k) => k,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_KEY",
                    "Key must be valid base64",
                )
            }
        };

        let ciphertext = match general_purpose::STANDARD.decode(ciphertext_b64) {
            Ok(text) => text,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_CIPHERTEXT",
                    "Ciphertext must be valid base64",
                )
            }
        };

        let nonce = match general_purpose::STANDARD.decode(nonce_b64) {
            Ok(n) => n,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_NONCE",
                    "Nonce must be valid base64",
                )
            }
        };

        // Perform decryption
        match BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce) {
            Ok(plaintext) => UniversalResponse {
                success: true,
                payload: json!({
                    "plaintext": general_purpose::STANDARD.encode(plaintext)
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "DECRYPTION_FAILED",
                &format!("AES decryption failed: {}", e),
            ),
        }
    }

    /// Handle key generation
    async fn handle_generate_key(&self, request: &UniversalRequest) -> UniversalResponse {
        let key_type = request
            .payload
            .get("key_type")
            .and_then(|v| v.as_str())
            .unwrap_or("ed25519");

        match key_type {
            "ed25519" => match BearDogCrypto::generate_ed25519_keypair() {
                Ok((private_key, public_key)) => UniversalResponse {
                    success: true,
                    payload: json!({
                        "private_key": general_purpose::STANDARD.encode(private_key),
                        "public_key": general_purpose::STANDARD.encode(public_key),
                        "key_type": "ed25519"
                    }),
                    metadata: std::collections::HashMap::new(),
                    processing_time_ms: 0,
                    system_id: request.system_id.clone(),
                    operation: request.operation.clone(),
                },
                Err(e) => self.error_response(
                    request.system_id.clone(),
                    "KEY_GENERATION_FAILED",
                    &format!("Ed25519 key generation failed: {}", e),
                ),
            },
            _ => self.error_response(
                request.system_id.clone(),
                "UNSUPPORTED_KEY_TYPE",
                &format!("Key type '{}' not supported", key_type),
            ),
        }
    }

    /// Handle key derivation
    async fn handle_derive_key(&self, request: &UniversalRequest) -> UniversalResponse {
        // Extract master key and derivation path from request parameters
        let payload = &request.payload;

        let master_key_b64 = match payload.get("master_key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_MASTER_KEY",
                    "Master key is required for derivation",
                )
            }
        };

        let derivation_path = match payload.get("derivation_path").and_then(|v| v.as_str()) {
            Some(path) => path,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_DERIVATION_PATH",
                    "Derivation path is required",
                )
            }
        };

        // Decode base64 inputs
        let master_key = match general_purpose::STANDARD.decode(master_key_b64) {
            Ok(key) => key,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_MASTER_KEY",
                    "Master key must be valid base64",
                )
            }
        };

        let salt = derivation_path.as_bytes();

        // Use path as salt for PBKDF2 key derivation
        match BearDogCrypto::derive_key_pbkdf2(&master_key, salt, 100_000, 32) {
            Ok(derived_key) => UniversalResponse {
                success: true,
                payload: json!({
                    "derived_key": general_purpose::STANDARD.encode(derived_key)
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "KEY_DERIVATION_FAILED",
                &format!("Key derivation failed: {}", e),
            ),
        }
    }

    /// Handle address generation
    async fn handle_generate_address(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::address_management::{AddressFormat, AddressManager};

        // Extract public key and address type from request parameters
        let payload = &request.payload;

        let public_key_b64 = match payload.get("public_key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_PUBLIC_KEY",
                    "Public key is required for address generation",
                )
            }
        };

        let address_type = match payload.get("address_type").and_then(|v| v.as_str()) {
            Some(at) => at,
            None => {
                return self.error_response(
                    request.system_id.clone(),
                    "MISSING_ADDRESS_TYPE",
                    "Address type is required",
                )
            }
        };

        // Decode base64 inputs
        let public_key = match general_purpose::STANDARD.decode(public_key_b64) {
            Ok(key) => key,
            Err(_) => {
                return self.error_response(
                    request.system_id.clone(),
                    "INVALID_PUBLIC_KEY",
                    "Public key must be valid base64",
                )
            }
        };

        let format = match address_type {
            "bitcoin" => AddressFormat::BitcoinLegacy,
            "ethereum" => AddressFormat::Ethereum,
            "beardog" => AddressFormat::BearDogNative,
            _ => {
                return self.error_response(
                    request.system_id.clone(),
                    "UNSUPPORTED_ADDRESS_TYPE",
                    &format!("Unsupported address type: {}", address_type),
                )
            }
        };

        let mut address_manager = AddressManager::new();
        match address_manager.generate_address_from_ed25519(&public_key, format) {
            Ok(address) => UniversalResponse {
                success: true,
                payload: json!({
                    "address": address,
                    "address_type": address_type
                }),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            },
            Err(e) => self.error_response(
                request.system_id.clone(),
                "ADDRESS_GENERATION_FAILED",
                &format!("Address generation failed: {}", e),
            ),
        }
    }
}
