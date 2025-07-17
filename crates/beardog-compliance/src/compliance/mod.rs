//! Main module file - exports and core functionality
//!
//! This module was refactored from a large file to improve maintainability.

// Re-export public types and functions
pub use handlers::*;
pub use types::*;

// Module declarations
mod handlers;
mod types;

#[cfg(test)]
mod tests;

// Core module functionality will be implemented here
