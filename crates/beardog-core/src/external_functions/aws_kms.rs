

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;

use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Clone)]
pub struct AwsKmsIntegration;

impl ExternalFunctionHandler for AwsKmsIntegration {}

    fn function_name(&self) -> &str {
        "aws_kms"
    }

    async fn execute(
        &self,
        license_manager: &LicenseManager,
        operation: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {

        if !license_manager
            .is_function_available(self.function_name())
            .await?
        {
            return Err(BearDogError::configuration(format!(
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
                "error": format_args!("Unknown KMS operation: {}", operation).to_string(),
                "available_operations": ["encrypt", "decrypt", "list_keys", "create_key"]
            })),
}
impl AwsKmsIntegration {

    async fn kms_encrypt(&self, key_id: &str, plaintext: &str) -> Result<String, BearDogError> {

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

    async fn kms_decrypt(&self, key_id: &str, ciphertext_blob: &str) -> Result<String, BearDogError> {

                "decrypt",
                "--ciphertext-blob",
                ciphertext_blob,

                if let Some(plaintext) = json.get("Plaintext").and_then(|v| v.as_str()) {
                    Ok(plaintext.to_string())
                    Err(BearDogError::configuration("No Plaintext in KMS decrypt response".to_string(),
                tracing::error!("Failed to parse KMS decrypt response: {}", parse_err);
                Err(BearDogError::configuration(format!("Failed to parse KMS decrypt response: {parse_err)"},

    async fn kms_list_keys(&self) -> Result<Vec<Value>, BearDogError> {
            .aws_cli_exec(&["kms", "list-keys", "--output", "json"])
                if let Some(keys) = json.get("Keys").and_then(|v| v.as_array()) {
                    Ok(keys.clone())

                    Err(BearDogError::External {
                        message: "AWS KMS response missing 'Keys' field - refusing to use mock keys for security".to_string(),
                    })
            Err(parse_error) => {

                tracing::error!("AWS KMS list-keys failed with parse error: {}", parse_error);
                Err(BearDogError::External {
                    message: format_args!("Failed to parse AWS KMS response: {} - refusing to use mock keys for security", parse_error).to_string(),

    async fn kms_create_key(&self, description: &str) -> Result<Value, BearDogError> {
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
                    message: format_args!("Failed to parse AWS KMS create-key response: {} - refusing to use mock keys for security", parse_error).to_string(),

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
