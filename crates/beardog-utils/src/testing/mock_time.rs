// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mock Time Source for Deterministic Testing
//!
//! Provides a mockable time source that enables **instant time travel** in tests,
//! eliminating the need for `thread::sleep()` or real time waiting.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};

/// Abstraction over time sources for testability
///
/// This trait allows production code to use real time (`SystemTimeSource`)
/// and test code to use controlled time (`MockTimeSource`).
pub trait TimeSource: Send + Sync + Clone {
    /// Get the current instant
    fn now(&self) -> Instant;

    /// Get the current system time
    fn system_now(&self) -> SystemTime;

    /// Calculate elapsed time from a start instant
    fn elapsed(&self, start: Instant) -> Duration {
        self.now().duration_since(start)
    }
}

/// Production time source using real system time
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemTimeSource;

impl TimeSource for SystemTimeSource {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn system_now(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// Mock time source for testing
///
/// Allows instant "time travel" without waiting for real time to pass.
///
/// ## Example
/// ```rust
/// use beardog_utils::testing::{MockTimeSource, TimeSource};
/// use std::time::Duration;
///
/// let time = MockTimeSource::new();
/// let start = time.now();
///
/// // Instantly advance time by 1 hour!
/// time.advance(Duration::from_secs(3600));
///
/// assert_eq!(time.elapsed(start), Duration::from_secs(3600));
/// ```
#[derive(Debug, Clone)]
pub struct MockTimeSource {
    /// Current mock time in nanoseconds since epoch
    nanos: Arc<AtomicU64>,
    /// Base instant for relative time calculations
    base_instant: Instant,
}

impl Default for MockTimeSource {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTimeSource {
    /// Create a new mock time source starting at the current time
    pub fn new() -> Self {
        Self {
            nanos: Arc::new(AtomicU64::new(0)),
            base_instant: Instant::now(),
        }
    }

    /// Create a mock time source starting at a specific time offset
    pub fn with_offset(offset: Duration) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "mock clock stores sub-second precision in u64"
        )]
        let nanos_u64 = offset.as_nanos() as u64;
        Self {
            nanos: Arc::new(AtomicU64::new(nanos_u64)),
            base_instant: Instant::now(),
        }
    }

    /// Advance time by the given duration
    ///
    /// This is **instant** - no actual time passes!
    ///
    /// ## Example
    /// ```rust
    /// # use beardog_utils::testing::{MockTimeSource, TimeSource};
    /// # use std::time::Duration;
    /// let time = MockTimeSource::new();
    /// let start = time.now();
    ///
    /// time.advance(Duration::from_secs(60));
    ///
    /// assert_eq!(time.elapsed(start), Duration::from_secs(60));
    /// ```
    pub fn advance(&self, duration: Duration) {
        #[expect(clippy::cast_possible_truncation, reason = "mock clock nanos in u64")]
        let delta = duration.as_nanos() as u64;
        self.nanos.fetch_add(delta, Ordering::SeqCst);
    }

    /// Set absolute time (from source creation)
    pub fn set_elapsed(&self, duration: Duration) {
        #[expect(clippy::cast_possible_truncation, reason = "mock clock nanos in u64")]
        let nanos_u64 = duration.as_nanos() as u64;
        self.nanos.store(nanos_u64, Ordering::SeqCst);
    }

    /// Reset time to zero
    pub fn reset(&self) {
        self.nanos.store(0, Ordering::SeqCst);
    }

    /// Get current elapsed time since creation
    pub fn elapsed_since_creation(&self) -> Duration {
        Duration::from_nanos(self.nanos.load(Ordering::SeqCst))
    }
}

impl TimeSource for MockTimeSource {
    fn now(&self) -> Instant {
        let nanos = self.nanos.load(Ordering::SeqCst);
        self.base_instant + Duration::from_nanos(nanos)
    }

    fn system_now(&self) -> SystemTime {
        let nanos = self.nanos.load(Ordering::SeqCst);
        SystemTime::now() + Duration::from_nanos(nanos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_source() {
        let source = SystemTimeSource;
        let now1 = source.now();
        let now2 = source.now();

        assert!(now2 >= now1);
    }

    #[test]
    fn test_mock_time_advance() {
        let time = MockTimeSource::new();
        let start = time.now();

        time.advance(Duration::from_secs(1));
        assert_eq!(time.elapsed(start), Duration::from_secs(1));

        time.advance(Duration::from_secs(59));
        assert_eq!(time.elapsed(start), Duration::from_secs(60));
    }

    #[test]
    fn test_mock_time_set_elapsed() {
        let time = MockTimeSource::new();

        time.set_elapsed(Duration::from_secs(100));
        assert_eq!(time.elapsed_since_creation(), Duration::from_secs(100));

        time.set_elapsed(Duration::from_secs(50));
        assert_eq!(time.elapsed_since_creation(), Duration::from_secs(50));
    }

    #[test]
    fn test_mock_time_reset() {
        let time = MockTimeSource::new();

        time.advance(Duration::from_secs(100));
        time.reset();

        assert_eq!(time.elapsed_since_creation(), Duration::ZERO);
    }

    #[test]
    fn test_mock_time_with_offset() {
        let time = MockTimeSource::with_offset(Duration::from_secs(1000));

        assert_eq!(time.elapsed_since_creation(), Duration::from_secs(1000));

        time.advance(Duration::from_secs(1));
        assert_eq!(time.elapsed_since_creation(), Duration::from_secs(1001));
    }

    #[test]
    fn test_mock_time_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let time = Arc::new(MockTimeSource::new());
        let mut handles = vec![];

        // Multiple threads advancing time concurrently
        for _ in 0..10 {
            let time = time.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    time.advance(Duration::from_millis(1));
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // All advances were recorded (10 threads * 100 advances * 1ms)
        assert_eq!(time.elapsed_since_creation(), Duration::from_millis(1000));
    }
}
