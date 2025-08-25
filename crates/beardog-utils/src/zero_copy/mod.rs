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


/// Zero-Copy Utilities Module
///
/// Provides comprehensive zero-copy optimizations to eliminate excessive cloning
/// throughout the `BearDog` codebase. This module focuses on the most common
/// cloning patterns identified in the performance audit.

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
/// Standard ecosystem service capabilities
// Re-export canonical capability constants
pub use beardog_types::constants::hsm::{
    COMMUNICATION_MESH_CAPABILITY, STORAGE_SERVICES_CAPABILITY, COMPUTE_ORCHESTRATION_CAPABILITY,
    AI_INTELLIGENCE_CAPABILITY, SECURITY_PROVIDER_CAPABILITY, SYSTEM_INTEGRATION_CAPABILITY,
    HSM_CAPABILITY, KEY_MANAGEMENT_CAPABILITY, SECURE_ENCLAVE_CAPABILITY
};
/// Validate service capability
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
/// Get all standard ecosystem capabilities
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
/// Zero-copy optimization statistics
#[derive(Debug, Default)]
pub struct ZeroCopyStats {
    /// Number of clones avoided
    pub clones_avoided: std::sync::atomic::AtomicU64,
    /// Memory bytes saved
    pub memory_saved: std::sync::atomic::AtomicU64,
    /// Cache hits for shared data
    pub cache_hits: std::sync::atomic::AtomicU64,
    /// Cache misses
    pub cache_misses: std::sync::atomic::AtomicU64,
/// Global zero-copy optimization manager
pub struct ZeroCopyManager {
    /// Statistics tracking
    stats: Arc<ZeroCopyStats>,
    /// Shared string cache for common strings
    string_cache: Arc<RwLock<HashMap<String, Weak<str>>>>,
    /// Configuration cache
    config_cache: Arc<RwLock<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>>,
    /// Cleanup timer
    last_cleanup: Arc<RwLock<Instant>>,}


impl ZeroCopyManager {
    /// Create a new zero-copy manager}


    pub fn new() -> Self {
        Self {
            stats: Arc::new(ZeroCopyStats::default()),
            string_cache: Arc::new(RwLock::new(HashMap::new())),
            config_cache: Arc::new(RwLock::new(HashMap::new())),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }
    /// Get or create an Arc<str> for a string, avoiding clones
    pub fn get_shared_string<S: AsRef<str>>(&self, s: S) -> Arc<str> {
        let s_ref = s.as_ref();
        // Try to find existing Arc<str>
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
        // Create new Arc<str> and cache it
        self.stats
            .cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let arc_str: Arc<str> = Arc::from(s_ref);
        let weak_str = Arc::downgrade(&arc_str);
            let mut cache = self.string_cache.write();
            cache.insert(s_ref.to_string(), weak_str);
        trace!("Created new shared string: {}", s_ref);
        arc_str
    /// Get or create shared configuration object
    pub fn get_shared_config<T: Clone + Send + Sync + 'static>(
        &self,
        key: &str,
        factory: impl FnOnce() -> T,
    ) -> Arc<T> {
        let type_key = format!("{}:{}", key, std::any::type_name::<T>());
        // Try to find existing config
            let cache = self.config_cache.read();
            if let Some(any_config) = cache.get(&type_key) {
                if let Ok(typed_config) = any_config.clone().downcast::<T>() {
                    trace!("Config cache hit for: {}", key);
                    return typed_config;
        // Create new config and cache it
        let config = Arc::new(factory());
            let mut cache = self.config_cache.write();
            cache.insert(type_key, config.clone());
        debug!("Created new shared config: {}", key);
        config
    /// Cleanup expired weak references
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
    /// Get current statistics
    pub fn get_stats(&self) -> Arc<ZeroCopyStats> {
        self.stats.clone()
impl Default for ZeroCopyManager {}


    fn default() -> Self {
        Self::new()
/// Global zero-copy manager instance
static GLOBAL_ZERO_COPY_MANAGER: std::sync::OnceLock<ZeroCopyManager> = std::sync::OnceLock::new();
/// Get the global zero-copy manager}


pub fn global_zero_copy_manager() -> &'static ZeroCopyManager {
    GLOBAL_ZERO_COPY_MANAGER.get_or_init(ZeroCopyManager::new)
/// Create a shared string that can be reused across the application
pub fn shared_string<S: AsRef<str>>(s: S) -> Arc<str> {
    global_zero_copy_manager().get_shared_string(s)
/// Create a shared configuration object}


pub fn shared_config<T: Clone + Send + Sync + 'static>(
    key: &str,
    factory: impl FnOnce() -> T,
) -> Arc<T> {
    global_zero_copy_manager().get_shared_config(key, factory)
/// Cow string utilities for zero-copy string handling}


pub fn cow_str_from_string(s: String) -> Cow<'static, str> {
    Cow::Owned(s)}


pub fn cow_str_from_str(s: &'static str) -> Cow<'static, str> {
    Cow::Borrowed(s)
/// Efficient string formatting without unnecessary allocations
#[macro_export]
macro_rules! zero_copy_format {
    ($template:expr) => {
        $crate::zero_copy::cow_str_from_str($template)
    };
    ($template:expr, $($args:expr),+) => {
        $crate::zero_copy::cow_str_from_string(format!($template, $($args),+))
/// Trait for types that can be optimized for zero-copy operations
pub trait ZeroCopyOptimized {
    /// Convert to a zero-copy optimized version
    fn to_zero_copy(&self) -> Self;
    /// Whether this instance is already zero-copy optimized
    fn is_zero_copy_optimized(&self) -> bool;
/// Implement zero-copy optimization for common types
impl ZeroCopyOptimized for String {}


    fn to_zero_copy(&self) -> Self {
        // For String, we can't avoid the clone, but we can intern common strings
        if self.len() < 256 && is_common_string(self) {
            // Convert common strings to shared strings to reduce duplication
            shared_string(self).to_string()
        } else {
            self.clone()
    fn is_zero_copy_optimized(&self) -> bool {
        // Check if this string is likely interned by checking against common patterns
        is_common_string(self)
impl<T: Clone> ZeroCopyOptimized for Vec<T> {
        // For Vec, return as-is since cloning is sometimes necessary
        // but we can optimize specific use cases
        self.clone()
        // Small vectors are generally acceptable to clone
        self.len() <= 10
/// Check if a string is commonly used and should be interned}


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
/// Zero-copy request/response builder
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
