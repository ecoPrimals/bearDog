

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, trace};
use beardog_types::canonical::configuration::{ApiConfig, DatabaseConfig}; // Use canonical configs
use beardog_types::canonical::configuration::SecurityConfig; // Use canonical security config

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConfigKey {

    type_id: TypeId,

    type_name: &'static str,

    name: String,
}
impl ConfigKey {}

    pub fn new<T: 'static>(name: String) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>(),
            name,
        }
    }

#[derive(Debug, Clone)]
pub struct ConfigMetadata {

    created_at: Instant,

    last_accessed: Instant,

    access_count: u64,

    size_hint: usize,}

impl ConfigMetadata {}

    fn new(size_hint: usize) -> Self {
        let now = Instant::now();
            created_at: now,
            last_accessed: now,
            access_count: 0,
            size_hint,}

    fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();

#[derive(Debug, Default)]
pub struct SharedConfigStats {

    pub hits: std::sync::atomic::AtomicU64,

    pub misses: std::sync::atomic::AtomicU64,

    pub configs_created: std::sync::atomic::AtomicU64,

    pub configs_evicted: std::sync::atomic::AtomicU64,}

impl SharedConfigManager {

    pub fn new() -> Self {
            configs: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metadata: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: SharedConfigStats::default(),

    pub fn get_or_create<T, F>(&self, name: &str, factory: F) -> Arc<T>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        let key = ConfigKey::new::<T>(name.to_string());

        {
            let configs = self.configs.read();
            if let Some(any_config) = configs.get(&key) {
                if let Ok(typed_config) = any_config.clone().downcast::<T>() {

                    {
                        let mut metadata = self.metadata.write();
                        if let Some(meta) = metadata.get_mut(&key) {
                            meta.access();
                        }
                    }
                    self.stats
                        .hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    trace!("Config cache hit for {}: {}", key.type_name, name);
                    return typed_config;
                }
            }

        self.stats
            .misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            .configs_created
        let config = Arc::new(factory());
        let size_hint = std::mem::size_of::<T>();

            let mut configs = self.configs.write();
            let mut metadata = self.metadata.write();
            configs.insert(key.clone(), config.clone());
            metadata.insert(key.clone(), ConfigMetadata::new(size_hint));
        debug!("Created new shared config {}: {}", key.type_name, name);
        config

    pub fn get<T>(&self, name: &str) -> Option<Arc<T>>
        let configs = self.configs.read();
        if let Some(any_config) = configs.get(&key) {
            if let Ok(typed_config) = any_config.clone().downcast::<T>() {

                {
                    let mut metadata = self.metadata.write();
                    if let Some(meta) = metadata.get_mut(&key) {
                        meta.access();
                self.stats
                    .hits
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                trace!("Config cache hit for {}: {}", key.type_name, name);
                return Some(typed_config);
        None

    pub fn set<T>(&self, name: &str, config: T) -> Arc<T>
        let config_arc = Arc::new(config);
            configs.insert(key.clone(), config_arc.clone());
        debug!("Set shared config {}: {}", key.type_name, name);
        config_arc

    pub fn remove<T>(&self, name: &str) -> bool
        T: 'static,
        let mut configs = self.configs.write();
        let mut metadata = self.metadata.write();
        let removed = configs.remove(&key).is_some();
        metadata.remove(&key);
        if removed {
            debug!("Removed shared config {}: {}", key.type_name, name);
        removed

    pub fn list_configs<T>(&self) -> Vec<String>
        let target_type_id = TypeId::of::<T>();
        configs
            .keys()
            .filter(|key| key.type_id == target_type_id)
            .map(|key| key.name.clone())
            .collect()

    pub fn get_stats(&self) -> &SharedConfigStats {
        &self.stats

    pub fn hit_rate(&self) -> f64 {
        let hits = self.stats.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.stats.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64

    pub fn cache_size(&self) -> usize {
        self.configs.read().len()

    pub fn estimated_memory_usage(&self) -> usize {
        let metadata = self.metadata.read();
        metadata.values().map(|meta| meta.size_hint).sum()

    pub fn cleanup_unused(&self, max_age: Duration) {
        let cutoff = Instant::now() - max_age;
        let initial_size = configs.len();

        let keys_to_remove: Vec<ConfigKey> = metadata
            .iter()
            .filter(|(_, meta)| meta.last_accessed < cutoff && meta.access_count == 0)
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            configs.remove(&key);
            metadata.remove(&key);
            self.stats
                .configs_evicted
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let removed = initial_size - configs.len();
        if removed > 0 {
            debug!("Cleaned up {} unused shared configs", removed);
impl Default for SharedConfigManager {}

    fn default() -> Self {
        Self::new()

pub trait Shareable: Clone + Send + Sync + 'static {

    fn default_name() -> &'static str {
        std::any::type_name::<Self>()

    fn estimated_size(&self) -> usize {
        std::mem::size_of::<Self>()

static GLOBAL_CONFIG_MANAGER: std::sync::OnceLock<SharedConfigManager> = std::sync::OnceLock::new();

pub fn global_config_manager() -> &'static SharedConfigManager {
    GLOBAL_CONFIG_MANAGER.get_or_init(SharedConfigManager::new)

pub fn shared_config<T, F>(name: &str, factory: F) -> Arc<T>
where
    T: Clone + Send + Sync + 'static,
    F: FnOnce() -> T,
{
    global_config_manager().get_or_create(name, factory)

pub fn get_shared_config<T>(name: &str) -> Option<Arc<T>>
    global_config_manager().get(name)

pub fn set_shared_config<T>(name: &str, config: T) -> Arc<T>
    global_config_manager().set(name, config)

pub fn shared_database_config<F>(factory: F) -> Arc<DatabaseConfig>
    F: FnOnce() -> DatabaseConfig,
    shared_config(DatabaseConfig::default_name(), factory)
pub fn shared_api_config<F>(factory: F) -> Arc<ApiConfig>
    F: FnOnce() -> ApiConfig,
    shared_config(ApiConfig::default_name(), factory)
pub fn shared_security_config<F>(factory: F) -> Arc<SecurityConfig>
    F: FnOnce() -> SecurityConfig,
    shared_config(SecurityConfig::default_name(), factory)
