//! Handler Middleware Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/middleware
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 4: Handler Middleware
    ///
    /// Tests middleware pattern for handlers:
    /// - Pre-processing middleware
    /// - Post-processing middleware
    /// - Middleware ordering
    /// - Middleware short-circuiting
    #[test]
    fn test_handler_middleware() {
        let mut pipeline = HandlerPipeline::new();

        // Track execution order
        let order = Arc::new(Mutex::new(Vec::new()));

        // Pre-processing middleware
        let order1 = order.clone();
        pipeline.add_middleware(Middleware::new(
            move |_ctx: &mut Context| {
                order1.lock().unwrap().push("pre1");
                MiddlewareResult::Continue
            },
            move |_ctx: &mut Context| {
                // Post-processing
                MiddlewareResult::Continue
            },
        ));

        // Second middleware
        let order2 = order.clone();
        pipeline.add_middleware(Middleware::new(
            move |_ctx: &mut Context| {
                order2.lock().unwrap().push("pre2");
                MiddlewareResult::Continue
            },
            move |_ctx: &mut Context| MiddlewareResult::Continue,
        ));

        // Core handler
        let order3 = order.clone();
        pipeline.set_handler(move |_ctx: &mut Context| {
            order3.lock().unwrap().push("handler");
            Ok(())
        });

        // Execute pipeline
        let mut ctx = Context::new();
        pipeline.execute(&mut ctx).unwrap();

        let execution_order = order.lock().unwrap();
        assert_eq!(*execution_order, vec!["pre1", "pre2", "handler"]);

        // Test middleware short-circuit
        let mut short_circuit = HandlerPipeline::new();
        let sc_order = Arc::new(Mutex::new(Vec::new()));

        let sc_order1 = sc_order.clone();
        short_circuit.add_middleware(Middleware::new(
            move |_ctx: &mut Context| {
                sc_order1.lock().unwrap().push("pre1");
                MiddlewareResult::Stop // Short-circuit
            },
            move |_ctx: &mut Context| MiddlewareResult::Continue,
        ));

        let sc_order2 = sc_order.clone();
        short_circuit.add_middleware(Middleware::new(
            move |_ctx: &mut Context| {
                sc_order2.lock().unwrap().push("pre2");
                MiddlewareResult::Continue
            },
            move |_ctx: &mut Context| MiddlewareResult::Continue,
        ));

        let sc_order3 = sc_order.clone();
        short_circuit.set_handler(move |_ctx: &mut Context| {
            sc_order3.lock().unwrap().push("handler");
            Ok(())
        });

        let mut ctx2 = Context::new();
        let result = short_circuit.execute(&mut ctx2);
        assert!(result.is_ok());

        let sc_execution_order = sc_order.lock().unwrap();
        assert_eq!(*sc_execution_order, vec!["pre1"]); // Stopped at first middleware

        // Test middleware with context modification
        let mut ctx_pipeline = HandlerPipeline::new();

        ctx_pipeline.add_middleware(Middleware::new(
            move |ctx: &mut Context| {
                ctx.set("key1", "value1");
                MiddlewareResult::Continue
            },
            move |_ctx: &mut Context| MiddlewareResult::Continue,
        ));

        ctx_pipeline.set_handler(move |ctx: &mut Context| {
            assert_eq!(ctx.get("key1"), Some(&"value1".to_string()));
            ctx.set("key2", "value2");
            Ok(())
        });

        let mut ctx3 = Context::new();
        ctx_pipeline.execute(&mut ctx3).unwrap();

        assert_eq!(ctx3.get("key1"), Some(&"value1".to_string()));
        assert_eq!(ctx3.get("key2"), Some(&"value2".to_string()));
    }
}
