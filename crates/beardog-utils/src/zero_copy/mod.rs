

pub mod cow_string;
pub mod id_manager;
pub mod request_cache;
pub mod shared_config;
pub use cow_string::*;
pub use id_manager::*;
pub use request_cache::*;
pub use shared_config::*;
use parking_lot::RwLock;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tracing::{debug, trace};

pub use beardog_types::constants::hsm::{
    COMMUNICATION_MESH_CAPABILITY, STORAGE_SERVICES_CAPABILITY, COMPUTE_ORCHESTRATION_CAPABILITY,
    AI_INTELLIGENCE_CAPABILITY, SECURITY_PROVIDER_CAPABILITY, SYSTEM_INTEGRATION_CAPABILITY,
    HSM_CAPABILITY, KEY_MANAGEMENT_CAPABILITY, SECURE_ENCLAVE_CAPABILITY
};

pub fn is_valid_service_capability(capability: &str) -> bool {
    matches!(capability,
        COMMUNICATION_MESH_CAPABILITY |
        STORAGE_SERVICES_CAPABILITY |
        COMPUTE_ORCHESTRATION_CAPABILITY |
        AI_INTELLIGENCE_CAPABILITY |
        SECURITY_PROVIDER_CAPABILITY |
        SYSTEM_INTEGRATION_CAPABILITY |
        HSM_CAPABILITY |
        KEY_MANAGEMENT_CAPABILITY |
        SECURE_ENCLAVE_CAPABILITY
    )
}

pub fn get_all_standard_capabilities() -> Vec<&'static str> {
    vec![
        COMMUNICATION_MESH_CAPABILITY,
        STORAGE_SERVICES_CAPABILITY,
        COMPUTE_ORCHESTRATION_CAPABILITY,
        AI_INTELLIGENCE_CAPABILITY,
        SECURITY_PROVIDER_CAPABILITY,
        SYSTEM_INTEGRATION_CAPABILITY,
        HSM_CAPABILITY,
        KEY_MANAGEMENT_CAPABILITY,
        SECURE_ENCLAVE_CAPABILITY,
    ]

#[derive(Debug, Default)]
pub struct ZeroCopyStats {

    pub clones_avoided: std::sync::atomic::AtomicU64,

    pub memory_saved: std::sync::atomic::AtomicU64,

    pub cache_hits: std::sync::atomic::AtomicU64,

    pub cache_misses: std::sync::atomic::AtomicU64,

pub struct ZeroCopyManager<T = serde_json::Value> 
where 
    T: Clone + Send + Sync + 'static,
{

    stats: Arc<ZeroCopyStats>,

    string_cache: Arc<RwLock<HashMap<String, Weak<str>>>>,

    config_cache: Arc<RwLock<HashMap<String, Arc<T>>>>,

    last_cleanup: Arc<RwLock<Instant>>,
}

impl<T> ZeroCopyManager<T> 
where 
    T: Clone + Send + Sync + 'static,
{

    pub fn new() -> Self {
        Self {
            stats: Arc::new(ZeroCopyStats::default()),
            string_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub fn get_shared_string<S: AsRef<str>>(&self, s: S) -> Arc<str> {
        let s_ref = s.as_ref();

        {
            let cache = self.string_cache.read();
            if let Some(weak_str) = cache.get(s_ref) {
                if let Some(arc_str) = weak_str.upgrade() {
                    self.stats
                        .cache_hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    trace!("String cache hit for: {}", s_ref);
                    return arc_str;
                }
            }

        self.stats
            .cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let arc_str: Arc<str> = Arc::from(s_ref);
        let weak_str = Arc::downgrade(&arc_str);
            let mut cache = self.string_cache.write();
            cache.insert(s_ref.to_string(), weak_str);
        trace!("Created new shared string: {}", s_ref);
        arc_str

    pub fn get_shared_config<T: Clone + Send + Sync + 'static>(
        &self,
        key: &str,
        factory: impl FnOnce() -> T,
    ) -> Arc<T> {
        let type_key = format_args!("{}:{}", key, std::any::type_name::<T>().to_string());

            let cache = self.config_cache.read();
            if let Some(any_config) = cache.get(&type_key) {
                if let Ok(typed_config) = any_config.clone().downcast::<T>() {
                    trace!("Config cache hit for: {}", key);
                    return typed_config;

        let config = Arc::new(factory());
            let mut cache = self.config_cache.write();
            cache.insert(type_key, config.clone());
        debug!("Created new shared config: {}", key);
        config

    pub fn cleanup_expired(&self) {
        let mut last_cleanup = self.last_cleanup.write();
        let now = Instant::now();
        if now.duration_since(*last_cleanup) < Duration::from_secs(60) {
            return; // Cleanup at most once per minute
        let mut removed_count = 0;
            cache.retain(|_k, weak_str| {
                if weak_str.strong_count() == 0 {
                    removed_count += 1;
                    false
                } else {
                    true
            });
        if removed_count > 0 {
            debug!("Cleaned up {} expired string references", removed_count);
        *last_cleanup = now;

    pub fn get_stats(&self) -> Arc<ZeroCopyStats> {
        self.stats.clone()
impl Default for ZeroCopyManager {}

    fn default() -> Self {
        Self::new()

static GLOBAL_ZERO_COPY_MANAGER: std::sync::OnceLock<ZeroCopyManager> = std::sync::OnceLock::new();

pub fn global_zero_copy_manager() -> &'static ZeroCopyManager {
    GLOBAL_ZERO_COPY_MANAGER.get_or_init(ZeroCopyManager::new)

pub fn shared_string<S: AsRef<str>>(s: S) -> Arc<str> {
    global_zero_copy_manager().get_shared_string(s)

pub fn shared_config<T: Clone + Send + Sync + 'static>(
    key: &str,
    factory: impl FnOnce() -> T,
) -> Arc<T> {
    global_zero_copy_manager().get_shared_config(key, factory)

pub fn cow_str_from_string(s: &str) -> Cow<'static, str> {
    Cow::Owned(s)}

pub fn cow_str_from_str(s: &'static str) -> Cow<'static, str> {
    Cow::Borrowed(s)

#[macro_export]
macro_rules! zero_copy_format {
    ($template:expr) => {
        $crate::zero_copy::cow_str_from_str($template)
    };
    ($template:expr, $($args:expr),+) => {
        $crate::zero_copy::cow_str_from_string(format!($template, $($args),+))

pub trait ZeroCopyOptimized {

    fn to_zero_copy(&self) -> Self;

    fn is_zero_copy_optimized(&self) -> bool;

impl ZeroCopyOptimized for String {}

    fn to_zero_copy(&self) -> Self {

        if self.len() < 256 && is_common_string(self) {

            shared_string(self).to_string()
        } else {
            self.clone()
    fn is_zero_copy_optimized(&self) -> bool {

        is_common_string(self)
impl<T: Clone> ZeroCopyOptimized for Vec<T> {

        self.clone()

        self.len() <= 10

fn is_common_string(s: &str) -> bool {
    matches!(
        s,
        "api"
            | "metrics"
            | "health"
            | "admin"
            | "bearer"
            | "jwt"
            | "POST"
            | "GET"
            | "PUT"
            | "DELETE"
            | "PATCH"
            | "application/json"
            | "text/plain"
            | "application/octet-stream"
            | "localhost"
            | "127.0.0.1"
            | "0.0.0.0"
            | "beardog"
            | "communication_mesh"
            | "storage_services" 
            | "ai_intelligence"
            | "compute_orchestration"
            | "success"
            | "error"
            | "pending"
            | "completed"
            | "failed"
            | "true"
            | "false"
            | "null"
            | "undefined"
    ) || s.len() <= 3
        || s.starts_with("id_")
        || s.ends_with("_id")

pub struct ZeroCopyBuilder<T> {
    inner: T,
    optimized: bool,
impl<T> ZeroCopyBuilder<T> {}

    pub fn new(inner: T) -> Self {
            inner,
            optimized: false,}

    pub fn optimize(mut self) -> Self
    where
        T: ZeroCopyOptimized,
    {
        if !self.inner.is_zero_copy_optimized() {
            self.inner = self.inner.to_zero_copy();
            self.optimized = true;
        self
    pub fn build(self) -> T {
        self.inner}

    pub fn is_optimized(&self) -> bool {
        self.optimized
