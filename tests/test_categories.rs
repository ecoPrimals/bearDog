// Test Categorization System
// Created: October 26, 2025
// Purpose: Standardize test organization and enable accurate coverage measurement

//! # Test Categorization System
//!
//! This module provides a standardized system for categorizing and tagging tests
//! to enable accurate coverage measurement and test filtering.
//!
//! ## Usage
//!
//! Add documentation tags to your tests:
//!
//! ```rust
//! // TEST_CATEGORY: unit
//! // TEST_DOMAIN: security
//! // TEST_PRIORITY: high
//! #[test]
//! fn test_security_encryption_basic() {
//!     // Test implementation
//! }
//! ```
//!
//! ## Categories
//!
//! - `unit` - Pure unit tests (single function/method)
//! - `integration` - Integration tests (multiple components)
//! - `e2e` - End-to-end tests (full system workflows)
//! - `chaos` - Chaos/fault injection tests
//! - `performance` - Performance/benchmark tests
//! - `security` - Security-specific validation
//! - `compliance` - Compliance/regulatory tests
//!
//! ## Domains
//!
//! - `core` - Core orchestration
//! - `security` - Security subsystem
//! - `hsm` - HSM operations
//! - `networking` - Network operations
//! - `genetics` - Genetic evolution
//! - `monitoring` - Monitoring/observability
//! - `workflows` - Workflow orchestration
//! - `adapters` - Adapter system
//! - `config` - Configuration
//! - `auth` - Authentication/authorization
//! - `crypto` - Cryptography
//! - `types` - Type system
//! - `errors` - Error handling

/// Test categories
pub mod category {
    pub const UNIT: &str = "unit";
    pub const INTEGRATION: &str = "integration";
    pub const E2E: &str = "e2e";
    pub const CHAOS: &str = "chaos";
    pub const PERFORMANCE: &str = "performance";
    pub const SECURITY: &str = "security";
    pub const COMPLIANCE: &str = "compliance";
    pub const PROPERTY: &str = "property";
}

/// Test domains
pub mod domain {
    pub const CORE: &str = "core";
    pub const SECURITY: &str = "security";
    pub const HSM: &str = "hsm";
    pub const NETWORKING: &str = "networking";
    pub const GENETICS: &str = "genetics";
    pub const MONITORING: &str = "monitoring";
    pub const WORKFLOWS: &str = "workflows";
    pub const ADAPTERS: &str = "adapters";
    pub const CONFIG: &str = "config";
    pub const AUTH: &str = "auth";
    pub const CRYPTO: &str = "crypto";
    pub const TYPES: &str = "types";
    pub const ERRORS: &str = "errors";
    pub const TUNNEL: &str = "tunnel";
    pub const API: &str = "api";
    pub const CLI: &str = "cli";
    pub const DEPLOY: &str = "deploy";
}

/// Test priorities
pub mod priority {
    pub const CRITICAL: &str = "critical";
    pub const HIGH: &str = "high";
    pub const MEDIUM: &str = "medium";
    pub const LOW: &str = "low";
}

/// Test requirements
pub mod requires {
    pub const HARDWARE: &str = "hardware";
    pub const NETWORK: &str = "network";
    pub const DATABASE: &str = "database";
    pub const FILESYSTEM: &str = "filesystem";
    pub const SLOW: &str = "slow";
}

/// Helper to check if running specific test categories
pub fn should_run_category(category: &str) -> bool {
    // Check environment variables for test filtering
    if let Ok(filter) = std::env::var("TEST_CATEGORY") {
        return filter.split(',').any(|c| c.trim() == category);
    }
    true
}

/// Helper to check if running specific test domains
pub fn should_run_domain(domain: &str) -> bool {
    if let Ok(filter) = std::env::var("TEST_DOMAIN") {
        return filter.split(',').any(|d| d.trim() == domain);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    #[test]
    fn test_category_constants_defined() {
        assert_eq!(category::UNIT, "unit");
        assert_eq!(category::INTEGRATION, "integration");
        assert_eq!(category::E2E, "e2e");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    #[test]
    fn test_domain_constants_defined() {
        assert_eq!(domain::CORE, "core");
        assert_eq!(domain::SECURITY, "security");
        assert_eq!(domain::HSM, "hsm");
    }
}
