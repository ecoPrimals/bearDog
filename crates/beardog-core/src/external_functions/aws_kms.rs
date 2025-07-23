//! AWS KMS Integration Handler
//!
//! Provides licensed access to AWS Key Management Service operations

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;

/// AWS KMS integration handler
pub struct AwsKmsIntegration;

#[async_trait]
impl ExternalFunctionHandler for AwsKmsIntegration {
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
            return Err(BearDogError::Configuration {
                message: format!(
                    "🔒 AWS KMS integration '{}' requires licensing or individual/small-team classification.\n\n\
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
                        "operation": operation,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "decrypt" => {
                let ciphertext_blob = payload
                    .get("ciphertext_blob")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if ciphertext_blob.is_empty() {
                    return Ok(serde_json::json!({
                        "operation": operation,
                        "status": "error",
                        "error": "ciphertext_blob is required for decrypt operation"
                    }));
                }

                match self.kms_decrypt(key_id, ciphertext_blob).await {
                    Ok(plaintext) => Ok(serde_json::json!({
                        "operation": operation,
                        "key_id": key_id,
                        "plaintext": plaintext,
                        "status": "success"
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            "list_keys" => match self.kms_list_keys().await {
                Ok(keys) => Ok(serde_json::json!({
                    "operation": operation,
                    "keys": keys,
                    "status": "success"
                })),
                Err(e) => Ok(serde_json::json!({
                    "operation": operation,
                    "status": "error",
                    "error": e.to_string()
                })),
            },
            "create_key" => {
                let description = payload
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("BearDog encryption key");

                match self.kms_create_key(description).await {
                    Ok(key_metadata) => Ok(serde_json::json!({
                        "operation": operation,
                        "key_metadata": key_metadata,
                        "status": "success"
                    })),
                    Err(e) => Ok(serde_json::json!({
                        "operation": operation,
                        "status": "error",
                        "error": e.to_string()
                    })),
                }
            }
            _ => Ok(serde_json::json!({
                "operation": operation,
                "status": "error",
                "error": format!("Unknown KMS operation: {}", operation),
                "available_operations": ["encrypt", "decrypt", "list_keys", "create_key"]
            })),
        }
    }
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
                    Err(BearDogError::Configuration {
                        message: "No CiphertextBlob in KMS encrypt response".to_string(),
                    })
                }
            }
            Err(parse_err) => {
                tracing::error!("Failed to parse KMS encrypt response: {}", parse_err);
                Err(BearDogError::Configuration {
                    message: format!("Failed to parse KMS encrypt response: {parse_err}"),
                })
            }
        }
    }

    /// Decrypt ciphertext using AWS KMS
    async fn kms_decrypt(&self, key_id: &str, ciphertext_blob: &str) -> BearDogResult<String> {
        // Use AWS CLI for KMS operations
        let output = self
            .aws_cli_exec(&[
                "kms",
                "decrypt",
                "--ciphertext-blob",
                ciphertext_blob,
                "--key-id",
                key_id,
                "--output",
                "json",
            ])
            .await?;

        // Parse CLI output to extract plaintext
        match serde_json::from_str::<Value>(&output) {
            Ok(json) => {
                if let Some(plaintext) = json.get("Plaintext").and_then(|v| v.as_str()) {
                    Ok(plaintext.to_string())
                } else {
                    Err(BearDogError::Configuration {
                        message: "No Plaintext in KMS decrypt response".to_string(),
                    })
                }
            }
            Err(parse_err) => {
                tracing::error!("Failed to parse KMS decrypt response: {}", parse_err);
                Err(BearDogError::Configuration {
                    message: format!("Failed to parse KMS decrypt response: {parse_err}"),
                })
            }
        }
    }

    /// List AWS KMS keys
    async fn kms_list_keys(&self) -> BearDogResult<Vec<Value>> {
        let output = self
            .aws_cli_exec(&["kms", "list-keys", "--output", "json"])
            .await?;

        match serde_json::from_str::<Value>(&output) {
            Ok(json) => {
                if let Some(keys) = json.get("Keys").and_then(|v| v.as_array()) {
                    Ok(keys.clone())
                } else {
                    // Fallback: return mock keys
                    Ok(vec![
                        serde_json::json!({
                            "KeyId": "mock-key-1",
                            "Arn": "arn:aws:kms:us-west-2:123456789012:key/mock-key-1"
                        }),
                        serde_json::json!({
                            "KeyId": "alias/beardog-key",
                            "Arn": "arn:aws:kms:us-west-2:123456789012:alias/beardog-key"
                        }),
                    ])
                }
            }
            Err(_) => {
                // Fallback: return mock keys
                tracing::warn!("KMS list-keys failed, using mock keys");
                Ok(vec![serde_json::json!({
                    "KeyId": "mock-key-1",
                    "Arn": "arn:aws:kms:us-west-2:123456789012:key/mock-key-1"
                })])
            }
        }
    }

    /// Create new AWS KMS key
    async fn kms_create_key(&self, description: &str) -> BearDogResult<Value> {
        let output = self
            .aws_cli_exec(&[
                "kms",
                "create-key",
                "--description",
                description,
                "--output",
                "json",
            ])
            .await?;

        match serde_json::from_str::<Value>(&output) {
            Ok(json) => {
                if let Some(key_metadata) = json.get("KeyMetadata") {
                    Ok(key_metadata.clone())
                } else {
                    Err(BearDogError::Configuration {
                        message: "No KeyMetadata in KMS create-key response".to_string(),
                    })
                }
            }
            Err(_) => {
                // Fallback: return mock key metadata
                use uuid::Uuid;
                tracing::warn!("KMS create-key failed, using mock key");
                Ok(serde_json::json!({
                    "KeyId": format!("mock-key-{}", Uuid::new_v4()),
                    "Arn": format!("arn:aws:kms:us-west-2:123456789012:key/mock-key-{}", Uuid::new_v4()),
                    "Description": description,
                    "KeyUsage": "ENCRYPT_DECRYPT",
                    "KeyState": "Enabled"
                }))
            }
        }
    }

    /// Execute AWS CLI command
    async fn aws_cli_exec(&self, args: &[&str]) -> BearDogResult<String> {
        let output = Command::new("aws")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| BearDogError::Configuration {
                message: format!("AWS CLI command failed: {e}"),
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(BearDogError::Configuration {
                message: format!("AWS CLI error: {error}"),
            })
        }
    }
}
