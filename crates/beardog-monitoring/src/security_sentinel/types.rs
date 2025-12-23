

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::SecuritySentinelConfig; // Use canonical security sentinel config
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,
}
impl ThreatLevel {

/// To Numeric operation.
    /// Converts to numeric
    pub fn to_numeric(&self) -> f64 {
        match self {
            ThreatLevel::Low => 1.0,
            ThreatLevel::Medium => 2.5,
            ThreatLevel::High => 4.0,
            ThreatLevel::Critical => 5.0,
        }
    }

/// From Numeric operation.
    /// Creates instance from numeric
    pub fn from_numeric(value: f64) -> Self {
        match value {
            v if v >= 4.5 => ThreatLevel::Critical,
            v if v >= 3.0 => ThreatLevel::High,
            v if v >= 2.0 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,

#[derive(Debug, Clone)]
    /// Whether operational is enabled
    pub operational: bool,
    pub performance_score: f64,
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// Collection of issues
    pub issues: Vec<String>,

pub struct CapabilitiesHealthReport {
    /// The overall readiness value
    pub overall_readiness: f64,
    /// Collection of capabilities
    pub capabilities: Vec<CapabilityStatus>,
    pub assessment_time: DateTime<Utc>,
    /// Collection of recommendations
    pub recommendations: Vec<String>,

pub struct AutonomyIndicator {
    /// Name of the indicator
    pub indicator_name: String,
    /// The score value
    pub score: f64,
    /// The description value
    pub description: String,
    /// The measured at value
    pub measured_at: DateTime<Utc>,

pub struct HumanDignityMetrics {
    /// The privacy protection score value
    pub privacy_protection_score: f64,
    /// The consent compliance score value
    pub consent_compliance_score: f64,
    /// The surveillance resistance score value
    pub surveillance_resistance_score: f64,
    /// The user empowerment score value
    pub user_empowerment_score: f64,

pub struct SovereigntyStatusReport {
    /// The sovereignty score value
    pub sovereignty_score: f64,
    /// Collection of autonomy indicators
    pub autonomy_indicators: Vec<AutonomyIndicator>,
    /// The human dignity metrics value
    pub human_dignity_metrics: HumanDignityMetrics,
    /// The independence score value
    pub independence_score: f64,

pub struct ThreatTrend {
    /// The trend type value
    pub trend_type: String,
    /// The direction value
    pub direction: TrendDirection,
    pub confidence: f64,
    pub timeframe_hours: u64,

pub enum TrendDirection {
    /// Currently increasing
    Increasing,
    /// Currently decreasing
    Decreasing,
    /// Represents stable variant
    Stable,
    /// Represents volatile variant
    Volatile,

pub struct ThreatLandscapeReport {
    /// The current threat level value
    pub current_threat_level: ThreatLevel,
    /// Number of active_threats
    pub active_threats: u32,
    /// Collection of threat trends
    pub threat_trends: Vec<ThreatTrend>,
    /// Collection of high priority indicators
    pub high_priority_indicators: Vec<super::ThreatIndicator>,

impl Default for SecuritySentinelConfig {}
impl Default for SecuritySentinelConfig {}
impl Default for SecuritySentinelConfig {}

    fn default(60,
            posture_alert_threshold: 0.7,
            threat_alert_threshold: 0.8,
            performance_alert_threshold: 0.6,
            enable_autonomous_response: false,
} 
