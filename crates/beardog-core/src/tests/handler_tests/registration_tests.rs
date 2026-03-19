// SPDX-License-Identifier: AGPL-3.0-only

//! Handler Registration Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/registration
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 1: Event Handler Registration and Dispatch
    ///
    /// Tests event handler lifecycle:
    /// - Handler registration
    /// - Event dispatch to registered handlers
    /// - Handler deregistration
    /// - Multiple handlers for same event
    #[test]
    fn test_event_handler_registration() {
        let mut dispatcher = EventDispatcher::new();

        // Create counter to track handler invocations
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        // Register handler
        let handler_id = dispatcher.register("test_event", move |event: &Event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            assert_eq!(event.name(), "test_event");
            Ok(())
        });

        assert_eq!(dispatcher.handler_count("test_event"), 1);

        // Dispatch event
        let event = Event::new("test_event");
        dispatcher.dispatch(&event).unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Dispatch again
        dispatcher.dispatch(&event).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 2);

        // Register second handler for same event
        let counter2 = Arc::new(AtomicUsize::new(0));
        let counter2_clone = counter2.clone();

        let _handler_id2 = dispatcher.register("test_event", move |_event: &Event| {
            counter2_clone.fetch_add(10, Ordering::SeqCst);
            Ok(())
        });

        assert_eq!(dispatcher.handler_count("test_event"), 2);

        // Dispatch should trigger both handlers
        dispatcher.dispatch(&event).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 3);
        assert_eq!(counter2.load(Ordering::SeqCst), 10);

        // Deregister first handler
        dispatcher.deregister(handler_id);
        assert_eq!(dispatcher.handler_count("test_event"), 1);

        // Dispatch should only trigger second handler
        dispatcher.dispatch(&event).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 3); // No change
        assert_eq!(counter2.load(Ordering::SeqCst), 20); // Increased

        // Test unregistered event
        let unknown_event = Event::new("unknown_event");
        let result = dispatcher.dispatch(&unknown_event);
        assert!(result.is_ok()); // Should succeed even if no handlers
    }
}
