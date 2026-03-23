// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

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

#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
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
    unused_comparisons
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
    unused_comparisons
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
    clippy::float_cmp
)]
#[cfg(test)]
mod coverage_gap;

/// Core threat detection engine and analysis
///
/// Provides the main threat detection engine and related security analysis operations.
pub mod threat;

pub use threat::ThreatDetectionEngine;
