//! Integration Test Modules
//! 
//! Modular organization of integration tests for better maintainability.
//! Replaces the monolithic integration_tests.rs file.

pub mod core_initialization;
pub mod api_endpoints;
pub mod threat_detection;
pub mod nestgate_adapter;
pub mod compliance_engine;
pub mod workflow_engine;
pub mod security_provider;
pub mod data_flow_integration;
pub mod performance_tests;
pub mod error_handling;

// Common test utilities and helpers
pub mod common;

/// Re-export common test utilities for convenience
pub use common::*; 