// SPDX-License-Identifier: AGPL-3.0-or-later

//! Modern Concurrent Testing Utilities
//!
//! Zero-sleep, zero-serial primitives for robust concurrent testing.
//! Philosophy: Test issues ARE production issues - build it right the first time.

use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Notify, Semaphore, broadcast, oneshot};
use tokio::time::{sleep, timeout};

// ============================================================================
// Completion Signaling
// ============================================================================

/// Spawn a task with explicit completion signal
///
/// Returns a `JoinHandle` and a receiver that signals when the task completes.
/// Use this instead of `sleep()` to wait for background tasks.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::spawn_with_completion;
///
/// #[tokio::test]
/// async fn test_background_task() {
///     let (handle, mut completion) = spawn_with_completion(async {
///         // Do work
///         42
///     });
///     
///     // Wait for actual completion, not arbitrary sleep
///     completion.await.unwrap();
///     let result = handle.await.unwrap();
///     assert_eq!(result, 42);
/// }
/// ```
pub fn spawn_with_completion<F, T>(task: F) -> (tokio::task::JoinHandle<T>, oneshot::Receiver<()>)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        let result = task.await;
        let _ = tx.send(()); // Signal completion
        result
    });
    (handle, rx)
}

/// Spawn multiple tasks with a broadcast completion signal
///
/// All tasks signal the same completion channel.
/// Use this for fan-out scenarios.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::spawn_many_with_completion;
///
/// #[tokio::test]
/// async fn test_concurrent_operations() {
///     let tasks = vec![
///         async { 1 },
///         async { 2 },
///         async { 3 },
///     ];
///     
///     let (handles, mut completion) = spawn_many_with_completion(tasks);
///     
///     // Wait for all to complete
///     for _ in 0..3 {
///         completion.recv().await.unwrap();
///     }
///     
///     // Collect results
///     let results: Vec<_> = futures::future::join_all(handles)
///         .await
///         .into_iter()
///         .map(|r| r.unwrap())
///         .collect();
///     
///     assert_eq!(results, vec![1, 2, 3]);
/// }
/// ```
pub fn spawn_many_with_completion<F, T>(
    tasks: Vec<F>,
) -> (Vec<tokio::task::JoinHandle<T>>, broadcast::Receiver<()>)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = broadcast::channel(tasks.len());

    let handles = tasks
        .into_iter()
        .map(|task| {
            let tx = tx.clone();
            tokio::spawn(async move {
                let result = task.await;
                let _ = tx.send(());
                result
            })
        })
        .collect();

    (handles, rx)
}

// ============================================================================
// Condition Waiting
// ============================================================================

/// Wait for a condition to become true (with timeout)
///
/// Use this instead of `sleep()` when waiting for state changes.
/// Yields between checks to be cooperative.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::wait_for;
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicBool, Ordering};
/// use std::time::Duration;
///
/// #[tokio::test]
/// async fn test_state_change() {
///     let ready = Arc::new(AtomicBool::new(false));
///     let ready_clone = ready.clone();
///     
///     tokio::spawn(async move {
///         tokio::time::sleep(Duration::from_millis(10)).await;
///         ready_clone.store(true, Ordering::SeqCst);
///     });
///     
///     // Wait for actual condition, not arbitrary time
///     wait_for(
///         || ready.load(Ordering::SeqCst),
///         Duration::from_secs(1),
///     ).await.unwrap();
/// }
/// ```
///
/// # Errors
///
/// Returns [`WaitError::Timeout`] if the condition is still false when the deadline elapses.
pub async fn wait_for<F>(condition: F, timeout_duration: Duration) -> Result<(), WaitError>
where
    F: Fn() -> bool + Send,
{
    timeout(timeout_duration, async {
        while !condition() {
            tokio::task::yield_now().await;
            // Small sleep to avoid busy-waiting
            sleep(Duration::from_micros(100)).await;
        }
    })
    .await
    .map_err(|_| WaitError::Timeout(timeout_duration))
}

/// Wait for a condition with a custom check interval
///
/// Use when you need control over the polling frequency.
///
/// # Errors
///
/// Returns [`WaitError::Timeout`] if the condition is still false when the deadline elapses.
pub async fn wait_for_with_interval<F>(
    condition: F,
    timeout_duration: Duration,
    check_interval: Duration,
) -> Result<(), WaitError>
where
    F: Fn() -> bool + Send,
{
    timeout(timeout_duration, async {
        while !condition() {
            sleep(check_interval).await;
        }
    })
    .await
    .map_err(|_| WaitError::Timeout(timeout_duration))
}

/// Errors surfaced by [`wait_for`] and [`with_timeout`].
#[derive(Debug, thiserror::Error)]
pub enum WaitError {
    /// Predicate or future did not complete before the deadline.
    #[error("Timeout after {0:?}")]
    Timeout(Duration),
}

// ============================================================================
// Synchronization Primitives
// ============================================================================

/// Barrier for coordinating multiple tasks
///
/// All tasks wait until N tasks have reached the barrier.
/// Use this instead of `sleep()` for synchronization.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::Barrier;
/// use std::sync::Arc;
///
/// #[tokio::test]
/// async fn test_synchronized_start() {
///     let barrier = Arc::new(Barrier::new(3));
///     let mut handles = vec![];
///     
///     for i in 0..3 {
///         let barrier = barrier.clone();
///         handles.push(tokio::spawn(async move {
///             // All tasks wait here until 3 have arrived
///             barrier.wait().await;
///             // Now all start simultaneously
///             i * 2
///         }));
///     }
///     
///     let results: Vec<_> = futures::future::join_all(handles)
///         .await
///         .into_iter()
///         .map(|r| r.unwrap())
///         .collect();
///     
///     assert_eq!(results, vec![0, 2, 4]);
/// }
/// ```
pub struct Barrier {
    count: Arc<AtomicBool>,
    remaining: Arc<tokio::sync::RwLock<usize>>,
    notify: Arc<Notify>,
    #[expect(
        dead_code,
        reason = "test harness helpers reserved for expanded stress scenarios"
    )]
    total: usize,
}

impl Barrier {
    /// `total` tasks must call [`Self::wait`] before any proceed.
    pub fn new(total: usize) -> Self {
        Self {
            count: Arc::new(AtomicBool::new(false)),
            remaining: Arc::new(tokio::sync::RwLock::new(total)),
            notify: Arc::new(Notify::new()),
            total,
        }
    }

    /// Decrements the arrival count; last arriver wakes everyone else.
    pub async fn wait(&self) {
        {
            let mut remaining = self.remaining.write().await;
            *remaining -= 1;
            if *remaining == 0 {
                self.count.store(true, Ordering::SeqCst);
                self.notify.notify_waiters();
                return;
            }
        }

        // Wait for all to arrive
        loop {
            if self.count.load(Ordering::SeqCst) {
                break;
            }
            self.notify.notified().await;
        }
    }
}

// ============================================================================
// Rate Limiting
// ============================================================================

/// Token bucket rate limiter using semaphore
///
/// Use this instead of `sleep()` for rate limiting in tests.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::RateLimiter;
/// use std::time::Duration;
/// use std::sync::Arc;
///
/// #[tokio::test]
/// async fn test_rate_limited_operations() {
///     let limiter = Arc::new(RateLimiter::new(10, Duration::from_millis(100)));
///     let mut handles = vec![];
///     
///     for _ in 0..100 {
///         let limiter = limiter.clone();
///         handles.push(tokio::spawn(async move {
///             limiter.acquire().await;
///             // Do rate-limited operation
///         }));
///     }
///     
///     futures::future::join_all(handles).await;
/// }
/// ```
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    #[expect(
        dead_code,
        reason = "test harness helpers reserved for expanded stress scenarios"
    )]
    refill_interval: Duration,
    #[expect(
        dead_code,
        reason = "test harness helpers reserved for expanded stress scenarios"
    )]
    permits_per_interval: usize,
}

impl RateLimiter {
    /// Spawns a background task that tops up the [`Semaphore`] every `refill_interval`.
    pub fn new(permits_per_interval: usize, refill_interval: Duration) -> Self {
        let semaphore = Arc::new(Semaphore::new(permits_per_interval));

        // Start background refill task
        let semaphore_clone = semaphore.clone();
        let interval = refill_interval;
        let permits = permits_per_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            interval_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval_timer.tick().await;

                // Refill permits
                let current = semaphore_clone.available_permits();
                if current < permits {
                    semaphore_clone.add_permits(permits - current);
                }
            }
        });

        Self {
            semaphore,
            refill_interval,
            permits_per_interval,
        }
    }

    /// Waits for a permit and intentionally leaks it (test-only burst pattern).
    pub async fn acquire(&self) {
        match self.semaphore.acquire().await {
            Ok(permit) => permit.forget(),
            Err(_) => unreachable!("Semaphore closed"),
        }
    }

    /// Current free permits in the underlying semaphore.
    pub fn available(&self) -> usize {
        self.semaphore.available_permits()
    }
}

// ============================================================================
// Test Utilities
// ============================================================================

/// Assert that a condition becomes true within a timeout
///
/// Preferred over sleep-based assertions.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::assert_eventually;
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicBool, Ordering};
/// use std::time::Duration;
///
/// #[tokio::test]
/// async fn test_eventual_consistency() {
///     let completed = Arc::new(AtomicBool::new(false));
///     let completed_clone = completed.clone();
///     
///     tokio::spawn(async move {
///         // Simulate async work
///         tokio::time::sleep(Duration::from_millis(10)).await;
///         completed_clone.store(true, Ordering::SeqCst);
///     });
///     
///     assert_eventually(
///         || completed.load(Ordering::SeqCst),
///         Duration::from_secs(1),
///         "Task should complete"
///     ).await;
/// }
/// ```
pub async fn assert_eventually<F>(condition: F, timeout_duration: Duration, message: &str)
where
    F: Fn() -> bool + Send,
{
    wait_for(condition, timeout_duration)
        .await
        .unwrap_or_else(|_| panic!("{message} (timeout after {timeout_duration:?})"));
}

/// Run a test with a strict timeout
///
/// Use this to catch tests that hang.
///
/// # Example
/// ```no_run
/// use beardog_utils::testing::concurrent::with_timeout;
/// use std::time::Duration;
///
/// #[tokio::test]
/// async fn test_no_hang() {
///     with_timeout(Duration::from_secs(5), async {
///         // Test code that should complete quickly
///         assert_eq!(1 + 1, 2);
///     }).await.unwrap();
/// }
/// ```
///
/// # Errors
///
/// Returns [`WaitError::Timeout`] if the inner future does not complete before the deadline.
pub async fn with_timeout<F, T>(timeout_duration: Duration, future: F) -> Result<T, WaitError>
where
    F: Future<Output = T>,
{
    timeout(timeout_duration, future)
        .await
        .map_err(|_| WaitError::Timeout(timeout_duration))
}

// ============================================================================
// Tests for the utilities themselves
// ============================================================================

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: testing
    // TEST_PRIORITY: normal

    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[tokio::test]
    async fn test_spawn_with_completion() {
        let (handle, completion) = spawn_with_completion(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        });

        // Wait for completion signal
        completion.await.unwrap();

        // Result should be ready
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_wait_for_success() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            flag_clone.store(true, Ordering::SeqCst);
        });

        wait_for(|| flag.load(Ordering::SeqCst), Duration::from_secs(1))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_wait_for_timeout() {
        let result = wait_for(
            || false, // Never true
            Duration::from_millis(50),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_barrier() {
        let barrier = Arc::new(Barrier::new(3));
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..3 {
            let barrier = barrier.clone();
            let counter = counter.clone();
            handles.push(tokio::spawn(async move {
                // All wait at barrier
                barrier.wait().await;
                // All start simultaneously
                counter.fetch_add(1, Ordering::SeqCst);
            }));
        }

        futures::future::join_all(handles).await;
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = Arc::new(RateLimiter::new(10, Duration::from_millis(100)));
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..50 {
            let limiter = limiter.clone();
            let counter = counter.clone();
            handles.push(tokio::spawn(async move {
                limiter.acquire().await;
                counter.fetch_add(1, Ordering::SeqCst);
            }));
        }

        futures::future::join_all(handles).await;
        assert_eq!(counter.load(Ordering::SeqCst), 50);
    }

    #[tokio::test]
    async fn test_assert_eventually() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            flag_clone.store(true, Ordering::SeqCst);
        });

        assert_eventually(
            || flag.load(Ordering::SeqCst),
            Duration::from_secs(1),
            "Flag should be set",
        )
        .await;
    }

    #[tokio::test]
    async fn test_spawn_many_with_completion() {
        let task = |out: i32| async move {
            tokio::time::sleep(Duration::from_millis(5)).await;
            out
        };
        let (handles, mut completion) = spawn_many_with_completion(vec![task(10), task(20)]);

        completion.recv().await.expect("first done");
        completion.recv().await.expect("second done");

        let sum: i32 = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|h| h.expect("join task"))
            .sum();
        assert_eq!(sum, 30);
    }

    #[tokio::test]
    async fn test_wait_for_with_interval_success() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(30)).await;
            flag_clone.store(true, Ordering::SeqCst);
        });

        wait_for_with_interval(
            || flag.load(Ordering::SeqCst),
            Duration::from_secs(1),
            Duration::from_millis(2),
        )
        .await
        .expect("condition met");
    }

    #[tokio::test]
    async fn test_wait_for_with_interval_timeout() {
        let err = wait_for_with_interval(
            || false,
            Duration::from_millis(40),
            Duration::from_millis(5),
        )
        .await
        .expect_err("should time out");
        assert!(matches!(err, WaitError::Timeout(_)));
    }

    #[tokio::test]
    async fn test_with_timeout_success() {
        let v = with_timeout(Duration::from_secs(1), async { 7usize })
            .await
            .expect("completes");
        assert_eq!(v, 7);
    }

    #[tokio::test]
    async fn test_with_timeout_elapsed() {
        let r = with_timeout(Duration::from_millis(25), async {
            tokio::time::sleep(Duration::from_secs(5)).await;
        })
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    #[should_panic(expected = "never becomes true")]
    async fn assert_eventually_panics_on_timeout() {
        assert_eventually(|| false, Duration::from_millis(80), "never becomes true").await;
    }

    #[tokio::test]
    async fn barrier_single_participant_proceeds_immediately() {
        let barrier = Arc::new(Barrier::new(1));
        barrier.wait().await;
    }

    #[tokio::test]
    async fn rate_limiter_available_reflects_acquire() {
        let limiter = Arc::new(RateLimiter::new(3, Duration::from_millis(200)));
        limiter.acquire().await;
        assert!(limiter.available() <= 2);
    }
}
