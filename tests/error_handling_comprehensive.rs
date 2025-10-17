// BearDog Error Handling Comprehensive Tests
// Tests error construction, propagation, and handling patterns

use beardog_errors::{BearDogError, BearDogResult};

#[test]
fn test_error_validation() {
    // Test validation error creation
    let error = BearDogError::validation("Field cannot be empty");

    let error_str = format!("{}", error);
    assert!(error_str.contains("Field cannot be empty"));
}

#[test]
fn test_error_configuration() {
    // Test configuration error creation
    let error = BearDogError::configuration("Invalid port number");

    let error_str = format!("{}", error);
    assert!(error_str.contains("Invalid port number"));
}

#[test]
fn test_error_not_found() {
    // Test not found error creation
    let error = BearDogError::not_found("Resource 'test-id-123' not found".to_string());

    let error_str = format!("{}", error);
    assert!(error_str.contains("test-id-123"));
}

#[test]
fn test_error_network() {
    // Test network error creation
    let error = BearDogError::network("Connection timeout".to_string());

    let error_str = format!("{}", error);
    assert!(error_str.contains("Connection timeout"));
}

#[test]
fn test_error_internal() {
    // Test internal error creation
    let error = BearDogError::internal("Unexpected state".to_string());

    let error_str = format!("{}", error);
    assert!(error_str.contains("Unexpected state"));
}

#[test]
fn test_error_invalid_input() {
    // Test invalid input error creation
    let error = BearDogError::invalid_input("Input must be a positive number");

    let error_str = format!("{}", error);
    assert!(error_str.contains("Input must be a positive number"));
}

#[test]
fn test_error_chain_validation() {
    // Test error chain with validation
    fn validate_input(input: &str) -> BearDogResult<()> {
        if input.is_empty() {
            return Err(BearDogError::validation("Input cannot be empty"));
        }
        Ok(())
    }

    let result = validate_input("");
    assert!(result.is_err());

    let result = validate_input("valid");
    assert!(result.is_ok());
}

#[test]
fn test_error_chain_not_found() {
    // Test error chain with not found
    fn find_resource(id: &str) -> BearDogResult<String> {
        if id == "missing" {
            return Err(BearDogError::not_found(format!(
                "Resource '{}' not found",
                id
            )));
        }
        Ok(format!("Resource: {}", id))
    }

    let result = find_resource("missing");
    assert!(result.is_err());

    let result = find_resource("exists");
    assert!(result.is_ok());
}

#[test]
fn test_error_propagation() {
    // Test error propagation through function chain
    fn level_3() -> BearDogResult<()> {
        Err(BearDogError::internal("Deep error".to_string()))
    }

    fn level_2() -> BearDogResult<()> {
        level_3()?;
        Ok(())
    }

    fn level_1() -> BearDogResult<()> {
        level_2()?;
        Ok(())
    }

    let result = level_1();
    assert!(result.is_err());
}

#[test]
fn test_error_recovery() {
    // Test error recovery patterns
    fn risky_operation(should_fail: bool) -> BearDogResult<i32> {
        if should_fail {
            Err(BearDogError::internal("Operation failed".to_string()))
        } else {
            Ok(42)
        }
    }

    // Test failure path
    let result = risky_operation(true);
    assert!(result.is_err());

    // Test recovery with default
    let value = risky_operation(true).unwrap_or(0);
    assert_eq!(value, 0);

    // Test success path
    let result = risky_operation(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_error_map() {
    // Test error mapping and transformation
    fn operation() -> BearDogResult<i32> {
        Err(BearDogError::internal("Failed".to_string()))
    }

    let result = operation();
    assert!(result.is_err());

    // Map error to default value
    let value = result.unwrap_or(100);
    assert_eq!(value, 100);
}

#[test]
fn test_error_or_else() {
    // Test error recovery with or_else
    fn primary_source() -> BearDogResult<String> {
        Err(BearDogError::not_found(
            "Primary source not found".to_string(),
        ))
    }

    fn fallback_source() -> BearDogResult<String> {
        Ok("Fallback data".to_string())
    }

    let result = primary_source().or_else(|_| fallback_source());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Fallback data");
}

#[test]
fn test_error_context_preservation() {
    // Test that error context is preserved
    let error = BearDogError::validation("Invalid email format for field 'email'");
    let error_string = format!("{}", error);

    assert!(error_string.contains("email"));
    assert!(error_string.contains("Invalid email format"));
}

#[test]
fn test_multiple_validation_errors() {
    // Test handling multiple validation errors
    fn validate_user_input(name: &str, email: &str, age: i32) -> BearDogResult<()> {
        if name.is_empty() {
            return Err(BearDogError::validation("Name is required"));
        }
        if !email.contains('@') {
            return Err(BearDogError::validation("Invalid email format"));
        }
        if !(0..=150).contains(&age) {
            return Err(BearDogError::validation("Age must be between 0 and 150"));
        }
        Ok(())
    }

    // Test each validation
    assert!(validate_user_input("", "test@example.com", 25).is_err());
    assert!(validate_user_input("John", "invalid", 25).is_err());
    assert!(validate_user_input("John", "test@example.com", -1).is_err());
    assert!(validate_user_input("John", "test@example.com", 200).is_err());

    // Test valid input
    assert!(validate_user_input("John", "test@example.com", 25).is_ok());
}

#[test]
fn test_error_debug_format() {
    // Test debug formatting of errors
    let error = BearDogError::internal("Test error".to_string());
    let debug_str = format!("{:?}", error);

    // Debug format should contain information
    assert!(!debug_str.is_empty());
}

#[test]
fn test_result_type_basic() {
    // Test BearDogResult type works correctly
    fn returns_result(succeed: bool) -> BearDogResult<String> {
        if succeed {
            Ok("Success".to_string())
        } else {
            Err(BearDogError::internal("Failed".to_string()))
        }
    }

    let success = returns_result(true);
    assert!(success.is_ok());
    assert_eq!(success.unwrap(), "Success");

    let failure = returns_result(false);
    assert!(failure.is_err());
}

#[test]
fn test_error_construction_variants() {
    // Test various error construction patterns
    let _err1 = BearDogError::validation("Validation message");
    let _err2 = BearDogError::not_found("Item not found".to_string());
    let _err3 = BearDogError::invalid_input("Invalid input");
    let _err4 = BearDogError::configuration("Config error");
    let _err5 = BearDogError::network("Network error".to_string());
    let _err6 = BearDogError::internal("Internal error".to_string());

    // All constructions should succeed without panicking
}

#[test]
fn test_error_chain_complex() {
    // Test complex error chain
    fn step_1() -> BearDogResult<i32> {
        Ok(1)
    }

    fn step_2(val: i32) -> BearDogResult<i32> {
        if val > 0 {
            Ok(val * 2)
        } else {
            Err(BearDogError::validation("Value must be positive"))
        }
    }

    fn step_3(val: i32) -> BearDogResult<i32> {
        if val < 100 {
            Ok(val + 10)
        } else {
            Err(BearDogError::validation("Value must be less than 100"))
        }
    }

    fn full_chain() -> BearDogResult<i32> {
        let v1 = step_1()?;
        let v2 = step_2(v1)?;
        let v3 = step_3(v2)?;
        Ok(v3)
    }

    let result = full_chain();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12); // (1 * 2) + 10
}

#[test]
fn test_option_to_result_conversion() {
    // Test converting Option to Result with error
    fn find_item(id: u32) -> Option<String> {
        if id == 42 {
            Some("Item 42".to_string())
        } else {
            None
        }
    }

    fn get_item(id: u32) -> BearDogResult<String> {
        find_item(id).ok_or_else(|| BearDogError::not_found(format!("Item '{}' not found", id)))
    }

    let result = get_item(42);
    assert!(result.is_ok());

    let result = get_item(99);
    assert!(result.is_err());
}

#[test]
fn test_error_early_return() {
    // Test early return pattern with ?
    fn process_data(data: &str) -> BearDogResult<usize> {
        if data.is_empty() {
            return Err(BearDogError::validation("Data cannot be empty"));
        }

        if data.len() > 1000 {
            return Err(BearDogError::validation("Data too large (max 1000 chars)"));
        }

        Ok(data.len())
    }

    assert!(process_data("").is_err());
    assert!(process_data(&"x".repeat(1001)).is_err());
    assert!(process_data("valid").is_ok());
}

#[test]
fn test_error_in_iterator() {
    // Test error handling in iterator chains
    fn parse_numbers(strings: &[&str]) -> BearDogResult<Vec<i32>> {
        let mut numbers = Vec::new();
        for s in strings {
            let num = s
                .parse::<i32>()
                .map_err(|_| BearDogError::validation("Invalid number format"))?;
            numbers.push(num);
        }
        Ok(numbers)
    }

    let valid = ["1", "2", "3"];
    assert!(parse_numbers(&valid).is_ok());

    let invalid = ["1", "not-a-number", "3"];
    assert!(parse_numbers(&invalid).is_err());
}
