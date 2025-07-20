//! # Software HSM Audit Logging
//!
//! This module provides comprehensive audit logging functionality for the Software HSM.
//! It tracks all operations, maintains audit trails, and provides query capabilities.

use super::types::*;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Persistent audit storage backend
#[derive(Debug)]
pub struct PersistentAuditStorage {
    /// File path for audit log storage
    pub file_path: std::path::PathBuf,
    /// In-memory cache for recent entries (performance optimization)
    pub cache: Arc<RwLock<VecDeque<AuditLogEntry>>>,
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Statistics cache
    pub stats_cache: Arc<RwLock<Option<AuditStatistics>>>,
    /// Cache invalidation timestamp
    pub stats_cache_timestamp: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
    /// Statistics cache TTL in seconds
    pub stats_cache_ttl: u64,
}

impl PersistentAuditStorage {
    /// Create new persistent audit storage
    pub async fn new(file_path: std::path::PathBuf, max_cache_size: usize) -> BearDogResult<Self> {
        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| BearDogError::Storage {
                    message: format!(
                        "Failed to create audit directory {}: {}",
                        parent.to_string_lossy(),
                        e
                    ),
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

        // Load recent entries into cache for performance
        storage.load_cache().await?;

        info!(
            "Persistent audit storage initialized: {:?}",
            storage.file_path
        );
        Ok(storage)
    }

    /// Load recent entries into cache
    async fn load_cache(&self) -> BearDogResult<()> {
        if !self.file_path.exists() {
            debug!("Audit log file does not exist yet, starting with empty cache");
            return Ok(());
        }

        let mut file = File::open(&self.file_path)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to open audit log file {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to read audit log file {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut cache = self.cache.write().await;
        let mut loaded_count = 0;

        // Parse each line as a separate JSON entry
        for line in contents.lines().rev().take(self.max_cache_size) {
            if !line.trim().is_empty() {
                match serde_json::from_str::<AuditLogEntry>(line.trim()) {
                    Ok(entry) => {
                        cache.push_front(entry);
                        loaded_count += 1;
                    }
                    Err(e) => {
                        warn!("Failed to parse audit log entry: {} - Error: {}", line, e);
                    }
                }
            }
        }

        info!("Loaded {} audit entries into cache", loaded_count);
        Ok(())
    }

    /// Append entry to file and cache
    pub async fn append_entry(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        // Add to cache first (for immediate availability)
        {
            let mut cache = self.cache.write().await;
            cache.push_back(entry.clone());

            // Maintain cache size limit
            while cache.len() > self.max_cache_size {
                cache.pop_front();
            }
        }

        // Append to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to open audit log file for appending {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let json_line = serde_json::to_string(entry).map_err(|e| BearDogError::Serialization {
            message: format!("Failed to serialize audit entry: {e}"),
        })?;

        let line_with_newline = format!("{json_line}\n");
        file.write_all(line_with_newline.as_bytes())
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to write audit entry to file {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        file.sync_all().await.map_err(|e| BearDogError::Storage {
            message: format!(
                "Failed to sync audit log file {}: {}",
                self.file_path.to_string_lossy(),
                e
            ),
        })?;

        // Invalidate statistics cache
        self.invalidate_stats_cache().await;

        debug!("Successfully appended audit entry to file and cache");
        Ok(())
    }

    /// Invalidate statistics cache
    async fn invalidate_stats_cache(&self) {
        let mut stats_cache = self.stats_cache.write().await;
        *stats_cache = None;
        let mut timestamp = self.stats_cache_timestamp.write().await;
        *timestamp = chrono::Utc::now();
    }

    /// Get all entries with optional filtering
    pub async fn get_entries(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        // First try cache for recent entries
        let cache_entries = {
            let cache = self.cache.read().await;
            cache.iter().cloned().collect::<Vec<_>>()
        };

        let mut results = Vec::new();

        // Filter cache entries
        for entry in cache_entries {
            if self.matches_filter(&entry, filter) {
                results.push(entry);
            }
        }

        // If we need older entries or cache doesn't have enough, read from file
        if results.len() < 1000 && self.file_path.exists() {
            let file_entries = self.read_file_entries(filter).await?;

            // Merge with cache results, removing duplicates
            for entry in file_entries {
                let is_duplicate = results.iter().any(|cached_entry| {
                    cached_entry.timestamp == entry.timestamp
                        && cached_entry.operation == entry.operation
                        && cached_entry.key_id == entry.key_id
                });

                if !is_duplicate && self.matches_filter(&entry, filter) {
                    results.push(entry);
                }
            }
        }

        // Sort by timestamp (newest first)
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        debug!("Retrieved {} audit entries matching filter", results.len());
        Ok(results)
    }

    /// Read entries from file with filtering
    async fn read_file_entries(
        &self,
        filter: &AuditLogFilter,
    ) -> BearDogResult<Vec<AuditLogEntry>> {
        let mut file = File::open(&self.file_path)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to open audit log file for reading {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to read audit log file {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut entries = Vec::new();

        for line in contents.lines() {
            if !line.trim().is_empty() {
                match serde_json::from_str::<AuditLogEntry>(line.trim()) {
                    Ok(entry) => {
                        if self.matches_filter(&entry, filter) {
                            entries.push(entry);
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to parse audit log entry from file: {} - Error: {}",
                            line, e
                        );
                    }
                }
            }
        }

        Ok(entries)
    }

    /// Check if entry matches filter criteria
    fn matches_filter(&self, entry: &AuditLogEntry, filter: &AuditLogFilter) -> bool {
        // Time range filter
        if let Some(start_time) = filter.start_time {
            if entry.timestamp < start_time {
                return false;
            }
        }

        if let Some(end_time) = filter.end_time {
            if entry.timestamp > end_time {
                return false;
            }
        }

        // Operation filter
        if let Some(ref operation) = filter.operation {
            if entry.operation != *operation {
                return false;
            }
        }

        // Key ID filter
        if let Some(ref key_id) = filter.key_id {
            if entry.key_id.as_ref() != Some(key_id) {
                return false;
            }
        }

        // User ID filter
        if let Some(ref user_id) = filter.user_id {
            if entry.user_id.as_ref() != Some(user_id) {
                return false;
            }
        }

        true
    }

    /// Generate statistics from all entries
    pub async fn generate_statistics(&self) -> BearDogResult<AuditStatistics> {
        // Check if cached statistics are still valid
        {
            let stats_cache = self.stats_cache.read().await;
            let timestamp = self.stats_cache_timestamp.read().await;

            if let Some(ref stats) = *stats_cache {
                let age = chrono::Utc::now().signed_duration_since(*timestamp);
                if age.num_seconds() < self.stats_cache_ttl as i64 {
                    debug!("Returning cached audit statistics");
                    return Ok(stats.clone());
                }
            }
        }

        info!("Generating fresh audit statistics");

        // Get all entries for statistics
        let all_entries = self.get_entries(&AuditLogFilter::new()).await?;

        let mut entries_by_operation = HashMap::new();
        let mut entries_by_result = HashMap::new();
        let mut entries_by_user = HashMap::new();
        let mut oldest_entry = None;
        let mut newest_entry = None;

        for entry in &all_entries {
            // Count by operation
            *entries_by_operation
                .entry(entry.operation.clone())
                .or_insert(0) += 1;

            // Count by result
            *entries_by_result.entry(entry.result.clone()).or_insert(0) += 1;

            // Count by user
            if let Some(ref user_id) = entry.user_id {
                *entries_by_user.entry(user_id.clone()).or_insert(0) += 1;
            }

            // Track oldest and newest
            if oldest_entry.is_none() || entry.timestamp < oldest_entry.unwrap() {
                oldest_entry = Some(entry.timestamp);
            }
            if newest_entry.is_none() || entry.timestamp > newest_entry.unwrap() {
                newest_entry = Some(entry.timestamp);
            }
        }

        let statistics = AuditStatistics {
            total_entries: all_entries.len(),
            entries_by_operation,
            entries_by_result,
            entries_by_user,
            oldest_entry,
            newest_entry,
        };

        // Cache the statistics
        {
            let mut stats_cache = self.stats_cache.write().await;
            *stats_cache = Some(statistics.clone());
            let mut timestamp = self.stats_cache_timestamp.write().await;
            *timestamp = chrono::Utc::now();
        }

        info!(
            "Generated statistics for {} audit entries",
            statistics.total_entries
        );
        Ok(statistics)
    }

    /// Purge entries older than specified date
    pub async fn purge_old_entries(
        &self,
        older_than: chrono::DateTime<chrono::Utc>,
    ) -> BearDogResult<usize> {
        info!("Starting purge of audit entries older than {}", older_than);

        if !self.file_path.exists() {
            debug!("Audit log file does not exist, no entries to purge");
            return Ok(0);
        }

        // Read all entries
        let mut file = File::open(&self.file_path)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to open audit log file for purging {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!(
                    "Failed to read audit log file for purging {}: {}",
                    self.file_path.to_string_lossy(),
                    e
                ),
            })?;

        let mut kept_entries = Vec::new();
        let mut purged_count = 0;

        // Parse and filter entries
        for line in contents.lines() {
            if !line.trim().is_empty() {
                match serde_json::from_str::<AuditLogEntry>(line.trim()) {
                    Ok(entry) => {
                        if entry.timestamp >= older_than {
                            kept_entries.push(line.to_string());
                        } else {
                            purged_count += 1;
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to parse audit entry during purge: {} - Error: {}",
                            line, e
                        );
                        // Keep unparseable entries to avoid data loss
                        kept_entries.push(line.to_string());
                    }
                }
            }
        }

        // Write back the kept entries
        if purged_count > 0 {
            let temp_path = self.file_path.with_extension("tmp");

            {
                let mut temp_file =
                    File::create(&temp_path)
                        .await
                        .map_err(|e| BearDogError::Storage {
                            message: format!(
                                "Failed to create temporary audit file {}: {}",
                                temp_path.to_string_lossy(),
                                e
                            ),
                        })?;

                for entry_line in kept_entries {
                    temp_file
                        .write_all(format!("{entry_line}\n").as_bytes())
                        .await
                        .map_err(|e| BearDogError::Storage {
                            message: format!(
                                "Failed to write to temporary audit file {}: {}",
                                temp_path.to_string_lossy(),
                                e
                            ),
                        })?;
                }

                temp_file
                    .sync_all()
                    .await
                    .map_err(|e| BearDogError::Storage {
                        message: format!(
                            "Failed to sync temporary audit file {}: {}",
                            temp_path.to_string_lossy(),
                            e
                        ),
                    })?;
            }

            // Replace original with temp file
            tokio::fs::rename(&temp_path, &self.file_path)
                .await
                .map_err(|e| BearDogError::Storage {
                    message: format!(
                        "Failed to replace original audit file with temp file {}: {}",
                        self.file_path.to_string_lossy(),
                        e
                    ),
                })?;

            // Update cache to remove purged entries
            {
                let mut cache = self.cache.write().await;
                cache.retain(|entry| entry.timestamp >= older_than);
            }

            // Invalidate statistics cache
            self.invalidate_stats_cache().await;

            info!(
                "Successfully purged {} audit entries older than {}",
                purged_count, older_than
            );
        } else {
            info!("No audit entries found older than {}", older_than);
        }

        Ok(purged_count)
    }
}

impl DefaultAuditLogger {
    /// Create a new default audit logger with persistent storage
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating default audit logger with persistent storage");

        // Create default audit directory
        let audit_dir = std::path::Path::new("./logs/audit");
        let audit_file = audit_dir.join("audit.log");

        let storage = PersistentAuditStorage::new(audit_file, 10000).await?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }

    /// Create audit logger with custom storage path
    pub async fn with_storage_path(storage_path: std::path::PathBuf) -> BearDogResult<Self> {
        info!(
            "Creating default audit logger with custom storage: {:?}",
            storage_path
        );

        let storage = PersistentAuditStorage::new(storage_path, 10000).await?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }

    /// Log operation with detailed information
    pub async fn log_detailed_operation(
        &self,
        operation: &str,
        result: &str,
        details: &std::collections::HashMap<String, String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry {
            timestamp: chrono::Utc::now(),
            operation: operation.to_string(),
            key_id: details.get("key_id").cloned(),
            user_id: details.get("user_id").cloned(),
            result: result.to_string(),
            details: details.clone(),
        };

        self.log_operation(&entry).await
    }

    /// Log successful operation
    pub async fn log_success(
        &self,
        operation: &str,
        key_id: Option<String>,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::success(operation.to_string(), key_id, user_id);
        self.log_operation(&entry).await
    }

    /// Log failed operation
    pub async fn log_failure(
        &self,
        operation: &str,
        key_id: Option<String>,
        user_id: Option<String>,
        error: String,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::failure(operation.to_string(), key_id, user_id, error);
        self.log_operation(&entry).await
    }

    /// Log key generation
    pub async fn log_key_generation(
        &self,
        key_id: &str,
        key_type: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("key_type".to_string(), key_type.to_string());

        let entry = AuditLogEntry::new(
            "key_generation".to_string(),
            Some(key_id.to_string()),
            user_id,
            "success".to_string(),
            details,
        );

        self.log_operation(&entry).await
    }

    /// Log key deletion
    pub async fn log_key_deletion(
        &self,
        key_id: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::success(
            "key_deletion".to_string(),
            Some(key_id.to_string()),
            user_id,
        );

        self.log_operation(&entry).await
    }

    /// Log cryptographic operation
    pub async fn log_crypto_operation(
        &self,
        operation: &str,
        key_id: &str,
        data_size: usize,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("data_size".to_string(), data_size.to_string());

        let entry = AuditLogEntry::new(
            format!("crypto_{operation}"),
            Some(key_id.to_string()),
            user_id,
            "success".to_string(),
            details,
        );

        self.log_operation(&entry).await
    }

    /// Log security event
    pub async fn log_security_event(
        &self,
        event_type: &str,
        severity: &str,
        description: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("event_type".to_string(), event_type.to_string());
        details.insert("severity".to_string(), severity.to_string());
        details.insert("description".to_string(), description.to_string());

        let entry = AuditLogEntry::new(
            "security_event".to_string(),
            None,
            user_id,
            "alert".to_string(),
            details,
        );

        self.log_operation(&entry).await
    }

    /// Get audit statistics
    pub async fn get_audit_statistics(&self) -> BearDogResult<AuditStatistics> {
        // Use the persistent storage to generate comprehensive statistics
        self.storage.generate_statistics().await
    }

    /// Purge old audit entries
    pub async fn purge_old_entries(
        &self,
        older_than: chrono::DateTime<chrono::Utc>,
    ) -> BearDogResult<usize> {
        info!("Purging audit entries older than {}", older_than);
        // Use the persistent storage to purge old entries
        self.storage.purge_old_entries(older_than).await
    }

    /// Export audit log
    pub async fn export_audit_log(&self, format: &str) -> BearDogResult<Vec<u8>> {
        info!("Exporting audit log in format: {}", format);

        // Get all audit entries
        let filter = AuditLogFilter::new();
        let entries = self.get_audit_log(&filter).await?;

        match format {
            "json" => {
                let json = serde_json::to_string_pretty(&entries).map_err(|e| {
                    BearDogError::Serialization {
                        message: e.to_string(),
                    }
                })?;
                Ok(json.into_bytes())
            }
            "csv" => {
                let mut csv = String::new();
                csv.push_str("timestamp,operation,key_id,user_id,result,details\n");

                for entry in entries {
                    let details_json = serde_json::to_string(&entry.details).unwrap_or_default();
                    csv.push_str(&format!(
                        "{},{},{},{},{},\"{}\"\n",
                        entry.timestamp.to_rfc3339(),
                        entry.operation,
                        entry.key_id.unwrap_or_default(),
                        entry.user_id.unwrap_or_default(),
                        entry.result,
                        details_json
                    ));
                }

                Ok(csv.into_bytes())
            }
            _ => Err(BearDogError::UnsupportedOperation {
                operation: format!("Unsupported audit format: {format}"),
            }),
        }
    }
}

#[async_trait]
impl AuditLogger for DefaultAuditLogger {
    /// Log HSM operation
    async fn log_operation(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        // Log to persistent storage (includes both file and cache)
        self.storage.append_entry(entry).await?;

        // Also log to tracing for immediate visibility
        info!(
            "AUDIT: {} - {} - {} - {:?}",
            entry.timestamp.to_rfc3339(),
            entry.operation,
            entry.result,
            entry.details
        );

        if entry.result == "failure" || entry.result == "error" {
            warn!(
                "AUDIT FAILURE: {} failed for key_id={:?} user_id={:?}",
                entry.operation, entry.key_id, entry.user_id
            );
        }

        debug!("Successfully logged audit entry to persistent storage");
        Ok(())
    }

    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        debug!("Getting audit log with filter: {:?}", filter);

        // Use persistent storage to get filtered entries
        let entries = self.storage.get_entries(filter).await?;

        info!(
            "Retrieved {} audit entries matching filter criteria",
            entries.len()
        );
        Ok(entries)
    }
}

/// In-memory audit logger for testing and development
pub struct InMemoryAuditLogger {
    entries: Arc<RwLock<VecDeque<AuditLogEntry>>>,
    max_entries: usize,
}

impl InMemoryAuditLogger {
    /// Create a new in-memory audit logger
    pub async fn new(max_entries: usize) -> BearDogResult<Self> {
        info!(
            "Creating in-memory audit logger with max {} entries",
            max_entries
        );

        Ok(Self {
            entries: Arc::new(RwLock::new(VecDeque::new())),
            max_entries,
        })
    }

    /// Get current entry count
    pub async fn get_entry_count(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }

    /// Clear all entries
    pub async fn clear(&self) -> BearDogResult<()> {
        let mut entries = self.entries.write().await;
        entries.clear();
        info!("Cleared all audit log entries");
        Ok(())
    }

    /// Get all entries
    pub async fn get_all_entries(&self) -> BearDogResult<Vec<AuditLogEntry>> {
        let entries = self.entries.read().await;
        Ok(entries.iter().cloned().collect())
    }
}

#[async_trait]
impl AuditLogger for InMemoryAuditLogger {
    /// Log HSM operation
    async fn log_operation(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        let mut entries = self.entries.write().await;

        // Add new entry
        entries.push_back(entry.clone());

        // Remove old entries if we exceed max
        while entries.len() > self.max_entries {
            entries.pop_front();
        }

        info!(
            "AUDIT: {} - {} - {} - {:?}",
            entry.timestamp.to_rfc3339(),
            entry.operation,
            entry.result,
            entry.details
        );

        Ok(())
    }

    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        let entries = self.entries.read().await;

        let filtered: Vec<AuditLogEntry> = entries
            .iter()
            .filter(|entry| {
                // Apply time filter
                if let Some(start_time) = filter.start_time {
                    if entry.timestamp < start_time {
                        return false;
                    }
                }
                if let Some(end_time) = filter.end_time {
                    if entry.timestamp > end_time {
                        return false;
                    }
                }

                // Apply operation filter
                if let Some(ref operation) = filter.operation {
                    if entry.operation != *operation {
                        return false;
                    }
                }

                // Apply key_id filter
                if let Some(ref key_id) = filter.key_id {
                    match &entry.key_id {
                        Some(entry_key_id) => {
                            if entry_key_id != key_id {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }

                // Apply user_id filter
                if let Some(ref user_id) = filter.user_id {
                    match &entry.user_id {
                        Some(entry_user_id) => {
                            if entry_user_id != user_id {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }

                true
            })
            .cloned()
            .collect();

        Ok(filtered)
    }
}

/// Audit statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditStatistics {
    /// Total number of audit entries
    pub total_entries: usize,
    /// Number of entries grouped by operation type
    pub entries_by_operation: std::collections::HashMap<String, usize>,
    /// Number of entries grouped by result status
    pub entries_by_result: std::collections::HashMap<String, usize>,
    /// Number of entries grouped by user
    pub entries_by_user: std::collections::HashMap<String, usize>,
    /// Timestamp of the oldest audit entry
    pub oldest_entry: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of the newest audit entry
    pub newest_entry: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create audit logger factory
pub async fn create_audit_logger(logger_type: &str) -> BearDogResult<Box<dyn AuditLogger>> {
    match logger_type {
        "default" => {
            let logger = DefaultAuditLogger::new().await?;
            Ok(Box::new(logger))
        }
        "memory" => {
            let logger = InMemoryAuditLogger::new(10000).await?;
            Ok(Box::new(logger))
        }
        _ => Err(BearDogError::UnsupportedOperation {
            operation: format!("Unsupported logger type: {logger_type}"),
        }),
    }
}
