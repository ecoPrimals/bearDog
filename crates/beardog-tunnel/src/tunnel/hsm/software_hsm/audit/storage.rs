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


// # Audit Storage Backend
//
// **EXTRACTED FROM LARGE FILE** - Persistent storage and caching (~300 lines)
// This module handles persistent audit log storage, caching, and low-level I/O operations.

// Import audit types from local types module
use super::types::{AuditLogEntry, AuditLogFilter, OperationResult};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::handlers::audit_management::AuditStatistics;
use serde_json;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, info};
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
        // Load recent entries into cache
        storage.load_cache().await?;
        info!(
            "✅ Persistent audit storage initialized: {:?}",
            storage.file_path
        );
        Ok(storage)
    }
    /// Load recent entries into cache for performance
    async fn load_cache(&self) -> BearDogResult<()> {
        debug!("📥 Loading recent audit entries into cache");
        // Try to read the file
        let mut file = match File::open(&self.file_path).await {
            Ok(file) => file,
            Err(_) => {
                debug!("Audit file does not exist yet, starting with empty cache");
                return Ok(());
            }
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| BearDogError::Storage {
                message: format!("Failed to read audit file: {e}"),
            })?;
        let mut cache = self.cache.write().await;
        // Parse each line as a JSON log entry
        for line in contents.lines().rev().take(self.max_cache_size) {
            if let Ok(entry) = serde_json::from_str::<AuditLogEntry>(line) {
                cache.push_front(entry);
        info!("✅ Loaded {} audit entries into cache", cache.len());
        Ok(())
    /// Append a new audit entry
    pub async fn append_entry(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        debug!("📝 Appending audit entry: {:?}", entry.operation);
        // Serialize entry to JSON
        let json_line = serde_json::to_string(entry).map_err(|e| BearDogError::Serialization {
            message: format!("Failed to serialize audit entry: {e}"),
        })?;
        // Append to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
                message: format!("Failed to open audit file for writing: {e}"),
        file.write_all(format!("{json_line}\n").as_bytes())
                message: format!("Failed to write audit entry: {e}"),
        file.flush().await.map_err(|e| BearDogError::Storage {
            message: format!("Failed to flush audit file: {e}"),
        // Update in-memory cache
        {
            let mut cache = self.cache.write().await;
            cache.push_back(entry.clone());
            // Maintain cache size limit
            if cache.len() > self.max_cache_size {
                cache.pop_front();
        // Invalidate statistics cache
        self.invalidate_stats_cache().await;
        debug!("✅ Audit entry appended successfully");
    /// Invalidate statistics cache
    async fn invalidate_stats_cache(&self) {
        let mut stats_cache = self.stats_cache.write().await;
        *stats_cache = None;
        let mut timestamp = self.stats_cache_timestamp.write().await;
        *timestamp = chrono::Utc::now();
    /// Get audit entries matching filter}


    pub async fn get_entries(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        debug!("🔍 Retrieving audit entries with filter: {:?}", filter);
        // If requesting recent entries and they fit in cache, use cache
        if filter.from_time.is_none() && filter.limit.unwrap_or(1000) <= self.max_cache_size {
            let cache = self.cache.read().await;
            let filtered: Vec<AuditLogEntry> = cache
                .iter()
                .filter(|entry| self.matches_filter(entry, filter))
                .take(filter.limit.unwrap_or(1000))
                .cloned()
                .collect();
            if !filtered.is_empty() {
                debug!("✅ Retrieved {} entries from cache", filtered.len());
                return Ok(filtered);
        // Fall back to reading from file
        self.read_file_entries(filter).await
    /// Read entries from file with filter
    async fn read_file_entries(
        &self,
        filter: &AuditLogFilter,
    ) -> BearDogResult<Vec<AuditLogEntry>> {
        debug!("📖 Reading audit entries from file");
                debug!("Audit file does not exist, returning empty results");
                return Ok(Vec::new());
        let mut entries = Vec::new();
        // Parse each line and apply filter
        for line in contents.lines() {
                if self.matches_filter(&entry, filter) {
                    entries.push(entry);
                    // Apply limit
                    if let Some(limit) = filter.limit {
                        if entries.len() >= limit {
                            break;
                        }
                    }
                }
        debug!("✅ Retrieved {} entries from file", entries.len());
        Ok(entries)
    /// Check if entry matches filter criteria
    fn matches_filter(&self, entry: &AuditLogEntry, filter: &AuditLogFilter) -> bool {
        // Check time range
        if let Some(start_time) = filter.from_time {
            if entry.timestamp < start_time {
                return false;
        if let Some(end_time) = filter.to_time {
            if entry.timestamp > end_time {
        // Check operation filter
        if let Some(ref operation) = filter.operation {
            if &entry.operation != operation {
        // Check user filter
        if let Some(ref user) = filter.actor {
            if entry.actor != *user {
        // Check result filter
        // Filter by result (success/failure) if specified
        if let Some(ref expected_result) = filter.result {
            match (&entry.result, expected_result) {
                (OperationResult::Success, OperationResult::Success) => {}
                (OperationResult::Failure(_), OperationResult::Failure(_)) => {}
                _ => return false,
        true
    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> BearDogResult<StorageStats> {
        let cache_size = self.cache.read().await.len();
        let file_size = match tokio::fs::metadata(&self.file_path).await {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        Ok(StorageStats {
            file_path: self.file_path.clone(),
            file_size_bytes: file_size,
            cache_size,
            max_cache_size: self.max_cache_size,
        })
/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub file_size_bytes: u64,
    pub cache_size: usize,
