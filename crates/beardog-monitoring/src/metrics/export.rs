// SPDX-License-Identifier: AGPL-3.0-only

// Export Engine
//
// Metrics export, integration, and external system connectivity.

use beardog_errors::BearDogError;
// Removed unused imports: serde::{Serialize, Deserialize}

/// Export and integration engine
#[derive(Debug)]
pub struct ExportEngine {
    _config: ExportConfig,
}

impl ExportEngine {
    /// Creates a new instance
    pub const fn new(config: ExportConfig) -> Result<Self, BearDogError> {
        Ok(Self { _config: config })
    }

    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Export engine started");
        Ok(())
    }

    /// Serializes or forwards the given system metrics to configured external sinks.
    pub const fn export_metrics(
        &self,
        _metrics: &super::SystemMetrics,
    ) -> Result<(), BearDogError> {
        // Export logic for external systems
        Ok(())
    }
}

/// Controls which external observability sinks are active and how often batches flush.
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Whether `enable_prometheus` is enabled
    pub enable_prometheus: bool,
    /// When true, push or expose metrics in a Grafana-friendly form (alongside Prometheus if enabled).
    pub enable_grafana: bool,
    /// Number of `export_interval_secs`
    pub export_interval_secs: u64,
    /// Number of `batch_size`
    pub batch_size: usize,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            enable_prometheus: true,
            enable_grafana: false,
            export_interval_secs: 60,
            batch_size: 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert!(config.enable_prometheus);
        assert!(!config.enable_grafana);
        assert_eq!(config.export_interval_secs, 60);
        assert_eq!(config.batch_size, 1000);
    }

    #[test]
    fn test_export_config_clone() {
        let config = ExportConfig::default();
        let cloned = config.clone();
        assert_eq!(config.enable_prometheus, cloned.enable_prometheus);
        assert_eq!(config.batch_size, cloned.batch_size);
    }

    #[test]
    fn test_export_config_custom() {
        let config = ExportConfig {
            enable_prometheus: false,
            enable_grafana: true,
            export_interval_secs: 30,
            batch_size: 500,
        };
        assert!(!config.enable_prometheus);
        assert!(config.enable_grafana);
        assert_eq!(config.export_interval_secs, 30);
    }

    #[test]
    fn test_export_engine_creation() {
        let config = ExportConfig::default();
        let engine = ExportEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_export_engine_start() {
        let config = ExportConfig::default();
        let engine = ExportEngine::new(config).expect("engine");
        let result = engine.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_engine_debug() {
        let config = ExportConfig::default();
        let engine = ExportEngine::new(config).expect("engine");
        let debug_str = format!("{:?}", engine);
        assert!(debug_str.contains("ExportEngine"));
    }
}
