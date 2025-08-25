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


/// # Privacy Protection Models
///
/// **EXTRACTED FROM LARGE FILE** - Internal data structures (~100 lines)
/// This module contains all the internal data structures and models used
/// by the privacy protection system.

use super::types::{
    ComplianceStatus, IndicatorType, PrivacyImpactLevel, PrivacyProtectionType,
    VulnerabilitySeverity, VulnerabilityType,
};
use beardog_types::canonical::AuditEventType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Internal privacy protection data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyProtectionInternal {
    pub protection_id: String,
    pub protection_type: PrivacyProtectionType,
    pub description: String,
    pub effectiveness_score: f64,
    pub enabled: bool,
    pub auto_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub configuration: HashMap<String, String>,
    pub metrics: ProtectionMetrics,
}
/// Internal privacy vulnerability data structure
pub struct PrivacyVulnerabilityInternal {
    pub vulnerability_id: String,
    pub vulnerability_type: VulnerabilityType,
    pub severity: VulnerabilitySeverity,
    pub detected_at: DateTime<Utc>,
    pub remediation_steps: Vec<String>,
    pub auto_remediated: bool,
    pub resolved: bool,
    pub metadata: HashMap<String, String>,
/// Traffic pattern analysis structure
pub struct TrafficPattern {
    pub pattern_id: String,
    pub source_ip: Option<String>,
    pub destination_pattern: String,
    pub frequency: u64,
    pub data_volume: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub suspicious_score: f64,
    pub indicators: Vec<IndicatorType>,
/// Surveillance detection result
pub struct SurveillanceIndicator {
    pub indicator_id: String,
    pub indicator_type: IndicatorType,
    pub detection_time: DateTime<Utc>,
    pub confidence: f64,
    pub source_data: HashMap<String, String>,
    pub severity: f64,
    pub auto_counter: bool,
/// Privacy audit event structure
pub struct PrivacyAuditEvent {
    pub event_id: String,
    pub event_type: AuditEventType,
    pub timestamp: DateTime<Utc>,
    pub user_action: Option<String>,
    pub data_accessed: Option<String>,
    pub privacy_impact: PrivacyImpactLevel,
    pub compliance_status: ComplianceStatus,
/// Metrics for privacy protection effectiveness
pub struct ProtectionMetrics {
    pub activation_count: u64,
    pub data_protected: u64,
    pub threats_blocked: u64,
    pub false_positive_rate: f64,
    pub effectiveness_trend: Vec<f64>,
