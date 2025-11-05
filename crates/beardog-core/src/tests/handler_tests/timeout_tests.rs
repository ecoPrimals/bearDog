//! Handler Timeout Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: core/handlers/timeout
//! TEST_PRIORITY: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 7: Handler Timeouts
    ///
    /// Tests handler timeout mechanisms:
    /// - Timeout configuration
    /// - Handler cancellation on timeout
    /// - Timeout error handling
    /// - Partial result handling
    #[test]
    fn test_handler_timeouts() {
        use std::thread;

        let timeout_dispatcher = TimeoutDispatcher::new();

        // Fast handler (should complete)
        let fast_result = timeout_dispatcher.dispatch_with_timeout(
            "fast_event",
            Duration::from_millis(100),
            || {
                thread::sleep(Duration::from_millis(10));
                42
            },
        );

        assert!(fast_result.is_ok());
        assert_eq!(fast_result.unwrap(), 42);

        // Slow handler (should timeout)
        let slow_result = timeout_dispatcher.dispatch_with_timeout(
            "slow_event",
            Duration::from_millis(50),
            || {
                thread::sleep(Duration::from_millis(200));
                99
            },
        );

        assert!(slow_result.is_err());
        let err = slow_result.unwrap_err();
        assert!(err.to_string().contains("timeout") || err.to_string().contains("Timeout"));

        // Test multiple handlers with different timeouts
        let results = Arc::new(Mutex::new(Vec::new()));

        let r1 = results.clone();
        let h1 = timeout_dispatcher.dispatch_async_with_timeout(
            "event1",
            Duration::from_millis(100),
            move || {
                thread::sleep(Duration::from_millis(20));
                r1.lock().unwrap().push(1);
            },
        );

        let r2 = results.clone();
        let h2 = timeout_dispatcher.dispatch_async_with_timeout(
            "event2",
            Duration::from_millis(50),
            move || {
                thread::sleep(Duration::from_millis(150)); // Will timeout
                r2.lock().unwrap().push(2);
            },
        );

        let r3 = results.clone();
        let h3 = timeout_dispatcher.dispatch_async_with_timeout(
            "event3",
            Duration::from_millis(100),
            move || {
                r3.lock().unwrap().push(3);
            },
        );

        // Wait for handlers
        assert!(h1.wait().is_ok());
        assert!(h2.wait().is_err()); // Timed out
        assert!(h3.wait().is_ok());

        let result_vec = results.lock().unwrap();
        assert!(result_vec.contains(&1));
        assert!(!result_vec.contains(&2)); // Didn't complete
        assert!(result_vec.contains(&3));

        // Test timeout with cleanup
        let cleanup_called = Arc::new(AtomicBool::new(false));
        let cleanup_flag = cleanup_called.clone();

        let cleanup_result = timeout_dispatcher.dispatch_with_timeout_and_cleanup(
            "cleanup_event",
            Duration::from_millis(50),
            || {
                thread::sleep(Duration::from_millis(200));
                "result"
            },
            move || {
                cleanup_flag.store(true, Ordering::SeqCst);
            },
        );

        assert!(cleanup_result.is_err());
        thread::sleep(Duration::from_millis(100)); // Allow cleanup to run
        assert!(cleanup_called.load(Ordering::SeqCst));
    }
}
