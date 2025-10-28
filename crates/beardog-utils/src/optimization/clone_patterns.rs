//! Clone Optimization Patterns for BearDog
//!
//! This module provides patterns and utilities for optimizing clone operations
//! throughout the BearDog codebase, focusing on performance improvements.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

/// Optimized Arc cloning pattern
/// Use Arc::clone(&arc) instead of arc.clone() for clarity and performance
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

/// String optimization patterns using Cow and Arc<str>
pub mod string_optimization {
    use super::*;

    /// Shared string pool for common strings
    pub struct SharedStringPool {
        pool: HashMap<String, Arc<str>>,
    }

    impl SharedStringPool {
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

    /// Convert Vec<String> to Vec<&str> for temporary operations
    pub fn string_vec_to_str_vec(strings: &[String]) -> Vec<&str> {
        strings.iter().map(|s| s.as_str()).collect()
    }
}

/// Configuration optimization patterns
pub mod config_optimization {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Wrapper for configuration that can be borrowed or owned
    #[derive(Debug)]
    pub struct ConfigCow<'a, T>
    where
        T: Clone,
    {
        pub inner: Cow<'a, T>,
    }

    impl<'a, T> ConfigCow<'a, T>
    where
        T: Clone,
    {
        pub fn borrowed(config: &'a T) -> Self {
            Self {
                inner: Cow::Borrowed(config),
            }
        }

        pub fn owned(config: T) -> Self {
            Self {
                inner: Cow::Owned(config),
            }
        }

        pub fn get(&self) -> &T {
            &self.inner
        }
    }

    /// Shared configuration using Arc
    pub struct SharedConfig<T> {
        config: Arc<T>,
    }

    impl<T> SharedConfig<T> {
        pub fn new(config: T) -> Self {
            Self {
                config: Arc::new(config),
            }
        }

        pub fn get(&self) -> &T {
            &self.config
        }

        pub fn clone_arc(&self) -> Arc<T> {
            Arc::clone(&self.config)
        }
    }
}

/// Metadata optimization patterns
pub mod metadata_optimization {
    use super::*;
    use serde_json::Value;

    /// Metadata that can be borrowed or owned
    pub struct MetadataCow<'a> {
        inner: Cow<'a, HashMap<String, Value>>,
    }

    impl<'a> MetadataCow<'a> {
        pub fn borrowed(metadata: &'a HashMap<String, Value>) -> Self {
            Self {
                inner: Cow::Borrowed(metadata),
            }
        }

        pub fn owned(metadata: HashMap<String, Value>) -> Self {
            Self {
                inner: Cow::Owned(metadata),
            }
        }

        pub fn get(&self) -> &HashMap<String, Value> {
            &self.inner
        }

        /// Convert to owned if needed for modification
        pub fn into_owned(self) -> HashMap<String, Value> {
            self.inner.into_owned()
        }
    }

    /// Shared metadata using Arc
    pub struct SharedMetadata {
        metadata: Arc<HashMap<String, Value>>,
    }

    impl SharedMetadata {
        pub fn new(metadata: HashMap<String, Value>) -> Self {
            Self {
                metadata: Arc::new(metadata),
            }
        }

        pub fn get(&self) -> &HashMap<String, Value> {
            &self.metadata
        }

        pub fn clone_arc(&self) -> Arc<HashMap<String, Value>> {
            Arc::clone(&self.metadata)
        }
    }
}

/// Request optimization patterns
pub mod request_optimization {
    use super::*;

    /// Request wrapper that avoids cloning when possible
    pub struct RequestRef<'a, T> {
        request: &'a T,
    }

    impl<'a, T> RequestRef<'a, T> {
        pub fn new(request: &'a T) -> Self {
            Self { request }
        }

        pub fn get(&self) -> &T {
            self.request
        }
    }

    /// Shared request using Arc for multiple consumers
    pub struct SharedRequest<T> {
        request: Arc<T>,
    }

    impl<T> SharedRequest<T> {
        pub fn new(request: T) -> Self {
            Self {
                request: Arc::new(request),
            }
        }

        pub fn get(&self) -> &T {
            &self.request
        }

        pub fn clone_arc(&self) -> Arc<T> {
            Arc::clone(&self.request)
        }
    }
}

/// Collection optimization patterns
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
    pub fn collect_ids_as_cow<T>(items: &[T]) -> Vec<Cow<str>>
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

/// Performance measurement utilities
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
        F2: Fn() -> (),
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

        // Should be the same Arc
        assert_eq!(s1.as_ptr(), s2.as_ptr());
    }

    #[test]
    fn test_config_optimization() {
        let config: HashMap<String, String> = HashMap::new();
        let shared = config_optimization::SharedConfig::new(config);
        let arc1 = shared.clone_arc();
        let arc2 = shared.clone_arc();

        // Should be the same Arc
        assert_eq!(Arc::as_ptr(&arc1), Arc::as_ptr(&arc2));
    }

    #[test]
    fn test_metadata_cow() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let cow = metadata_optimization::MetadataCow::borrowed(&metadata);
        assert_eq!(cow.get().len(), 1);
    }

    #[test]
    fn test_collection_optimization() {
        let items = vec!["a", "b", "c"];
        let refs = collection_optimization::collect_ids_as_refs(&items);
        assert_eq!(refs, vec!["a", "b", "c"]);
    }
}
