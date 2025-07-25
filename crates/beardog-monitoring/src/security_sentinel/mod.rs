//! Security Sentinel Monitoring System
//!
//! **BearDog is a SENTINEL, not surveillance**
//!
//! This module provides security-focused monitoring that maintains self-awareness
//! of BearDog's security posture, capabilities, and protective effectiveness.
//!
//! ## Core Principles
//! - **Self-Awareness**: Monitor our own security capabilities and readiness
//! - **Protective Intelligence**: Understand threats to better protect humans
//! - **Internal Focus**: Watch our systems, never surveill users
//! - **Human Dignity**: All monitoring serves human empowerment, not control
//!
//! ## Security Sentinel Areas
//! - **Posture Assessment**: Current security stance and capability health
//! - **Threat Landscape**: Environmental awareness for better protection
//! - **Capability Monitoring**: Effectiveness of security tools and systems
//! - **Performance Sentinel**: Security function performance and reliability
//! - **Sovereignty Health**: Autonomy and independence maintenance

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

pub mod capability_monitor;
pub mod performance_sentinel;
pub mod posture;
pub mod sovereignty_health;
pub mod threat_landscape;

#[cfg(test)]
mod tests;

// Re-export main types
pub use capability_monitor::SecurityCapabilityMonitor;
pub use performance_sentinel::{AlertManager, PerformanceSentinel, PerformanceThresholds, PerformanceTrends};
pub use posture::SecurityPostureMonitor;
pub use sovereignty_health::SovereigntyHealthMonitor;
pub use threat_landscape::ThreatLandscapeIntelligence;

/// Main Security Sentinel - Central monitoring hub
pub struct SecuritySentinel {
    /// Security posture monitoring
    posture_monitor: Arc<SecurityPostureMonitor>,
    /// Threat landscape intelligence
    threat_intelligence: Arc<ThreatLandscapeIntelligence>,
    /// Security capability monitoring  
    capability_monitor: Arc<SecurityCapabilityMonitor>,
    /// Performance monitoring for security functions
    performance_sentinel: Arc<PerformanceSentinel>,
    /// Sovereignty and autonomy health monitoring
    sovereignty_monitor: Arc<SovereigntyHealthMonitor>,
    /// Sentinel configuration
    config: SecuritySentinelConfig,
    /// Sentinel statistics
    stats: SecuritySentinelStats,
}

/// Configuration for security sentinel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySentinelConfig {
    /// Enable security sentinel monitoring
    pub enabled: bool,
    /// Monitoring interval in seconds  
    pub monitoring_interval_secs: u64,
    /// Enable threat landscape intelligence gathering
    pub enable_threat_intelligence: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
    /// Enable sovereignty health checks
    pub enable_sovereignty_monitoring: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Data retention policy
    pub data_retention_hours: u64,
}

impl Default for SecuritySentinelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitoring_interval_secs: 30, // Every 30 seconds
            enable_threat_intelligence: true,
            enable_performance_monitoring: true,
            enable_sovereignty_monitoring: true,
            alert_thresholds: AlertThresholds::default(),
            data_retention_hours: 24, // 24 hours of data
        }
    }
}

/// Alert thresholds for security monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Minimum security posture score (0.0-1.0)
    pub min_security_posture_score: f64,
    /// Maximum acceptable threat exposure level
    pub max_threat_exposure_level: ThreatLevel,
    /// Minimum capability health score (0.0-1.0)  
    pub min_capability_health_score: f64,
    /// Maximum response time for security functions (ms)
    pub max_security_response_time_ms: u64,
    /// Minimum sovereignty score (0.0-1.0)
    pub min_sovereignty_score: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            min_security_posture_score: 0.7,
            max_threat_exposure_level: ThreatLevel::Medium,
            min_capability_health_score: 0.8,
            max_security_response_time_ms: 1000,
            min_sovereignty_score: 0.9,
        }
    }
}

/// Threat level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Security sentinel statistics
#[derive(Debug, Default)]
pub struct SecuritySentinelStats {
    /// Total security assessments performed
    pub security_assessments: AtomicU64,
    /// Threats identified and analyzed
    pub threats_analyzed: AtomicU64,
    /// Security capabilities checked
    pub capabilities_monitored: AtomicU64,
    /// Performance evaluations completed
    pub performance_checks: AtomicU64,
    /// Sovereignty health checks performed
    pub sovereignty_checks: AtomicU64,
    /// Alerts generated
    pub alerts_generated: AtomicU64,
    /// Current monitoring status
    pub monitoring_active: AtomicBool,
    /// Last full assessment time
    pub last_assessment: Arc<RwLock<Option<DateTime<Utc>>>>,
}

/// Comprehensive security status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatusReport {
    /// Timestamp of the report
    pub timestamp: DateTime<Utc>,
    /// Overall security posture score (0.0-1.0)
    pub overall_security_score: f64,
    /// Current threat landscape assessment
    pub threat_landscape: ThreatLandscapeReport,
    /// Security capabilities health
    pub capabilities_health: CapabilitiesHealthReport,
    /// Performance of security functions
    pub performance_metrics: SecurityPerformanceMetrics,
    /// Sovereignty and autonomy status
    pub sovereignty_status: SovereigntyStatusReport,
    /// Recommended actions
    pub recommendations: Vec<SecurityRecommendation>,
    /// Current alert status
    pub alert_status: AlertStatus,
}

/// Threat landscape report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatLandscapeReport {
    /// Current overall threat level
    pub threat_level: ThreatLevel,
    /// Specific threats identified
    pub active_threats: Vec<ThreatIndicator>,
    /// Threat trends over time
    pub threat_trends: Vec<ThreatTrend>,
    /// Intelligence confidence level
    pub intelligence_confidence: f64,
}

/// Security capability health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesHealthReport {
    /// Overall capability health score
    pub overall_health_score: f64,
    /// Individual capability statuses
    pub capability_statuses: Vec<CapabilityStatus>,
    /// Degraded or failed capabilities
    pub degraded_capabilities: Vec<String>,
    /// Recommended capability improvements
    pub improvement_recommendations: Vec<String>,
}

/// Security performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPerformanceMetrics {
    /// Average response time for security functions (ms)
    pub avg_response_time_ms: f64,
    /// Security operations per second
    pub security_ops_per_sec: f64,
    /// Error rate for security functions
    pub security_error_rate: f64,
    /// Resource utilization for security processes
    pub security_resource_usage: ResourceUsage,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage by security processes (percentage)
    pub cpu_usage_percent: f64,
    /// Memory usage by security processes (bytes)
    pub memory_usage_bytes: u64,
    /// Network usage for security communications (bytes/sec)
    pub network_usage_bytes_per_sec: u64,
}

/// Sovereignty status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyStatusReport {
    /// Overall sovereignty health score (0.0-1.0)
    pub sovereignty_score: f64,
    /// Autonomy indicators
    pub autonomy_indicators: Vec<AutonomyIndicator>,
    /// Human dignity preservation metrics
    pub human_dignity_metrics: HumanDignityMetrics,
    /// Independence from external dependencies
    pub independence_score: f64,
}

/// Security recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    /// Recommendation priority
    pub priority: RecommendationPriority,
    /// Recommendation category
    pub category: String,
    /// Recommendation description
    pub description: String,
    /// Estimated impact of implementing recommendation
    pub impact: String,
    /// Estimated effort to implement
    pub effort: String,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Additional supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub threat_type: String,
    pub severity: ThreatLevel,
    pub confidence: f64,
    pub detected_at: DateTime<Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    pub threat_type: String,
    pub trend: String, // "increasing", "decreasing", "stable"
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatus {
    pub capability_name: String,
    pub health_score: f64,
    pub status: String, // "healthy", "degraded", "failed"
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyIndicator {
    pub indicator_name: String,
    pub score: f64,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanDignityMetrics {
    pub privacy_protection_score: f64,
    pub consent_compliance_score: f64,
    pub surveillance_resistance_score: f64,
    pub user_empowerment_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStatus {
    pub active_alerts: u32,
    pub critical_alerts: u32,
    pub last_alert_time: Option<DateTime<Utc>>,
    pub alert_trends: String,
}

impl SecuritySentinel {
    /// Create a new security sentinel with default configuration
    pub fn new() -> Self {
        Self::with_config(SecuritySentinelConfig::default())
    }

    /// Create security sentinel with custom configuration  
    pub fn with_config(config: SecuritySentinelConfig) -> Self {
        info!("🛡️ Initializing Security Sentinel - Protector of Human Dignity");

        let posture_monitor = Arc::new(SecurityPostureMonitor::new());
        let threat_intelligence = Arc::new(ThreatLandscapeIntelligence::new());
        let capability_monitor = Arc::new(SecurityCapabilityMonitor::new());
        let performance_thresholds = PerformanceThresholds::default();
        let alert_manager = Arc::new(AlertManager::new());
        let performance_sentinel = Arc::new(PerformanceSentinel::new(
            performance_thresholds,
            alert_manager,
        ).expect("Failed to create PerformanceSentinel")); // Convert ? to expect for constructor
        let sovereignty_monitor = Arc::new(SovereigntyHealthMonitor::new());

        Self {
            posture_monitor,
            threat_intelligence,
            capability_monitor,
            performance_sentinel,
            sovereignty_monitor,
            config,
            stats: SecuritySentinelStats::default(),
        }
    }

    /// Start the security sentinel monitoring
    pub async fn start_monitoring(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enabled {
            info!("🛡️ Security Sentinel disabled in configuration");
            return Ok(());
        }

        info!("🚀 Starting Security Sentinel - Guardian Mode Activated");
        self.stats.monitoring_active.store(true, Ordering::Relaxed);

        // Start monitoring loop
        let sentinel = self.clone();
        tokio::spawn(async move {
            sentinel.monitoring_loop().await;
        });

        Ok(())
    }

    /// Main monitoring loop - the heart of the sentinel
    async fn monitoring_loop(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(
            self.config.monitoring_interval_secs,
        ));

        info!("🔍 Security Sentinel monitoring loop started - Watching over security");

        loop {
            interval.tick().await;

            if let Err(e) = self.perform_security_assessment().await {
                error!("🚨 Security assessment failed: {}", e);
                continue;
            }

            // Update last assessment time
            {
                let mut last_assessment = self.stats.last_assessment.write().await;
                *last_assessment = Some(Utc::now());
            }
        }
    }

    /// Perform comprehensive security assessment
    pub async fn perform_security_assessment(
        &self,
    ) -> Result<SecurityStatusReport, Box<dyn std::error::Error + Send + Sync>> {
        self.stats
            .security_assessments
            .fetch_add(1, Ordering::Relaxed);

        let timestamp = Utc::now();

        // Gather intelligence from all monitoring components
        let threat_landscape = self.threat_intelligence.assess_threat_landscape().await;
        let capabilities_health = self.capability_monitor.assess_capabilities().await;
        let performance_trends = self.performance_sentinel.get_performance_trends(30).await?;
        let sovereignty_status = self.sovereignty_monitor.assess_sovereignty().await;

        // Convert performance trends to security metrics
        let security_performance_metrics = self.convert_trends_to_security_metrics(&performance_trends);

        // Calculate overall security score
        let overall_security_score = self
            .calculate_overall_security_score(
                &threat_landscape,
                &capabilities_health,
                &security_performance_metrics,
                &sovereignty_status,
            )
            .await;

        // Generate recommendations
        let recommendations = self
            .generate_security_recommendations(
                &threat_landscape,
                &capabilities_health,
                &security_performance_metrics,
                &sovereignty_status,
            )
            .await;

        // Check alert conditions
        let alert_status = self
            .evaluate_alert_conditions(&capabilities_health, &security_performance_metrics)
            .await;

        let report = SecurityStatusReport {
            timestamp,
            overall_security_score,
            threat_landscape,
            capabilities_health,
            performance_metrics: security_performance_metrics,
            sovereignty_status,
            recommendations,
            alert_status,
        };

        // Log security status
        match overall_security_score {
            s if s >= 0.9 => info!("🛡️ Security Status: EXCELLENT (Score: {:.2})", s),
            s if s >= 0.7 => info!("⚠️  Security Status: GOOD (Score: {:.2})", s),
            s if s >= 0.5 => warn!("⚠️  Security Status: DEGRADED (Score: {:.2})", s),
            s => error!("🚨 Security Status: CRITICAL (Score: {:.2})", s),
        }

        Ok(report)
    }

    /// Convert PerformanceTrends to SecurityPerformanceMetrics
    fn convert_trends_to_security_metrics(&self, trends: &PerformanceTrends) -> SecurityPerformanceMetrics {
        SecurityPerformanceMetrics {
            avg_response_time_ms: trends.avg_latency_ms as f64,
            security_ops_per_sec: if trends.sample_count > 0 { 
                trends.sample_count as f64 / (trends.window_minutes as f64 * 60.0) 
            } else { 0.0 },
            security_error_rate: 0.0, // Not available from trends, conservative default
            security_resource_usage: ResourceUsage {
                cpu_usage_percent: trends.avg_cpu_percent,
                memory_usage_bytes: (trends.avg_memory_mb * 1024.0 * 1024.0) as u64,
                network_usage_bytes_per_sec: 0, // Not available from trends
            },
        }
    }

    /// Calculate overall security score from all components
    async fn calculate_overall_security_score(
        &self,
        threat_landscape: &ThreatLandscapeReport,
        capabilities_health: &CapabilitiesHealthReport,
        performance_metrics: &SecurityPerformanceMetrics,
        sovereignty_status: &SovereigntyStatusReport,
    ) -> f64 {
        // Weighted scoring: Capabilities(40%) + Sovereignty(30%) + Performance(20%) + Threats(10%)
        let threat_score = match threat_landscape.threat_level {
            ThreatLevel::Low => 1.0,
            ThreatLevel::Medium => 0.7,
            ThreatLevel::High => 0.4,
            ThreatLevel::Critical => 0.1,
        };

        let performance_score = if performance_metrics.avg_response_time_ms <= 100.0 {
            1.0
        } else if performance_metrics.avg_response_time_ms <= 500.0 {
            0.8
        } else if performance_metrics.avg_response_time_ms <= 1000.0 {
            0.6
        } else {
            0.3
        };

        (capabilities_health.overall_health_score * 0.4)
            + (sovereignty_status.sovereignty_score * 0.3)
            + (performance_score * 0.2)
            + (threat_score * 0.1)
    }

    /// Generate security recommendations based on current state
    async fn generate_security_recommendations(
        &self,
        _threat_landscape: &ThreatLandscapeReport,
        capabilities_health: &CapabilitiesHealthReport,
        performance_metrics: &SecurityPerformanceMetrics,
        sovereignty_status: &SovereigntyStatusReport,
    ) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        // Capability recommendations
        if capabilities_health.overall_health_score < 0.7 {
            recommendations.push(SecurityRecommendation {
                priority: RecommendationPriority::High,
                category: "Capabilities".to_string(),
                description:
                    "Some security capabilities are degraded. Investigate failed components."
                        .to_string(),
                impact: "Improved security resilience".to_string(),
                effort: "Medium".to_string(),
            });
        }

        // Performance recommendations
        if performance_metrics.avg_response_time_ms > 500.0 {
            recommendations.push(SecurityRecommendation {
                priority: RecommendationPriority::Medium,
                category: "Performance".to_string(),
                description: "Security response times are elevated. Consider optimization."
                    .to_string(),
                impact: "Faster threat response".to_string(),
                effort: "Medium".to_string(),
            });
        }

        // Sovereignty recommendations
        if sovereignty_status.sovereignty_score < 0.8 {
            recommendations.push(SecurityRecommendation {
                priority: RecommendationPriority::Critical,
                category: "Sovereignty".to_string(),
                description: "Human dignity or autonomy metrics are below threshold.".to_string(),
                impact: "Enhanced human empowerment".to_string(),
                effort: "High".to_string(),
            });
        }

        recommendations
    }

    /// Evaluate alert conditions
    async fn evaluate_alert_conditions(
        &self,
        capabilities_health: &CapabilitiesHealthReport,
        performance_metrics: &SecurityPerformanceMetrics,
    ) -> AlertStatus {
        let mut active_alerts = 0;
        let mut critical_alerts = 0;

        if capabilities_health.overall_health_score
            < self.config.alert_thresholds.min_capability_health_score
        {
            active_alerts += 1;
            if capabilities_health.overall_health_score < 0.5 {
                critical_alerts += 1;
            }
        }

        if performance_metrics.avg_response_time_ms
            > self.config.alert_thresholds.max_security_response_time_ms as f64
        {
            active_alerts += 1;
            if performance_metrics.avg_response_time_ms > 5000.0 {
                critical_alerts += 1;
            }
        }

        if active_alerts > 0 {
            self.stats.alerts_generated.fetch_add(1, Ordering::Relaxed);
        }

        AlertStatus {
            active_alerts,
            critical_alerts,
            last_alert_time: if active_alerts > 0 {
                Some(Utc::now())
            } else {
                None
            },
            alert_trends: "stable".to_string(), // Could be calculated from history
        }
    }

    /// Get current security sentinel statistics
    pub async fn get_sentinel_stats(&self) -> SecuritySentinelStats {
        SecuritySentinelStats {
            security_assessments: AtomicU64::new(
                self.stats.security_assessments.load(Ordering::Relaxed),
            ),
            threats_analyzed: AtomicU64::new(self.stats.threats_analyzed.load(Ordering::Relaxed)),
            capabilities_monitored: AtomicU64::new(
                self.stats.capabilities_monitored.load(Ordering::Relaxed),
            ),
            performance_checks: AtomicU64::new(
                self.stats.performance_checks.load(Ordering::Relaxed),
            ),
            sovereignty_checks: AtomicU64::new(
                self.stats.sovereignty_checks.load(Ordering::Relaxed),
            ),
            alerts_generated: AtomicU64::new(self.stats.alerts_generated.load(Ordering::Relaxed)),
            monitoring_active: AtomicBool::new(
                self.stats.monitoring_active.load(Ordering::Relaxed),
            ),
            last_assessment: Arc::new(RwLock::new(*self.stats.last_assessment.read().await)),
        }
    }

    /// Stop security sentinel monitoring
    pub async fn stop_monitoring(&self) {
        info!("🛑 Stopping Security Sentinel monitoring");
        self.stats.monitoring_active.store(false, Ordering::Relaxed);
    }
}

// Implement Clone for SecuritySentinel to enable tokio::spawn
impl Clone for SecuritySentinel {
    fn clone(&self) -> Self {
        Self {
            posture_monitor: Arc::clone(&self.posture_monitor),
            threat_intelligence: Arc::clone(&self.threat_intelligence),
            capability_monitor: Arc::clone(&self.capability_monitor),
            performance_sentinel: Arc::clone(&self.performance_sentinel),
            sovereignty_monitor: Arc::clone(&self.sovereignty_monitor),
            config: self.config.clone(),
            stats: SecuritySentinelStats::default(), // Fresh stats for clone
        }
    }
}

impl Default for SecuritySentinel {
    fn default() -> Self {
        Self::new()
    }
}
