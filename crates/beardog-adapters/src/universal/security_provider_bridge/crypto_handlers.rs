// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Cryptographic operation handlers for the security provider bridge

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
            "generate_key" => self.handle_generate_key(request).await,
            "derive_key" => self.handle_derive_key(request).await,
            "generate_address" => self.handle_generate_address(request).await,
                "unsupported_key_operation",
                &format!(
                    "Key management operation '{}' not supported",
                    request.operation
                ),
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
                    "MISSING_MESSAGE",
                    "Message is required for signing",
        // Decode base64 inputs
        let private_key = match general_purpose::STANDARD.decode(private_key_b64) {
            Ok(key) => key,
            Err(_) => {
                    "INVALID_PRIVATE_KEY",
                    "Private key must be valid base64",
        let message = match general_purpose::STANDARD.decode(message_b64) {
            Ok(msg) => msg,
                    "INVALID_MESSAGE",
                    "Message must be valid base64",
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
                "SIGNING_FAILED",
                &format!("Ed25519 signing failed: {}", e),
    /// Handle Ed25519 verification
    async fn handle_ed25519_verify(&self, request: &UniversalRequest) -> UniversalResponse {
        let public_key_b64 = match payload.get("public_key").and_then(|v| v.as_str()) {
                    "MISSING_PUBLIC_KEY",
                    "Public key is required for verification",
                    "Message is required for verification",
        let signature_b64 = match payload.get("signature").and_then(|v| v.as_str()) {
            Some(sig) => sig,
                    "MISSING_SIGNATURE",
                    "Signature is required for verification",
        let public_key = match general_purpose::STANDARD.decode(public_key_b64) {
                    "INVALID_PUBLIC_KEY",
                    "Public key must be valid base64",
        let signature = match general_purpose::STANDARD.decode(signature_b64) {
            Ok(sig) => sig,
                    "INVALID_SIGNATURE",
                    "Signature must be valid base64",
        // Perform verification
        match BearDogCrypto::verify_ed25519_signature(&public_key, &message, &signature) {
            Ok(is_valid) => UniversalResponse {
                    "verified": is_valid
                "VERIFICATION_FAILED",
                &format!("Ed25519 verification failed: {}", e),
    /// Handle AES encryption
    async fn handle_aes_encrypt(&self, request: &UniversalRequest) -> UniversalResponse {
        let key_b64 = match payload.get("key").and_then(|v| v.as_str()) {
                    "MISSING_KEY",
                    "Key is required for encryption",
        let plaintext_b64 = match payload.get("plaintext").and_then(|v| v.as_str()) {
            Some(text) => text,
                    "MISSING_PLAINTEXT",
                    "Plaintext is required for encryption",
        let key = match general_purpose::STANDARD.decode(key_b64) {
            Ok(k) => k,
                    "INVALID_KEY",
                    "Key must be valid base64",
        let plaintext = match general_purpose::STANDARD.decode(plaintext_b64) {
            Ok(text) => text,
                    "INVALID_PLAINTEXT",
                    "Plaintext must be valid base64",
        // Generate nonce (12 bytes for AES-GCM)
        let nonce = match BearDogCrypto::generate_secure_nonce(12) {
            Ok(n) => n,
                    "NONCE_GENERATION_FAILED",
                    "Failed to generate encryption nonce",
        // Perform encryption
        match BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(&nonce)) {
            Ok((ciphertext, nonce_used)) => UniversalResponse {
                   "ciphertext": general_purpose::STANDARD.encode(ciphertext),
                    "nonce": general_purpose::STANDARD.encode(nonce_used)
                "ENCRYPTION_FAILED",
                &format!("AES encryption failed: {}", e),
    /// Handle AES decryption
    async fn handle_aes_decrypt(&self, request: &UniversalRequest) -> UniversalResponse {
                    "Key is required for decryption",
        let ciphertext_b64 = match payload.get("ciphertext").and_then(|v| v.as_str()) {
                    "MISSING_CIPHERTEXT",
                    "Ciphertext is required for decryption",
        let nonce_b64 = match payload.get("nonce").and_then(|v| v.as_str()) {
            Some(n) => n,
                    "MISSING_NONCE",
                    "Nonce is required for decryption",
        let ciphertext = match general_purpose::STANDARD.decode(ciphertext_b64) {
                    "INVALID_CIPHERTEXT",
                    "Ciphertext must be valid base64",
        let nonce = match general_purpose::STANDARD.decode(nonce_b64) {
                    "INVALID_NONCE",
                    "Nonce must be valid base64",
        // Perform decryption
        match BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce) {
            Ok(plaintext) => UniversalResponse {
                    "plaintext": general_purpose::STANDARD.encode(plaintext)
                "DECRYPTION_FAILED",
                &format!("AES decryption failed: {}", e),
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
                    "KEY_GENERATION_FAILED",
                    &format!("Ed25519 key generation failed: {}", e),
                "UNSUPPORTED_KEY_TYPE",
                &format!("Key type '{}' not supported", key_type),
    /// Handle key derivation
    async fn handle_derive_key(&self, request: &UniversalRequest) -> UniversalResponse {
        // Extract master key and derivation path from request parameters
        let master_key_b64 = match payload.get("master_key").and_then(|v| v.as_str()) {
                    "MISSING_MASTER_KEY",
                    "Master key is required for derivation",
        let derivation_path = match payload.get("derivation_path").and_then(|v| v.as_str()) {
            Some(path) => path,
                    "MISSING_DERIVATION_PATH",
                    "Derivation path is required",
        let master_key = match general_purpose::STANDARD.decode(master_key_b64) {
                    "INVALID_MASTER_KEY",
                    "Master key must be valid base64",
        let salt = derivation_path.as_bytes();
        // Use path as salt for PBKDF2 key derivation
        match BearDogCrypto::derive_key_pbkdf2(&master_key, salt, 100_000, 32) {
            Ok(derived_key) => UniversalResponse {
                    "derived_key": general_purpose::STANDARD.encode(derived_key)
                "KEY_DERIVATION_FAILED",
                &format!("Key derivation failed: {}", e),
    /// Handle address generation
    async fn handle_generate_address(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::address_management::{AddressFormat, AddressManager};
        // Extract public key and address type from request parameters
                    "Public key is required for address generation",
        let address_type = match payload.get("address_type").and_then(|v| v.as_str()) {
            Some(at) => at,
                    "MISSING_ADDRESS_TYPE",
                    "Address type is required",
        let format = match address_type {
            "bitcoin" => AddressFormat::BitcoinLegacy,
            "ethereum" => AddressFormat::Ethereum,
            "beardog" => AddressFormat::BearDogNative,
            _ => {
                    "UNSUPPORTED_ADDRESS_TYPE",
                    &format!("Unsupported address type: {}", address_type),
        let mut address_manager = AddressManager::new();
        match address_manager.generate_address_from_ed25519(&public_key, format) {
            Ok(address) => UniversalResponse {
                    "address": address,
                    "address_type": address_type
                "ADDRESS_GENERATION_FAILED",
                &format!("Address generation failed: {}", e),
}
