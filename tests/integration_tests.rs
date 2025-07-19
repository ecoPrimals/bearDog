//! Integration Tests for BearDog
//! 
//! Modular integration tests organized by functionality.
//! Replaces the previous monolithic 1787-line test file with
//! well-organized, maintainable test modules.

// Import all modular test modules
mod integration;

// Re-export for IDE and test runner discovery
pub use integration::*; 