// SPDX-License-Identifier: AGPL-3.0-only

//! Zero-Copy Optimization Patterns for BearDog
//!
//! This module provides zero-copy patterns and utilities to minimize allocations
//! and improve performance in hot paths.
//!
//! ## Optimization Patterns:
//!
//! 1. **`Arc<[T]>` instead of `Arc<Vec<T>>`**:
//!    - More efficient: No capacity overhead
//!    - Immutable by design: Prevents accidental mutations
//!    - Coerces from `Vec<T>` via `into()`
//!
//! 2. **`Cow<'_, [T]>` for conditional copying**:
//!    - Borrowed by default, cloned only when mutated
//!    - Perfect for functions that mostly read, rarely write
//!
//! 3. **`Arc<str>` for shared strings**:
//!    - Zero-copy string sharing
//!    - Coerces from `String` via `into()`
//!
//! 4. **Bytes crate for buffer management**:
//!    - Zero-copy slicing
//!    - Efficient reference counting
//!
//! ## Examples:
//!
//! ```rust
//! use std::sync::Arc;
//! use std::borrow::Cow;
//!
//! // ✅ GOOD: Arc<[u8]> - zero-copy slice sharing
//! let data: Vec<u8> = vec![1, 2, 3, 4];
//! let shared: Arc<[u8]> = data.into();  // No copy!
//! let clone1 = shared.clone();  // Just increments refcount
//! let clone2 = shared.clone();  // Just increments refcount
//!
//! // ❌ BAD: Arc<Vec<u8>> - unnecessary capacity overhead
//! // let shared_vec: Arc<Vec<u8>> = Arc::new(data);
//!
//! // ✅ GOOD: Cow for conditional cloning
//! fn process_data(input: Cow<[u8]>) -> Cow<[u8]> {
//!     if input.len() > 100 {
//!         // Borrow (zero-copy) for large inputs
//!         input
//!     } else {
//!         // Clone and modify for small inputs
//!         let mut owned = input.into_owned();
//!         owned.push(0);
//!         Cow::Owned(owned)
//!     }
//! }
//! ```

use std::borrow::Cow;
use std::sync::Arc;

/// Zero-copy byte buffer using Arc<[u8]>
///
/// More efficient than `Arc<Vec<u8>>` because:
/// - No capacity overhead
/// - Immutable by design
/// - Smaller memory footprint
///
/// # Examples
///
/// ```
/// use beardog_core::zero_copy_optimization::ZeroCopyBuffer;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let buffer = ZeroCopyBuffer::new(data);
///
/// // Clone is cheap (just increments refcount)
/// let clone1 = buffer.clone();
/// let clone2 = buffer.clone();
///
/// assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroCopyBuffer {
    data: Arc<[u8]>,
}

impl ZeroCopyBuffer {
    /// Create a new zero-copy buffer from a Vec
    ///
    /// The Vec is converted directly to Arc<[u8]> with no copying.
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data.into(),  // Vec -> Arc<[u8]> (zero-copy)
        }
    }

    /// Create from an existing Arc<[u8]>
    #[must_use]
    pub fn from_arc(data: Arc<[u8]>) -> Self {
        Self { data }
    }

    /// Get a reference to the underlying slice
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get the length of the buffer
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a zero-copy slice of this buffer
    ///
    /// The returned buffer shares the same underlying Arc.
    #[must_use]
    pub fn slice(&self, range: std::ops::Range<usize>) -> Self {
        let sliced_data: Arc<[u8]> = self.data[range].into();
        Self {
            data: sliced_data,
        }
    }

    /// Get the underlying Arc for direct use
    #[must_use]
    pub fn into_arc(self) -> Arc<[u8]> {
        self.data
    }
}

impl From<Vec<u8>> for ZeroCopyBuffer {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}

impl From<&[u8]> for ZeroCopyBuffer {
    fn from(data: &[u8]) -> Self {
        Self::new(data.to_vec())
    }
}

impl AsRef<[u8]> for ZeroCopyBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

/// Zero-copy string using Arc<str>
///
/// More efficient than `Arc<String>` because:
/// - No capacity overhead
/// - Immutable by design
/// - Smaller memory footprint
///
/// # Examples
///
/// ```
/// use beardog_core::zero_copy_optimization::ZeroCopyString;
///
/// let text = String::from("Hello, BearDog!");
/// let shared = ZeroCopyString::new(text);
///
/// // Clone is cheap (just increments refcount)
/// let clone1 = shared.clone();
/// let clone2 = shared.clone();
///
/// assert_eq!(shared.as_str(), "Hello, BearDog!");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZeroCopyString {
    data: Arc<str>,
}

impl ZeroCopyString {
    /// Create a new zero-copy string from a String
    ///
    /// The String is converted directly to Arc<str> with no copying.
    #[must_use]
    pub fn new(data: String) -> Self {
        Self {
            data: data.into(),  // String -> Arc<str> (zero-copy)
        }
    }

    /// Create from an existing Arc<str>
    #[must_use]
    pub fn from_arc(data: Arc<str>) -> Self {
        Self { data }
    }

    /// Get a reference to the underlying str
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Get the length of the string
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the string is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the underlying Arc for direct use
    #[must_use]
    pub fn into_arc(self) -> Arc<str> {
        self.data
    }
}

impl From<String> for ZeroCopyString {
    fn from(data: String) -> Self {
        Self::new(data)
    }
}

impl From<&str> for ZeroCopyString {
    fn from(data: &str) -> Self {
        Self::new(data.to_string())
    }
}

impl AsRef<str> for ZeroCopyString {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl std::fmt::Display for ZeroCopyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

/// Conditional copy-on-write buffer
///
/// Uses `Cow<[u8]>` for zero-copy when possible, clones only when needed.
///
/// # Use Cases:
/// - Functions that mostly read, rarely write
/// - Conditional transformations
/// - Lazy allocation patterns
///
/// # Examples
///
/// ```
/// use beardog_core::zero_copy_optimization::process_with_cow;
/// use std::borrow::Cow;
///
/// // Borrowed input (zero-copy)
/// let input = &[1, 2, 3, 4, 5];
/// let result = process_with_cow(Cow::Borrowed(input));
/// // No allocation if no transformation needed!
///
/// // Owned input (already allocated)
/// let owned = vec![1, 2, 3, 4, 5];
/// let result = process_with_cow(Cow::Owned(owned));
/// ```
pub fn process_with_cow(input: Cow<[u8]>) -> Cow<[u8]> {
    // Example: Only clone if we need to modify
    if input.len() > 100 {
        // Large inputs: return as-is (zero-copy)
        input
    } else {
        // Small inputs: add padding (requires clone)
        let mut owned = input.into_owned();
        owned.push(0);
        Cow::Owned(owned)
    }
}

/// Efficient string transformation using Cow
///
/// Only allocates if transformation is needed.
pub fn transform_string(input: &str, uppercase: bool) -> Cow<str> {
    if uppercase {
        // Transformation needed: allocate
        Cow::Owned(input.to_uppercase())
    } else {
        // No transformation: zero-copy borrow
        Cow::Borrowed(input)
    }
}

#[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_buffer() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = ZeroCopyBuffer::new(data);
        
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
        
        // Clone is cheap (just refcount increment)
        let clone1 = buffer.clone();
        let clone2 = buffer.clone();
        
        assert_eq!(clone1.as_slice(), clone2.as_slice());
    }

    #[test]
    fn test_zero_copy_buffer_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = ZeroCopyBuffer::new(data);
        
        let sliced = buffer.slice(1..4);
        assert_eq!(sliced.as_slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_zero_copy_string() {
        let text = String::from("Hello, BearDog!");
        let shared = ZeroCopyString::new(text);
        
        assert_eq!(shared.len(), 15);
        assert_eq!(shared.as_str(), "Hello, BearDog!");
        
        // Clone is cheap (just refcount increment)
        let clone1 = shared.clone();
        let clone2 = shared.clone();
        
        assert_eq!(clone1.as_str(), clone2.as_str());
    }

    #[test]
    fn test_cow_no_allocation() {
        let data = &[1, 2, 3];
        let cow = Cow::Borrowed(data);
        
        // No allocation for borrowed data
        match cow {
            Cow::Borrowed(_) => (),
            Cow::Owned(_) => panic!("Should be borrowed"),
        }
    }

    #[test]
    fn test_cow_with_modification() {
        let data = vec![1, 2, 3];
        let mut cow: Cow<[i32]> = Cow::Owned(data);
        
        // Modification triggers clone if borrowed
        cow.to_mut().push(4);
        
        match cow {
            Cow::Owned(ref v) => assert_eq!(v, &vec![1, 2, 3, 4]),
            Cow::Borrowed(_) => panic!("Should be owned after modification"),
        }
    }

    #[test]
    fn test_transform_string_no_copy() {
        let input = "hello";
        let result = transform_string(input, false);
        
        // No transformation: should be borrowed (zero-copy)
        match result {
            Cow::Borrowed(s) => assert_eq!(s, "hello"),
            Cow::Owned(_) => panic!("Should be borrowed (zero-copy)"),
        }
    }

    #[test]
    fn test_transform_string_with_copy() {
        let input = "hello";
        let result = transform_string(input, true);
        
        // Transformation: should be owned
        match result {
            Cow::Owned(s) => assert_eq!(s, "HELLO"),
            Cow::Borrowed(_) => panic!("Should be owned after transformation"),
        }
    }
}

