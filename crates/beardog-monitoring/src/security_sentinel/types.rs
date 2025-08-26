

use beardog_errors::BearDogResult;
use beardog_types::canonical::configuration::SecuritySentinelConfig; // Use canonical security sentinel config
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {

    Low,

    Medium,

    High,

    Critical,
}
impl ThreatLevel {

    pub fn to_numeric(&self) -> f64 {
        match self {
            ThreatLevel::Low => 1.0,
            ThreatLevel::Medium => 2.5,
            ThreatLevel::High => 4.0,
            ThreatLevel::Critical => 5.0,
        }
    }

    pub fn from_numeric(value: f64) -> Self {
        match value {
            v if v >= 4.5 => ThreatLevel::Critical,
            v if v >= 3.0 => ThreatLevel::High,
            v if v >= 2.0 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub struct CapabilityStatus {
    pub capability_name: String,
    pub operational: bool,
    pub performance_score: f64,
    pub last_check: DateTime<Utc>,
    pub issues: Vec<String>,

pub struct CapabilitiesHealthReport {
    pub overall_readiness: f64,
    pub capabilities: Vec<CapabilityStatus>,
    pub assessment_time: DateTime<Utc>,
    pub recommendations: Vec<String>,

pub struct AutonomyIndicator {
    pub indicator_name: String,
    pub score: f64,
    pub description: String,
    pub measured_at: DateTime<Utc>,

pub struct HumanDignityMetrics {
    pub privacy_protection_score: f64,
    pub consent_compliance_score: f64,
    pub surveillance_resistance_score: f64,
    pub user_empowerment_score: f64,

pub struct SovereigntyStatusReport {
    pub sovereignty_score: f64,
    pub autonomy_indicators: Vec<AutonomyIndicator>,
    pub human_dignity_metrics: HumanDignityMetrics,
    pub independence_score: f64,

pub struct ThreatTrend {
    pub trend_type: String,
    pub direction: TrendDirection,
    pub confidence: f64,
    pub timeframe_hours: u64,

pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,

pub struct ThreatLandscapeReport {
    pub current_threat_level: ThreatLevel,
    pub active_threats: u32,
    pub threat_trends: Vec<ThreatTrend>,
    pub high_priority_indicators: Vec<super::ThreatIndicator>,

impl Default for SecuritySentinelConfig {}

    fn default() -> Self {
        Self {
            monitoring_interval_seconds: 60,
            posture_alert_threshold: 0.7,
            threat_alert_threshold: 0.8,
            performance_alert_threshold: 0.6,
            enable_autonomous_response: false,
} 
