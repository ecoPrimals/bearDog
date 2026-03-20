// SPDX-License-Identifier: AGPL-3.0-only

//! Clone Optimization Patterns for BearDog
//!
//! This module provides patterns and utilities for optimizing clone operations
//! throughout the BearDog codebase, focusing on performance improvements.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

/// Optimized `Arc` cloning: prefer [`Arc::clone`] on references over moving clones blindly.
///
/// Use `Arc::clone(&arc)` instead of `arc.clone()` for clarity and performance
pub mod arc_optimization {
    use super::*;

    /// Efficient Arc cloning for shared resources
    pub fn clone_arc_efficiently<T>(arc: &Arc<T>) -> Arc<T> {
        Arc::clone(arc)
    }

    /// Example of optimized Arc usage in provider creation
    pub fn create_provider_optimized<T>(core: &Arc<T>, instance_id: String) -> (Arc<T>, String) {
        (Arc::clone(core), instance_id) // Move instance_id instead of cloning
    }
}

/// String deduplication via [`Arc<str>`] and cheap borrows into `&str` slices.
pub mod string_optimization {
    use super::*;

    /// In-process intern table mapping `String` keys to shared [`Arc<str>`] values.
    pub struct SharedStringPool {
        pool: HashMap<String, Arc<str>>,
    }

    impl Default for SharedStringPool {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SharedStringPool {
        /// Empty pool; call [`Self::get_shared`] to populate.
        pub fn new() -> Self {
            Self {
                pool: HashMap::new(),
            }
        }

        /// Get or create a shared string
        pub fn get_shared(&mut self, s: &str) -> Arc<str> {
            if let Some(shared) = self.pool.get(s) {
                Arc::clone(shared)
            } else {
                let shared: Arc<str> = s.into();
                self.pool.insert(s.to_string(), Arc::clone(&shared));
                shared
            }
        }
    }

    /// Convert `Vec<String>` to `Vec<&str>` for temporary operations
    pub fn string_vec_to_str_vec(strings: &[String]) -> Vec<&str> {
        strings.iter().map(|s| s.as_str()).collect()
    }
}

/// Share or borrow configuration: [`Cow`] for single owners, [`Arc`] for multi-consumer.
pub mod config_optimization {
    use super::*;

    /// Wrapper for configuration that can be borrowed or owned
    #[derive(Debug)]
    pub struct ConfigCow<'a, T>
    where
        T: Clone,
    {
        /// Either a borrowed view or an owned clone of `T`.
        pub inner: Cow<'a, T>,
    }

    impl<'a, T> ConfigCow<'a, T>
    where
        T: Clone,
    {
        /// Zero-copy wrapper around an existing `T` reference.
        pub fn borrowed(config: &'a T) -> Self {
            Self {
                inner: Cow::Borrowed(config),
            }
        }

        /// Moves `config` into an owned [`Cow`].
        pub fn owned(config: T) -> Self {
            Self {
                inner: Cow::Owned(config),
            }
        }

        /// Borrows the active configuration (shared or owned).
        pub fn get(&self) -> &T {
            &self.inner
        }
    }

    /// Immutable config behind [`Arc`] for cheap multi-handle sharing.
    pub struct SharedConfig<T> {
        config: Arc<T>,
    }

    impl<T> SharedConfig<T> {
        /// Wraps `config` in a new [`Arc`].
        pub fn new(config: T) -> Self {
            Self {
                config: Arc::new(config),
            }
        }

        /// Shared reference to the inner value.
        pub fn get(&self) -> &T {
            &self.config
        }

        /// Increments the [`Arc`] and returns a new handle.
        pub fn clone_arc(&self) -> Arc<T> {
            Arc::clone(&self.config)
        }
    }
}

/// JSON metadata as [`Cow`] or shared [`Arc`] to avoid cloning large maps.
pub mod metadata_optimization {
    use super::*;
    use serde_json::Value;

    /// Metadata that can be borrowed or owned
    pub struct MetadataCow<'a> {
        inner: Cow<'a, HashMap<String, Value>>,
    }

    impl<'a> MetadataCow<'a> {
        /// Borrows an existing metadata map without allocating.
        pub fn borrowed(metadata: &'a HashMap<String, Value>) -> Self {
            Self {
                inner: Cow::Borrowed(metadata),
            }
        }

        /// Takes ownership of a metadata map.
        pub fn owned(metadata: HashMap<String, Value>) -> Self {
            Self {
                inner: Cow::Owned(metadata),
            }
        }

        /// Read-only view of the map regardless of borrow state.
        pub fn get(&self) -> &HashMap<String, Value> {
            &self.inner
        }

        /// Convert to owned if needed for modification
        pub fn into_owned(self) -> HashMap<String, Value> {
            self.inner.into_owned()
        }
    }

    /// Shared JSON metadata for handlers that only need read access.
    pub struct SharedMetadata {
        metadata: Arc<HashMap<String, Value>>,
    }

    impl SharedMetadata {
        /// Stores `metadata` in a new [`Arc`].
        pub fn new(metadata: HashMap<String, Value>) -> Self {
            Self {
                metadata: Arc::new(metadata),
            }
        }

        /// Borrows the shared map.
        pub fn get(&self) -> &HashMap<String, Value> {
            &self.metadata
        }

        /// Clones the [`Arc`] handle (not the map contents).
        pub fn clone_arc(&self) -> Arc<HashMap<String, Value>> {
            Arc::clone(&self.metadata)
        }
    }
}

/// Pass requests by reference or [`Arc`] so downstream layers never clone large bodies twice.
pub mod request_optimization {
    use super::*;

    /// Zero-cost handle to a request living elsewhere for the lifetime `'a`.
    pub struct RequestRef<'a, T> {
        request: &'a T,
    }

    impl<'a, T> RequestRef<'a, T> {
        /// Wraps a borrowed request.
        pub fn new(request: &'a T) -> Self {
            Self { request }
        }

        /// Returns the borrowed request.
        pub fn get(&self) -> &T {
            self.request
        }
    }

    /// Shared request using Arc for multiple consumers
    pub struct SharedRequest<T> {
        request: Arc<T>,
    }

    impl<T> SharedRequest<T> {
        /// Moves `request` behind an [`Arc`].
        pub fn new(request: T) -> Self {
            Self {
                request: Arc::new(request),
            }
        }

        /// Immutable view of the shared request.
        pub fn get(&self) -> &T {
            &self.request
        }

        /// Additional [`Arc`] handle for fan-out.
        pub fn clone_arc(&self) -> Arc<T> {
            Arc::clone(&self.request)
        }
    }
}

/// Iterator- and [`Cow`]-based collection patterns to skip intermediate `Vec` clones.
pub mod collection_optimization {
    use super::*;

    /// Avoid cloning when collecting IDs
    pub fn collect_ids_as_refs<T>(items: &[T]) -> Vec<&str>
    where
        T: AsRef<str>,
    {
        items.iter().map(|item| item.as_ref()).collect()
    }

    /// Collect IDs using Cow for flexibility
    pub fn collect_ids_as_cow<T>(items: &[T]) -> Vec<Cow<'_, str>>
    where
        T: AsRef<str>,
    {
        items
            .iter()
            .map(|item| Cow::Borrowed(item.as_ref()))
            .collect()
    }

    /// Use iterators instead of collecting when possible
    pub fn process_items_lazy<T, F, R>(items: &[T], processor: F) -> impl Iterator<Item = R> + '_
    where
        F: Fn(&T) -> R + 'static,
        R: 'static,
    {
        items.iter().map(processor)
    }
}

/// Wall-clock helpers for comparing clone-heavy vs reference-heavy code paths.
pub mod performance_measurement {
    use std::time::{Duration, Instant};

    /// Measure clone operation performance
    pub fn measure_clone_performance<T, F>(operation: F, iterations: usize) -> Duration
    where
        T: Clone,
        F: Fn() -> T,
    {
        let start = Instant::now();

        for _ in 0..iterations {
            let _result = operation();
        }

        start.elapsed()
    }

    /// Compare clone vs reference performance
    pub fn compare_clone_vs_ref<T, F1, F2>(
        clone_op: F1,
        ref_op: F2,
        iterations: usize,
    ) -> (Duration, Duration)
    where
        T: Clone,
        F1: Fn() -> T,
        F2: Fn(),
    {
        let clone_time = measure_clone_performance(clone_op, iterations);

        let start = Instant::now();
        for _ in 0..iterations {
            ref_op();
        }
        let ref_time = start.elapsed();

        (clone_time, ref_time)
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_arc_optimization() {
        let data = Arc::new(String::from("test"));
        let optimized = arc_optimization::clone_arc_efficiently(&data);
        assert_eq!(*optimized, *data);
    }

    #[test]
    fn test_string_optimization() {
        let mut pool = string_optimization::SharedStringPool::new();
        let s1 = pool.get_shared("test");
        let s2 = pool.get_shared("test");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Should be the same Arc
        assert_eq!(s1.as_ptr(), s2.as_ptr());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_config_optimization() {
        let config: HashMap<String, String> = HashMap::new();
        let shared = config_optimization::SharedConfig::new(config);
        let arc1 = shared.clone_arc();
        let arc2 = shared.clone_arc();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Should be the same Arc
        assert_eq!(Arc::as_ptr(&arc1), Arc::as_ptr(&arc2));
    }

    #[test]
    fn test_metadata_cow() {
        let mut metadata = HashMap::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let cow = metadata_optimization::MetadataCow::borrowed(&metadata);
        assert_eq!(cow.get().len(), 1);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_collection_optimization() {
        let items = vec!["a", "b", "c"];
        let refs = collection_optimization::collect_ids_as_refs(&items);
        assert_eq!(refs, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_create_provider_optimized() {
        let core = Arc::new(42);
        let (cloned_arc, id) =
            arc_optimization::create_provider_optimized(&core, "id123".to_string());
        assert_eq!(*cloned_arc, 42);
        assert_eq!(id, "id123");
    }

    #[test]
    fn test_shared_string_pool_default() {
        let _ = string_optimization::SharedStringPool::default();
        // Successfully created default pool
    }

    #[test]
    fn test_shared_string_pool_multiple_different_strings() {
        let mut pool = string_optimization::SharedStringPool::new();
        let s1 = pool.get_shared("first");
        let s2 = pool.get_shared("second");
        let s3 = pool.get_shared("first"); // Should reuse s1

        assert_ne!(s1.as_ptr(), s2.as_ptr());
        assert_eq!(s1.as_ptr(), s3.as_ptr());
    }

    #[test]
    fn test_string_vec_to_str_vec() {
        let strings = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let strs = string_optimization::string_vec_to_str_vec(&strings);
        assert_eq!(strs, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_string_vec_to_str_vec_empty() {
        let strings: Vec<String> = vec![];
        let strs = string_optimization::string_vec_to_str_vec(&strings);
        assert!(strs.is_empty());
    }

    #[test]
    fn test_config_cow_borrowed() {
        let config = 42;
        let cow = config_optimization::ConfigCow::borrowed(&config);
        assert_eq!(*cow.get(), 42);
    }

    #[test]
    fn test_config_cow_owned() {
        let cow = config_optimization::ConfigCow::owned(42);
        assert_eq!(*cow.get(), 42);
    }

    #[test]
    fn test_shared_config_new() {
        let config: HashMap<String, i32> = [("key".to_string(), 42)].iter().cloned().collect();
        let shared = config_optimization::SharedConfig::new(config);
        assert_eq!(shared.get().get("key"), Some(&42));
    }

    #[test]
    fn test_shared_config_get() {
        let mut config = HashMap::new();
        config.insert("test".to_string(), "value".to_string());
        let shared = config_optimization::SharedConfig::new(config);

        let retrieved = shared.get();
        assert_eq!(retrieved.get("test"), Some(&"value".to_string()));
    }

    #[test]
    fn test_metadata_cow_owned() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let cow = metadata_optimization::MetadataCow::owned(metadata);
        assert_eq!(cow.get().len(), 1);
        assert_eq!(cow.get().get("key"), Some(&serde_json::json!("value")));
    }

    #[test]
    fn test_metadata_cow_into_owned() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!(42));

        let cow = metadata_optimization::MetadataCow::owned(metadata);
        let owned = cow.into_owned();
        assert_eq!(owned.get("key"), Some(&serde_json::json!(42)));
    }

    #[test]
    fn test_shared_metadata_new() {
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), serde_json::json!("data"));

        let shared = metadata_optimization::SharedMetadata::new(metadata);
        assert_eq!(shared.get().len(), 1);
    }

    #[test]
    fn test_shared_metadata_clone_arc() {
        let metadata = HashMap::new();
        let shared = metadata_optimization::SharedMetadata::new(metadata);

        let arc1 = shared.clone_arc();
        let arc2 = shared.clone_arc();
        assert_eq!(Arc::as_ptr(&arc1), Arc::as_ptr(&arc2));
    }

    #[test]
    fn test_request_ref_new() {
        let request = "test request";
        let request_ref = request_optimization::RequestRef::new(&request);
        assert_eq!(*request_ref.get(), "test request");
    }

    #[test]
    fn test_shared_request_new() {
        let request = vec![1, 2, 3];
        let shared = request_optimization::SharedRequest::new(request);
        assert_eq!(shared.get(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_shared_request_clone_arc() {
        let request = "test".to_string();
        let shared = request_optimization::SharedRequest::new(request);

        let arc1 = shared.clone_arc();
        let arc2 = shared.clone_arc();
        assert_eq!(Arc::as_ptr(&arc1), Arc::as_ptr(&arc2));
    }

    #[test]
    fn test_collect_ids_as_cow() {
        let items = vec!["first".to_string(), "second".to_string()];
        let cows = collection_optimization::collect_ids_as_cow(&items);

        assert_eq!(cows.len(), 2);
        assert_eq!(cows[0], "first");
        assert_eq!(cows[1], "second");
    }

    #[test]
    fn test_collect_ids_as_cow_empty() {
        let items: Vec<String> = vec![];
        let cows = collection_optimization::collect_ids_as_cow(&items);
        assert!(cows.is_empty());
    }

    #[test]
    fn test_process_items_lazy() {
        let items = vec![1, 2, 3, 4, 5];
        let processor = |x: &i32| x * 2;
        let results: Vec<_> =
            collection_optimization::process_items_lazy(&items, processor).collect();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_measure_clone_performance() {
        let operation = || vec![1, 2, 3];
        let duration = performance_measurement::measure_clone_performance(operation, 100);
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_compare_clone_vs_ref() {
        let data = vec![1; 1000];
        let clone_op = || data.clone();
        let ref_op = || {
            let _ = &data;
        };

        let (clone_time, ref_time) =
            performance_measurement::compare_clone_vs_ref(clone_op, ref_op, 100);

        // Both should complete successfully
        assert!(clone_time.as_nanos() > 0);
        assert!(ref_time.as_nanos() > 0);
    }

    #[test]
    fn test_collect_ids_as_refs_with_strings() {
        let items = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let refs = collection_optimization::collect_ids_as_refs(&items);
        assert_eq!(refs, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn test_collect_ids_as_refs_empty() {
        let items: Vec<String> = vec![];
        let refs = collection_optimization::collect_ids_as_refs(&items);
        assert!(refs.is_empty());
    }
}
