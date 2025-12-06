//! Handler Concurrency Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/concurrency
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 6: Handler Concurrency
    ///
    /// Tests concurrent handler execution:
    /// - Thread-safe handler invocation
    /// - Concurrent event processing
    /// - Handler synchronization
    /// - Race condition prevention
    #[test]
    fn test_handler_concurrency() {
        use std::thread;

        // Create thread-safe dispatcher
        let dispatcher = Arc::new(Mutex::new(EventDispatcher::new()));

        // Shared counter protected by mutex
        let counter = Arc::new(Mutex::new(0usize));

        // Register handler
        let counter_clone = counter.clone();
        dispatcher
            .lock()
            .unwrap()
            .register("concurrent_event", move |_event: &Event| {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                Ok(())
            });

        // Spawn multiple threads to dispatch events concurrently
        let mut handles = vec![];

        for _ in 0..10 {
            let dispatcher_clone = dispatcher.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let event = Event::new("concurrent_event");
                    dispatcher_clone.lock().unwrap().dispatch(&event).unwrap();
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all events were processed
        assert_eq!(*counter.lock().unwrap(), 1000);

        // Test concurrent handler registration
        let dispatcher2 = Arc::new(Mutex::new(EventDispatcher::new()));
        let registration_counter = Arc::new(AtomicUsize::new(0));

        let mut reg_handles = vec![];

        for i in 0..5 {
            let dispatcher_clone = dispatcher2.clone();
            let counter_clone = registration_counter.clone();

            let handle = thread::spawn(move || {
                let handler_counter = counter_clone.clone();
                dispatcher_clone.lock().unwrap().register(
                    &format!("event_{}", i),
                    move |_event: &Event| {
                        handler_counter.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    },
                );
            });
            reg_handles.push(handle);
        }

        for handle in reg_handles {
            handle.join().unwrap();
        }

        // Dispatch to all registered handlers
        for i in 0..5 {
            let event = Event::new(&format!("event_{}", i));
            dispatcher2.lock().unwrap().dispatch(&event).unwrap();
        }

        assert_eq!(registration_counter.load(Ordering::SeqCst), 5);

        // Test concurrent stateful handler
        let state = Arc::new(Mutex::new(HandlerState::new()));
        let state_clone = state.clone();

        let stateful_handler = Arc::new(Mutex::new(StatefulHandler::new(
            move |_event: &Event, state: &mut HandlerState| state.increment_counter("concurrent"),
            state_clone,
        )));

        let mut state_handles = vec![];

        for _ in 0..10 {
            let handler_clone = stateful_handler.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let event = Event::new("test");
                    handler_clone.lock().unwrap().handle(&event);
                }
            });
            state_handles.push(handle);
        }

        for handle in state_handles {
            handle.join().unwrap();
        }

        assert_eq!(state.lock().unwrap().get_counter("concurrent"), 1000);
    }
}
