

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::monitoring::MetricsCollector;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PredictiveScaler {

    metrics: Arc<MetricsCollector>,

    config: super::config::PredictiveScalingConfig,

    state: Arc<RwLock<ScalingState>>,
}

#[derive(Debug, Clone)]
    target_scale: u32,
    last_scaling_event: Option<std::time::Instant>,
}

impl PredictiveScaler {

/// New operation.
    /// Creates a new instance
    pub fn new(config: super::config::PredictiveScalingConfig) -> Self {
        Self {
            metrics: Arc::new(MetricsCollector::new()),
            config,
            state: Arc::new(RwLock::new(ScalingState::default())),
        }
    }

/// Predict Scaling operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn predict_scaling(&self) -> Result<u32, BearDogError> {
        let current_metrics = self.metrics.collect_current_metrics()?;
        let state = self.state.read();

        let predicted_scale = if current_metrics.cpu_utilization > 0.8 {
            state.current_scale.saturating_add(1)
        } else if current_metrics.cpu_utilization < 0.3 && state.current_scale > 1 {
            state.current_scale.saturating_sub(1)
        } else {
            state.current_scale
        };

        Ok(predicted_scale)
    }

/// Apply Scaling operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn apply_scaling(&self, target_scale: u32) -> Result<(), BearDogError> {
        let mut state = self.state.write();
        state.target_scale = target_scale;
        state.last_scaling_event = Some(std::time::Instant::now());

        tracing::info!(
            "Applied predictive scaling: {} -> {}",
            state.current_scale,
            target_scale
        );
        state.current_scale = target_scale;

        Ok(())
    }
}
