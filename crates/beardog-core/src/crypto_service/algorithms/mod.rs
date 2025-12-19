//! Cryptographic algorithms

// Allow pedantic lints for algorithm implementations
// These functions have comprehensive documentation and error handling
// The `# Errors` sections would be repetitive (all return "encryption/decryption failed")
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::double_must_use)]

pub mod asymmetric;
pub mod discovery;
pub mod hashing;
pub mod symmetric;

pub use asymmetric::*;
pub use discovery::*;
pub use hashing::*;
pub use symmetric::*;
