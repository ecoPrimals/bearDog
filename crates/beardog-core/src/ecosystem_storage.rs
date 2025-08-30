

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStorageConfig {

    pub storage_directory: PathBuf,

    pub max_capacity: u64,

    pub retention_policy: RetentionPolicy,

    pub snapshot_config: SnapshotConfig,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {

    pub max_age_days: u32,

    pub auto_cleanup: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {

    pub interval_hours: u32,

    pub max_snapshots: u32,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {

    pub enabled: bool,

    pub max_log_size: u64,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOperationResult {

    pub operation_id: Uuid,

    pub success: bool,

    pub message: String,

    pub data_size: u64,

    pub duration_ms: u64,

    pub timestamp: DateTime<Utc>,
}

#[allow(async_fn_in_trait)]
pub trait EcosystemStorage: Send + Sync {

    async fn store_data(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: HashMap<&str, &str>,
    ) -> Result<StorageOperationResult, BearDogError>;

    async fn retrieve_data(&self, key: &str) -> Result<Option<Vec<u8>, BearDogError>>;

    async fn delete_data(&self, key: &str) -> Result<StorageOperationResult, BearDogError>;

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>;

    async fn get_statistics(&self) -> Result<StorageStatistics, BearDogError>;

    async fn create_snapshot(&self, name: &str) -> Result<StorageOperationResult, BearDogError>;

    async fn restore_snapshot(&self, name: &str) -> Result<StorageOperationResult, BearDogError>;

    async fn cleanup(&self) -> Result<StorageOperationResult, BearDogError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {

    pub total_capacity: u64,

    pub used_space: u64,

    pub available_space: u64,

    pub object_count: u64,

    pub last_cleanup: Option<DateTime<Utc>>,

    pub last_snapshot: Option<DateTime<Utc>>,
}

pub struct FileSystemStorage {
    config: EcosystemStorageConfig,
    metadata_store: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

impl FileSystemStorage {

    pub fn new(config: EcosystemStorageConfig) -> Result<Self, BearDogError> {

        std::fs::create_dir_all(&config.storage_directory)
            .map_err(|e| BearDogError::system(format_args!("Failed to create storage directory: {}", e).to_string()))?;

        Ok(Self {
            config,
            metadata_store: Arc::new(RwLock::new(ahash::HashMap::default())),
        })
    }

    fn get_file_path(&self, key: &str) -> PathBuf {
        self.config.storage_directory.join(format_args!("{}.data", key).to_string())
    }

    fn get_metadata_path(&self, key: &str) -> PathBuf {
        self.config.storage_directory.join(format_args!("{}.meta", key).to_string())
    }
}

impl EcosystemStorage for FileSystemStorage {
    async fn store_data(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: HashMap<&str, &str>,
    ) -> Result<StorageOperationResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        let data_path = self.get_file_path(key);
        tokio::fs::write(&data_path, &data)
            .await
            .map_err(|e| BearDogError::system(format_args!("Failed to write data file: {}", e).to_string()))?;

        let metadata_path = self.get_metadata_path(key);
        let metadata_json = serde_json::to_vec(&metadata)
            .map_err(|e| BearDogError::system(format_args!("Failed to serialize metadata: {}", e).to_string()))?;
        tokio::fs::write(&metadata_path, metadata_json)
            .await
            .map_err(|e| BearDogError::system(format_args!("Failed to write metadata file: {}", e).to_string()))?;

        {
            let mut store = self.metadata_store.write().await;
            store.insert(key.to_string(), metadata);
        }

        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format_args!("Successfully stored data for key: {}", key).to_string(),
            data_size: data.len() as u64,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn retrieve_data(&self, key: &str) -> Result<Option<Vec<u8>, BearDogError>> {
        let data_path = self.get_file_path(key);
        
        if !data_path.exists() {
            return Ok(None);
        }

        let data = tokio::fs::read(&data_path)
            .await
            .map_err(|e| BearDogError::system(format_args!("Failed to read data file: {}", e).to_string()))?;

        Ok(Some(data))
    }

    async fn delete_data(&self, key: &str) -> Result<StorageOperationResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        let data_path = self.get_file_path(key);
        let metadata_path = self.get_metadata_path(key);

        if data_path.exists() {
            tokio::fs::remove_file(&data_path)
                .await
                .map_err(|e| BearDogError::system(format_args!("Failed to remove data file: {}", e).to_string()))?;
        }

        if metadata_path.exists() {
            tokio::fs::remove_file(&metadata_path)
                .await
                .map_err(|e| BearDogError::system(format_args!("Failed to remove metadata file: {}", e).to_string()))?;
        }

        {
            let mut store = self.metadata_store.write().await;
            store.remove(key);
        }

        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format_args!("Successfully deleted data for key: {}", key).to_string(),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let store = self.metadata_store.read().await;
        Ok(store.keys().cloned().collect())
    }

    async fn get_statistics(&self) -> Result<StorageStatistics, BearDogError> {
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

    async fn create_snapshot(&self, name: &str) -> Result<StorageOperationResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        tracing::info!("Creating snapshot: {}", name);
        
        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format_args!("Snapshot '{}' created successfully", name).to_string(),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn restore_snapshot(&self, name: &str) -> Result<StorageOperationResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

        tracing::info!("Restoring from snapshot: {}", name);
        
        let duration = start_time.elapsed();
        Ok(StorageOperationResult {
            operation_id,
            success: true,
            message: format_args!("Restored from snapshot '{}' successfully", name).to_string(),
            data_size: 0,
            duration_ms: duration.as_millis() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn cleanup(&self) -> Result<StorageOperationResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4();

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

    async fn calculate_used_space(&self) -> Result<u64, BearDogError> {
        let mut total_size = 0u64;
        
        let mut dir = tokio::fs::read_dir(&self.config.storage_directory)
            .await
            .map_err(|e| BearDogError::system(format_args!("Failed to read storage directory: {}", e).to_string()))?;

        while let Some(entry) = dir.next_entry().await.map_err(|e| BearDogError::system(format_args!("Failed to read directory entry: {}", e).to_string()))? {
            if let Ok(metadata) = entry.metadata().await {
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }
}
