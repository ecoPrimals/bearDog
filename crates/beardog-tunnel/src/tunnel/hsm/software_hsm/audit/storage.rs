//! Persistent audit storage implementation

use super::super::types::{AuditLogEntry, AuditLogFilter};
use beardog_errors::BearDogError;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Persistent audit storage implementation
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used in future implementation
pub struct PersistentAuditStorage {
    file_path: std::path::PathBuf,
    cache: Arc<RwLock<VecDeque<AuditLogEntry>>>,
    max_cache_size: usize,
    stats_cache: Arc<RwLock<Option<StorageStats>>>,
    stats_cache_timestamp: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
    pub stats_cache_ttl: u64,
}

impl PersistentAuditStorage {
    /// Create new persistent audit storage
    pub async fn new(
        file_path: std::path::PathBuf,
        max_cache_size: usize,
    ) -> Result<Self, BearDogError> {
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| BearDogError::System {
                    message: format!(
                        "Failed to create audit directory {}: {}",
                        parent.to_string_lossy(),
                        e
                    ),
                    category: beardog_errors::SystemErrorCategory::FileSystem,
                })?;
        }

        let storage = Self {
            file_path,
            cache: Arc::new(RwLock::new(VecDeque::new())),
            max_cache_size,
            stats_cache: Arc::new(RwLock::new(None)),
            stats_cache_timestamp: Arc::new(RwLock::new(chrono::Utc::now())),
            stats_cache_ttl: 300, // 5 minutes
        };

        storage.load_cache().await?;
        info!(
            "✅ Persistent audit storage initialized: {:?}",
            storage.file_path
        );
        Ok(storage)
    }

    async fn load_cache(&self) -> Result<(), BearDogError> {
        debug!("📥 Loading recent audit entries into cache");

        let mut file = match File::open(&self.file_path).await {
            Ok(file) => file,
            Err(_) => {
                debug!("Audit file does not exist yet, starting with empty cache");
                return Ok(());
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| BearDogError::System {
                message: format!("Failed to read audit file: {e}"),
                category: beardog_errors::SystemErrorCategory::FileSystem,
            })?;

        let mut cache = self.cache.write().await;
        for line in contents.lines().rev().take(self.max_cache_size) {
            if let Ok(entry) = serde_json::from_str::<AuditLogEntry>(line) {
                cache.push_front(entry);
            }
        }

        debug!("Loaded {} audit entries into cache", cache.len());
        Ok(())
    }

    /// Append new audit entry
    pub async fn append_entry(&self, entry: &AuditLogEntry) -> Result<(), BearDogError> {
        // Add to cache
        let mut cache = self.cache.write().await;
        cache.push_back(entry.clone());
        if cache.len() > self.max_cache_size {
            cache.pop_front();
        }
        drop(cache);

        // Persist to file
        let json = serde_json::to_string(entry).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize audit entry: {e}"))
        })?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await
            .map_err(|e| BearDogError::System {
                message: format!("Failed to open audit file: {e}"),
                category: beardog_errors::SystemErrorCategory::FileSystem,
            })?;

        file.write_all(json.as_bytes())
            .await
            .map_err(|e| BearDogError::System {
                message: format!("Failed to write audit entry: {e}"),
                category: beardog_errors::SystemErrorCategory::FileSystem,
            })?;

        file.write_all(b"\n")
            .await
            .map_err(|e| BearDogError::System {
                message: format!("Failed to write newline: {e}"),
                category: beardog_errors::SystemErrorCategory::FileSystem,
            })?;

        file.flush().await.map_err(|e| BearDogError::System {
            message: format!("Failed to flush audit file: {e}"),
            category: beardog_errors::SystemErrorCategory::FileSystem,
        })?;

        Ok(())
    }

    /// Log audit entry (alias for append_entry)
    pub async fn log_entry(&self, entry: AuditLogEntry) -> Result<(), BearDogError> {
        self.append_entry(&entry).await
    }

    /// Get audit entries matching filter
    pub async fn get_entries(
        &self,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, BearDogError> {
        let cache = self.cache.read().await;

        let filtered: Vec<AuditLogEntry> = cache
            .iter()
            .filter(|entry| filter.matches(entry))
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<StorageStats, BearDogError> {
        let cache_size = self.cache.read().await.len();
        let file_size = match tokio::fs::metadata(&self.file_path).await {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        };

        Ok(StorageStats {
            file_path: self.file_path.clone(),
            file_size_bytes: file_size,
            cache_size,
            max_cache_size: self.max_cache_size,
        })
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub file_path: std::path::PathBuf,
    pub file_size_bytes: u64,
    pub cache_size: usize,
    pub max_cache_size: usize,
}
