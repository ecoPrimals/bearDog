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


/// Incident timeline management
///
/// This module contains types and functionality for managing incident timeline
/// entries and tracking event history.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Incident timeline entry
/// Represents a single entry in the incident timeline
/// with timestamp and action details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTimelineEntry {
    /// Entry timestamp
    pub timestamp: DateTime<Utc>,
    /// Action or event description
    pub action: String,
    /// Person or system performing action
    pub actor: String,
    /// Entry type
    pub entry_type: TimelineEntryType,
    /// Additional details
    pub details: Option<String>,
}
/// Timeline entry type enumeration
/// Categorizes different types of timeline entries
/// for better organization and filtering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimelineEntryType {
    /// Detection event
    Detection,
    /// Investigation activity
    Investigation,
    /// Containment action
    Containment,
    /// Remediation action
    Remediation,
    /// Communication event
    Communication,
    /// Status change
    StatusChange,
    /// External coordination
    ExternalCoordination,
    /// Administrative action
    Administrative,}


impl Default for IncidentTimelineEntry {}


    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            action: String::new(),
            actor: String::new(),
            entry_type: TimelineEntryType::Investigation,
            details: None,
        }
    }
impl IncidentTimelineEntry {
    /// Create a new timeline entry
    ///
    /// # Arguments
    /// * `action` - Action or event description
    /// * `actor` - Person or system performing action
    /// * `entry_type` - Type of timeline entry
    /// # Returns
    /// A new `IncidentTimelineEntry` instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentTimelineEntry, TimelineEntryType};
    /// let entry = IncidentTimelineEntry::new(
    ///     "Malware detected on workstation".to_string(),
    ///     "antivirus-system".to_string(),
    ///     TimelineEntryType::Detection
    /// );
    /// ```
    pub fn new(action: String, actor: String, entry_type: TimelineEntryType) -> Self {
            action,
            actor,
            entry_type,
    /// Add details to entry
    /// * `details` - Additional details
    /// let mut entry = IncidentTimelineEntry::new(
    ///     "System isolated".to_string(),
    ///     "analyst-001".to_string(),
    ///     TimelineEntryType::Containment
    /// entry.add_details("Network connection severed".to_string());
    /// assert!(entry.details.is_some());}


    pub fn add_details(&mut self, details: String) {
        self.details = Some(details);
    /// Get entry age in minutes
    /// Minutes since entry creation
    ///     "Action taken".to_string(),
    ///     "system".to_string(),
    ///     TimelineEntryType::Investigation
    /// let age = entry.age_minutes();
    /// assert!(age >= 0);
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
impl std::fmt::Display for TimelineEntryType {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineEntryType::Detection => write!(f, "Detection"),
            TimelineEntryType::Investigation => write!(f, "Investigation"),
            TimelineEntryType::Containment => write!(f, "Containment"),
            TimelineEntryType::Remediation => write!(f, "Remediation"),
            TimelineEntryType::Communication => write!(f, "Communication"),
            TimelineEntryType::StatusChange => write!(f, "Status Change"),
            TimelineEntryType::ExternalCoordination => write!(f, "External Coordination"),
            TimelineEntryType::Administrative => write!(f, "Administrative"),
