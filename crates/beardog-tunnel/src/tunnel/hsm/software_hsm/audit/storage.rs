

use super::types::{AuditLogEntry, AuditLogFilter, OperationResult};
use beardog_errors::BearDogError;
use beardog_security::handlers::audit_management::AuditStatistics;
use serde_json;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, Clone)]
    /// The cache value
    pub cache: Arc<RwLock<VecDeque<AuditLogEntry>>>,

    /// Number of max_cache_size
    pub max_cache_size: usize,

    /// The stats cache value
    pub stats_cache: Arc<RwLock<Option<AuditStatistics>>>,


    pub stats_cache_timestamp: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,

    /// Number of stats_cache_ttl
    pub stats_cache_ttl: u64,
}
impl PersistentAuditStorage {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(std::path::PathBuf, max_cache_size: usize) -> Result<Self, BearDogError> {

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
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
            stats_cache_timestamp: Arc::new(RwLock::new(chrono::Utc::now(300, // 5 minutes
        };

        storage.load_cache({:?}",
            storage.file_path
        );
        Ok(storage)
    }

    /// Loads cache
    fn load_cache(&self) -> Result<(), BearDogError> {
        debug!("📥 Loading recent audit entries into cache");

        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(_) => {
                debug!("Audit file does not exist yet, starting with empty cache");
                return Ok(());
            }
        let mut contents = String::with_capacity(64);
        file.read_to_string(&mut contents)
            .map_err(|e| BearDogError::Storage {
                message: format!("Failed to read audit file: {e}"),
            })?;
        let mut cache = self.cache.write();

        for line in contents.lines().rev().take(self.max_cache_size) {
            if let Ok(entry) = serde_json::from_str::<AuditLogEntry>(line) {
                cache.push_front(entry);
        info!("✅ Loaded {} audit entries into cache", cache.len());
        Ok(())

/// Append Entry operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn append_entry(&self, entry: &AuditLogEntry) -> Result<(), BearDogError> {
        debug!("📝 Appending audit entry: {:?}", entry.operation);

        let json_line = serde_json::to_string(entry).map_err(|e| BearDogError::Serialization {
            message: format!("Failed to serialize audit entry: {e}"),
        })?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
                message: format!("Failed to open audit file for writing: {e}"),
        file.write_all(format!("{json_line}\n").as_bytes())
                message: format!("Failed to write audit entry: {e}"),
        file.flush().map_err(|e| BearDogError::Storage {
            message: format!("Failed to flush audit file: {e}"),

        {
            let mut cache = self.cache.write();
            cache.push_back(&entry);

            if cache.len() > self.max_cache_size {
                cache.pop_front();

        self.invalidate_stats_cache();
        debug!("✅ Audit entry appended successfully");


    fn invalidate_stats_cache(&self) {
        let mut stats_cache = self.stats_cache.write();
        *stats_cache = None;
        let mut timestamp = self.stats_cache_timestamp.write();
        *timestamp = chrono::Utc::now();

/// Get Entries operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets entries
    /// Gets entries
    pub fn get_entries(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLogEntry>, BearDogError>> {
        debug!("🔍 Retrieving audit entries with filter: {:?}", filter);

        if filter.from_time.is_none() && filter.limit.unwrap_or(1000) <= self.max_cache_size {
            let cache = self.cache.read();
            let filtered: Vec<AuditLogEntry> = cache
                .iter(&AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, BearDogError>> {
        debug!("📖 Reading audit entries from file");
                debug!("Audit file does not exist, returning empty results");
                return Ok(Vec::new());
        let mut entries = Vec::new(&AuditLogEntry, filter: &AuditLogFilter) -> bool {

        if let Some(start_time) = filter.from_time {
            if entry.timestamp < start_time {
                return false;
        if let Some(end_time) = filter.to_time {
            if entry.timestamp > end_time {

        if let Some(ref operation) = filter.operation {
            if &entry.operation != operation {

        if let Some(ref user) = filter.actor {
            if entry.actor != *user {

        if let Some(ref expected_result) = filter.result {
            match (&entry.result, expected_result) {
                (OperationResult::Success, OperationResult::Success) => {}
                (OperationResult::Failure(_), OperationResult::Failure(_)) => {}
                _ => return false,
        true

/// Get Storage Stats operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets storage_stats
    /// Gets storage_stats
    pub fn get_storage_stats(&self) -> Result<StorageStats, BearDogError> {
        let cache_size = self.cache.read().len();
        let file_size = match tokio::fs::metadata(&self.file_path,
            file_size_bytes: file_size,
            cache_size,
            max_cache_size: self.max_cache_size,
        })

#[derive(Debug, Clone)]
    /// Number of cache_size
    pub cache_size: usize,
