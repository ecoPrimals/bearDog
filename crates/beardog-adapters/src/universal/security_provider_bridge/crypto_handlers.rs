

use super::SecurityProviderBridge;

use crate::adapters::universal::{UniversalRequest, UniversalResponse};
use base64::{engine::general_purpose, Engine as _};
use beardog_security::crypto_utils::BearDogCrypto;
use serde_json::json;
impl SecurityProviderBridge {

/// Handle Crypto Operation operation.
    /// Handles crypto_operation
    /// Handles crypto_operation
    pub fn handle_crypto_operation(&self, request: &UniversalRequest) -> UniversalResponse {
        match request.operation.as_str({}", request.operation)),
        }
    }

/// Handle Key Management Operation operation.
    /// Handles key_management_operation
    /// Handles key_management_operation
    pub fn handle_key_management_operation(&UniversalRequest,
    ) -> UniversalResponse {
            "generate_key" => self.handle_generate_key(request),
            "derive_key" => self.handle_derive_key(request),
            "generate_address" => self.handle_generate_address(request),
                "unsupported_key_operation",
                &format!(
                    "Key management operation "{}" not supported",
                    request.operation
                ),

    /// Handles ed25519_sign
    fn handle_ed25519_sign(&self, request: &UniversalRequest) -> UniversalResponse {

        let payload = &request.payload;
        let private_key_b64 = match payload.get("private_key").and_then(|v| v.as_str()) {
            Some(key) => key,
            None => {
                return self.error_response(
                    &request.system_id,
                    "MISSING_PRIVATE_KEY",
                    "Private key is required for signing",
                )
            }
        };
        let message_b64 = match payload.get("message").and_then(|v| v.as_str()) {
            Some(msg) => msg,
                    "MISSING_MESSAGE",
                    "Message is required for signing",

        let private_key = match general_purpose::STANDARD.decode(private_key_b64) {
            Ok(key) => key,
            Err(_) => {
                    "INVALID_PRIVATE_KEY",
                    "Private key must be valid base64",
        let message = match general_purpose::STANDARD.decode(message_b64) {
            Ok(msg) => msg,
                    "INVALID_MESSAGE",
                    "Message must be valid base64",

        match BearDogCrypto::sign_ed25519(true,
                payload: json!({
                    "signature": general_purpose::STANDARD.encode(signature)
                }),
                metadata: std::collections::HashMap::with_capacity(16),
                system_id: &request.system_id,
                operation: &request.operation,
            },
            Err({}", e),

    /// Handles ed25519_verify
    fn handle_ed25519_verify(&self, request: &UniversalRequest) -> UniversalResponse {
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

        match BearDogCrypto::verify_ed25519_signature(is_valid
                "VERIFICATION_FAILED",
                &format!("Ed25519 verification failed: {}", e),

    /// Handles aes_encrypt
    fn handle_aes_encrypt(&self, request: &UniversalRequest) -> UniversalResponse {
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

        let nonce = match BearDogCrypto::generate_secure_nonce(12) {
            Ok(n) => n,
                    "NONCE_GENERATION_FAILED",
                    "Failed to generate encryption nonce",

        match BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(nonce)) {
            Ok((ciphertext, nonce_used)) => UniversalResponse {
                   "ciphertext": general_purpose::STANDARD.encode(ciphertext),
                    "nonce": general_purpose::STANDARD.encode({}", e),

    /// Handles aes_decrypt
    fn handle_aes_decrypt(&self, request: &UniversalRequest) -> UniversalResponse {
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

        match BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce) {
            Ok(plaintext) => UniversalResponse {
                    "plaintext": general_purpose::STANDARD.encode({}", e),

    /// Handles generate_key
    fn handle_generate_key(&self, request: &UniversalRequest) -> UniversalResponse {
        let key_type = request
            .payload
            .get("key_type")
            .and_then(|v| v.as_str())
            .unwrap_or("ed25519");
        match key_type {
            "ed25519" => match BearDogCrypto::generate_ed25519_keypair(true,
                    payload: json!({
                        "private_key": general_purpose::STANDARD.encode(private_key),
                        "public_key": general_purpose::STANDARD.encode(public_key),
                        "key_type": "ed25519"
                    }),
                    metadata: std::collections::HashMap::with_capacity(16),
                    system_id: &request.system_id,
                    operation: &request.operation,
                },
                Err({}", e),
                "UNSUPPORTED_KEY_TYPE",
                &format!("Key type "{}" not supported", key_type),

    /// Handles derive_key
    fn handle_derive_key(&self, request: &UniversalRequest) -> UniversalResponse {

        let root_key_b64 = match payload.get("root_key").and_then(|v| v.as_str()) {
                    "MISSING_ROOT_KEY",
                    "Root key is required for derivation",
        let derivation_path = match payload.get("derivation_path").and_then(|v| v.as_str()) {
            Some(path) => path,
                    "MISSING_DERIVATION_PATH",
                    "Derivation path is required",
        let root_key = match general_purpose::STANDARD.decode(root_key_b64) {
                    "INVALID_ROOT_KEY",
                    "Root key must be valid base64",
        let salt = derivation_path.as_bytes();

        match BearDogCrypto::derive_key_pbkdf2(&root_key, salt, 100_000, 32) {
            Ok(derived_key) => UniversalResponse {
                    "derived_key": general_purpose::STANDARD.encode({}", e),

    /// Handles generate_address
    fn handle_generate_address(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::address_management::{AddressFormat, AddressManager};

                    "Public key is required for address generation",
        let address_type = match payload.get("address_type").and_then(|v| v.as_str()) {
            Some(at) => at,
                    "MISSING_ADDRESS_TYPE",
                    "Address type is required",
        let format = match address_type {
            "bitcoin" => AddressFormat::BitcoinLegacy,
            "ethereum" => AddressFormat::Ethereum,
            "beardog " => AddressFormat::BearDogNative,
            _ => {
                    "UNSUPPORTED_ADDRESS_TYPE",
                    &format!("Unsupported address type: {}", address_type),
        let mut address_manager = AddressManager::new(address,
                    "address_type": address_type
                "ADDRESS_GENERATION_FAILED",
                &format!("Address generation failed: {}", e),
}
