// SPDX-License-Identifier: AGPL-3.0-only

//! # Threat Detection Types - Modular Architecture
//!
//! This module provides comprehensive threat detection types organized into focused,
//! maintainable modules using canonical BearDog systems.
//!
//! ## 🎯 **Modular Structure**
//!
//! The threat detection system is organized into logical modules:
//! - **`core`**: Core types, configuration, and events (~330 lines)
//! - **`intelligence`**: Threat intelligence and ML models (~320 lines)  
//! - **`response`**: Incident response and security events (~330 lines)
//!
//! ## 🏗️ **Architecture Benefits**
//!
//! - **Maintainability**: Each module under 350 lines
//! - **Single Responsibility**: Clear separation of concerns
//! - **Canonical Integration**: Uses unified constants and error handling
//! - **Type Safety**: Comprehensive type coverage with proper validation
//! - **Performance**: Zero-cost abstractions and efficient serialization
//!
//! ## 📦 **Module Organization**
//!
//! ```rust
//! use beardog_threat::threat::types::modules::{
//!     core::{ThreatEvent, ThreatDetectionConfig, ThreatSeverity},
//!     intelligence::{ThreatIntelligenceFeed, MlModel, ThreatAnalysisResult},
//!     response::{IncidentResponse, SecurityEvent, ResponseAction},
//! };
//! ```
//!
//! ## 🚀 **Usage Examples**
//!
//! ```rust
//! use beardog_threat::threat::types::modules::core::*;
//!
//! // Create a threat detection configuration
//! let config = ThreatDetectionConfig::default();
//!
//! // Create a threat event
//! let threat = ThreatEvent {
//!     id: "threat-001".to_string(),
//!     threat_type: ThreatType::Malware,
//!     severity: ThreatSeverity::High,
//!     // ... other fields
//! };
//! ```

/// Core threat detection types and configuration
pub mod core;

/// Threat intelligence and machine learning types
pub mod intelligence;

/// Incident response and security event management
pub mod response;

// Re-export commonly used types for convenience
pub use core::{
    ThreatDetectionConfig, ThreatEvent, ThreatSeverity, ThreatType, ThreatStatus,
    ThreatSource, ThreatTarget, ThreatAction, ThreatIndicator, MitigationStep,
    DetectionMethod, SourceClassification, AssetCriticality, ProtectionLevel,
    IndicatorType,
};

pub use intelligence::{
    ThreatIntelligenceFeed, MlModel, MlModelType, ThreatAnalysisResult,
    AnalysisMethod, ThreatHuntingQuery, ThreatHuntingResult, 
    IntelligenceFeedType, FeedAuthConfig,
};

pub use response::{
    SecurityEvent, SecurityEventType, IncidentResponse, ResponseStatus,
    ResponseAction, ActionStatus, ResponsePriority, AutomatedResponseRule,
    RuleCondition, ResponseMetrics, ResponseTimelineEntry,
};

// Re-export engine types for backward compatibility
pub use super::engine::*; 