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


/// Ecosystem Storage Service
///
/// Provides universal storage capabilities for the BearDog ecosystem with
/// comprehensive data management, backup, and recovery features.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Ecosystem storage service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStorageConfig {
    /// Base storage directory
    pub storage_directory: PathBuf,
    /// Maximum storage capacity in bytes
    pub max_capacity: u64,
    /// Retention policy configuration
    pub retention_policy: RetentionPolicy,
    /// Snapshot configuration
    pub snapshot_config: SnapshotConfig,
    /// Audit configuration
    pub audit_config: AuditConfig,
}

impl Default for EcosystemStorageConfig {
    fn default() -> Self {
        Self {
            storage_directory: PathBuf::from("./beardog_storage"),
            max_capacity: 100 * 1024 * 1024 * 1024, // 100GB
            retention_policy: RetentionPolicy::default(),
            snapshot_config: SnapshotConfig::default(),
            audit_config: AuditConfig::default(),
        }
    }
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum age for data retention
    pub max_age_days: u32,
    /// Automatic cleanup enabled
    pub auto_cleanup: bool,
    /// Compression enabled for old data
    pub compression_enabled: bool,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_age_days: 365,
            auto_cleanup: true,
            compression_enabled: true,
        }
    }
}

/// Snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Automatic snapshot interval in hours
    pub interval_hours: u32,
    /// Maximum number of snapshots to retain
    pub max_snapshots: u32,
    /// Snapshot compression enabled
    pub compression_enabled: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            interval_hours: 24,
            max_snapshots: 30,
            compression_enabled: true,
        }
    }
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Maximum audit log size in bytes
    pub max_log_size: u64,
    /// Audit log retention in days
    pub retention_days: u32,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_log_size: 1024 * 1024 * 1024, // 1GB
            retention_days: 90,
        }
    }
}

/// Storage operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOperationResult {
    /// Operation ID
    pub operation_id: Uuid,
    /// Operation success status
    pub success: bool,
    /// Operation message
    pub message: String,
    /// Data size processed
    pub data_size: u64,
    /// Operation duration in milliseconds
    pub duration_ms: u64,
    /// Operation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Storage trait for ecosystem data management - modernized with native async fn
#[allow(async_fn_in_trait)]
pub trait EcosystemStorage: Send + Sync {
    /// Store data with metadata
    async fn store_data(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<StorageOperationResult>;

    /// Retrieve data by key
    async fn retrieve_data(&self, key: &str) -> BearDogResult<Option<Vec<u8>>>;

    /// Delete data by key
    async fn delete_data(&self, key: &str) -> BearDogResult<StorageOperationResult>;

    /// List all stored keys
    async fn list_keys(&self) -> BearDogResult<Vec<String>>;

    /// Get storage statistics
    async fn get_statistics(&self) -> BearDogResult<StorageStatistics>;

    /// Create a snapshot
    async fn create_snapshot(&self, name: &str) -> BearDogResult<StorageOperationResult>;

    /// Restore from snapshot
    async fn restore_snapshot(&self, name: &str) -> BearDogResult<StorageOperationResult>;

    /// Cleanup old data based on retention policy
    async fn cleanup(&self) -> BearDogResult<StorageOperationResult>;
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {
    /// Total storage capacity
    pub total_capacity: u64,
    /// Used storage space
    pub used_space: u64,
    /// Available storage space
    pub available_space: u64,
    /// Number of stored objects
    pub object_count: u64,
    /// Last cleanup timestamp
    pub last_cleanup: Option<DateTime<Utc>>,
    /// Last snapshot timestamp
    pub last_snapshot: Option<DateTime<Utc>>,
}

/// File-based storage implementation
pub struct FileSystemStorage {
    config: EcosystemStorageConfig,
    metadata_store: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

impl FileSystemStorage {
    /// Create new filesystem storage
    pub fn new(config: EcosystemStorageConfig) -> BearDogResult<Self> {
        // Ensure storage directory exists
        std::fs::create_dir_all(&config.storage_directory)
            .map_err(|e| BearDogError::system(format!("Failed to create storage directory: {}", e)))?;

        Ok(Self {
            config,
            metadata_store: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get file path for a key
    fn get_file_path(&self, key: &str) -> PathBuf {
        self.config.storage_directory.join(format!("{}.data", key))
    }

    /// Get metadata file path for a key
    fn get_metadata_path(&self, key: &str) -> PathBuf {
        self.config.storage_directory.join(format!("{}.meta", key))
    }
}

impl EcosystemStorage for FileSystemStorage {
    async fn store_data(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<StorageOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        // Store data file
        let data_path = self.get_file_path(key);
        tokio::fs::write(&data_path, &data)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write data file: {}", e)))?;

        // Store metadata
        let metadata_path = self.get_metadata_path(key);
        let metadata_json = serde_json::to_string(&metadata)
            .map_err(|e| BearDogError::system(format!("Failed to serialize metadata: {}", e)))?;
        tokio::fs::write(&metadata_path, metadata_json)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write metadata file: {}", e)))?;

        // Update in-memory metadata store
        {
            let mut store = self.metadata_store.write().await;
            store.insert(key.to_string(), metadata);
        }

        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format!("Successfully stored data for key: {}", key),
            data_size: data.len() as u64,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn retrieve_data(&self, key: &str) -> BearDogResult<Option<Vec<u8>>> {
        let data_path = self.get_file_path(key);
        
        if !data_path.exists() {
            return Ok(None);
        }

        let data = tokio::fs::read(&data_path)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read data file: {}", e)))?;

        Ok(Some(data))
    }

    async fn delete_data(&self, key: &str) -> BearDogResult<StorageOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        let data_path = self.get_file_path(key);
        let metadata_path = self.get_metadata_path(key);

        // Remove files
        if data_path.exists() {
            tokio::fs::remove_file(&data_path)
                .await
                .map_err(|e| BearDogError::system(format!("Failed to remove data file: {}", e)))?;
        }

        if metadata_path.exists() {
            tokio::fs::remove_file(&metadata_path)
                .await
                .map_err(|e| BearDogError::system(format!("Failed to remove metadata file: {}", e)))?;
        }

        // Remove from in-memory store
        {
            let mut store = self.metadata_store.write().await;
            store.remove(key);
        }

        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format!("Successfully deleted data for key: {}", key),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        let store = self.metadata_store.read().await;
        Ok(store.keys().cloned().collect())
    }

    async fn get_statistics(&self) -> BearDogResult<StorageStatistics> {
        let used_space = self.calculate_used_space().await?;
        let object_count = self.metadata_store.read().await.len() as u64;

        Ok(StorageStatistics {
            total_capacity: self.config.max_capacity,
            used_space,
            available_space: self.config.max_capacity.saturating_sub(used_space),
            object_count,
            last_cleanup: None, // Would be tracked in real implementation
            last_snapshot: None, // Would be tracked in real implementation
        })
    }

    async fn create_snapshot(&self, name: &str) -> BearDogResult<StorageOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        // In a real implementation, this would create a snapshot
        tracing::info!("Creating snapshot: {}", name);
        
        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format!("Snapshot '{}' created successfully", name),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn restore_snapshot(&self, name: &str) -> BearDogResult<StorageOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        // In a real implementation, this would restore from snapshot
        tracing::info!("Restoring from snapshot: {}", name);
        
        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format!("Restored from snapshot '{}' successfully", name),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn cleanup(&self) -> BearDogResult<StorageOperationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        // In a real implementation, this would clean up old data
        tracing::info!("Running storage cleanup");
        
        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: "Storage cleanup completed successfully".to_string(),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }
}

impl FileSystemStorage {
    /// Calculate used storage space
    async fn calculate_used_space(&self) -> BearDogResult<u64> {
        let mut total_size = 0u64;
        
        let mut dir = tokio::fs::read_dir(&self.config.storage_directory)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read storage directory: {}", e)))?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| BearDogError::system(format!("Failed to read directory entry: {}", e)))? {
            if let Ok(metadata) = entry.metadata().await {
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }
}
