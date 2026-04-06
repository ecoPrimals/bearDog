// SPDX-License-Identifier: AGPL-3.0-or-later

//! Testing Utilities for Truly Concurrent Tests
//!
//! **Build gating:** this tree is compiled only when building `beardog-utils` as a test
//! target (`cfg(test)`) or with the `test-utils` feature. It is not part of the default
//! production library surface.
//!
//! This module provides infrastructure for writing **deterministic, concurrent tests**
//! without relying on `sleep()` or other timing-based synchronization.
//!
//! ## Philosophy
//!
//! > **"Test issues ARE production issues"**
//!
//! Tests must be:
//! - ✅ **Deterministic**: Same result every time
//! - ✅ **Fast**: No waiting for real time
//! - ✅ **Concurrent**: Run in parallel safely
//! - ✅ **Robust**: No race conditions or flaky behavior
//!
//! ## Patterns
//!
//! ### **1. Mock Time** (for expiration/timeout tests)
//! ```rust
//! use beardog_utils::testing::MockTimeSource;
//!
//! #[test]
//! fn test_cache_expiration() {
//!     let time = MockTimeSource::new();
//!     let cache = Cache::with_time_source(time.clone());
//!     
//!     cache.insert("key", "value", Duration::from_secs(60));
//!     time.advance(Duration::from_secs(61)); // Instant!
//!     
//!     assert!(cache.get("key").is_none());
//! }
//! ```
//!
//! ### **2. Event Synchronization** (for async operations)
//! ```rust
//! use beardog_utils::testing::EventWaiter;
//!
//! #[tokio::test]
//! async fn test_async_operation() {
//!     let (waiter, trigger) = EventWaiter::new();
//!     
//!     spawn_operation(move |result| {
//!         trigger.notify(result);
//!     });
//!     
//!     let result = waiter.wait(Duration::from_secs(1)).await.unwrap();
//!     assert!(result.is_success());
//! }
//! ```
//!
//! ### **3. Test Barriers** (for multi-task coordination)
//! ```rust
//! use beardog_utils::testing::TestBarrier;
//!
//! #[tokio::test]
//! async fn test_concurrent_operations() {
//!     let barrier = TestBarrier::new(3);
//!     
//!     for i in 0..3 {
//!         let barrier = barrier.clone();
//!         tokio::spawn(async move {
//!             do_work(i);
//!             barrier.wait().await;
//!         });
//!     }
//!     
//!     barrier.wait().await; // All tasks completed
//! }
//! ```
//!
//! ### **4. Tokio Time Control** (for async sleep tests)
//! ```rust
//! #[tokio::test(start_paused = true)]
//! async fn test_delayed_operation() {
//!     let task = tokio::spawn(async {
//!         tokio::time::sleep(Duration::from_secs(3600)).await;
//!         "done"
//!     });
//!     
//!     tokio::time::advance(Duration::from_secs(3600)).await;
//!     assert_eq!(task.await.unwrap(), "done");
//! }
//! ```

pub mod concurrent;
pub mod mock_time;
pub mod sync;

pub use mock_time::{MockTimeSource, SystemTimeSource, TimeSource};
pub use sync::{EventWaiter, TestBarrier, TestLatch};

/// Re-export commonly used testing utilities
pub mod prelude {
    pub use super::concurrent::{
        Barrier, RateLimiter, assert_eventually, spawn_with_completion, wait_for,
    };
    pub use super::mock_time::{MockTimeSource, TimeSource};
    pub use super::sync::{EventWaiter, TestBarrier, TestLatch};
}
