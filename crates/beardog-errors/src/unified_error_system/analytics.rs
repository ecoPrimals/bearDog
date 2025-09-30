//! # Error Analytics Module
//!
//! This module contains error analytics and pattern analysis functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use super::ErrorSeverity;

/// **ERROR ANALYTICS** - Error analytics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalytics {
    /// Error severity level
    pub severity: ErrorSeverity,

    /// Error category
    pub category: String,

    /// Error subcategory
    pub subcategory: Option<String>,

    /// Error frequency information
    pub frequency: ErrorFrequency,

    /// Error pattern information
    pub pattern: ErrorPattern,

    /// Error impact assessment
    pub impact: ErrorImpact,

    /// Error trends
    pub trends: ErrorTrends,

    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// **ERROR FREQUENCY** - Error occurrence frequency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorFrequency {
    /// Total occurrence count
    pub total_count: u64,

    /// Occurrences in the last hour
    pub last_hour: u32,

    /// Occurrences in the last day
    pub last_day: u32,

    /// Occurrences in the last week
    pub last_week: u32,

    /// First occurrence timestamp
    pub first_occurrence: Option<SystemTime>,

    /// Last occurrence timestamp
    pub last_occurrence: SystemTime,

    /// Average time between occurrences
    pub average_interval: Option<Duration>,
}

/// **ERROR PATTERN** - Error pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    /// Pattern type
    pub pattern_type: PatternType,

    /// Pattern confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Pattern description
    pub description: String,

    /// Related error patterns
    pub related_patterns: Vec<String>,

    /// Pattern triggers
    pub triggers: Vec<String>,

    /// Pattern correlation factors
    pub correlations: HashMap<String, f64>,
}

/// **ERROR IMPACT** - Assessment of error impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorImpact {
    /// Business impact level
    pub business_impact: ImpactLevel,

    /// Technical impact level
    pub technical_impact: ImpactLevel,

    /// User impact level
    pub user_impact: ImpactLevel,

    /// Affected systems
    pub affected_systems: Vec<String>,

    /// Affected users count
    pub affected_users: Option<u32>,

    /// Estimated downtime
    pub estimated_downtime: Option<Duration>,

    /// Financial impact estimate
    pub financial_impact: Option<f64>,
}

/// **ERROR TRENDS** - Error trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTrends {
    /// Trend direction
    pub direction: TrendDirection,

    /// Trend strength (0.0 to 1.0)
    pub strength: f64,

    /// Trend duration
    pub duration: Duration,

    /// Predicted next occurrence
    pub predicted_next: Option<SystemTime>,

    /// Seasonal patterns
    pub seasonal: bool,

    /// Trend analysis timestamp
    pub analyzed_at: SystemTime,
}

/// **PATTERN TYPE** - Types of error patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Recurring error at regular intervals
    Recurring,
    /// Burst of errors in short time
    Burst,
    /// Gradual increase in error rate
    Escalating,
    /// Random occurrence pattern
    Random,
    /// Cascade failure pattern
    Cascade,
    /// Threshold breach pattern
    Threshold,
    /// Custom pattern
    Custom(String),
}

/// **IMPACT LEVEL** - Levels of impact assessment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImpactLevel {
    /// No significant impact
    None,
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// **TREND DIRECTION** - Direction of error trends
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Error rate is increasing
    Increasing,
    /// Error rate is decreasing
    Decreasing,
    /// Error rate is stable
    Stable,
    /// Error rate is fluctuating
    Fluctuating,
    /// Insufficient data for trend analysis
    Unknown,
}

impl ErrorAnalytics {
    /// Create new error analytics with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            severity: ErrorSeverity::default(),
            category: "unknown".to_string(),
            subcategory: None,
            frequency: ErrorFrequency::new(),
            pattern: ErrorPattern::new(),
            impact: ErrorImpact::new(),
            trends: ErrorTrends::new(),
            custom_metrics: HashMap::new(),
        }
    }

    /// Create analytics with category
    #[must_use]
    pub fn with_category(category: &str) -> Self {
        Self {
            category: category.to_string(),
            ..Self::new()
        }
    }

    /// Create analytics with severity and category
    #[must_use]
    pub fn with_severity_category(severity: ErrorSeverity, category: &str) -> Self {
        Self {
            severity,
            category: category.to_string(),
            ..Self::new()
        }
    }

    /// Add a custom metric
    pub fn add_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    /// Update error frequency
    pub fn update_frequency(&mut self) {
        self.frequency.total_count += 1;
        self.frequency.last_occurrence = SystemTime::now();

        if self.frequency.first_occurrence.is_none() {
            self.frequency.first_occurrence = Some(SystemTime::now());
        }
    }

    /// Calculate risk score based on analytics
    #[must_use]
    pub fn calculate_risk_score(&self) -> f64 {
        let severity_weight = match self.severity {
            ErrorSeverity::Low => 0.1,
            ErrorSeverity::Medium => 0.3,
            ErrorSeverity::High => 0.7,
            ErrorSeverity::Critical => 1.0,
        };

        let frequency_weight = if self.frequency.total_count > 100 {
            1.0
        } else if self.frequency.total_count > 10 {
            0.7
        } else if self.frequency.total_count > 1 {
            0.3
        } else {
            0.1
        };

        let impact_weight = match self.impact.business_impact {
            ImpactLevel::None => 0.0,
            ImpactLevel::Low => 0.2,
            ImpactLevel::Medium => 0.5,
            ImpactLevel::High => 0.8,
            ImpactLevel::Critical => 1.0,
        };

        (severity_weight * 0.4_f64 + frequency_weight * 0.3_f64 + impact_weight * 0.3_f64)
            .min(1.0_f64)
    }
}

impl ErrorFrequency {
    /// Create new error frequency
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_count: 0,
            last_hour: 0,
            last_day: 0,
            last_week: 0,
            first_occurrence: None,
            last_occurrence: SystemTime::now(),
            average_interval: None,
        }
    }
}

impl ErrorPattern {
    /// Create new error pattern
    #[must_use]
    pub fn new() -> Self {
        Self {
            pattern_type: PatternType::Random,
            confidence: 0.0,
            description: "No pattern detected".to_string(),
            related_patterns: Vec::new(),
            triggers: Vec::new(),
            correlations: HashMap::new(),
        }
    }

    /// Create pattern with type
    #[must_use]
    pub fn with_type(pattern_type: PatternType) -> Self {
        Self {
            pattern_type,
            ..Self::new()
        }
    }
}

impl ErrorImpact {
    /// Create new error impact assessment
    #[must_use]
    pub fn new() -> Self {
        Self {
            business_impact: ImpactLevel::None,
            technical_impact: ImpactLevel::None,
            user_impact: ImpactLevel::None,
            affected_systems: Vec::new(),
            affected_users: None,
            estimated_downtime: None,
            financial_impact: None,
        }
    }

    /// Get overall impact level
    #[must_use]
    pub fn overall_impact(&self) -> ImpactLevel {
        [
            self.business_impact,
            self.technical_impact,
            self.user_impact,
        ]
        .iter()
        .max()
        .copied()
        .unwrap_or(ImpactLevel::None)
    }
}

impl ErrorTrends {
    /// Create new error trends analysis
    #[must_use]
    pub fn new() -> Self {
        Self {
            direction: TrendDirection::Unknown,
            strength: 0.0,
            duration: Duration::from_secs(0),
            predicted_next: None,
            seasonal: false,
            analyzed_at: SystemTime::now(),
        }
    }
}

// Default implementations
impl Default for ErrorAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorFrequency {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorImpact {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorTrends {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PatternType {
    fn default() -> Self {
        Self::Random
    }
}

impl Default for ImpactLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Unknown
    }
}

// Display implementations
impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Recurring => write!(f, "RECURRING"),
            Self::Burst => write!(f, "BURST"),
            Self::Escalating => write!(f, "ESCALATING"),
            Self::Random => write!(f, "RANDOM"),
            Self::Cascade => write!(f, "CASCADE"),
            Self::Threshold => write!(f, "THRESHOLD"),
            Self::Custom(pattern) => write!(f, "CUSTOM({})", pattern),
        }
    }
}

impl std::fmt::Display for ImpactLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Low => write!(f, "LOW"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::High => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl std::fmt::Display for TrendDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increasing => write!(f, "INCREASING"),
            Self::Decreasing => write!(f, "DECREASING"),
            Self::Stable => write!(f, "STABLE"),
            Self::Fluctuating => write!(f, "FLUCTUATING"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}
