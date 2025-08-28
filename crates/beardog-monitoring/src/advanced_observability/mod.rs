

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

    pub metric_collector: Arc<RealTimeMetricCollector>,

    pub analytics_processor: Arc<AnalyticsProcessor>,

    pub predictive_engine: Arc<PredictiveMaintenanceEngine>,

    pub alert_manager: Arc<IntelligentAlertManager>,

    pub trace_collector: Arc<DistributedTraceCollector>,

    pub autonomous_healer: Arc<AutonomousHealingSystem>,

    pub dashboard_engine: Arc<RealTimeDashboard>,
}

impl AdvancedObservabilityEngine {

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

    pub async fn initialize(&self) -> Result<(), BearDogError> {

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
            task.await?;
        }

        Ok(())
    }

    pub async fn start(&self) -> Result<(), BearDogError> {

        self.metric_collector.start().await?;
        self.analytics_processor.start().await?;
        self.predictive_engine.start().await?;
        self.alert_manager.start().await?;
        self.trace_collector.start().await?;
        self.autonomous_healer.start().await?;
        self.dashboard_engine.start().await?;

        Ok(())
    }

    pub async fn health_status(&self) -> Result<ObservabilityHealthStatus, BearDogError> {
        Ok(ObservabilityHealthStatus {
            metric_collector_healthy: self.metric_collector.is_healthy().await?,
            analytics_healthy: self.analytics_processor.is_healthy().await?,
            predictive_engine_healthy: self.predictive_engine.is_healthy().await?,
            alert_manager_healthy: self.alert_manager.is_healthy().await?,
            trace_collector_healthy: self.trace_collector.is_healthy().await?,
            autonomous_healer_healthy: self.autonomous_healer.is_healthy().await?,
            dashboard_healthy: self.dashboard_engine.is_healthy().await?,
        })
    }

    pub async fn shutdown(&self) -> Result<(), BearDogError> {

        self.dashboard_engine.shutdown().await?;
        self.autonomous_healer.shutdown().await?;
        self.trace_collector.shutdown().await?;
        self.alert_manager.shutdown().await?;
        self.predictive_engine.shutdown().await?;
        self.analytics_processor.shutdown().await?;
        self.metric_collector.shutdown().await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityHealthStatus {
    pub metric_collector_healthy: bool,
    pub analytics_healthy: bool,
    pub predictive_engine_healthy: bool,
    pub alert_manager_healthy: bool,
    pub trace_collector_healthy: bool,
    pub autonomous_healer_healthy: bool,
    pub dashboard_healthy: bool,
}

impl ObservabilityHealthStatus {

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