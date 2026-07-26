// SPDX-License-Identifier: AGPL-3.0-or-later

// TRAIT IMPLEMENTATIONS: BearDogProvider (required by HsmProvider)

use super::{Fido2MultiCredentialProvider, Fido2ProviderConfig};
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::{
    ProviderCapability, ProviderHealth, ProviderMetrics,
};
use std::collections::BTreeMap;
use std::time::SystemTime;

impl beardog_traits::unified::BearDogProvider for Fido2MultiCredentialProvider {
    type Error = BearDogError;
    type Config = Fido2ProviderConfig;

    fn provider_id(&self) -> &'static str {
        "fido2_multi_credential"
    }

    fn provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn capabilities(&self) -> Vec<ProviderCapability> {
        vec![
            ProviderCapability {
                name: "multi_credential".to_string(),
                description: "Multiple credentials per device".to_string(),
                parameters: vec![],
                enabled: true,
            },
            ProviderCapability {
                name: "hardware_entropy".to_string(),
                description: "Hardware random number generation".to_string(),
                parameters: vec![],
                enabled: self.device_info.capabilities.hmac_secret,
            },
            ProviderCapability {
                name: "user_presence".to_string(),
                description: "User presence verification (button press)".to_string(),
                parameters: vec![],
                enabled: true,
            },
        ]
    }

    async fn health_check(&self) -> Result<ProviderHealth, Self::Error> {
        use beardog_types::canonical::providers_unified::traits::{
            HealthStatus, NetworkIoMetrics, ResourceUsage,
        };

        Ok(ProviderHealth {
            status: HealthStatus::Healthy,
            timestamp: SystemTime::now(),
            details: BTreeMap::from([
                ("device".to_string(), self.device_info.product.clone()),
                (
                    "protocol".to_string(),
                    format!("CTAP2 {:?}", self.device_info.protocol_versions),
                ),
            ]),
            resource_usage: ResourceUsage {
                cpu_percent: 0.0,
                memory_bytes: 0,
                memory_percent: 0.0,
                network_io: NetworkIoMetrics {
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                },
                disk_io: BTreeMap::new(),
            },
            last_error: None,
        })
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "Credential counts as f64 for provider metrics display"
    )]
    async fn metrics(&self) -> Result<ProviderMetrics, Self::Error> {
        use beardog_types::canonical::providers_unified::traits::CustomMetric;

        let creds = self.credentials.read().await;
        Ok(ProviderMetrics {
            timestamp: SystemTime::now(),
            performance: BTreeMap::new(),
            custom_metrics: vec![
                CustomMetric {
                    name: "credentials_count".to_string(),
                    value: creds.len() as f64,
                    unit: "count".to_string(),
                    description: "Number of credentials stored".to_string(),
                    tags: BTreeMap::new(),
                },
                CustomMetric {
                    name: "max_credentials".to_string(),
                    value: self.device_info.capabilities.max_resident_keys.unwrap_or(0) as f64,
                    unit: "count".to_string(),
                    description: "Maximum credentials supported".to_string(),
                    tags: BTreeMap::new(),
                },
            ],
            system_metrics: beardog_types::canonical::providers_unified::traits::SystemMetrics {
                uptime_seconds: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                avg_response_time_ms: 0.0,
                active_connections: 0,
                error_rate: 0.0,
            },
        })
    }
}

