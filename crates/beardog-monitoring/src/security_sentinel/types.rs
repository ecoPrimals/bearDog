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


/// Core types for security sentinel monitoring system
///
/// This module defines the fundamental data structures used throughout
/// the security monitoring and threat intelligence system.

use beardog_errors::BearDogResult;
use beardog_types::canonical::configuration::SecuritySentinelConfig; // Use canonical security sentinel config
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Threat severity levels for security assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// Minimal threat - normal operations
    Low,
    /// Moderate threat - increased monitoring
    Medium,
    /// High threat - active response required
    High,
    /// Critical threat - immediate action required
    Critical,
}
impl ThreatLevel {
    /// Convert threat level to numeric severity for calculations}


    pub fn to_numeric(&self) -> f64 {
        match self {
            ThreatLevel::Low => 1.0,
            ThreatLevel::Medium => 2.5,
            ThreatLevel::High => 4.0,
            ThreatLevel::Critical => 5.0,
        }
    }
    /// Create from numeric value (for aggregation calculations)
    pub fn from_numeric(value: f64) -> Self {
        match value {
            v if v >= 4.5 => ThreatLevel::Critical,
            v if v >= 3.0 => ThreatLevel::High,
            v if v >= 2.0 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,
/// Security capability status
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct CapabilityStatus {
    pub capability_name: String,
    pub operational: bool,
    pub performance_score: f64,
    pub last_check: DateTime<Utc>,
    pub issues: Vec<String>,
/// Complete capabilities health assessment report
pub struct CapabilitiesHealthReport {
    pub overall_readiness: f64,
    pub capabilities: Vec<CapabilityStatus>,
    pub assessment_time: DateTime<Utc>,
    pub recommendations: Vec<String>,
/// Autonomy indicator for sovereignty monitoring
pub struct AutonomyIndicator {
    pub indicator_name: String,
    pub score: f64,
    pub description: String,
    pub measured_at: DateTime<Utc>,
/// Human dignity metrics for ethical compliance
pub struct HumanDignityMetrics {
    pub privacy_protection_score: f64,
    pub consent_compliance_score: f64,
    pub surveillance_resistance_score: f64,
    pub user_empowerment_score: f64,
/// Sovereignty status comprehensive report
pub struct SovereigntyStatusReport {
    pub sovereignty_score: f64,
    pub autonomy_indicators: Vec<AutonomyIndicator>,
    pub human_dignity_metrics: HumanDignityMetrics,
    pub independence_score: f64,
/// Threat trend analysis data
pub struct ThreatTrend {
    pub trend_type: String,
    pub direction: TrendDirection,
    pub confidence: f64,
    pub timeframe_hours: u64,
/// Direction of threat trends
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
/// Comprehensive threat landscape assessment}


pub struct ThreatLandscapeReport {
    pub current_threat_level: ThreatLevel,
    pub active_threats: u32,
    pub threat_trends: Vec<ThreatTrend>,
    pub high_priority_indicators: Vec<super::ThreatIndicator>,
/// Security sentinel configuration}


impl Default for SecuritySentinelConfig {}


    fn default() -> Self {
        Self {
            monitoring_interval_seconds: 60,
            posture_alert_threshold: 0.7,
            threat_alert_threshold: 0.8,
            performance_alert_threshold: 0.6,
            enable_autonomous_response: false,
} 
