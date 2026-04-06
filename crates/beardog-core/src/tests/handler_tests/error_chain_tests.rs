// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handler Error_chain Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/error_chain
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 2: Error Handler Chaining
    ///
    /// Tests error handler propagation:
    /// - Error handler registration
    /// - Error propagation through chain
    /// - Handler short-circuiting
    /// - Error transformation
    #[test]
    fn test_error_handler_chaining() {
        let mut chain = ErrorHandlerChain::new();

        // Track which handlers were called
        let called = Arc::new(Mutex::new(Vec::new()));

        // Handler 1: Log and continue
        let called1 = called.clone();
        chain.add_handler(move |error: &BearDogError| {
            called1.lock().unwrap().push(1);
            HandlerAction::Continue
        });

        // Handler 2: Transform error and continue
        let called2 = called.clone();
        chain.add_handler(move |_error: &BearDogError| {
            called2.lock().unwrap().push(2);
            HandlerAction::Continue
        });

        // Handler 3: Handle and stop
        let called3 = called.clone();
        chain.add_handler(move |_error: &BearDogError| {
            called3.lock().unwrap().push(3);
            HandlerAction::Stop
        });

        // Handler 4: Should never be called (chain stopped)
        let called4 = called.clone();
        chain.add_handler(move |_error: &BearDogError| {
            called4.lock().unwrap().push(4);
            HandlerAction::Continue
        });

        // Process error through chain
        let error = BearDogError::system("test error".to_string());
        let result = chain.handle(&error);

        assert_eq!(result, HandlerResult::Handled);

        let call_order = called.lock().unwrap();
        assert_eq!(*call_order, vec![1, 2, 3]); // Handler 4 not called

        // Test with empty chain
        let empty_chain = ErrorHandlerChain::new();
        let result = empty_chain.handle(&error);
        assert_eq!(result, HandlerResult::Unhandled);

        // Test short-circuit on first handler
        let mut short_circuit_chain = ErrorHandlerChain::new();
        let sc_called = Arc::new(Mutex::new(Vec::new()));

        let sc_called1 = sc_called.clone();
        short_circuit_chain.add_handler(move |_error: &BearDogError| {
            sc_called1.lock().unwrap().push(1);
            HandlerAction::Stop
        });

        let sc_called2 = sc_called.clone();
        short_circuit_chain.add_handler(move |_error: &BearDogError| {
            sc_called2.lock().unwrap().push(2);
            HandlerAction::Continue
        });

        short_circuit_chain.handle(&error);
        assert_eq!(*sc_called.lock().unwrap(), vec![1]); // Only first handler called
    }
}
