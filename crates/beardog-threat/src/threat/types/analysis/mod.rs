//! Threat Analysis Types Module
//!
//! This module contains types for threat analysis, security events,
//! and analysis results.
//!
//! The module is organized into focused sub-modules:
//! - `events`: Security event structures
//! - `results`: Threat analysis results
//! - `metrics`: Analysis performance metrics
//! - `correlation`: Event correlation analysis
//! - `session`: Analysis session management
//!
//! ## Features
//! - Security event structures
//! - Threat analysis results
//! - Analysis statistics
//! - Event correlation
//! - Performance metrics
//! - Session management
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::analysis::{SecurityEvent, ThreatAnalysisResult};
//! use chrono::Utc;
//! use std::collections::HashMap;
//!
//! let event = SecurityEvent {
//!     event_id: "event-001".to_string(),
//!     timestamp: Utc::now(),
//!     event_type: "login_attempt".to_string(),
//!     source_ip: "192.168.1.100".to_string(),
//!     destination_ip: "10.0.0.1".to_string(),
//!     user_id: "user123".to_string(),
//!     ..Default::default()
//! };
//! ```

pub mod correlation;
pub mod events;
pub mod metrics;
pub mod results;
pub mod session;

// Re-export all types for backward compatibility
pub use correlation::*;
pub use events::*;
pub use metrics::*;
pub use results::*;
pub use session::*;

// Re-export MlPrediction from ml_engine for compatibility
pub use crate::threat::ml_engine::MlPrediction;
