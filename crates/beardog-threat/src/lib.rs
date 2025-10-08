//! # BearDog Threat Detection and Analysis
//!
//! Advanced threat detection and security analysis for the BearDog ecosystem,
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
//! ```rust,no_run
//! use beardog_threat::ThreatDetectionEngine;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize threat detection
//! let engine = ThreatDetectionEngine::new()?;
//!
//! // Analyze potential threats
//! let threat_level = engine.analyze_activity("suspicious_pattern").await?;
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
//! All threat detection operations maintain memory safety with zero unsafe code.

/// Core threat detection engine and analysis
///
/// Provides the main threat detection engine and related security analysis operations.
pub mod threat;

pub use threat::ThreatDetectionEngine;
