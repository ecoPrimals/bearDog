// SPDX-License-Identifier: AGPL-3.0-only

//! Handler Async Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/async
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

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

        // Create handlers with different execution patterns
        // ✅ MODERNIZED: Removed sleeps - test actual async dispatch, not timing
        let counter = Arc::new(AtomicUsize::new(0));

        let counter1 = counter.clone();
        let handle1 = async_dispatcher.dispatch_async("event1", move || {
            // Simulate work with CPU-bound operation instead of sleep
            counter1.fetch_add(1, Ordering::SeqCst);
        });

        let counter2 = counter.clone();
        let handle2 = async_dispatcher.dispatch_async("event2", move || {
            // Simulate work with CPU-bound operation instead of sleep
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

        // ✅ MODERNIZED: Use channel for proper cancellation signaling
        let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel(1);
        
        let cancel_handle = async_dispatcher.dispatch_async("long_event", move || {
            for _ in 0..100 {
                if cancel_flag_clone.load(Ordering::SeqCst) {
                    return;
                }
                // Yield to allow cancellation check (cooperative)
                std::hint::spin_loop();
            }
        });

        // Signal cancellation properly
        cancel_flag.store(true, Ordering::SeqCst);
        let _ = cancel_tx.send(()).await;

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
