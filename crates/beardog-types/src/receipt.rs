// SPDX-License-Identifier: AGPL-3.0-only

// Universal Operation Receipt System
// Provides verifiable proof of operation execution for all BearDog operations

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// Universal operation receipt for all BearDog operations
/// Provides verifiable proof of operation execution with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationReceipt {
    /// Unique receipt identifier (UUID v4)
    pub receipt_id: String,

    /// Operation type (key-generate, key-derive, key-mix, encrypt, decrypt, etc.)
    pub operation: String,

    /// ISO 8601 timestamp of operation
    pub timestamp: String,

    /// Operation result (success or failure with error)
    pub result: OperationResult,

    /// Key details (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_info: Option<KeyInfo>,

    /// HSM details (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_info: Option<HsmInfo>,

    /// Optional metadata (extensible)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Cryptographic signature for verification (future)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Chain reference to previous receipt (for lineage tracking)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_receipt_id: Option<String>,
}

/// Operation result with error details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum OperationResult {
    /// Operation succeeded
    Success,
    /// Operation failed with error message
    Failure {
        /// Human-readable or serialized error description
        #[serde(rename = "error")]
        error: String,
    },
}

/// Key information included in receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Key identifier
    pub key_id: String,

    /// Cryptographic algorithm (AES-256-GCM, etc.)
    pub algorithm: String,

    /// Generation number (0 = root, 1+ = derived)
    pub generation: u32,

    /// Parent key ID (for derived keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_key_id: Option<String>,

    /// Expiration timestamp (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// Key usage restrictions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,

    /// Key purpose/derivation purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

/// HSM information included in receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    /// HSM name/identifier
    pub name: String,

    /// Vendor name (Yubico, SoloKeys, Google, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Model identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// HSM type (hardware, software, mobile)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
}

impl OperationReceipt {
    /// Create a new successful receipt for an operation
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            receipt_id: Uuid::new_v4().to_string(),
            operation: operation.into(),
            timestamp: Utc::now().to_rfc3339(),
            result: OperationResult::Success,
            key_info: None,
            hsm_info: None,
            metadata: HashMap::new(),
            signature: None,
            previous_receipt_id: None,
        }
    }

    /// Create a failed receipt with error message
    pub fn failure(operation: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            receipt_id: Uuid::new_v4().to_string(),
            operation: operation.into(),
            timestamp: Utc::now().to_rfc3339(),
            result: OperationResult::Failure {
                error: error.into(),
            },
            key_info: None,
            hsm_info: None,
            metadata: HashMap::new(),
            signature: None,
            previous_receipt_id: None,
        }
    }

    /// Builder: Add key information
    pub fn with_key_info(mut self, key_info: KeyInfo) -> Self {
        self.key_info = Some(key_info);
        self
    }

    /// Builder: Add HSM information
    pub fn with_hsm_info(mut self, hsm_info: HsmInfo) -> Self {
        self.hsm_info = Some(hsm_info);
        self
    }

    /// Builder: Add metadata field
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Builder: Add parent receipt ID (for operation chains)
    pub fn with_parent_receipt(mut self, parent_id: impl Into<String>) -> Self {
        self.previous_receipt_id = Some(parent_id.into());
        self
    }

    /// Save receipt to JSON file
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load receipt from JSON file
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let receipt: Self = serde_json::from_str(&json)?;
        receipt.validate()?;
        Ok(receipt)
    }

    /// Validate receipt structure and required fields
    pub fn validate(&self) -> Result<(), String> {
        if self.receipt_id.is_empty() {
            return Err("Missing receipt_id".to_string());
        }
        if self.operation.is_empty() {
            return Err("Missing operation".to_string());
        }
        if self.timestamp.is_empty() {
            return Err("Missing timestamp".to_string());
        }

        // Validate UUID format
        if Uuid::parse_str(&self.receipt_id).is_err() {
            return Err(format!("Invalid receipt_id UUID: {}", self.receipt_id));
        }

        // Validate ISO 8601 timestamp
        if chrono::DateTime::parse_from_rfc3339(&self.timestamp).is_err() {
            return Err(format!("Invalid ISO 8601 timestamp: {}", self.timestamp));
        }

        Ok(())
    }

    /// Check if operation was successful
    pub const fn is_success(&self) -> bool {
        matches!(self.result, OperationResult::Success)
    }

    /// Get error message if operation failed
    pub fn error_message(&self) -> Option<&str> {
        match &self.result {
            OperationResult::Failure { error } => Some(error),
            OperationResult::Success => None,
        }
    }
}

/// Helper to generate standardized receipt filename
pub fn generate_receipt_filename(operation: &str) -> String {
    format!(
        "receipt-{}-{}.json",
        operation,
        Utc::now().format("%Y%m%d-%H%M%S-%3f")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_creation() {
        let receipt = OperationReceipt::new("key-generate");
        assert_eq!(receipt.operation, "key-generate");
        assert!(receipt.is_success());
        assert!(receipt.validate().is_ok());
    }

    #[test]
    fn test_receipt_with_key_info() {
        let receipt = OperationReceipt::new("key-generate").with_key_info(KeyInfo {
            key_id: "test-key".to_string(),
            algorithm: "AES-256-GCM".to_string(),
            generation: 0,
            parent_key_id: None,
            expires_at: None,
            usage: None,
            purpose: None,
        });

        assert!(receipt.key_info.is_some());
        assert_eq!(receipt.key_info.unwrap().key_id, "test-key");
    }

    #[test]
    fn test_receipt_failure() {
        let receipt = OperationReceipt::failure("key-generate", "HSM not found");
        assert!(!receipt.is_success());
        assert_eq!(receipt.error_message(), Some("HSM not found"));
    }

    #[test]
    fn test_receipt_validation() {
        let receipt = OperationReceipt::new("test-operation");
        assert!(receipt.validate().is_ok());

        let invalid = OperationReceipt {
            receipt_id: "invalid-uuid".to_string(),
            operation: "test".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            result: OperationResult::Success,
            key_info: None,
            hsm_info: None,
            metadata: HashMap::new(),
            signature: None,
            previous_receipt_id: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_receipt_serialization() {
        use serde_json::json;

        let receipt =
            OperationReceipt::new("key-generate").with_metadata("test_key", json!("test_value"));

        let json = serde_json::to_string(&receipt).unwrap();
        let deserialized: OperationReceipt = serde_json::from_str(&json).unwrap();

        assert_eq!(receipt.receipt_id, deserialized.receipt_id);
        assert_eq!(receipt.operation, deserialized.operation);
    }
}
