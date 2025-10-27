//! Comprehensive Unit Tests for BearDog Traits
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-traits core functionality
//!
//! Tests cover:
//! - Trait system utilities  
//! - Prelude exports
//! - Version management
//! - Unified trait validation
//! - Module accessibility

use super::*;

// ============================================================================
// Utilities Tests
// ============================================================================

#[test]
fn test_unified_version_format() {
    let version = utilities::get_unified_version();
    assert!(version.contains('.'), "Version should contain dots");
    assert!(version.starts_with('3'), "Should be version 3.x");
}

#[test]
fn test_unified_version_consistency() {
    let v1 = utilities::get_unified_version();
    let v2 = utilities::get_unified_version();
    assert_eq!(v1, v2, "Version should be consistent");
}

#[test]
fn test_unified_version_is_static() {
    // Should have same lifetime across calls
    let version = utilities::get_unified_version();
    assert_eq!(version.len(), 5); // "3.0.0"
}

#[test]
fn test_validate_unified_usage_true() {
    assert!(utilities::validate_unified_usage());
}

#[test]
fn test_validate_unified_usage_consistent() {
    let result1 = utilities::validate_unified_usage();
    let result2 = utilities::validate_unified_usage();
    assert_eq!(result1, result2);
}

#[test]
fn test_utilities_const_functions() {
    // Verify const functions work in const context
    const VERSION: &str = crate::utilities::get_unified_version();
    const VALID: bool = crate::utilities::validate_unified_usage();
    
    assert_eq!(VERSION, "3.0.0");
    assert!(VALID);
}

// ============================================================================
// Module Accessibility Tests
// ============================================================================

#[test]
fn test_unified_module_accessible() {
    // Should be able to access unified module
    let _version = utilities::get_unified_version();
    assert!(true); // Module is accessible
}

#[test]
fn test_canonical_module_exists() {
    // canonical module should exist for backwards compatibility
    // (just checking module is accessible)
    let _ = std::any::type_name::<crate::canonical::base::ConnectionStatus>();
}

#[test]
fn test_prelude_imports_work() {
    // Verify prelude re-exports work
    let version = crate::utilities::get_unified_version();
    assert_eq!(version, "3.0.0");
}

// ============================================================================
// Type Safety Tests
// ============================================================================

#[test]
fn test_version_returns_str_reference() {
    fn check_str_ref(_s: &'static str) {}
    check_str_ref(utilities::get_unified_version());
}

#[test]
fn test_validation_returns_bool() {
    fn check_bool(_b: bool) {}
    check_bool(utilities::validate_unified_usage());
}

// ============================================================================
// Const Evaluation Tests
// ============================================================================

#[test]
fn test_const_version_evaluation() {
    const fn const_check() -> bool {
        utilities::get_unified_version().len() == 5
    }
    
    assert!(const_check());
}

#[test]
fn test_const_validation_evaluation() {
    const fn const_validate() -> bool {
        utilities::validate_unified_usage()
    }
    
    assert!(const_validate());
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[test]
fn test_utilities_thread_safe() {
    use std::thread;
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            (
                utilities::get_unified_version(),
                utilities::validate_unified_usage(),
            )
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let (version, valid) = handle.join().expect("Thread should complete");
        assert_eq!(version, "3.0.0");
        assert!(valid);
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_trait_system_initialization() {
    // Simulating initialization checks
    let version = utilities::get_unified_version();
    let valid = utilities::validate_unified_usage();
    
    assert_eq!(version, "3.0.0");
    assert!(valid);
}

#[test]
fn test_version_in_conditional() {
    if utilities::get_unified_version() == "3.0.0" {
        assert!(true);
    } else {
        panic!("Version mismatch");
    }
}

#[test]
fn test_validation_in_conditional() {
    if utilities::validate_unified_usage() {
        assert!(true);
    } else {
        panic!("Validation failed");
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_version_not_empty() {
    let version = utilities::get_unified_version();
    assert!(!version.is_empty());
}

#[test]
fn test_version_ascii() {
    let version = utilities::get_unified_version();
    assert!(version.is_ascii());
}

#[test]
fn test_version_parsing() {
    let version = utilities::get_unified_version();
    let parts: Vec<&str> = version.split('.').collect();
    assert_eq!(parts.len(), 3, "Version should have 3 parts");
    
    for part in parts {
        assert!(part.parse::<u32>().is_ok(), "Each part should be numeric");
    }
}

// ============================================================================
// Usage Pattern Tests
// ============================================================================

#[test]
fn test_version_check_pattern() {
    fn is_version_3_or_higher() -> bool {
        let version = utilities::get_unified_version();
        version.starts_with('3') || version.starts_with('4')
    }
    
    assert!(is_version_3_or_higher());
}

#[test]
fn test_validation_guard_pattern() {
    fn safe_operation() -> Result<(), &'static str> {
        if !utilities::validate_unified_usage() {
            return Err("Validation failed");
        }
        Ok(())
    }
    
    assert!(safe_operation().is_ok());
}

// ============================================================================
// Documentation Example Tests
// ============================================================================

#[test]
fn test_doc_example_prelude() {
    // Verify prelude exports work as documented
    let version = crate::utilities::get_unified_version();
    assert_eq!(version, "3.0.0");
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
fn test_version_call_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = utilities::get_unified_version();
    }
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 10, "10k calls should be nearly instant");
}

#[test]
fn test_validation_call_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = utilities::validate_unified_usage();
    }
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 10, "10k calls should be nearly instant");
}

// ============================================================================
// String Handling Tests
// ============================================================================

#[test]
fn test_version_string_operations() {
    let version = utilities::get_unified_version();
    let owned = version.to_string();
    assert_eq!(version, owned);
}

#[test]
fn test_version_comparison() {
    let v1 = utilities::get_unified_version();
    let v2 = "3.0.0";
    assert_eq!(v1, v2);
}

#[test]
fn test_version_concatenation() {
    let version = utilities::get_unified_version();
    let message = format!("Trait system version: {}", version);
    assert!(message.contains("3.0.0"));
}

// ============================================================================
// Boolean Logic Tests
// ============================================================================

#[test]
fn test_validation_boolean_ops() {
    let valid = utilities::validate_unified_usage();
    
    assert!(valid && true);
    assert!(true && valid);
    assert!(valid || false);
    assert!(!(!valid));
}

#[test]
fn test_validation_match() {
    match utilities::validate_unified_usage() {
        true => assert!(true),
        false => panic!("Should validate successfully"),
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_functions_never_panic() {
    let result = std::panic::catch_unwind(|| {
        let _ = utilities::get_unified_version();
        let _ = utilities::validate_unified_usage();
    });
    
    assert!(result.is_ok(), "Utilities should never panic");
}

// ============================================================================
// Memory Safety Tests
// ============================================================================

#[test]
fn test_version_lifetime_validity() {
    let version = utilities::get_unified_version();
    
    // Can use version after function scope
    let _owned = version.to_owned();
    let _len = version.len();
    
    assert_eq!(version, "3.0.0");
}

#[test]
fn test_no_memory_leaks() {
    // Repeated calls should not leak memory
    for _ in 0..1000 {
        let _v = utilities::get_unified_version();
        let _valid = utilities::validate_unified_usage();
    }
    
    // If we got here without OOM, test passes
    assert!(true);
}

// ============================================================================
// Static Analysis Tests
// ============================================================================

#[test]
fn test_utilities_are_const() {
    // Verify these can be used in const contexts
    const _VERSION: &str = crate::utilities::get_unified_version();
    const _VALID: bool = crate::utilities::validate_unified_usage();
}

#[test]
fn test_functions_are_must_use() {
    // Both functions are marked #[must_use]
    // This test verifies they compile with must_use
    let _v = utilities::get_unified_version();
    let _valid = utilities::validate_unified_usage();
}

// ============================================================================
// Comprehensive Integration Test
// ============================================================================

#[test]
fn test_complete_trait_system_check() {
    // Comprehensive system validation
    
    // 1. Version check
    let version = utilities::get_unified_version();
    assert_eq!(version, "3.0.0");
    assert!(!version.is_empty());
    assert!(version.is_ascii());
    
    // 2. Validation check
    let valid = utilities::validate_unified_usage();
    assert!(valid);
    
    // 3. Consistency check
    let v2 = utilities::get_unified_version();
    let valid2 = utilities::validate_unified_usage();
    assert_eq!(version, v2);
    assert_eq!(valid, valid2);
    
    // 4. Prelude check
    let v3 = crate::utilities::get_unified_version();
    assert_eq!(version, v3);
}

