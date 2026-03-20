// SPDX-License-Identifier: AGPL-3.0-only

//! Heuristic [`CloneOptimizer`], [`SharedOwnership`], and [`CopyOnWrite`] helpers.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

/// Counters for analysis passes and estimated bytes saved.
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of types_analyzed
    pub types_analyzed: u64,
    /// Number of optimizations_applied
    pub optimizations_applied: u64,
    /// Number of memory_saved
    pub memory_saved: u64,
}

/// Maps type names to the last inferred [`CloneOptimizationStrategy`].
#[derive(Debug, Clone)]
pub struct CloneOptimizer {
    strategies: HashMap<String, CloneOptimizationStrategy>,
    stats: OptimizationStats,
}

/// High-level clone-avoidance tactic chosen from usage pattern strings.
#[derive(Debug, Clone)]
pub enum CloneOptimizationStrategy {
    /// Prefer [`Arc`] for cross-thread sharing.
    SharedOwnership,
    /// Prefer [`Cow`] when mutation is rare.
    CopyOnWrite,
    /// Use references instead of cloning
    BorrowInstead,
    /// Use zero-copy string interning
    ZeroCopy,
}

impl CloneOptimizer {
    /// Empty strategy map and zeroed stats.
    pub fn new() -> Self {
        Self {
            strategies: HashMap::with_capacity(16),
            stats: OptimizationStats::default(),
        }
    }

    /// Records and returns a strategy from a coarse `usage_pattern` label.
    pub fn analyze_type(
        &mut self,
        type_name: &str,
        usage_pattern: &str,
    ) -> CloneOptimizationStrategy {
        let strategy = match usage_pattern {
            "shared_across_threads" => CloneOptimizationStrategy::SharedOwnership,
            "occasional_modification" => CloneOptimizationStrategy::CopyOnWrite,
            "read_only_access" => CloneOptimizationStrategy::BorrowInstead,
            "string_operations" => CloneOptimizationStrategy::ZeroCopy,
            _ => CloneOptimizationStrategy::BorrowInstead,
        };

        self.strategies
            .insert(type_name.to_string(), strategy.clone());
        self.stats.types_analyzed += 1;

        strategy
    }

    /// Looks up the last strategy chosen for `type_name`.
    pub fn get_optimization(&self, type_name: &str) -> Option<&CloneOptimizationStrategy> {
        self.strategies.get(type_name)
    }

    /// Borrow aggregate statistics.
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }
}

impl Default for CloneOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Newtype around [`Arc<T>`] with cheap [`Clone`] via refcounting.
#[derive(Debug)]
pub struct SharedOwnership<T> {
    inner: Arc<T>,
}

impl<T> SharedOwnership<T> {
    /// Wraps `value` in a fresh [`Arc`].
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }

    /// Adopts an existing [`Arc`].
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self { inner: arc }
    }
}

impl<T> Clone for SharedOwnership<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> std::ops::Deref for SharedOwnership<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// [`Cow`]-backed cell that clones lazily on mutation.
#[derive(Debug, Clone)]
pub struct CopyOnWrite<T: Clone + 'static> {
    inner: Cow<'static, T>,
}

impl<T: Clone + 'static> CopyOnWrite<T> {
    /// Starts in fully owned mode.
    pub fn new(value: T) -> Self {
        Self {
            inner: Cow::Owned(value),
        }
    }

    /// Borrows a `'static` value until mutation forces a clone.
    pub fn from_borrowed(value: &'static T) -> Self {
        Self {
            inner: Cow::Borrowed(value),
        }
    }

    /// Ensures unique ownership, cloning if currently borrowed.
    pub fn to_mut(&mut self) -> &mut T {
        self.inner.to_mut()
    }
}

impl<T: Clone + 'static> std::ops::Deref for CopyOnWrite<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Small composable functions demonstrating each strategy.
pub mod patterns {
    use super::*;

    /// Optimize String Sharing operation.
    pub fn optimize_string_sharing(s: &str) -> Arc<str> {
        Arc::from(s)
    }

    /// Optimize Config Sharing operation.
    pub fn optimize_config_sharing<T: Clone>(config: &T) -> SharedOwnership<T> {
        SharedOwnership::new(config.clone())
    }

    /// Clones `value` into an owned [`CopyOnWrite`] container.
    pub fn create_copy_on_write<T: Clone + 'static>(value: &'static T) -> CopyOnWrite<T> {
        CopyOnWrite::new(value.clone())
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_optimizer() {
        let mut optimizer = CloneOptimizer::new();

        let strategy = optimizer.analyze_type("String", "shared_across_threads");
        assert!(matches!(
            strategy,
            CloneOptimizationStrategy::SharedOwnership
        ));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        let strategy = optimizer.analyze_type("Vec<u8>", "occasional_modification");
        assert!(matches!(strategy, CloneOptimizationStrategy::CopyOnWrite));

        assert_eq!(optimizer.get_stats().types_analyzed, 2);
    }

    #[test]
    fn test_shared_ownership() {
        let data = vec![1, 2, 3, 4, 5];
        let shared1 = SharedOwnership::new(data);
        let shared2 = shared1.clone();

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(shared1.len(), 5);
        assert_eq!(shared2.len(), 5);

        // Verify they share the same Arc
        assert_eq!(Arc::as_ptr(&shared1.inner), Arc::as_ptr(&shared2.inner));
    }

    #[test]
    fn test_copy_on_write() {
        let original_data = vec![1, 2, 3];
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut cow = CopyOnWrite::new(original_data);

        // Initially owned
        assert_eq!(cow.len(), 3);

        // Modify through to_mut
        cow.to_mut().push(4);
        assert_eq!(cow.len(), 4);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_patterns() {
        let shared_str = patterns::optimize_string_sharing("test string");
        assert_eq!(&*shared_str, "test string");

        let config = vec![1, 2, 3];
        let shared_config = patterns::optimize_config_sharing(&config);
        assert_eq!(shared_config.len(), 3);
    }

    #[test]
    fn test_clone_optimizer_default() {
        let optimizer = CloneOptimizer::default();
        assert_eq!(optimizer.get_stats().types_analyzed, 0);
        assert_eq!(optimizer.get_stats().optimizations_applied, 0);
    }

    #[test]
    fn test_analyze_all_patterns() {
        let mut optimizer = CloneOptimizer::new();

        let s1 = optimizer.analyze_type("A", "shared_across_threads");
        assert!(matches!(s1, CloneOptimizationStrategy::SharedOwnership));

        let s2 = optimizer.analyze_type("B", "occasional_modification");
        assert!(matches!(s2, CloneOptimizationStrategy::CopyOnWrite));

        let s3 = optimizer.analyze_type("C", "read_only_access");
        assert!(matches!(s3, CloneOptimizationStrategy::BorrowInstead));

        let s4 = optimizer.analyze_type("D", "string_operations");
        assert!(matches!(s4, CloneOptimizationStrategy::ZeroCopy));

        let s5 = optimizer.analyze_type("E", "unknown_pattern");
        assert!(matches!(s5, CloneOptimizationStrategy::BorrowInstead));

        assert_eq!(optimizer.get_stats().types_analyzed, 5);
    }

    #[test]
    fn test_get_optimization_existing() {
        let mut optimizer = CloneOptimizer::new();
        optimizer.analyze_type("MyType", "shared_across_threads");

        let opt = optimizer.get_optimization("MyType");
        assert!(opt.is_some());
        assert!(matches!(
            opt.unwrap(),
            CloneOptimizationStrategy::SharedOwnership
        ));
    }

    #[test]
    fn test_get_optimization_missing() {
        let optimizer = CloneOptimizer::new();
        assert!(optimizer.get_optimization("NonExistent").is_none());
    }

    #[test]
    fn test_shared_ownership_from_arc() {
        let arc = Arc::new(vec![10, 20, 30]);
        let shared = SharedOwnership::from_arc(arc);
        assert_eq!(shared.len(), 3);
        assert_eq!(shared[0], 10);
    }

    #[test]
    fn test_copy_on_write_from_borrowed() {
        static DATA: Vec<i32> = Vec::new();
        let cow = CopyOnWrite::from_borrowed(&DATA);
        assert_eq!(cow.len(), 0);
    }

    #[test]
    fn test_copy_on_write_to_mut_triggers_copy() {
        let mut cow = CopyOnWrite::new(vec![1, 2, 3]);
        cow.to_mut().push(4);
        assert_eq!(cow.len(), 4);
    }

    #[test]
    fn test_create_copy_on_write_pattern() {
        static VALUE: i32 = 42;
        let cow = patterns::create_copy_on_write(&VALUE);
        assert_eq!(*cow, 42);
    }
}
