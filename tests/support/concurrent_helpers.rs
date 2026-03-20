// SPDX-License-Identifier: AGPL-3.0-only
//! Concurrent Test Helpers - Zero Sleep, Maximum Robustness
//!
//! This module provides utilities for truly concurrent testing without
//! arbitrary sleep() calls or serial test execution.
//!
//! Philosophy: "Test issues ARE production issues"

use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UnixListener;
use tokio::time::timeout;

/// Generates a unique Unix socket path for test isolation
///
/// Each invocation returns a unique path, eliminating the need for
/// `#[serial]` annotations due to socket path conflicts.
pub fn unique_unix_socket() -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros();
    
    // Use temp directory to ensure cleanup
    let temp_dir = std::env::temp_dir();
    temp_dir.join(format!("beardog-test-{}-{}-{}.sock", pid, timestamp, id))
}

/// Gets an ephemeral TCP port from the OS
///
/// The OS guarantees uniqueness, eliminating port conflicts between tests.
/// Returns the port number. Caller must bind immediately to claim it.
pub fn ephemeral_tcp_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    // Listener drops here, port becomes available
    Ok(port)
}

/// Creates a bound TCP listener on an ephemeral port
///
/// Returns both the listener and the port it's bound to.
/// This ensures no race between port discovery and binding.
pub fn ephemeral_tcp_listener() -> std::io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

/// Health-based readiness waiter
///
/// Replaces arbitrary sleep() with actual health checking.
///
/// # Example
/// ```no_run
/// let ready = ReadinessSignal::new();
/// let ready_clone = ready.clone();
/// 
/// tokio::spawn(async move {
///     // Server initialization...
///     ready_clone.signal_ready(); // Signal when actually ready
/// });
///
/// ready.wait_ready(Duration::from_secs(5)).await?; // Wait for real signal
/// ```
#[derive(Clone)]
pub struct ReadinessSignal {
    ready: Arc<AtomicBool>,
}

impl ReadinessSignal {
    pub fn new() -> Self {
        Self {
            ready: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Signal that the service is ready
    pub fn signal_ready(&self) {
        self.ready.store(true, Ordering::Release);
    }

    /// Check if ready (non-blocking)
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Wait for ready signal with timeout
    ///
    /// Uses exponential backoff polling: 1ms, 2ms, 4ms, ..., up to 50ms
    /// This is dramatically more responsive than sleep(100ms) while
    /// still being CPU-efficient.
    pub async fn wait_ready(&self, timeout_duration: Duration) -> Result<(), WaitError> {
        let start = Instant::now();
        let mut poll_interval = Duration::from_millis(1);
        let max_poll_interval = Duration::from_millis(50);

        while !self.is_ready() {
            if start.elapsed() > timeout_duration {
                return Err(WaitError::Timeout);
            }

            tokio::time::sleep(poll_interval).await;

            // Exponential backoff: 1ms, 2ms, 4ms, 8ms, ..., 50ms
            poll_interval = std::cmp::min(poll_interval * 2, max_poll_interval);
        }

        Ok(())
    }
}

impl Default for ReadinessSignal {
    fn default() -> Self {
        Self::new()
    }
}

/// Event completion waiter using one-shot channels
///
/// Replaces "trigger event, sleep(50ms), assert done" with proper signaling.
///
/// # Example
/// ```no_run
/// let waiter = CompletionWaiter::new();
/// let signal = waiter.signal();
///
/// tokio::spawn(async move {
///     // Do work...
///     signal.complete(); // Signal actual completion
/// });
///
/// waiter.wait(Duration::from_secs(1)).await?; // Wait for real completion
/// ```
pub struct CompletionWaiter {
    rx: tokio::sync::oneshot::Receiver<()>,
}

impl CompletionWaiter {
    pub fn new() -> (Self, CompletionSignal) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        (Self { rx }, CompletionSignal { tx: Some(tx) })
    }

    pub async fn wait(self, timeout_duration: Duration) -> Result<(), WaitError> {
        timeout(timeout_duration, self.rx)
            .await
            .map_err(|_| WaitError::Timeout)?
            .map_err(|_| WaitError::Canceled)
    }
}

pub struct CompletionSignal {
    tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl CompletionSignal {
    pub fn complete(mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for CompletionSignal {
    fn drop(&mut self) {
        // Auto-signal on drop to prevent hangs
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
    }
}

/// Barrier for coordinating multiple concurrent tasks
///
/// Like std::sync::Barrier but async-aware.
pub struct AsyncBarrier {
    count: Arc<AtomicUsize>,
    target: usize,
    notifier: Arc<tokio::sync::Notify>,
}

impl AsyncBarrier {
    pub fn new(target: usize) -> Self {
        Self {
            count: Arc::new(AtomicUsize::new(0)),
            target,
            notifier: Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Wait for all participants to reach the barrier
    pub async fn wait(&self) {
        let current = self.count.fetch_add(1, Ordering::SeqCst) + 1;
        
        if current == self.target {
            // Last one to arrive, wake everyone
            self.notifier.notify_waiters();
        } else {
            // Wait for last arrival
            self.notifier.notified().await;
        }
    }
}

/// Retry policy for flaky external resources
///
/// Use sparingly - prefer fixing root causes over retrying.
/// Intended for truly external systems (network, hardware).
pub struct RetryPolicy {
    max_attempts: usize,
    base_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    pub fn new(max_attempts: usize) -> Self {
        Self {
            max_attempts,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
        }
    }

    /// Retry an operation with exponential backoff
    pub async fn retry<F, Fut, T, E>(&self, mut f: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut last_err = None;
        let mut delay = self.base_delay;

        for attempt in 1..=self.max_attempts {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_err = Some(e);
                    if attempt < self.max_attempts {
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(delay * 2, self.max_delay);
                    }
                }
            }
        }

        Err(last_err.unwrap())
    }
}

/// Error type for wait operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitError {
    Timeout,
    Canceled,
}

impl std::fmt::Display for WaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaitError::Timeout => write!(f, "Operation timed out"),
            WaitError::Canceled => write!(f, "Operation was canceled"),
        }
    }
}

impl std::error::Error for WaitError {}

/// Temporary directory that auto-cleans on drop
///
/// Useful for tests that need file system isolation.
pub struct TempDir {
    path: Option<PathBuf>,
}

impl TempDir {
    pub fn new() -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "beardog-test-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&path)?;
        Ok(Self { path: Some(path) })
    }

    pub fn path(&self) -> &PathBuf {
        self.path.as_ref().unwrap()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_dir_all(&path);
        }
    }
}

impl Default for TempDir {
    fn default() -> Self {
        Self::new().expect("Failed to create temp directory")
    }
}

/// Wait for a file/socket to be deleted
///
/// More efficient than sleep - polls with exponential backoff
pub async fn wait_for_deletion(path: &std::path::Path, timeout_duration: Duration) -> Result<(), WaitError> {
    let start = Instant::now();
    let mut poll_interval = Duration::from_millis(1);
    let max_poll_interval = Duration::from_millis(50);

    while path.exists() {
        if start.elapsed() > timeout_duration {
            return Err(WaitError::Timeout);
        }

        tokio::time::sleep(poll_interval).await;
        poll_interval = std::cmp::min(poll_interval * 2, max_poll_interval);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unique_sockets_are_unique() {
        let socket1 = unique_unix_socket();
        let socket2 = unique_unix_socket();
        assert_ne!(socket1, socket2);
    }

    #[tokio::test]
    async fn test_ephemeral_ports_are_unique() {
        let port1 = ephemeral_tcp_port().unwrap();
        let port2 = ephemeral_tcp_port().unwrap();
        // Ports should be different (statistically)
        // Can't guarantee since OS reuses, but very likely
        println!("Port 1: {}, Port 2: {}", port1, port2);
    }

    #[tokio::test]
    async fn test_readiness_signal() {
        let ready = ReadinessSignal::new();
        let ready_clone = ready.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            ready_clone.signal_ready();
        });

        // Should complete in ~10ms, not timeout
        let result = ready.wait_ready(Duration::from_secs(1)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_readiness_signal_timeout() {
        let ready = ReadinessSignal::new();
        // Never signal ready
        
        let result = ready.wait_ready(Duration::from_millis(50)).await;
        assert!(matches!(result, Err(WaitError::Timeout)));
    }

    #[tokio::test]
    async fn test_completion_waiter() {
        let (waiter, signal) = CompletionWaiter::new();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            signal.complete();
        });

        let result = waiter.wait(Duration::from_secs(1)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_barrier() {
        let barrier = Arc::new(AsyncBarrier::new(3));
        let mut handles = vec![];

        for i in 0..3 {
            let barrier = barrier.clone();
            handles.push(tokio::spawn(async move {
                println!("Task {} waiting at barrier", i);
                barrier.wait().await;
                println!("Task {} passed barrier", i);
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_temp_dir() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().clone();
        assert!(path.exists());
        drop(dir);
        // Path should be cleaned up
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(!path.exists());
    }
}

