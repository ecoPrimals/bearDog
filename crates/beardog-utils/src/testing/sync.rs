// SPDX-License-Identifier: AGPL-3.0-only

//! Synchronization Primitives for Concurrent Testing
//!
//! Provides event-driven synchronization to replace `sleep()`-based waiting in tests.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLock};

/// Event waiter for async test synchronization
///
/// Allows waiting for an event without sleeping.
///
/// ## Example
/// ```rust
/// use beardog_utils::testing::EventWaiter;
/// use std::time::Duration;
///
/// #[tokio::test]
/// async fn test_async_operation() {
///     let (waiter, trigger) = EventWaiter::<String>::new();
///     
///     tokio::spawn(async move {
///         // Simulate async work
///         trigger.notify("result".to_string()).await;
///     });
///     
///     let result = waiter.wait(Duration::from_secs(1)).await.unwrap();
///     assert_eq!(result, "result");
/// }
/// ```
#[derive(Clone)]
pub struct EventWaiter<T: Clone + Send + Sync> {
    value: Arc<RwLock<Option<T>>>,
    notify: Arc<Notify>,
}

/// Trigger for EventWaiter
pub struct EventTrigger<T: Clone + Send + Sync> {
    value: Arc<RwLock<Option<T>>>,
    notify: Arc<Notify>,
}

impl<T: Clone + Send + Sync> EventWaiter<T> {
    /// Create a new event waiter/trigger pair
    pub fn new() -> (Self, EventTrigger<T>) {
        let value = Arc::new(RwLock::new(None));
        let notify = Arc::new(Notify::new());

        let waiter = Self {
            value: value.clone(),
            notify: notify.clone(),
        };

        let trigger = EventTrigger { value, notify };

        (waiter, trigger)
    }

    /// Wait for the event with a timeout
    ///
    /// Returns the event value or an error if timeout occurs.
    pub async fn wait(&self, timeout: Duration) -> Result<T, EventWaitError> {
        tokio::time::timeout(timeout, async {
            loop {
                // Check if value is available
                {
                    let guard = self.value.read().await;
                    if let Some(ref val) = *guard {
                        return val.clone();
                    }
                }

                // Wait for notification
                self.notify.notified().await;
            }
        })
        .await
        .map_err(|_| EventWaitError::Timeout)
    }

    /// Wait indefinitely for the event
    pub async fn wait_indefinite(&self) -> T {
        loop {
            {
                let guard = self.value.read().await;
                if let Some(ref val) = *guard {
                    return val.clone();
                }
            }

            self.notify.notified().await;
        }
    }
}

impl<T: Clone + Send + Sync> EventTrigger<T> {
    /// Trigger the event with a value
    pub async fn notify(self, value: T) {
        *self.value.write().await = Some(value);
        self.notify.notify_waiters();
    }

    /// Trigger the event synchronously (blocking)
    ///
    /// **Note**: Prefer `notify()` in async contexts. This is only for
    /// synchronous code that needs to trigger async waiters.
    pub fn notify_blocking(self, value: T) {
        // Block on async operation
        let rt = tokio::runtime::Handle::try_current()
            .ok()
            .unwrap_or_else(|| {
                tokio::runtime::Runtime::new()
                    .expect("failed to create runtime")
                    .handle()
                    .clone()
            });

        rt.block_on(async {
            *self.value.write().await = Some(value);
            self.notify.notify_waiters();
        });
    }
}

/// Error type for event waiting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventWaitError {
    Timeout,
}

impl std::fmt::Display for EventWaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "event wait timed out"),
        }
    }
}

impl std::error::Error for EventWaitError {}

/// Test barrier for coordinating multiple concurrent tasks
///
/// All tasks must reach the barrier before any can proceed.
///
/// ## Example
/// ```rust
/// use beardog_utils::testing::TestBarrier;
///
/// #[tokio::test]
/// async fn test_concurrent_operations() {
///     let barrier = TestBarrier::new(3);
///     
///     for i in 0..3 {
///         let barrier = barrier.clone();
///         tokio::spawn(async move {
///             println!("Task {} starting", i);
///             barrier.wait().await;
///             println!("Task {} proceeding", i);
///         });
///     }
///     
///     // Wait for all tasks
///     barrier.wait().await;
/// }
/// ```
#[derive(Clone)]
pub struct TestBarrier {
    count: Arc<AtomicUsize>,
    target: usize,
    notify: Arc<Notify>,
    generation: Arc<AtomicUsize>,
}

impl TestBarrier {
    /// Create a new barrier for `n` tasks
    pub fn new(n: usize) -> Self {
        Self {
            count: Arc::new(AtomicUsize::new(0)),
            target: n,
            notify: Arc::new(Notify::new()),
            generation: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Wait at the barrier
    ///
    /// Blocks until all `n` tasks have called `wait()`.
    pub async fn wait(&self) {
        let gen = self.generation.load(Ordering::SeqCst);
        let count = self.count.fetch_add(1, Ordering::SeqCst) + 1;

        if count >= self.target {
            // Last task to arrive - reset and notify all
            self.count.store(0, Ordering::SeqCst);
            self.generation.fetch_add(1, Ordering::SeqCst);
            self.notify.notify_waiters();
        } else {
            // Wait for notification, but check generation to handle spurious wakeups
            loop {
                self.notify.notified().await;
                if self.generation.load(Ordering::SeqCst) > gen {
                    break;
                }
            }
        }
    }
}

/// Test latch for one-time synchronization
///
/// Similar to a barrier but only triggers once and allows any number of waiters.
///
/// ## Example
/// ```rust
/// use beardog_utils::testing::TestLatch;
///
/// #[tokio::test]
/// async fn test_initialization() {
///     let latch = TestLatch::new();
///     
///     let latch_clone = latch.clone();
///     tokio::spawn(async move {
///         // Simulate initialization work
///         latch_clone.trigger();
///     });
///     
///     // Wait for initialization
///     latch.wait().await;
/// }
/// ```
#[derive(Clone)]
pub struct TestLatch {
    triggered: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl TestLatch {
    /// Create a new latch
    pub fn new() -> Self {
        Self {
            triggered: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Trigger the latch
    ///
    /// All waiting tasks will be released.
    pub fn trigger(&self) {
        self.triggered.store(true, Ordering::SeqCst);
        self.notify.notify_waiters();
    }

    /// Wait for the latch to be triggered
    pub async fn wait(&self) {
        if self.triggered.load(Ordering::SeqCst) {
            return;
        }

        loop {
            self.notify.notified().await;
            if self.triggered.load(Ordering::SeqCst) {
                return;
            }
        }
    }

    /// Check if the latch has been triggered without waiting
    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::SeqCst)
    }
}

impl Default for TestLatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_event_waiter() {
        let (waiter, trigger) = EventWaiter::new();

        tokio::spawn(async move {
            trigger.notify(42).await;
        });

        let result = waiter.wait(Duration::from_secs(1)).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_event_waiter_timeout() {
        let (waiter, _trigger) = EventWaiter::<i32>::new();

        let result = waiter.wait(Duration::from_millis(10)).await;
        assert_eq!(result, Err(EventWaitError::Timeout));
    }

    #[tokio::test]
    async fn test_barrier() {
        let barrier = TestBarrier::new(3);
        let mut handles = vec![];

        for i in 0..3 {
            let barrier = barrier.clone();
            handles.push(tokio::spawn(async move {
                barrier.wait().await;
                i
            }));
        }

        // All tasks should complete
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, i);
        }
    }

    #[tokio::test]
    async fn test_barrier_reuse() {
        let barrier = TestBarrier::new(2);

        // First round
        let b1 = barrier.clone();
        let b2 = barrier.clone();

        let h1 = tokio::spawn(async move { b1.wait().await });
        let h2 = tokio::spawn(async move { b2.wait().await });

        h1.await.unwrap();
        h2.await.unwrap();

        // Second round
        let b1 = barrier.clone();
        let b2 = barrier.clone();

        let h1 = tokio::spawn(async move { b1.wait().await });
        let h2 = tokio::spawn(async move { b2.wait().await });

        h1.await.unwrap();
        h2.await.unwrap();
    }

    #[tokio::test]
    async fn test_latch() {
        let latch = TestLatch::new();

        assert!(!latch.is_triggered());

        let latch_clone = latch.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            latch_clone.trigger();
        });

        latch.wait().await;
        assert!(latch.is_triggered());
    }

    #[tokio::test]
    async fn test_latch_multiple_waiters() {
        let latch = TestLatch::new();
        let mut handles = vec![];

        for _ in 0..10 {
            let latch = latch.clone();
            handles.push(tokio::spawn(async move {
                latch.wait().await;
            }));
        }

        // Trigger after spawning waiters
        latch.trigger();

        // All waiters should complete
        for handle in handles {
            handle.await.unwrap();
        }
    }
}
