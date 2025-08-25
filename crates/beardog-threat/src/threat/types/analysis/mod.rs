// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Threat Analysis Types Module
///
/// This module contains types for threat analysis, security events,
/// and analysis results.
/// The module is organized into focused sub-modules:
/// - `events`: Security event structures
/// - `results`: Threat analysis results
/// - `metrics`: Analysis performance metrics
/// - `correlation`: Event correlation analysis
/// - `session`: Analysis session management
/// ## Features
/// - Security event structures
/// - Threat analysis results
/// - Analysis statistics
/// - Event correlation
/// - Performance metrics
/// - Session management
/// ## Example
/// ```rust
/// use beardog::threat::types::analysis::{SecurityEvent, ThreatAnalysisResult};
/// use chrono::Utc;
/// use std::collections::HashMap;
/// let event = SecurityEvent {
///     event_id: "event-001".to_string(),
///     timestamp: Utc::now(),
///     event_type: "login_attempt".to_string(),
///     source_ip: "192.168.1.100".to_string(),
///     destination_ip: "10.0.0.1".to_string(),
///     user_id: "user123".to_string(),
///     ..Default::default()
/// };
/// ```
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
