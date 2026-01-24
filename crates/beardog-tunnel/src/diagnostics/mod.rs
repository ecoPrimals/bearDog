//! Production Diagnostics Module
//!
//! **Philosophy**: MOVE diagnostic logging here (not REMOVE) - fossil record principle
//!
//! All diagnostic logging preserved for future debugging and troubleshooting.
//! Enable with: `cargo build --features diagnostics`
//!
//! ## Why This Module Exists
//!
//! During the 100% Pure Rust HTTPS evolution, extensive diagnostic logging was
//! added to debug TLS 1.3 integration issues. This logging was CRITICAL for:
//! - Identifying AAD (Additional Authenticated Data) issues
//! - Debugging transcript hash mismatches
//! - Verifying key derivation at each step
//! - Comparing with RFC 8448 test vectors
//!
//! Rather than DELETE this valuable diagnostic capability, we MOVED it here.
//! It can be re-enabled anytime for troubleshooting production issues.
//!
//! ## Usage
//!
//! **Development/Debugging**:
//! ```bash
//! cargo build --features diagnostics
//! cargo test --features diagnostics -- --nocapture
//! ```
//!
//! **Production** (default):
//! ```bash
//! cargo build --release
//! # Zero diagnostic overhead (inlined no-ops)
//! ```
//!
//! ## Performance
//!
//! When the `diagnostics` feature is **disabled** (default):
//! - All diagnostic functions compile to **zero-cost no-ops**
//! - Completely inlined and optimized away
//! - **Zero performance impact**
//!
//! When the `diagnostics` feature is **enabled**:
//! - Full verbose logging to stderr
//! - Hex dumps of keys, nonces, AAD, ciphertexts
//! - Perfect for debugging upstream integration issues

pub mod crypto;

/// Diagnostic macro - only active with "diagnostics" feature
///
/// # Example
/// ```no_run
/// use beardog_tunnel::diagnostic;
/// 
/// diagnostic!("🔍 Debug value: {}", 42);  // Only prints if feature enabled
/// ```
#[macro_export]
macro_rules! diagnostic {
    ($($arg:tt)*) => {
        #[cfg(feature = "diagnostics")]
        eprintln!($($arg)*);
    };
}

// Re-export for convenience
pub use crypto::*;

