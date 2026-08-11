// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]

//! # `BearDog` Threat Detection and Analysis
//!
//! Advanced threat detection and security analysis for the `BearDog` ecosystem,
//! providing real-time threat identification, ML-powered analysis, and automated response.
//!
//! ## Features
//!
//! - **Real-Time Detection**: Continuous threat monitoring and detection
//! - **ML-Powered Analysis**: Machine learning for threat pattern recognition
//! - **Behavioral Analysis**: Anomaly detection through behavioral patterns
//! - **Automated Response**: Intelligent threat mitigation and response
//! - **Threat Intelligence**: Integration with threat intelligence feeds
//! - **Zero-Day Detection**: Novel threat pattern identification
//!
//! ## Example
//!
//! ```rust
//! use beardog_threat::ThreatDetectionEngine;
//! use beardog_types::canonical::config::domains::threat::CanonicalThreatDetectionConfig;
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize threat detection with default configuration
//! let config = CanonicalThreatDetectionConfig::default();
//! let engine = ThreatDetectionEngine::new(config)?;
//!
//! // Engine is now ready for threat detection
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The threat detection system uses multiple analysis layers:
//! - **Signature Matching**: Known threat pattern detection
//! - **Behavioral Analysis**: Anomaly detection through ML
//! - **Heuristic Analysis**: Rule-based threat identification
//! - **Threat Correlation**: Cross-reference multiple threat indicators
//!
//! ## Safety
//!
//! All threat detection operations maintain memory safety with full memory safety.

// October 27, 2025: Comprehensive test expansion
#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "crate-level threat test modules: exhaustive coverage (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod threat_comprehensive_tests;

// October 31, 2025: Week 3 Test Expansion
#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "crate-level threat test modules: exhaustive coverage (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod tests;

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::float_cmp,
    reason = "coverage_gap tests: float comparisons and exhaustive patterns (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod coverage_gap;

#[cfg(test)]
pub(crate) mod float_assert {
    /// Assert two `f64` values are approximately equal (test helper).
    pub fn near_f64(actual: f64, expected: f64) {
        const EPS: f64 = 1e-9;
        let diff = (actual - expected).abs();
        assert!(diff < EPS, "expected {expected}, got {actual} (|Δ|={diff})");
    }
}

/// Core threat detection engine and analysis
///
/// Provides the main threat detection engine and related security analysis operations.
pub mod threat;

pub use threat::ThreatDetectionEngine;
