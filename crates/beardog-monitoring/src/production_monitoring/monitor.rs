

use super::types::*;
use super::metrics_collector::MetricsCollector;
use super::alert_manager::AlertManager;
use super::health_checker::HealthChecker;
use super::performance_analyzer::PerformanceAnalyzer;
use super::security_monitor::SecurityMonitor;
use super::biome_tracker::BiomeTracker;
use beardog_errors::BearDogError;
use std::sync::Arc;

#[derive(Debug, Clone)]
    alert_manager: Arc<AlertManager>,
    health_checker: Arc<HealthChecker>,
    performance_analyzer: Arc<PerformanceAnalyzer>,
    security_monitor: Arc<SecurityMonitor>,
    biome_tracker: Arc<BiomeTracker>,
    config: MonitoringConfig,
}

impl ProductionMonitor {

/// New operation.
    /// Creates a new instance
    pub fn new(config: MonitoringConfig) -> Self {
        let metrics_collector = Arc::new(MetricsCollector::new());
        let alert_manager = Arc::new(AlertManager::new());
        let health_checker = Arc::new(HealthChecker::new());
        let performance_analyzer = Arc::new(PerformanceAnalyzer::new());
        let security_monitor = Arc::new(SecurityMonitor::new());
        let biome_tracker = Arc::new(BiomeTracker::new());

        Self {
            metrics_collector,
            alert_manager,
            health_checker,
            performance_analyzer,
            security_monitor,
            biome_tracker,
            config,
        }
    }

/// Start Monitoring operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Starts monitoring
    /// Starts monitoring
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {

        self.metrics_collector.start_collection()?;
        self.alert_manager.start_monitoring()?;
        self.health_checker.start_health_checks()?;
        self.performance_analyzer.start_analysis()?;
        self.security_monitor.start_monitoring()?;
        self.biome_tracker.start_tracking()?;

        Ok(())
    }

/// Stop Monitoring operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Stops monitoring
    /// Stops monitoring
    pub fn stop_monitoring(&self) -> Result<(), BearDogError> {

        self.metrics_collector.stop_collection()?;
        self.alert_manager.stop_monitoring()?;
        self.health_checker.stop_health_checks()?;
        self.performance_analyzer.stop_analysis()?;
        self.security_monitor.stop_monitoring()?;
        self.biome_tracker.stop_tracking()?;

        Ok(())
    }

/// Get System Health operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets system_health
    /// Gets system_health
    pub fn get_system_health(&self) -> Result<f64, BearDogError> {
        self.health_checker.get_overall_health_score()
    }

/// Get Metrics Snapshot operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets metrics_snapshot
    /// Gets metrics_snapshot
    pub fn get_metrics_snapshot(&self) -> Result<MetricsSnapshot, BearDogError> {
        self.metrics_collector.get_snapshot()
    }

/// Get Active Alerts operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets active_alerts
    /// Gets active_alerts
    pub fn get_active_alerts(&self) -> Result<Vec<Alert>, BearDogError> {
        self.alert_manager.get_active_alerts()
    }

///
/// # Errors
/// Returns an error if the operation fails.
    pub fn get_performance_analysis(&self) -> Result<Vec<OptimizationSuggestion>, BearDogError> {
        self.performance_analyzer.get_optimization_suggestions()
    }

/// Get Security Events operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets security_events
    /// Gets security_events
    pub fn get_security_events(&self) -> Result<Vec<SecurityEvent>, BearDogError> {
        self.security_monitor.get_recent_events()
    }

/// Get Tracked Biomes operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets tracked_biomes
    /// Gets tracked_biomes
    pub fn get_tracked_biomes(&self) -> Result<Vec<TrackedBiome>, BearDogError> {
        self.biome_tracker.get_all_biomes()
    }
} 
