// SPDX-License-Identifier: AGPL-3.0-or-later

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
pub struct PersistentAuditStorage {
    file_path: std::path::PathBuf,
    cache: Arc<RwLock<VecDeque<AuditLogEntry>>>,
    max_cache_size: usize,
    _stats_cache: Arc<RwLock<Option<StorageStats>>>,
    _stats_cache_timestamp: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
    /// Time-to-live for statistics cache in seconds
    pub stats_cache_ttl: u64,
}

impl PersistentAuditStorage {
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
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
            _stats_cache: Arc::new(RwLock::new(None)),
            _stats_cache_timestamp: Arc::new(RwLock::new(chrono::Utc::now())),
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

        let Ok(mut file) = File::open(&self.file_path).await else {
            debug!("Audit file does not exist yet, starting with empty cache");
            return Ok(());
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

    /// # Errors
    ///
    /// Returns an error if serialization fails.
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

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log audit entry (alias for `append_entry`)
    pub async fn log_entry(&self, entry: AuditLogEntry) -> Result<(), BearDogError> {
        self.append_entry(&entry).await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
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

    /// # Errors
    ///
    /// Returns an error if hashing fails.
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

#[cfg(test)]
mod tests {
    use super::super::types::{AuditLogEntry, AuditLogFilter, OperationResult};
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    fn sample_entry(op: &str) -> AuditLogEntry {
        AuditLogEntry {
            timestamp: Utc::now(),
            operation: op.to_string(),
            user_id: Some("u1".to_string()),
            key_id: Some("k1".to_string()),
            result: OperationResult::Success,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[tokio::test]
    async fn persistent_audit_storage_new_append_get_and_stats() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("audit.log");
        let storage = PersistentAuditStorage::new(path.clone(), 100)
            .await
            .expect("storage init");

        assert_eq!(storage.stats_cache_ttl, 300);
        let e1 = sample_entry("op_a");
        storage.append_entry(&e1).await.expect("append first entry");
        storage
            .log_entry(sample_entry("op_b"))
            .await
            .expect("log_entry alias");

        let all = storage
            .get_entries(&AuditLogFilter::default())
            .await
            .expect("get all");
        assert_eq!(all.len(), 2);

        let filtered = storage
            .get_entries(&AuditLogFilter {
                operation: Some("op_a".to_string()),
                ..Default::default()
            })
            .await
            .expect("filtered");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].operation, "op_a");

        let stats = storage.get_storage_stats().await.expect("stats");
        assert_eq!(stats.file_path, path);
        assert!(stats.file_size_bytes > 0);
        assert_eq!(stats.cache_size, 2);
        assert_eq!(stats.max_cache_size, 100);
    }

    #[tokio::test]
    async fn persistent_audit_storage_load_cache_skips_invalid_json_lines() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("audit_partial.log");
        let valid = sample_entry("valid_line");
        let line = serde_json::to_string(&valid).expect("serialize entry");
        tokio::fs::write(&path, format!("not-json\n{line}\n"))
            .await
            .expect("seed file");

        let storage = PersistentAuditStorage::new(path, 10)
            .await
            .expect("load with bad line");
        let entries = storage
            .get_entries(&AuditLogFilter::default())
            .await
            .expect("get");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].operation, "valid_line");
    }

    #[tokio::test]
    async fn persistent_audit_storage_cache_eviction_respects_max() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("audit_evict.log");
        let storage = PersistentAuditStorage::new(path, 2).await.expect("storage");

        for i in 0..5 {
            let mut e = sample_entry("same_op");
            e.operation = format!("op_{i}");
            storage.append_entry(&e).await.expect("append");
        }
        let entries = storage
            .get_entries(&AuditLogFilter::default())
            .await
            .expect("get");
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn persistent_audit_storage_missing_file_starts_empty() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("missing.log");
        let storage = PersistentAuditStorage::new(path, 5)
            .await
            .expect("new without existing file");
        let entries = storage
            .get_entries(&AuditLogFilter::default())
            .await
            .expect("get");
        assert!(entries.is_empty());
        let stats = storage.get_storage_stats().await.expect("stats");
        assert_eq!(stats.cache_size, 0);
        assert_eq!(stats.file_size_bytes, 0);
    }

    #[tokio::test]
    async fn audit_log_filter_time_bounds() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("audit_time.log");
        let storage = PersistentAuditStorage::new(path, 20)
            .await
            .expect("storage");

        let mut old = sample_entry("old");
        old.timestamp = Utc::now() - chrono::Duration::hours(2);
        storage.append_entry(&old).await.expect("old");

        let mut new = sample_entry("new");
        new.timestamp = Utc::now();
        storage.append_entry(&new).await.expect("new");

        let from = Utc::now() - chrono::Duration::hours(1);
        let list = storage
            .get_entries(&AuditLogFilter {
                from_time: Some(from),
                ..Default::default()
            })
            .await
            .expect("from filter");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].operation, "new");
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    /// Path to the audit log file
    pub file_path: std::path::PathBuf,
    /// Size of the audit log file in bytes
    pub file_size_bytes: u64,
    /// Number of entries in the cache
    pub cache_size: usize,
    /// Maximum cache capacity
    pub max_cache_size: usize,
}
