//! Incident response types
//!
//! This module contains types for incident response management,
//! including incident structures and status tracking.
//!
//! ## Features
//! - Incident response structure
//! - Status tracking throughout incident lifecycle
//! - Containment and remediation actions
//! - Lessons learned capture
//! - Team assignment and coordination
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{IncidentResponse, IncidentStatus};
//! use chrono::Utc;
//!
//! let incident = IncidentResponse {
//!     incident_id: "INC-2024-001".to_string(),
//!     threat_id: "threat-001".to_string(),
//!     status: IncidentStatus::Open,
//!     created_at: Utc::now(),
//!     updated_at: Utc::now(),
//!     description: "Malware detected on workstation".to_string(),
//!     ..Default::default()
//! };
//! ```

pub mod metrics;
pub mod response;
pub mod status;
pub mod team;
pub mod timeline;

// Re-export all types for backward compatibility
pub use metrics::*;
pub use response::*;
pub use status::*;
pub use team::*;
pub use timeline::*;
