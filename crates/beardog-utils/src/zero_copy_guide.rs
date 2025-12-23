//! Zero-copy pattern usage guide and examples
//!
//! This module provides examples and helpers for adopting zero-copy patterns
//! throughout the BearDog codebase.

use std::borrow::Cow;
use std::sync::Arc;

/// Example: String interning for repeated values
///
/// Use `shared_string()` from beardog-utils for frequently used strings
///
/// ```rust
/// use beardog_utils::zero_copy::shared_string;
///
/// // ❌ BEFORE: Creates new String each time
/// let id1 = "service-123".to_string();
/// let id2 = "service-123".to_string(); // Duplicate allocation!
///
/// // ✅ AFTER: Shared allocation
/// let id1 = shared_string("service-123");
/// let id2 = shared_string("service-123"); // Reuses same allocation!
/// ```
pub mod string_interning {
    use super::*;

    /// Service IDs that benefit from interning
    pub const COMMON_SERVICE_IDS: &[&str] = &[
        "beardog-auth",
        "beardog-api",
        "beardog-tunnel",
        "beardog-monitoring",
        "beardog-security",
    ];

    /// Common endpoint URLs that benefit from interning
    pub const COMMON_ENDPOINTS: &[&str] = &[
        "http://localhost:8080",
        "http://localhost:8443",
        "https://localhost:8443",
    ];
}

/// Example: Cow for configuration access
///
/// Use Cow<T> for read-mostly scenarios to avoid unnecessary clones
///
/// ```rust
/// use std::borrow::Cow;
///
/// pub struct Config {
///     timeout: u64,
/// }
///
/// // ❌ BEFORE: Full clone
/// pub fn get_config(config: &Config) -> Config {
///     config.clone() // Expensive!
/// }
///
/// // ✅ AFTER: Borrow when possible
/// pub fn get_config<'a>(config: &'a Config) -> Cow<'a, Config> {
///     Cow::Borrowed(config) // Zero copy!
/// }
/// ```
pub mod cow_patterns {
    use super::*;

    /// Config accessor that uses Cow
    pub fn access_config<'a, T: Clone>(config: &'a T, needs_modification: bool) -> Cow<'a, T> {
        if needs_modification {
            Cow::Owned(config.clone()) // Clone only when needed
        } else {
            Cow::Borrowed(config) // Zero copy most of the time
        }
    }
}

/// Example: Arc for shared ownership
///
/// Use Arc<T> for shared ownership with cheap clones
///
/// ```rust
/// use std::sync::Arc;
///
/// pub struct ExpensiveData {
///     data: Vec<u8>,
/// }
///
/// // ✅ GOOD: Arc for shared ownership
/// let data = Arc::new(ExpensiveData { data: vec![0; 1000] });
/// let thread1 = Arc::clone(&data); // Just increments ref count
/// let thread2 = Arc::clone(&data); // One allocation, shared
/// ```
pub mod arc_patterns {
    use super::*;

    /// Shared configuration using Arc
    pub type SharedConfig<T> = Arc<T>;

    /// Create shared config
    pub fn make_shared<T>(config: T) -> SharedConfig<T> {
        Arc::new(config)
    }
}

/// Example: &str instead of String in APIs
///
/// Accept borrowed strings when ownership is not needed
///
/// ```rust
/// // ❌ BEFORE: Forces allocation
/// pub fn process_id(id: String) {
///     println!("ID: {}", id);
/// }
///
/// // ✅ AFTER: Accepts borrowed
/// pub fn process_id(id: &str) {
///     println!("ID: {}", id);
/// }
///
/// // ✅ BEST: Generic for flexibility
/// pub fn process_id_generic(id: impl AsRef<str>) {
///     println!("ID: {}", id.as_ref());
/// }
/// ```
pub mod string_apis {
    /// Process ID with borrowed string
    pub fn process_id(id: &str) -> String {
        format!("Processed: {}", id)
    }

    /// Generic version for maximum flexibility
    pub fn process_id_generic(id: impl AsRef<str>) -> String {
        format!("Processed: {}", id.as_ref())
    }
}

/// Example: bytes crate for network I/O
///
/// Use bytes::Bytes for zero-copy network operations
///
/// ```rust
/// use bytes::Bytes;
///
/// // ❌ BEFORE: Copies on split
/// fn split_packet(mut vec: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
///     let half = vec.split_off(vec.len() / 2); // Allocates new Vec
///     (vec, half)
/// }
///
/// // ✅ AFTER: Zero-copy split
/// fn split_packet_zero_copy(bytes: Bytes) -> (Bytes, Bytes) {
///     let mid = bytes.len() / 2;
///     let second = bytes.slice(mid..); // No allocation!
///     let first = bytes.slice(..mid);   // No allocation!
///     (first, second)
/// }
/// ```
pub mod bytes_patterns {
    #[cfg(feature = "bytes")]
    use bytes::Bytes;

    #[cfg(feature = "bytes")]
    /// Split packet using zero-copy slicing
    pub fn split_packet(bytes: Bytes) -> (Bytes, Bytes) {
        let mid = bytes.len() / 2;
        let second = bytes.slice(mid..);
        let first = bytes.slice(..mid);
        (first, second)
    }
}

/// Priority targets for zero-copy conversion
pub mod conversion_targets {
    /// Service IDs - High priority
    /// Currently: Many `String` allocations
    /// Target: Use `shared_string()` or `Arc<str>`
    /// Impact: 10-20% memory reduction
    pub const SERVICE_IDS_PRIORITY: &str = "HIGH";

    /// Configuration access - High priority
    /// Currently: Many `.clone()` calls
    /// Target: Use `Cow<Config>` pattern
    /// Impact: 5-10% memory reduction
    pub const CONFIG_ACCESS_PRIORITY: &str = "HIGH";

    /// Network I/O - High priority
    /// Currently: `Vec<u8>` with copies
    /// Target: Use `bytes::Bytes`
    /// Impact: 15-25% memory reduction in network code
    pub const NETWORK_IO_PRIORITY: &str = "HIGH";

    /// API parameters - Medium priority
    /// Currently: `String` parameters
    /// Target: Use `&str` or `impl AsRef<str>`
    /// Impact: 5-10% memory reduction
    pub const API_PARAMS_PRIORITY: &str = "MEDIUM";

    /// Format operations - Medium priority
    /// Currently: `format!()` everywhere
    /// Target: Use `zero_copy_format!()` macro
    /// Impact: 5-10% memory reduction
    pub const FORMAT_OPS_PRIORITY: &str = "MEDIUM";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_clone_is_cheap() {
        let data = Arc::new(vec![0u8; 1000]);
        let clone1 = Arc::clone(&data);
        let clone2 = Arc::clone(&data);

        // All point to same allocation
        assert_eq!(Arc::strong_count(&data), 3);
        assert_eq!(Arc::strong_count(&clone1), 3);
        assert_eq!(Arc::strong_count(&clone2), 3);
    }

    #[test]
    fn test_cow_borrowed_no_clone() {
        let value = vec![1, 2, 3];
        let cow: Cow<Vec<i32>> = Cow::Borrowed(&value);

        // Verify it's borrowed, not cloned
        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_str_api_no_allocation() {
        // Passing &str doesn't allocate
        let result = string_apis::process_id("test-id");
        assert!(result.contains("test-id"));
    }

    #[test]
    fn test_generic_str_api_accepts_both() {
        // Can pass &str
        let result1 = string_apis::process_id_generic("test");
        assert!(result1.contains("test"));

        // Can pass String
        let result2 = string_apis::process_id_generic(String::from("test2"));
        assert!(result2.contains("test2"));
    }
}

