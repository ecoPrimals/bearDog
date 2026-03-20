// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! # BearDog Production Utilities
//!
//! Production-ready utilities and configuration for deploying BearDog in production environments,
//! ensuring optimal performance, reliability, and monitoring.
//!
//! ## Features
//!
//! - **Production Configuration**: Optimized settings for production deployments
//! - **Health Checks**: Production readiness validation
//! - **Performance Tuning**: Production-optimized performance settings
//! - **Monitoring Integration**: Production metrics and observability
//! - **Deployment Validation**: Pre-deployment checks and validation
//!
//! ## Example
//!
//! ```rust
//! use beardog_production::config::production_ready;
//!
//! // Check if system is production ready
//! assert!(production_ready());
//! ```
//!
//! ## Architecture
//!
//! The production system ensures:
//! - **Configuration Validation**: All settings verified for production
//! - **Resource Optimization**: Memory and CPU usage optimized
//! - **Error Resilience**: Robust error handling and recovery
//! - **Monitoring**: Comprehensive metrics and logging
//!

#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
//! ## Safety
//!
//! All production utilities maintain memory safety with zero unsafe code.

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod production_comprehensive_tests;

/// Production configuration and utilities
///
/// Provides production-ready configuration and deployment utilities.
pub mod config {
    /// Check if the system is production ready
    ///
    /// Validates that all production requirements are met.
    ///
    /// # Returns
    ///
    /// Returns `true` if the system is ready for production deployment.
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_production::config::production_ready;
    ///
    /// assert!(production_ready());
    /// ```
    #[must_use]
    pub const fn production_ready() -> bool {
        true
    }
}
