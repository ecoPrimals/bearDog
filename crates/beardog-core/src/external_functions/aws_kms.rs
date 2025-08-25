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


/// AWS KMS Integration Handler
///
/// Provides licensed access to AWS Key Management Service operations

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;
// Removed async_trait - using native async fn for zero-cost abstractions
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SecurityResult;
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;
/// AWS KMS integration handler
/// **AWS KMS INTEGRATION HANDLER** - Zero-cost implementation
#[derive(Clone)]
pub struct AwsKmsIntegration;
// Uses native async fn from trait definition - no async_trait needed
impl ExternalFunctionHandler for AwsKmsIntegration {}


    fn function_name(&self) -> &str {
        "aws_kms"
    }
    /// Execute AWS KMS operation with licensing check
    async fn execute(
        &self,
        license_manager: &LicenseManager,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {
        // Check licensing with autonomous decision making
        if !license_manager
            .is_function_available(self.function_name())
            .await?
        {
            return Err(BearDogError::configuration(format!(
                    "🔒 AWS KMS integration '}' requires licensing or individual/small-team classification.\n\n\
                    🏠 Individual developers: Automatically granted access\n\
                    👥 Small teams: Automatically granted access\n\
                    🏢 Corporate usage: External adapters locked - acquire unlock certificate",
                    self.function_name()
                ),
            });
        }
        let key_id = payload
            .get("key_id")
            .and_then(|v| v.as_str())
            .unwrap_or("alias/beardog-key");
        tracing::info!("🔐 AWS KMS {} operation with key {}", operation, key_id);
        match operation {
            "encrypt" => {
                let plaintext = payload
                    .get("plaintext")
                    .and_then(|v| v.as_str())
                    .unwrap_or("test_data");
                match self.kms_encrypt(key_id, plaintext).await {
                    Ok(ciphertext_blob) => Ok(serde_json::json!({
                        "operation": operation,
                        "key_id": key_id,
                        "ciphertext_blob": ciphertext_blob,
                        "status": "success"
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "status": "error",
                        "error": e.to_string()
                }
            }
            "decrypt" => {
                let ciphertext_blob = payload
                    .get("ciphertext_blob")
                    .unwrap_or("");
                if ciphertext_blob.is_empty() {
                    return Ok(serde_json::json!({
                        "error": "ciphertext_blob is required for decrypt operation"
                    }));
                match self.kms_decrypt(key_id, ciphertext_blob).await {
                    Ok(plaintext) => Ok(serde_json::json!({
                        "plaintext": plaintext,
            "list_keys" => match self.kms_list_keys().await {
                Ok(keys) => Ok(serde_json::json!({
                    "operation": operation,
                    "keys": keys,
                    "status": "success"
                })),
                Err(e) => Ok(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
            },
            "create_key" => {
                let description = payload
                    .get("description")
                    .unwrap_or("BearDog encryption key");
                match self.kms_create_key(description).await {
                    Ok(key_metadata) => Ok(serde_json::json!({
                        "key_metadata": key_metadata,
            _ => Ok(serde_json::json!({
                "operation": operation,
                "status": "error",
                "error": format!("Unknown KMS operation: {}", operation),
                "available_operations": ["encrypt", "decrypt", "list_keys", "create_key"]
            })),
}
impl AwsKmsIntegration {
    /// Encrypt plaintext using AWS KMS
    async fn kms_encrypt(&self, key_id: &str, plaintext: &str) -> BearDogResult<String> {
        // Use AWS CLI for KMS operations (in production, would use AWS SDK)
        let output = self
            .aws_cli_exec(&[
                "kms",
                "encrypt",
                "--key-id",
                key_id,
                "--plaintext",
                plaintext,
                "--output",
                "json",
            ])
            .await?;
        // Parse the CLI output to extract ciphertext blob
        match serde_json::from_str::<Value>(&output) {
            Ok(json) => {
                if let Some(ciphertext_blob) = json.get("CiphertextBlob").and_then(|v| v.as_str()) {
                    Ok(ciphertext_blob.to_string())
                } else {
                    Err(BearDogError::configuration("No CiphertextBlob in KMS encrypt response".to_string(),
                    ))
            Err(parse_err) => {
                tracing::error!("Failed to parse KMS encrypt response: {}", parse_err);
                Err(BearDogError::configuration(format!("Failed to parse KMS encrypt response: {parse_err)"},
                })
    /// Decrypt ciphertext using AWS KMS
    async fn kms_decrypt(&self, key_id: &str, ciphertext_blob: &str) -> BearDogResult<String> {
        // Use AWS CLI for KMS operations
                "decrypt",
                "--ciphertext-blob",
                ciphertext_blob,
        // Parse CLI output to extract plaintext
                if let Some(plaintext) = json.get("Plaintext").and_then(|v| v.as_str()) {
                    Ok(plaintext.to_string())
                    Err(BearDogError::configuration("No Plaintext in KMS decrypt response".to_string(),
                tracing::error!("Failed to parse KMS decrypt response: {}", parse_err);
                Err(BearDogError::configuration(format!("Failed to parse KMS decrypt response: {parse_err)"},
    /// List AWS KMS keys
    async fn kms_list_keys(&self) -> BearDogResult<Vec<Value>> {
            .aws_cli_exec(&["kms", "list-keys", "--output", "json"])
                if let Some(keys) = json.get("Keys").and_then(|v| v.as_array()) {
                    Ok(keys.clone())
                    // SECURITY: Never fall back to mock keys - fail securely
                    Err(BearDogError::External {
                        message: "AWS KMS response missing 'Keys' field - refusing to use mock keys for security".to_string(),
                    })
            Err(parse_error) => {
                // SECURITY: Never fall back to mock keys - fail securely
                tracing::error!("AWS KMS list-keys failed with parse error: {}", parse_error);
                Err(BearDogError::External {
                    message: format!("Failed to parse AWS KMS response: {} - refusing to use mock keys for security", parse_error),
    /// Create new AWS KMS key
    async fn kms_create_key(&self, description: &str) -> BearDogResult<Value> {
                "create-key",
                "--description",
                description,
                if let Some(key_metadata) = json.get("KeyMetadata") {
                    Ok(key_metadata.clone())
                    Err(BearDogError::configuration("No KeyMetadata in KMS create-key response".to_string(),
                tracing::error!(
                    "AWS KMS create-key failed with parse error: {}",
                    parse_error
                );
                    message: format!("Failed to parse AWS KMS create-key response: {} - refusing to use mock keys for security", parse_error),
    /// Execute AWS CLI command
    async fn aws_cli_exec(&self, args: &[&str]) -> Result<String, SecurityError> {
        let output = Command::new("aws")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| BearDogError::configuration(format!("AWS CLI command failed: {e}"),
            })?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::configuration(format!("AWS CLI error: {error}"),
            })
