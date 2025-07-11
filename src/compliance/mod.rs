//! Main module file - exports and core functionality
//! 
//! This module was refactored from a large file to improve maintainability.

// Re-export public types and functions
pub use types::*;
pub use handlers::*;

// Module declarations
mod types;
mod handlers;

#[cfg(test)]
mod tests;

// Core module functionality will be implemented here
