// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Modern Async Test Synchronization Utilities
//!
//! This module provides reusable patterns for test synchronization without sleeps.
//!
//! **Philosophy**: Test issues = Production issues
//! - Use channels, barriers, and watch for synchronization
//! - Never use `sleep` for coordination (only for simulating real delays)
//! - Tests should be deterministic and concurrent-safe
//!
//! `TEST_CATEGORY`: utility
//! `TEST_DOMAIN`: testing
//! `TEST_PRIORITY`: high

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Barrier, Notify, RwLock, watch};
use tokio::time::timeout;

/// Signal for component readiness
///
/// Use this when a component needs to signal it's ready for testing
///
/// # Example
/// ```rust
/// # use tokio::sync::watch;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let (ready_tx, mut ready_rx) = watch::channel(false);
///
/// // Component signals ready
/// tokio::spawn(async move {
///     // Initialize...
///     ready_tx.send(true).unwrap();
/// });
///
/// // Test waits for ready
/// while !*ready_rx.borrow() {
///     ready_rx.changed().await.unwrap();
/// }
/// # });
/// ```
pub type ReadySignal = watch::Receiver<bool>;

/// Create a ready signal pair
#[must_use]
pub fn ready_signal() -> (watch::Sender<bool>, ReadySignal) {
    watch::channel(false)
}

/// Wait for a condition to become true with timeout
///
/// Returns `Ok(())` if condition becomes true, `Err` on timeout
///
/// # Example
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use std::sync::atomic::{AtomicBool, Ordering};
/// # use beardog_errors::BearDogError;
/// # async fn example() -> Result<(), BearDogError> {
/// let ready = Arc::new(AtomicBool::new(false));
///
/// // Some async operation sets ready to true
/// // ...
///
/// // Wait for it
/// wait_for_condition(
///     || ready.load(Ordering::Relaxed),
///     std::time::Duration::from_secs(5)
/// ).await?;
/// # Ok(())
/// # }
/// ```
/// # Errors
///
/// Returns [`BearDogError`](beardog_errors::BearDogError) if the condition is
/// not met within `max_duration`.
pub async fn wait_for_condition<F>(
    mut condition: F,
    max_duration: Duration,
) -> Result<(), beardog_errors::BearDogError>
where
    F: FnMut() -> bool,
{
    timeout(max_duration, async {
        while !condition() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .map_err(|_| {
        beardog_errors::BearDogError::internal("Timeout waiting for condition".to_string())
    })
}

/// Barrier for coordinating multiple concurrent tasks
///
/// All tasks wait until all have reached the barrier
///
/// # Example
/// ```rust
/// # use std::sync::Arc;
/// # use tokio::sync::Barrier;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let barrier = Arc::new(Barrier::new(3));
/// let mut handles = vec![];
///
/// for i in 0..3 {
///     let b = barrier.clone();
///     handles.push(tokio::spawn(async move {
///         // Each task does setup
///         // ...
///         
///         // Wait for all to be ready
///         b.wait().await;
///         
///         // Now all start simultaneously
///     }));
/// }
///
/// for h in handles {
///     h.await.unwrap();
/// }
/// # });
/// ```
pub type TaskBarrier = Arc<Barrier>;

/// Create a barrier for N tasks
#[must_use]
pub fn task_barrier(n: usize) -> TaskBarrier {
    Arc::new(Barrier::new(n))
}

/// State change notification using watch channel
///
/// Use this when a component needs to notify about state changes
///
/// # Example
/// ```rust
/// # use tokio::sync::watch;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// #[derive(Clone, PartialEq)]
/// enum State { Init, Running, Stopped }
///
/// let (state_tx, mut state_rx) = watch::channel(State::Init);
///
/// // Component updates state
/// tokio::spawn(async move {
///     state_tx.send(State::Running).unwrap();
/// });
///
/// // Test waits for specific state
/// while *state_rx.borrow() != State::Running {
///     state_rx.changed().await.unwrap();
/// }
/// # });
/// ```
pub type StateNotifier<T> = (watch::Sender<T>, watch::Receiver<T>);

/// Create a state notifier
pub fn state_notifier<T: Clone>(initial: T) -> StateNotifier<T> {
    watch::channel(initial)
}

/// One-shot notification using Notify
///
/// Use for single event notifications
///
/// # Example
/// ```rust
/// # use std::sync::Arc;
/// # use tokio::sync::Notify;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let notify = Arc::new(Notify::new());
///
/// let n = notify.clone();
/// tokio::spawn(async move {
///     // Do work...
///     n.notify_one();
/// });
///
/// // Wait for notification
/// notify.notified().await;
/// # });
/// ```
pub type EventNotify = Arc<Notify>;

/// Create an event notifier
#[must_use]
pub fn event_notify() -> EventNotify {
    Arc::new(Notify::new())
}

/// Shared state wrapper for concurrent testing
///
/// Provides async-safe access to shared state
///
/// # Example
/// ```rust
/// # use std::sync::Arc;
/// # use tokio::sync::RwLock;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let state = Arc::new(RwLock::new(vec![0; 10]));
///
/// let mut handles = vec![];
/// for i in 0..5 {
///     let s = state.clone();
///     handles.push(tokio::spawn(async move {
///         s.write().await.push(i);
///     }));
/// }
///
/// for h in handles {
///     h.await.unwrap();
/// }
///
/// assert_eq!(state.read().await.len(), 15);
/// # });
/// ```
pub type SharedState<T> = Arc<RwLock<T>>;

/// Create shared state
pub fn shared_state<T>(value: T) -> SharedState<T> {
    Arc::new(RwLock::new(value))
}

/// Assert that a condition becomes true within a timeout
///
/// # Example
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use std::sync::atomic::{AtomicUsize, Ordering};
/// # async fn example() {
/// let count = Arc::new(AtomicUsize::new(0));
///
/// // Some async work increments count
/// // ...
///
/// // Assert it reaches 10 within 1 second
/// assert_eventually!(
///     count.load(Ordering::Relaxed) >= 10,
///     std::time::Duration::from_secs(1),
///     "Count should reach 10"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! assert_eventually {
    ($condition:expr, $timeout:expr, $msg:expr) => {{
        use std::time::Duration;
        use tokio::time::timeout;

        let result = timeout($timeout, async {
            while !$condition {
                tokio::task::yield_now().await;
            }
        })
        .await;

        assert!(result.is_ok(), "{}", $msg);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_ready_signal() {
        let (tx, mut rx) = ready_signal();
        assert!(!*rx.borrow());

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            tx.send(true).unwrap();
        });

        while !*rx.borrow() {
            rx.changed().await.unwrap();
        }

        assert!(*rx.borrow());
    }

    #[tokio::test]
    async fn test_wait_for_condition() {
        let ready = Arc::new(AtomicBool::new(false));
        let r = ready.clone();

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            r.store(true, Ordering::Relaxed);
        });

        wait_for_condition(|| ready.load(Ordering::Relaxed), Duration::from_secs(1))
            .await
            .unwrap();

        assert!(ready.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_task_barrier() {
        let barrier = task_barrier(3);
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..3 {
            let b = barrier.clone();
            let c = counter.clone();
            handles.push(tokio::spawn(async move {
                // All wait at barrier
                b.wait().await;
                // All increment simultaneously
                c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(counter.load(Ordering::Relaxed), 3);
    }

    #[tokio::test]
    async fn test_state_notifier() {
        #[derive(Clone, PartialEq, Debug)]
        enum State {
            Init,
            Running,
            Stopped,
        }

        let (tx, mut rx) = state_notifier(State::Init);

        tokio::spawn(async move {
            tokio::task::yield_now().await;
            tx.send(State::Running).unwrap();
            tokio::task::yield_now().await;
            tx.send(State::Stopped).unwrap();
        });

        // Wait for Running
        while *rx.borrow() != State::Running {
            rx.changed().await.unwrap();
        }
        assert_eq!(*rx.borrow(), State::Running);

        // Wait for Stopped
        while *rx.borrow() != State::Stopped {
            rx.changed().await.unwrap();
        }
        assert_eq!(*rx.borrow(), State::Stopped);
    }

    #[tokio::test]
    async fn test_event_notify() {
        let notify = event_notify();
        let done = Arc::new(AtomicBool::new(false));

        let n = notify.clone();
        let d = done.clone();
        tokio::spawn(async move {
            tokio::task::yield_now().await;
            d.store(true, Ordering::Relaxed);
            n.notify_one();
        });

        notify.notified().await;
        assert!(done.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_shared_state() {
        let state = shared_state(Vec::<usize>::new());
        let mut handles = vec![];

        for i in 0..10 {
            let s = state.clone();
            handles.push(tokio::spawn(async move {
                s.write().await.push(i);
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let final_state = state.read().await;
        assert_eq!(final_state.len(), 10);
    }
}
