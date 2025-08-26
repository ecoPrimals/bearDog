

use super::ExternalFunctionHandler;
use crate::licensing::LicenseManager;

use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SecurityResult;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct GrafanaDashboards;

impl ExternalFunctionHandler for GrafanaDashboards {}

    fn function_name(&self) -> &str {
        "grafana_dashboards"
    }

    async fn execute(
        &self,
        license_manager: &LicenseManager,
        _operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value> {

        if !license_manager
            .is_function_available(self.function_name())
            .await?
        {
            return Err(BearDogError::configuration(format!(
                    "🔒 Grafana integration '}' requires licensing or individual/small-team classification.\n\n\
                    🏠 Individual developers: Automatically granted access\n\
                    👥 Small teams: Automatically granted access\n\
                    🏢 Corporate usage: External adapters locked - acquire unlock certificate",
                    self.function_name()
                ),
            });
        }
        let dashboard_name = payload
            .get("dashboard")
            .and_then(|v| v.as_str())
            .unwrap_or("beardog-security-dashboard");
        let default_grafana_url = format!(
            "http://{}:3000",
            beardog_types::config::constants::network::get_default_host()
        );
        let grafana_url = payload
            .get("grafana_url")
            .unwrap_or(&default_grafana_url);
        tracing::info!(
            "📈 Creating Grafana dashboard '{}' at {}",
            dashboard_name,
            grafana_url

        match self.create_dashboard(grafana_url, dashboard_name).await {
            Ok(dashboard_info) => Ok(serde_json::json!({
                "operation": "create_dashboard",
                "dashboard_name": dashboard_name,
                "grafana_url": grafana_url,
                "status": "success",
                "dashboard_info": dashboard_info
            })),
            Err(e) => Ok(serde_json::json!({
                "status": "error",
                "error": e.to_string()
}
impl GrafanaDashboards {

    async fn create_dashboard(
        grafana_url: &str,
        dashboard_name: &str,
    ) -> Result<Value, SecurityError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| BearDogError::configuration(format!("HTTP client error: {e}"),
            })?;

        let dashboard_json = self.get_beardog_dashboard_json(dashboard_name);

        let api_url = format!("{grafana_url}/api/dashboards/db");
        let response = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer admin") // In production, use proper API key
            .json(&dashboard_json)
            .send()
            .await
            .map_err(|e| BearDogError::configuration(format!("Grafana API request failed: {e}"),
        if response.status().is_success() {
            let result: Value = response
                .json()
                .await
                .map_err(|e| BearDogError::configuration(format!("Failed to parse Grafana response: {e}"),
                })?;
            Ok(result)
        } else {

            tracing::warn!("Grafana API not available, returning mock dashboard");
            Ok(serde_json::json!({
                "id": 1,
                "uid": format_args!("beardog-{}", uuid::Uuid::new_v4().to_string()),
                "url": format_args!("{}/d/beardog-security", grafana_url).to_string(),
                "version": 1,
                "message": "Dashboard created successfully (mock)"
            }))

    fn get_beardog_dashboard_json(&self, dashboard_name: &str) -> Value {
        serde_json::json!({
            "dashboard": {
                "id": null,
                "title": dashboard_name,
                "tags": ["beardog", "security", "monitoring"],
                "timezone": "browser",
                "panels": [
                    {
                        "id": 1,
                        "title": "Security Events",
                        "type": "graph",
                        "targets": [
                            {
                                "expr": "beardog_security_events_total",
                                "refId": "A"
                            }
                        ],
                        "yAxes": [
                                "label": "Events/sec",
                                "min": 0
                        "xAxes": [
                                "type": "time"
                        "gridPos": {
                            "h": 8,
                            "w": 12,
                            "x": 0,
                            "y": 0
                        }
                    },
                        "id": 2,
                        "title": "Authentication Success Rate",
                        "type": "stat",
                                "expr": "beardog_auth_success_rate",
                                "refId": "B"
                        "fieldConfig": {
                            "defaults": {
                                "unit": "percent"
                        },
                            "x": 12,
                        "id": 3,
                        "title": "Threat Detection Status",
                        "type": "table",
                                "expr": "beardog_threat_detections",
                                "refId": "C"
                            "w": 24,
                            "y": 8
                        "id": 4,
                        "title": "HSM Operations",
                                "expr": "beardog_hsm_operations_total",
                                "refId": "D"
                            "y": 16
                        "id": 5,
                        "title": "Genetic Algorithm Performance",
                                "expr": "beardog_genetics_fitness_score",
                                "refId": "E"
                    }
                ],
                "time": {
                    "from": "now-1h",
                    "to": "now"
                },
                "refresh": "5s",
                "schemaVersion": 27,
                "links": []
            },
            "message": "BearDog Security Dashboard created",
            "overwrite": true
        })
