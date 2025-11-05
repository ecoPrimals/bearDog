//! Handler State Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: core/handlers/state
//! TEST_PRIORITY: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 5: Handler State Management
    ///
    /// Tests stateful handlers:
    /// - Handler state initialization
    /// - State updates across invocations
    /// - State isolation between handlers
    /// - State persistence
    #[test]
    fn test_handler_state_management() {
        // Create stateful handler
        let state = Arc::new(Mutex::new(HandlerState::new()));

        let state1 = state.clone();
        let mut handler = StatefulHandler::new(
            move |event: &Event, state: &mut HandlerState| {
                let count = state.increment_counter("events");
                state.set_last_event(event.name());
                count
            },
            state1,
        );

        // Invoke handler multiple times
        let event1 = Event::new("event1");
        let count1 = handler.handle(&event1);
        assert_eq!(count1, 1);

        let event2 = Event::new("event2");
        let count2 = handler.handle(&event2);
        assert_eq!(count2, 2);

        let event3 = Event::new("event3");
        let count3 = handler.handle(&event3);
        assert_eq!(count3, 3);

        // Verify state
        let state_guard = state.lock().unwrap();
        assert_eq!(state_guard.get_counter("events"), 3);
        assert_eq!(state_guard.last_event(), "event3");
        drop(state_guard);

        // Test state isolation
        let state2 = Arc::new(Mutex::new(HandlerState::new()));
        let state2_clone = state2.clone();

        let mut handler2 = StatefulHandler::new(
            move |_event: &Event, state: &mut HandlerState| {
                state.increment_counter("handler2_events")
            },
            state2_clone,
        );

        let count = handler2.handle(&event1);
        assert_eq!(count, 1);

        // First handler's state should be unchanged
        let state1_guard = state.lock().unwrap();
        assert_eq!(state1_guard.get_counter("events"), 3);
        assert_eq!(state1_guard.get_counter("handler2_events"), 0);
        drop(state1_guard);

        // Test state reset
        let state3 = Arc::new(Mutex::new(HandlerState::new()));
        let state3_clone = state3.clone();

        let mut handler3 = StatefulHandler::new(
            move |_event: &Event, state: &mut HandlerState| state.increment_counter("test"),
            state3_clone,
        );

        handler3.handle(&event1);
        handler3.handle(&event1);
        handler3.handle(&event1);

        assert_eq!(state3.lock().unwrap().get_counter("test"), 3);

        state3.lock().unwrap().reset();
        assert_eq!(state3.lock().unwrap().get_counter("test"), 0);

        // Test state with multiple counters
        let state4 = Arc::new(Mutex::new(HandlerState::new()));
        let state4_clone = state4.clone();

        let mut multi_handler = StatefulHandler::new(
            move |event: &Event, state: &mut HandlerState| state.increment_counter(event.name()),
            state4_clone,
        );

        multi_handler.handle(&Event::new("type_a"));
        multi_handler.handle(&Event::new("type_a"));
        multi_handler.handle(&Event::new("type_b"));

        let state4_guard = state4.lock().unwrap();
        assert_eq!(state4_guard.get_counter("type_a"), 2);
        assert_eq!(state4_guard.get_counter("type_b"), 1);
    }
}
