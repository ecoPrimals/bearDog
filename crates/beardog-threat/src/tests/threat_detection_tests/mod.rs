//! Threat Detection Comprehensive Test Suite
//!
//! TEST_CATEGORY: unit + integration
//! TEST_DOMAIN: threat-detection
//! TEST_PRIORITY: critical
//!
//! This module provides comprehensive testing for threat detection mechanisms.
//! Originally consolidated from a single 1,986-line file, now split into focused modules.
//!
//! ## Test Modules
//!
//! - `types` - Shared test helper types and utilities
//! - `anomaly_tests` - Anomaly detection and pattern matching tests
//! - `behavioral_tests` - Behavioral analysis tests
//! - `monitoring_tests` - Real-time monitoring and classification tests
//! - `intelligence_tests` - Threat intelligence and incident response tests

pub mod anomaly_tests;
pub mod behavioral_tests;
pub mod intelligence_tests;
pub mod monitoring_tests;
pub mod types;

// Re-export commonly used items for convenience
