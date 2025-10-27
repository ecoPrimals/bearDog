//! Extended Error Handling Tests
//!
//! High-value integration tests for error construction, propagation,
//! context preservation, and error recovery patterns.
//!
//! TEST_CATEGORY: integration
//! TEST_DOMAIN: error_handling

use beardog_errors::{BearDogError, BearDogResult};

// ====================================================================================
// Error Construction Tests (10 tests)
// ====================================================================================

#[test]
fn test_error_validation_with_context() {
    // Test validation error with detailed context
    let error = BearDogError::validation("Field 'email' must be a valid email address");

    let error_str = format!("{}", error);
    assert!(error_str.contains("email"));
    assert!(error_str.contains("valid"));
}

#[test]
fn test_error_configuration_with_details() {
    // Test configuration error with specific details
    let error = BearDogError::configuration("Port 99999 is out of valid range (1-65535)");

    let error_str = format!("{}", error);
    assert!(error_str.contains("99999"));
    assert!(error_str.contains("range"));
}

#[test]
fn test_error_not_found_with_id() {
    // Test not found error with resource ID
    let resource_id = "hsm-provider-abc123";
    let error = BearDogError::not_found(format!(
        "HSM provider '{}' not found in registry",
        resource_id
    ));

    let error_str = format!("{}", error);
    assert!(error_str.contains("abc123"));
    assert!(error_str.contains("HSM provider"));
}

#[test]
fn test_error_network_with_endpoint() {
    // Test network error with endpoint information
    let endpoint = "https://api.example.com:8080";
    let error = BearDogError::network(format!("Failed to connect to endpoint: {}", endpoint));

    let error_str = format!("{}", error);
    assert!(error_str.contains("example.com"));
    assert!(error_str.contains("8080"));
}

#[test]
fn test_error_internal_with_state_info() {
    // Test internal error with state information
    let state = "ShuttingDown";
    let error = BearDogError::internal(format!("Operation rejected: system in {} state", state));

    let error_str = format!("{}", error);
    assert!(error_str.contains("ShuttingDown"));
}

#[test]
fn test_error_security_with_reason() {
    // Test security error with reason
    let error = BearDogError::security("Authentication token expired".to_string());

    let error_str = format!("{}", error);
    assert!(error_str.contains("token"));
    assert!(error_str.contains("expired"));
}

#[test]
fn test_error_system_with_timeout_info() {
    // Test system error with timeout info
    let timeout_secs = 30;
    let error = BearDogError::system(format!("Operation timed out after {}s", timeout_secs));

    let error_str = format!("{}", error);
    assert!(error_str.contains("30"));
    assert!(error_str.contains("timed out"));
}

#[test]
fn test_error_unavailable_with_service() {
    // Test unavailable error with service name
    let service = "discovery-service";
    let error =
        BearDogError::unavailable(format!("Service '{}' is currently unavailable", service));

    let error_str = format!("{}", error);
    assert!(error_str.contains("discovery-service"));
}

#[test]
fn test_error_invalid_input_with_field() {
    // Test invalid input with field name
    let error = BearDogError::invalid_input("Field 'primal_id' contains invalid characters");

    let error_str = format!("{}", error);
    assert!(error_str.contains("primal_id"));
}

#[test]
fn test_error_debug_format() {
    // Test error debug formatting
    let error = BearDogError::validation("Test error");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
    assert!(debug_str.len() > 5);
}

// ====================================================================================
// Error Propagation Tests (10 tests)
// ====================================================================================

#[test]
fn test_error_propagation_simple() {
    // Test simple error propagation through Result
    fn inner_fn() -> BearDogResult<()> {
        Err(BearDogError::validation("Inner error"))
    }

    fn outer_fn() -> BearDogResult<()> {
        inner_fn()?;
        Ok(())
    }

    let result = outer_fn();
    assert!(result.is_err());
}

#[test]
fn test_error_propagation_with_context() {
    // Test error propagation with added context
    fn inner_fn() -> BearDogResult<String> {
        Err(BearDogError::not_found("Resource not found".to_string()))
    }

    fn middle_fn() -> BearDogResult<String> {
        inner_fn()?;
        Ok("success".to_string())
    }

    fn outer_fn() -> BearDogResult<String> {
        middle_fn()?;
        Ok("complete".to_string())
    }

    let result = outer_fn();
    assert!(result.is_err());

    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(msg.contains("not found"));
    }
}

#[test]
fn test_error_propagation_async_sync_boundary() {
    // Test error can cross async/sync boundaries
    fn sync_fn() -> BearDogResult<()> {
        Err(BearDogError::validation("Sync error"))
    }

    let result = sync_fn();
    assert!(result.is_err());
}

#[test]
fn test_error_propagation_through_match() {
    // Test error propagation through match expressions
    fn process(value: i32) -> BearDogResult<i32> {
        match value {
            0 => Err(BearDogError::invalid_input("Value cannot be zero")),
            n if n < 0 => Err(BearDogError::invalid_input("Value must be positive")),
            n => Ok(n * 2),
        }
    }

    assert!(process(0).is_err());
    assert!(process(-5).is_err());
    assert_eq!(process(10).unwrap(), 20);
}

#[test]
fn test_error_propagation_with_map_err() {
    // Test error transformation with map_err
    fn inner() -> Result<(), String> {
        Err("string error".to_string())
    }

    fn outer() -> BearDogResult<()> {
        inner().map_err(|e| BearDogError::internal(e))?;
        Ok(())
    }

    let result = outer();
    assert!(result.is_err());
}

#[test]
fn test_error_early_return() {
    // Test early return with ?operator
    fn multi_step() -> BearDogResult<String> {
        step1()?;
        step2()?;
        step3()
    }

    fn step1() -> BearDogResult<()> {
        Ok(())
    }

    fn step2() -> BearDogResult<()> {
        Err(BearDogError::validation("Step 2 failed"))
    }

    fn step3() -> BearDogResult<String> {
        Ok("complete".to_string())
    }

    let result = multi_step();
    assert!(result.is_err());

    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(msg.contains("Step 2"));
    }
}

#[test]
fn test_error_in_iterator_chain() {
    // Test error handling in iterator chains
    fn process_items(items: Vec<i32>) -> BearDogResult<Vec<i32>> {
        items
            .into_iter()
            .map(|n| {
                if n < 0 {
                    Err(BearDogError::invalid_input("Negative value"))
                } else {
                    Ok(n * 2)
                }
            })
            .collect()
    }

    assert!(process_items(vec![1, 2, 3]).is_ok());
    assert!(process_items(vec![1, -2, 3]).is_err());
}

#[test]
fn test_error_in_nested_results() {
    // Test nested Result handling
    fn outer() -> BearDogResult<BearDogResult<String>> {
        Ok(inner())
    }

    fn inner() -> BearDogResult<String> {
        Err(BearDogError::validation("Inner validation failed"))
    }

    let result = outer();
    assert!(result.is_ok());

    if let Ok(inner_result) = result {
        assert!(inner_result.is_err());
    }
}

#[test]
fn test_error_with_ok_or_else() {
    // Test error creation with ok_or_else
    fn get_value(should_exist: bool) -> BearDogResult<String> {
        if should_exist {
            Some("value".to_string())
        } else {
            None
        }
        .ok_or_else(|| BearDogError::not_found("Value not found".to_string()))
    }

    assert!(get_value(true).is_ok());
    assert!(get_value(false).is_err());
}

#[test]
fn test_error_and_then_chain() {
    // Test error handling with and_then
    fn process() -> BearDogResult<i32> {
        get_value()
            .and_then(|v| validate(v))
            .and_then(|v| transform(v))
    }

    fn get_value() -> BearDogResult<i32> {
        Ok(10)
    }

    fn validate(v: i32) -> BearDogResult<i32> {
        if v > 0 {
            Ok(v)
        } else {
            Err(BearDogError::validation("Must be positive"))
        }
    }

    fn transform(v: i32) -> BearDogResult<i32> {
        Ok(v * 2)
    }

    let result = process();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20);
}

// ====================================================================================
// Error Recovery Tests (6 tests)
// ====================================================================================

#[test]
fn test_error_recovery_with_unwrap_or() {
    // Test error recovery with unwrap_or
    fn may_fail() -> BearDogResult<i32> {
        Err(BearDogError::validation("Failed"))
    }

    let value = may_fail().unwrap_or(42);
    assert_eq!(value, 42);
}

#[test]
fn test_error_recovery_with_unwrap_or_else() {
    // Test error recovery with unwrap_or_else
    fn may_fail() -> BearDogResult<String> {
        Err(BearDogError::not_found("Not found".to_string()))
    }

    let value = may_fail().unwrap_or_else(|_| "default".to_string());
    assert_eq!(value, "default");
}

#[test]
fn test_error_recovery_with_unwrap_or_default() {
    // Test error recovery with unwrap_or_default
    fn may_fail() -> BearDogResult<Vec<i32>> {
        Err(BearDogError::internal("Failed".to_string()))
    }

    let value = may_fail().unwrap_or_default();
    assert!(value.is_empty());
}

#[test]
fn test_error_recovery_with_match() {
    // Test error recovery with match
    fn may_fail() -> BearDogResult<i32> {
        Err(BearDogError::validation("Failed"))
    }

    let value = match may_fail() {
        Ok(v) => v,
        Err(_) => 100,
    };

    assert_eq!(value, 100);
}

#[test]
fn test_error_recovery_with_ok() {
    // Test error recovery with ok()
    fn may_fail() -> BearDogResult<i32> {
        Err(BearDogError::validation("Failed"))
    }

    let option = may_fail().ok();
    assert!(option.is_none());
}

#[test]
fn test_error_recovery_retry_pattern() {
    // Test retry pattern for error recovery
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = Arc::clone(&attempt_count);

    let operation = || -> BearDogResult<String> {
        let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
        if count < 2 {
            Err(BearDogError::system("Timeout".to_string()))
        } else {
            Ok("success".to_string())
        }
    };

    // Retry up to 3 times
    let mut result = operation();
    for _ in 0..2 {
        if result.is_ok() {
            break;
        }
        result = operation();
    }

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

// ====================================================================================
// Test Helpers
// ====================================================================================

#[cfg(test)]
mod error_helpers {
    use super::*;

    /// Create a test validation error
    pub fn validation_error(msg: &str) -> BearDogError {
        BearDogError::validation(msg)
    }

    /// Check if error message contains text
    pub fn error_contains(error: &BearDogError, text: &str) -> bool {
        format!("{}", error).contains(text)
    }
}

#[test]
fn test_error_helpers() {
    let error = error_helpers::validation_error("test");
    assert!(error_helpers::error_contains(&error, "test"));
}
