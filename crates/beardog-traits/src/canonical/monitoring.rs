use super::base::{BaseProvider, ServiceHealth};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Temporary type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[allow(clippy::type_complexity)]
pub trait MonitoringProvider: BaseProvider {
    fn record_metric(
        name: &str,
        value: f64,
        tags: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets metrics
    fn get_metrics(
        names: &[&str],
        time_range: Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, f64>, BearDogError>> + Send;

    fn record_event(
        event_type: &str,
        data: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets service_health
    fn get_service_health(
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<ServiceHealth, BearDogError>> + Send;

    /// Creates alert
    fn create_alert(
        name: &str,
        condition: &str,
        severity: AlertSeverity,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Removes alert
    fn remove_alert(
        alert_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn list_alerts(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<HashMap<String, String>>, BearDogError>> + Send;
}
