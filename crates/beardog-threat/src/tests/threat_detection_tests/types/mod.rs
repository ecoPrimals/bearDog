// SPDX-License-Identifier: AGPL-3.0-only

//! Threat Detection Test Helper Types (Modular)
//!
//! This module contains all the shared test helper types used across
//! the threat detection test suite, organized into logical sub-modules.

#![allow(dead_code)]

pub mod adaptive;
pub mod alerting;
pub mod anomaly;
pub mod behavior;
pub mod classifier;
pub mod composite;
pub mod false_positive;
pub mod intelligence;
pub mod monitoring;
pub mod pattern;
pub mod rate_limit;
pub mod refiner;
pub mod response;

// Re-export commonly used types for convenience
pub use adaptive::AdaptiveDetector;
pub use alerting::{AlertHandler, ThreatAlert, ThreatEvent};
pub use anomaly::AnomalyDetector;
pub use behavior::{BehaviorAnalyzer, BehaviorEvent, Threat, ThreatType};
pub use classifier::ThreatClassifier;
pub use composite::{CompositeThreat, ThreatCategory};
pub use false_positive::FalsePositiveHandler;
pub use intelligence::{Ioc, IocType, ThreatFeed, ThreatIntelligence};
pub use monitoring::ThreatMonitor;
pub use pattern::{PatternMatcher, ThreatPattern, ThreatSeverity};
pub use rate_limit::RateLimitDetector;
pub use refiner::PatternRefiner;
pub use response::{
    EscalationLevel, EscalationWorkflow, IncidentResponseEngine, ResponseAction, ResponseExecutor,
    ResponseRule, ResponseTracker,
};
