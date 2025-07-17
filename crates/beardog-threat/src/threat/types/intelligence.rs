//! Threat intelligence types
//!
//! This module contains types for threat intelligence feeds, indicators,
//! and threat intelligence data structures.
//!
//! ## Features
//! - Threat intelligence feed management
//! - Indicator of compromise (IOC) types
//! - Feed status and update tracking
//! - Intelligence data correlation
//! - Threat actor attribution
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{ThreatIntelligenceFeed, ThreatIndicator, IndicatorType};
//! use chrono::Utc;
//!
//! let indicator = ThreatIndicator {
//!     indicator_type: IndicatorType::IpAddress,
//!     value: "192.168.1.100".to_string(),
//!     confidence: 0.9,
//!     confidence_score: 0.9,
//!     first_seen: Utc::now(),
//!     last_seen: Utc::now(),
//!     description: "Known malicious IP".to_string(),
//!     ..Default::default()
//! };
//! ```
//!
//! This module is organized into focused sub-modules:
//! - `enums` - Enumeration types for feed types, statuses, and frequencies
//! - `indicators` - Threat indicator types and functionality
//! - `feeds` - Threat intelligence feed management

pub mod enums;
pub mod feeds;
pub mod indicators;

// Re-export all types for backward compatibility
pub use enums::*;
pub use feeds::*;
pub use indicators::*;
