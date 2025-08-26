

use std::sync::Arc;
use beardog::config::core::BearDogConfig;
use beardog::core::BearDogCore;
use beardog::errors::BearDogResult;

pub fn create_test_config() -> BearDogConfig {
    let mut config = BearDogConfig::default();

    config.api.bind_address = "127.0.0.1:0".to_string(); // Random port
    config.api.auth.auth_method = beardog_types::config::network::AuthMethod::Ed25519;

    config.threat_detection.enabled = true;

    config.adapters.external_systems.rust_ecosystem.nestgate =
        Some(beardog::config::RustProjectConfig {
            enabled: true,
            endpoint: "https://nestgate.test:8443".to_string(),
            timeout_ms: 5000,
            tls: None,
            auth: None,
        });
    config.compliance.enabled_standards = vec!["GDPR".to_string(), "HIPAA".to_string()];

    config.database.url = ":memory:".to_string();

    config
}

pub async fn create_test_core() -> BearDogResult<Arc<BearDogCore>> {
    let config = create_test_config();
    let core = Arc::new(BearDogCore::new(config).await?);
    core.start().await?;
    Ok(core)
}

pub fn create_minimal_test_config() -> BearDogConfig {
    let mut config = BearDogConfig::default();
    config.database.url = ":memory:".to_string();
    config.api.bind_address = "127.0.0.1:0".to_string();
    config
}

pub fn create_test_config_with_features(features: TestFeatures) -> BearDogConfig {
    let mut config = create_test_config();
    
    if features.threat_detection {
        config.threat_detection.enabled = true;
        config.threat_detection.real_time_detection = true;
    }
    
    if features.compliance {
        config.compliance.enabled_standards = vec![
            "GDPR".to_string(), 
            "HIPAA".to_string(), 
            "SOX".to_string()
        ];
        config.compliance.strict_mode = features.strict_compliance;
    }
    
    if features.workflows {
        config.workflows.enabled = true;
        config.workflows.multi_party_approval = true;
    }
    
    config
}

#[derive(Default)]
pub struct TestFeatures {
    pub threat_detection: bool,
    pub compliance: bool,
    pub strict_compliance: bool,
    pub workflows: bool,
    pub nestgate_adapter: bool,
    pub api_server: bool,
}

impl TestFeatures {

    pub fn all_enabled() -> Self {
        Self {
            threat_detection: true,
            compliance: true,
            strict_compliance: true,
            workflows: true,
            nestgate_adapter: true,
            api_server: true,
        }
    }

    pub fn essential_only() -> Self {
        Self {
            threat_detection: true,
            compliance: false,
            strict_compliance: false,
            workflows: false,
            nestgate_adapter: false,
            api_server: false,
        }
    }
}

pub mod assertions {
    use beardog::errors::BearDogResult;

    pub fn assert_success<T>(result: &BearDogResult<T>) {
        if let Err(e) = result {
            panic!("Expected success but got error: {:?}", e);
        }
    }

    pub fn assert_error_contains<T>(result: &BearDogResult<T>, expected_error: &str) {
        match result {
            Ok(_) => panic!("Expected error containing '{}' but got success", expected_error),
            Err(e) => {
                let error_str = format_args!("{:?}", e).to_string();
                if !error_str.contains(expected_error) {
                    panic!(
                        "Expected error containing '{}' but got: {}", 
                        expected_error, error_str
                    );
                }
            }
        }
    }
}

pub mod test_data {
    use beardog::security::{Subject, SubjectType, Action, ActionType, Resource};
    use std::collections::HashMap;

    pub fn create_test_subject(id: &str, subject_type: SubjectType) -> Subject {
        Subject {
            id: id.to_string(),
            subject_type,
            attributes: HashMap::from([
                ("department".to_string(), "engineering".to_string()),
                ("clearance".to_string(), "standard".to_string()),
            ]),
            roles: vec!["user".to_string()],
            clearance_level: 3,
        }
    }

    pub fn create_test_action(action_type: ActionType, details: &str) -> Action {
        Action {
            action_type,
            resource_id: "test-resource".to_string(),
            details: details.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn create_test_resource(id: &str) -> Resource {
        Resource {
            id: id.to_string(),
            resource_type: "file".to_string(),
            classification: beardog::security::ResourceClassification::Internal,
            attributes: HashMap::from([
                ("owner".to_string(), "system".to_string()),
                ("created".to_string(), "2025-01-01".to_string()),
            ]),
        }
    }

    pub fn create_test_compliance_data() -> serde_json::Value {
        serde_json::json!({
            "data_processing": {
                "personal_data_types": ["email", "name"],
                "processing_purpose": "user_authentication",
                "legal_basis": "contract",
                "retention_period": 365
            },
            "data_subject": {
                "id": "test-user-123",
                "consent_given": true,
                "consent_date": "2025-01-15T10:00:00Z"
            }
        })
    }
}

pub mod timing {
    use std::time::{Duration, Instant};
    use tokio::time::timeout;

    pub async fn wait_for_condition<F, Fut>(
        condition: F, 
        timeout_duration: Duration, 
        check_interval: Duration
    ) -> bool 
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = Instant::now();
        
        while start.elapsed() < timeout_duration {
            if condition().await {
                return true;
            }
            tokio::time::sleep(check_interval).await;
        }
        
        false
    }

    pub async fn with_timeout<F, Fut, T>(
        future: F,
        timeout_duration: Duration
    ) -> Result<T, tokio::time::error::Elapsed>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        timeout(timeout_duration, future()).await
    }
} 