//! Concurrent Test Utilities
//!
//! Modern patterns for concurrent testing without sleep-based synchronization.
//!
//! # Principles
//!
//! 1. **No Sleep for Synchronization** - Use channels, atomics, barriers
//! 2. **No Sleep for Ordering** - Use happens-before relationships
//! 3. **Deterministic Tests** - No timing-dependent behavior
//! 4. **Fast Tests** - Complete in milliseconds, not seconds
//!
//! # Examples
//!
//! ```rust
//! use concurrent_utils::*;
//!
//! // Wait for condition
//! let (signal, rx) = completion_signal();
//! tokio::spawn(async move {
//!     do_work().await;
//!     signal.complete();
//! });
//! rx.await.expect("Work completed");
//!
//! // Coordinate tasks
//! let barrier = create_barrier(3);
//! for _ in 0..3 {
//!     let b = barrier.clone();
//!     tokio::spawn(async move {
//!         b.wait().await;
//!         // All start together
//!     });
//! }
//! ```

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{oneshot, watch, Barrier, Notify};

/// Signal for one-time completion events
///
/// Use instead of sleep-based waiting for task completion.
///
/// # Example
/// ```rust
/// let (signal, rx) = completion_signal();
/// tokio::spawn(async move {
///     expensive_operation().await;
///     signal.complete();
/// });
/// tokio::time::timeout(Duration::from_secs(5), rx).await
///     .expect("Should complete within 5s");
/// ```
pub struct CompletionSignal {
    tx: Option<oneshot::Sender<()>>,
}

impl CompletionSignal {
    /// Complete the signal, waking the receiver
    pub fn complete(mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
    }
}

/// Create a completion signal
#[must_use]
pub fn completion_signal() -> (CompletionSignal, oneshot::Receiver<()>) {
    let (tx, rx) = oneshot::channel();
    (CompletionSignal { tx: Some(tx) }, rx)
}

/// Wait for a condition to become true
///
/// Polls the condition efficiently without sleep-based busy waiting.
/// Use for waiting on state changes.
///
/// # Example
/// ```rust
/// let status = Arc::new(AtomicBool::new(false));
/// let status_clone = status.clone();
///
/// tokio::spawn(async move {
///     do_work().await;
///     status_clone.store(true, Ordering::Release);
/// });
///
/// wait_for_condition(
///     || status.load(Ordering::Acquire),
///     Duration::from_secs(5)
/// ).await.expect("Condition met");
/// ```
pub async fn wait_for_condition<F>(mut condition: F, timeout: Duration) -> Result<(), BearDogError>
where
    F: FnMut() -> bool + Send + 'static,
{
    let (tx, mut rx) = watch::channel(false);

    let handle = tokio::spawn(async move {
        while !condition() {
            tokio::task::yield_now().await;
        }
        let _ = tx.send(true);
    });

    let result = tokio::select! {
        _ = rx.changed() => Ok(()),
        () = tokio::time::sleep(timeout) => {
            handle.abort();
            Err(BearDogError::internal("Timeout waiting for condition".to_string()))
        }
    };

    result
}

/// Wait for multiple conditions concurrently
///
/// All conditions must be true before returning.
pub async fn wait_for_all_conditions<F>(
    conditions: Vec<F>,
    timeout: Duration,
) -> Result<(), BearDogError>
where
    F: FnMut() -> bool + Send + 'static,
{
    let mut handles = vec![];
    for condition in conditions {
        handles.push(wait_for_condition(condition, timeout));
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

/// Create a barrier for coordinating multiple tasks
///
/// All tasks wait at the barrier until all have arrived.
///
/// # Example
/// ```rust
/// let barrier = create_barrier(3);
/// for i in 0..3 {
///     let b = barrier.clone();
///     tokio::spawn(async move {
///         // Preparation
///         b.wait().await;
///         // All start together now
///     });
/// }
/// ```
#[must_use]
pub fn create_barrier(n: usize) -> Arc<Barrier> {
    Arc::new(Barrier::new(n))
}

/// Create a notification for one-to-many signaling
///
/// Multiple waiters can be woken by a single notify.
///
/// # Example
/// ```rust
/// let notify = create_notify();
/// let n = notify.clone();
///
/// tokio::spawn(async move {
///     do_work().await;
///     n.notify_waiters();
/// });
///
/// notify.notified().await;
/// ```
#[must_use]
pub fn create_notify() -> Arc<Notify> {
    Arc::new(Notify::new())
}

/// Observable state for test coordination
///
/// Multiple readers can watch for state changes.
///
/// # Example
/// ```rust
/// let (mut state, rx) = observable_state(Status::Starting);
///
/// tokio::spawn(async move {
///     state.update(Status::Running);
/// });
///
/// let mut watch = rx.clone();
/// while *watch.borrow() != Status::Running {
///     watch.changed().await.unwrap();
/// }
/// ```
pub struct ObservableState<T> {
    tx: watch::Sender<T>,
}

impl<T: Clone> ObservableState<T> {
    /// Update the state, notifying all watchers
    pub fn update(&mut self, value: T) {
        let _ = self.tx.send(value);
    }

    /// Get current state
    #[must_use]
    pub fn current(&self) -> T {
        self.tx.borrow().clone()
    }
}

/// Create an observable state
pub fn observable_state<T: Clone>(initial: T) -> (ObservableState<T>, watch::Receiver<T>) {
    let (tx, rx) = watch::channel(initial);
    (ObservableState { tx }, rx)
}

/// Wait for observable state to match predicate
pub async fn wait_for_state<T, F>(
    mut rx: watch::Receiver<T>,
    mut predicate: F,
    timeout: Duration,
) -> Result<T, BearDogError>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let result = tokio::time::timeout(timeout, async move {
        loop {
            let current = rx.borrow_and_update().clone();
            if predicate(&current) {
                return Ok::<T, BearDogError>(current);
            }
            rx.changed()
                .await
                .map_err(|_| BearDogError::internal("State channel closed".to_string()))?;
        }
    })
    .await
    .map_err(|_| BearDogError::internal("Timeout waiting for state".to_string()))?;

    result
}

/// Run multiple async tasks concurrently
///
/// Returns results in order.
pub async fn run_concurrent<F, T>(tasks: Vec<F>) -> Result<Vec<T>, BearDogError>
where
    F: std::future::Future<Output = Result<T, BearDogError>> + Send + 'static,
    T: Send + 'static,
{
    let handles: Vec<_> = tasks.into_iter().map(|task| tokio::spawn(task)).collect();

    let mut results = Vec::new();
    for handle in handles {
        let result = handle
            .await
            .map_err(|e| BearDogError::internal(format!("Task panicked: {e}")))??;
        results.push(result);
    }

    Ok(results)
}

/// Test fixture builder with isolated state
///
/// Creates fresh state per test, preventing cross-test contamination.
#[derive(Debug, Clone)]
pub struct TestFixture<T> {
    state: Arc<T>,
}

impl<T> TestFixture<T> {
    /// Create new isolated test fixture
    pub fn new(state: T) -> Self {
        Self {
            state: Arc::new(state),
        }
    }

    /// Get reference to state
    #[must_use]
    pub fn state(&self) -> &T {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_completion_signal() {
        let (signal, rx) = completion_signal();

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            signal.complete();
        });

        tokio::time::timeout(Duration::from_secs(1), rx)
            .await
            .expect("Should complete")
            .expect("Signal received");
    }

    #[tokio::test]
    async fn test_wait_for_condition() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            flag_clone.store(true, Ordering::Release);
        });

        wait_for_condition(move || flag.load(Ordering::Acquire), Duration::from_secs(1))
            .await
            .expect("Condition met");
    }

    #[tokio::test]
    async fn test_barrier() {
        let barrier = create_barrier(3);
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..3 {
            let b = barrier.clone();
            let c = counter.clone();
            let handle = tokio::spawn(async move {
                // Increment before barrier
                c.fetch_add(1, Ordering::SeqCst);
                // Wait for all
                b.wait().await;
                // All should see count=3 now
                assert_eq!(c.load(Ordering::SeqCst), 3);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Task completed");
        }
    }

    #[tokio::test]
    async fn test_observable_state() {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Status {
            Starting,
            Running,
            #[allow(dead_code)] // Reserved for future graceful shutdown patterns
            Stopped,
        }

        let (mut state, rx) = observable_state(Status::Starting);

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            state.update(Status::Running);
        });

        let result = wait_for_state(rx, |s| matches!(s, Status::Running), Duration::from_secs(1))
            .await
            .expect("State changed");

        assert_eq!(result, Status::Running);
    }

    #[tokio::test]
    async fn test_run_concurrent() {
        let tasks: Vec<_> = (0..5)
            .map(|i| async move {
                tokio::task::yield_now().await;
                Ok::<_, BearDogError>(i * 2)
            })
            .collect();

        let results = run_concurrent(tasks)
            .await
            .expect("All tasks should succeed");

        assert_eq!(results.len(), 5);
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result, &(i * 2));
        }
    }
}
