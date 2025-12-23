

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod metrics_collector;
pub mod analytics_processor;
pub mod predictive_engine;
pub mod alert_manager;
pub mod trace_collector;
pub mod autonomous_healer;
pub mod dashboard;

pub use metrics_collector::*;
pub use analytics_processor::*;
pub use predictive_engine::*;
pub use alert_manager::*;
pub use trace_collector::*;
pub use autonomous_healer::*;
pub use dashboard::*;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub struct AdvancedObservabilityEngine {

    /// The metric collector value
    pub metric_collector: Arc<RealTimeMetricCollector>,

    /// The analytics processor value
    pub analytics_processor: Arc<AnalyticsProcessor>,

    /// The predictive engine value
    pub predictive_engine: Arc<PredictiveMaintenanceEngine>,

    /// The alert manager value
    pub alert_manager: Arc<IntelligentAlertManager>,

    /// The trace collector value
    pub trace_collector: Arc<DistributedTraceCollector>,

    /// The autonomous healer value
    pub autonomous_healer: Arc<AutonomousHealingSystem>,

    /// The dashboard engine value
    pub dashboard_engine: Arc<RealTimeDashboard>,
}

impl AdvancedObservabilityEngine {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            metric_collector: Arc::new(RealTimeMetricCollector::new()?),
            analytics_processor: Arc::new(AnalyticsProcessor::new()?),
            predictive_engine: Arc::new(PredictiveMaintenanceEngine::new()?),
            alert_manager: Arc::new(IntelligentAlertManager::new()?),
            trace_collector: Arc::new(DistributedTraceCollector::new()?),
            autonomous_healer: Arc::new(AutonomousHealingSystem::new()?),
            dashboard_engine: Arc::new(RealTimeDashboard::new()?),
        })
    }

/// Initialize operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&self) -> Result<(), BearDogError> {

        let tasks = vec![
            self.metric_collector.initialize(),
            self.analytics_processor.initialize(), 
            self.predictive_engine.initialize(),
            self.alert_manager.initialize(),
            self.trace_collector.initialize(),
            self.autonomous_healer.initialize(),
            self.dashboard_engine.initialize(),
        ];

        for task in tasks {
            task?;
        }

        Ok(())
    }

/// Start operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {

        self.metric_collector.start()?;
        self.analytics_processor.start()?;
        self.predictive_engine.start()?;
        self.alert_manager.start()?;
        self.trace_collector.start()?;
        self.autonomous_healer.start()?;
        self.dashboard_engine.start()?;

        Ok(())
    }

/// Health Status operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_status(&self) -> Result<ObservabilityHealthStatus, BearDogError> {
        Ok(ObservabilityHealthStatus {
            metric_collector_healthy: self.metric_collector.is_healthy()?,
            analytics_healthy: self.analytics_processor.is_healthy()?,
            predictive_engine_healthy: self.predictive_engine.is_healthy()?,
            alert_manager_healthy: self.alert_manager.is_healthy()?,
            trace_collector_healthy: self.trace_collector.is_healthy()?,
            autonomous_healer_healthy: self.autonomous_healer.is_healthy()?,
            dashboard_healthy: self.dashboard_engine.is_healthy(bool,
    /// Whether analytics_healthy is enabled
    pub analytics_healthy: bool,
    /// Whether predictive_engine_healthy is enabled
    pub predictive_engine_healthy: bool,
    /// Whether alert_manager_healthy is enabled
    pub alert_manager_healthy: bool,
    /// Whether trace_collector_healthy is enabled
    pub trace_collector_healthy: bool,
    /// Whether autonomous_healer_healthy is enabled
    pub autonomous_healer_healthy: bool,
    /// Whether dashboard_healthy is enabled
    pub dashboard_healthy: bool,
}

impl ObservabilityHealthStatus {

/// Is Fully Healthy operation.
    /// Checks if fully healthy
    /// Checks if fully healthy
    pub fn is_fully_healthy(&self) -> bool {
        self.metric_collector_healthy
            && self.analytics_healthy
            && self.predictive_engine_healthy
            && self.alert_manager_healthy
            && self.trace_collector_healthy
            && self.autonomous_healer_healthy
            && self.dashboard_healthy
    }
} 
