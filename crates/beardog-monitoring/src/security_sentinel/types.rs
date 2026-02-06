//! # Security Sentinel Types
//!
//! This module provides types for security monitoring and threat assessment
//! in the security sentinel system.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================
// Threat Levels
// ============================================================

/// Threat level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// Low threat level
    Low,

    /// Medium threat level
    Medium,

    /// High threat level
    High,

    /// Critical threat level
    Critical,
}

impl Default for ThreatLevel {
    fn default() -> Self {
        Self::Low
    }
}

impl ThreatLevel {
    /// Convert to numeric value
    pub fn to_numeric(&self) -> f64 {
        match self {
            ThreatLevel::Low => 1.0,
            ThreatLevel::Medium => 2.5,
            ThreatLevel::High => 4.0,
            ThreatLevel::Critical => 5.0,
        }
    }

    /// Create from numeric value
    pub fn from_numeric(value: f64) -> Self {
        match value {
            v if v >= 4.5 => ThreatLevel::Critical,
            v if v >= 3.0 => ThreatLevel::High,
            v if v >= 2.0 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,
        }
    }

    /// Check if requires immediate attention
    pub fn requires_immediate_attention(&self) -> bool {
        matches!(self, ThreatLevel::High | ThreatLevel::Critical)
    }
}

// ============================================================
// Capability Status
// ============================================================

/// Capability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatus {
    /// Capability name
    pub name: String,

    /// Whether operational
    pub operational: bool,

    /// Performance score (0.0 - 1.0)
    pub performance_score: f64,

    /// Last check timestamp
    pub last_check: DateTime<Utc>,

    /// Issues detected
    pub issues: Vec<String>,
}

impl Default for CapabilityStatus {
    fn default() -> Self {
        Self {
            name: String::new(),
            operational: true,
            performance_score: 1.0,
            last_check: Utc::now(),
            issues: Vec::new(),
        }
    }
}

// ============================================================
// Health Reports
// ============================================================

/// Capabilities health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesHealthReport {
    /// Overall readiness score (0.0 - 1.0)
    pub overall_readiness: f64,

    /// Individual capability statuses
    pub capabilities: Vec<CapabilityStatus>,

    /// Assessment timestamp
    pub assessment_time: DateTime<Utc>,

    /// Recommendations
    pub recommendations: Vec<String>,
}

impl Default for CapabilitiesHealthReport {
    fn default() -> Self {
        Self {
            overall_readiness: 1.0,
            capabilities: Vec::new(),
            assessment_time: Utc::now(),
            recommendations: Vec::new(),
        }
    }
}

// ============================================================
// Autonomy and Sovereignty
// ============================================================

/// Autonomy indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyIndicator {
    /// Indicator name
    pub indicator_name: String,

    /// Score (0.0 - 1.0)
    pub score: f64,

    /// Description
    pub description: String,

    /// Measurement timestamp
    pub measured_at: DateTime<Utc>,
}

impl Default for AutonomyIndicator {
    fn default() -> Self {
        Self {
            indicator_name: String::new(),
            score: 1.0,
            description: String::new(),
            measured_at: Utc::now(),
        }
    }
}

/// Human dignity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanDignityMetrics {
    /// Privacy protection score (0.0 - 1.0)
    pub privacy_protection_score: f64,

    /// Consent compliance score (0.0 - 1.0)
    pub consent_compliance_score: f64,

    /// Surveillance resistance score (0.0 - 1.0)
    pub surveillance_resistance_score: f64,

    /// User empowerment score (0.0 - 1.0)
    pub user_empowerment_score: f64,
}

impl Default for HumanDignityMetrics {
    fn default() -> Self {
        Self {
            privacy_protection_score: 1.0,
            consent_compliance_score: 1.0,
            surveillance_resistance_score: 1.0,
            user_empowerment_score: 1.0,
        }
    }
}

impl HumanDignityMetrics {
    /// Calculate overall score
    pub fn overall_score(&self) -> f64 {
        (self.privacy_protection_score
            + self.consent_compliance_score
            + self.surveillance_resistance_score
            + self.user_empowerment_score)
            / 4.0
    }
}

/// Sovereignty status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyStatusReport {
    /// Overall sovereignty score (0.0 - 1.0)
    pub sovereignty_score: f64,

    /// Autonomy indicators
    pub autonomy_indicators: Vec<AutonomyIndicator>,

    /// Human dignity metrics
    pub human_dignity_metrics: HumanDignityMetrics,

    /// Independence score (0.0 - 1.0)
    pub independence_score: f64,
}

impl Default for SovereigntyStatusReport {
    fn default() -> Self {
        Self {
            sovereignty_score: 1.0,
            autonomy_indicators: Vec::new(),
            human_dignity_metrics: HumanDignityMetrics::default(),
            independence_score: 1.0,
        }
    }
}

// ============================================================
// Threat Analysis
// ============================================================

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,

    /// Decreasing trend
    Decreasing,

    /// Stable trend
    Stable,

    /// Volatile trend
    Volatile,
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Stable
    }
}

/// Threat trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    /// Trend type
    pub trend_type: String,

    /// Direction
    pub direction: TrendDirection,

    /// Confidence (0.0 - 1.0)
    pub confidence: f64,

    /// Timeframe in hours
    pub timeframe_hours: u64,
}

impl Default for ThreatTrend {
    fn default() -> Self {
        Self {
            trend_type: String::new(),
            direction: TrendDirection::Stable,
            confidence: 1.0,
            timeframe_hours: 24,
        }
    }
}

/// Threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator ID
    pub id: String,

    /// Indicator type
    pub indicator_type: String,

    /// Severity (0.0 - 1.0)
    pub severity: f64,

    /// Description
    pub description: String,

    /// Detection timestamp
    pub detected_at: DateTime<Utc>,

    /// Associated metadata
    pub metadata: HashMap<String, String>,
}

impl Default for ThreatIndicator {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            indicator_type: String::new(),
            severity: 0.0,
            description: String::new(),
            detected_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

/// Threat landscape report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatLandscapeReport {
    /// Current threat level
    pub current_threat_level: ThreatLevel,

    /// Active threats count
    pub active_threats: u32,

    /// Threat trends
    pub threat_trends: Vec<ThreatTrend>,

    /// High priority indicators
    pub high_priority_indicators: Vec<ThreatIndicator>,
}

impl Default for ThreatLandscapeReport {
    fn default() -> Self {
        Self {
            current_threat_level: ThreatLevel::Low,
            active_threats: 0,
            threat_trends: Vec::new(),
            high_priority_indicators: Vec::new(),
        }
    }
}

// ============================================================
// Configuration (re-exported from canonical)
// ============================================================

// Note: SecuritySentinelConfig is defined in beardog_types::canonical::configuration
// and re-exported from there for consistency across the codebase.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_level_numeric_conversion() {
        assert_eq!(ThreatLevel::Low.to_numeric(), 1.0);
        assert_eq!(ThreatLevel::Critical.to_numeric(), 5.0);
        assert_eq!(ThreatLevel::from_numeric(4.5), ThreatLevel::Critical);
        assert_eq!(ThreatLevel::from_numeric(1.0), ThreatLevel::Low);
    }

    #[test]
    fn test_threat_level_immediate_attention() {
        assert!(!ThreatLevel::Low.requires_immediate_attention());
        assert!(!ThreatLevel::Medium.requires_immediate_attention());
        assert!(ThreatLevel::High.requires_immediate_attention());
        assert!(ThreatLevel::Critical.requires_immediate_attention());
    }

    #[test]
    fn test_human_dignity_score() {
        let metrics = HumanDignityMetrics::default();
        assert_eq!(metrics.overall_score(), 1.0);
    }

    #[test]
    fn test_capability_status_default() {
        let status = CapabilityStatus::default();
        assert!(status.operational);
        assert_eq!(status.performance_score, 1.0);
    }

    #[test]
    fn test_threat_landscape_default() {
        let report = ThreatLandscapeReport::default();
        assert_eq!(report.current_threat_level, ThreatLevel::Low);
        assert_eq!(report.active_threats, 0);
    }
}
