// Storage Operations Module

use super::types::{StorageOperation, StorageStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequest {
    /// Request identifier
    pub request_id: Uuid,
    /// Operation type
    /// The operation value
    pub operation: StorageOperation,
    /// Storage key/path
    /// The key value
    pub key: String,
    /// Optional data
    pub data: Option<Vec<u8>>,
    /// Request metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Request timeout in seconds
    pub timeout_secs: Option<u64>,
    /// Optional source key
    pub source_key: Option<String>,
    /// Optional destination key
    pub destination_key: Option<String>,
}

/// Storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResponse {
    /// Request identifier
    pub request_id: Uuid,
    /// Response status
    /// Current status of the component
    pub status: StorageStatus,
    /// Optional data
    pub data: Option<Vec<u8>>,
    /// Response metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Operation duration in milliseconds
    /// Number of `duration_ms`
    pub duration_ms: u64,
    /// Error message (if operation failed)
    /// Optional error message
    pub error_message: Option<String>,
    /// Optional storage location
    pub storage_location: Option<String>,
}

impl StorageRequest {
    /// Create a new storage request
    /// Creates a new instance
    pub fn new(operation: StorageOperation, key: String) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            operation,
            key,
            data: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            timeout_secs: None,
            source_key: None,
            destination_key: None,
        }
    }

    /// Create a store request
    pub fn store(key: String, data: Vec<u8>) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            operation: StorageOperation::Store,
            key,
            data: Some(data),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            timeout_secs: None,
            source_key: None,
            destination_key: None,
        }
    }

    /// Create a retrieve request
    pub fn retrieve(key: String) -> Self {
        Self::new(StorageOperation::Retrieve, key)
    }

    /// Create a delete request
    /// Removes
    /// Removes
    pub fn delete(key: String) -> Self {
        Self::new(StorageOperation::Delete, key)
    }

    /// Create a list request
    pub fn list(prefix: String) -> Self {
        Self::new(StorageOperation::List, prefix)
    }
}

impl StorageResponse {
    /// Create a success response
    pub fn success(request_id: Uuid, data: Option<Vec<u8>>) -> Self {
        Self {
            request_id,
            status: StorageStatus::Success,
            data,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
            error_message: None,
            storage_location: None,
        }
    }

    /// Create a failure response
    pub fn failure(request_id: Uuid, error_message: String) -> Self {
        Self {
            request_id,
            status: StorageStatus::Failed,
            data: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
            error_message: Some(error_message),
            storage_location: None,
        }
    }
}
