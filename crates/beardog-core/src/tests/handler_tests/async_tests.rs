//! Handler Async Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: core/handlers/async
//! TEST_PRIORITY: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 3: Async Handler Execution
    ///
    /// Tests asynchronous handler patterns:
    /// - Non-blocking handler execution
    /// - Handler completion tracking
    /// - Concurrent handler execution
    /// - Handler result collection
    #[test]
    fn test_async_handler_execution() {
        use std::thread;

        let async_dispatcher = AsyncDispatcher::new();

        // Create handlers with different execution times
        let counter = Arc::new(AtomicUsize::new(0));

        let counter1 = counter.clone();
        let handle1 = async_dispatcher.dispatch_async("event1", move || {
            thread::sleep(Duration::from_millis(10));
            counter1.fetch_add(1, Ordering::SeqCst);
        });

        let counter2 = counter.clone();
        let handle2 = async_dispatcher.dispatch_async("event2", move || {
            thread::sleep(Duration::from_millis(5));
            counter2.fetch_add(10, Ordering::SeqCst);
        });

        let counter3 = counter.clone();
        let handle3 = async_dispatcher.dispatch_async("event3", move || {
            counter3.fetch_add(100, Ordering::SeqCst);
        });

        // At this point, handlers are executing
        assert_eq!(async_dispatcher.pending_count(), 3);

        // Wait for all handlers
        handle1.wait().unwrap();
        handle2.wait().unwrap();
        handle3.wait().unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 111);
        assert_eq!(async_dispatcher.pending_count(), 0);

        // Test handler cancellation
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag_clone = cancel_flag.clone();

        let cancel_handle = async_dispatcher.dispatch_async("long_event", move || {
            for _ in 0..100 {
                if cancel_flag_clone.load(Ordering::SeqCst) {
                    return;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        // Cancel the handler
        thread::sleep(Duration::from_millis(50));
        cancel_flag.store(true, Ordering::SeqCst);

        let result = cancel_handle.wait();
        assert!(result.is_ok()); // Handler should complete (with early exit)

        // Test error in async handler
        let error_handle = async_dispatcher.dispatch_async("error_event", move || {
            panic!("Handler error");
        });

        let result = error_handle.wait();
        assert!(result.is_err()); // Should capture panic
    }
}
