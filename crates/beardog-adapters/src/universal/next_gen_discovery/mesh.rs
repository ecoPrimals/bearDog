

use super::types::{ServiceEndpoint, ServiceMeshConfig};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    active_services: Vec<ServiceEndpoint>,
    optimization_metrics: MeshMetrics,
}

#[derive(Debug, Clone)]
    /// Number of healthy_services
    pub healthy_services: usize,
    pub avg_response_time_ms: f64,
    /// The success rate value
    pub success_rate: f64,
    /// The optimization score value
    pub optimization_score: f64,
}

#[derive(Debug, Clone)]
    pub avg_response_time: Duration,
    /// Mapping of service health distribution
    pub service_health_distribution: HashMap<String, usize>,
    /// The optimization effectiveness value
    pub optimization_effectiveness: f64,
}

impl DynamicServiceMesh {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: ServiceMeshConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            active_services: Vec::new(),
            optimization_metrics: MeshMetrics::default(Vec<ServiceEndpoint>,
    ) -> Result<Vec<ServiceEndpoint>, BearDogError> {

        let mut optimized = endpoints;

        optimized.sort_by(|a, b| {
            use beardog_types::canonical::HealthStatus;
            match (&a.health_status, &b.health_status) {
                (HealthStatus::Healthy, HealthStatus::Healthy) => std::cmp::Ordering::Equal,
                (HealthStatus::Healthy, _) => std::cmp::Ordering::Less,
                (_, HealthStatus::Healthy) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });

        Ok(optimized)
    }

/// Get Statistics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> Result<MeshStatistics, BearDogError> {
        let mut health_distribution = HashMap::with_capacity(16);
        health_distribution.insert(
            "healthy".to_string(),
            self.optimization_metrics.healthy_services,
        );
        health_distribution.insert(
            "total".to_string(),
            self.optimization_metrics.total_services,
        );

        Ok(MeshStatistics {
            active_services: self.active_services.len(Duration::from_millis(
                self.optimization_metrics.avg_response_time_ms as u64,
            ),
            service_health_distribution: health_distribution,
            optimization_effectiveness: self.optimization_metrics.optimization_score,
        })
    }

/// Get Optimization Score operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets optimization_score
    /// Gets optimization_score
    pub fn get_optimization_score(&[ServiceEndpoint],
    ) -> Result<(), BearDogError> {
        use beardog_types::canonical::HealthStatus;

        self.active_services = services.to_vec();

        self.optimization_metrics.total_services = services.len();
        self.optimization_metrics.healthy_services = services
            .iter()
            .filter(|s| matches!(s.health_status, HealthStatus::Healthy))
            .count();

        if self.optimization_metrics.total_services > 0 {
            self.optimization_metrics.success_rate = self.optimization_metrics.healthy_services
                as f64
                / self.optimization_metrics.total_services as f64;
        }

        self.optimization_metrics.optimization_score =
            self.optimization_metrics.success_rate * 0.8 + 0.2; // Base score + health factor

        Ok(())
    }
}
