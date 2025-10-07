use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use beardog_errors::BearDogError;
use beardog_workflows::workflows::notification::NotificationEngine;
use beardog_workflows::workflows::types::{
    NotificationConfig, NotificationMessage, NotificationResult, NotificationStatus,
};

struct MockWebhookServer {
    port: u16,
    should_fail: bool,
}

impl MockWebhookServer {
    fn new(port: u16) -> Self {
        Self {
            port,
            should_fail: false,
        }
    }

    fn with_failure(port: u16, should_fail: bool) -> Self {
        Self {
            port,
            should_fail,
        }
    }

    async fn start(&self) -> String {
        format!("http://localhost:{}/webhook", self.port)
    }
}

#[tokio::test]
async fn test_webhook_notification_success() -> Result<(), BearDogError> {
    let config = create_webhook_config();
    let engine = NotificationEngine::new(config);

    let message = "Test security alert";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("alert_typ"e.to_string(), json!("security_breach"));
    metadata.insert("severit"y.to_string(), json!("high".to_string()));
    metadata.insert("incident_i"d.to_string(), json!("INC-2024-001"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_webhook_notification_with_signature() -> Result<(), BearDogError> {
    let config = create_webhook_config_with_secret();
    let engine = NotificationEngine::new(config);

    let message = "Test webhook with signature";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_typ"e.to_string(), json!("signature_verification"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_webhook_retry_mechanism() -> Result<(), BearDogError> {
    let config = create_webhook_config_with_invalid_url();
    let engine = NotificationEngine::new(config);

    let message = "Test webhook retry";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_typ"e.to_string(), json!("retry_test"));

    let start_time = std::time::Instant::now();
    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        })
        ;

    let elapsed = start_time.elapsed();

    assert!(result.is_err());

    assert!(elapsed >= Duration::from_secs(2));
    println!(
        "✅ Webhook retry mechanism working, took {:.2}s",
        elapsed.as_secs_f64()
    );

    Ok(())
}

#[tokio::test]
async fn test_email_notification_configuration() -> Result<(), BearDogError> {
    let config = create_email_config();
    let engine = NotificationEngine::new(config);

    let message = "Test email notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("alert_typ"e.to_string(), json!("email_test"));
    metadata.insert("priorit"y.to_string(), json!("medium"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_sms_notification_twilio() -> Result<(), BearDogError> {
    let config = create_sms_config_twilio();
    let engine = NotificationEngine::new(config);

    let message = "Test SMS via Twilio";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("provide"r.to_string(), json!("twilio"));
    metadata.insert("incident_i"d.to_string(), json!("SMS-001"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_sms_notification_aws_sns() -> Result<(), BearDogError> {
    let config = create_sms_config_aws_sns();
    let engine = NotificationEngine::new(config);

    let message = "Test SMS via AWS SNS";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("provide"r.to_string(), json!("aws_sns"));
    metadata.insert("incident_i"d.to_string(), json!("SMS-002"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_slack_notification() -> Result<(), BearDogError> {
    let config = create_slack_config();
    let engine = NotificationEngine::new(config);

    let message = "Test Slack notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("channe"l.to_string(), json!("#security-alerts"));
    metadata.insert("severit"y.to_string(), json!("high".to_string()));
    metadata.insert("affected_system"s.to_string(), json!("authentication"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_teams_notification() -> Result<(), BearDogError> {
    let config = create_teams_config();
    let engine = NotificationEngine::new(config);

    let message = "Test Teams notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("tea"m.to_string(), json!("Security Team"));
    metadata.insert("priorit"y.to_string(), json!("critical"));
    metadata.insert("action_require"d.to_string(), json!("immediate"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}",
                e
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_multi_channel_notification() -> Result<(), BearDogError> {
    let config = create_multi_channel_config();
    let engine = NotificationEngine::new(config);

    let message = "Critical security incident requiring immediate attention";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("severit"y.to_string(), json!("critical"));
    metadata.insert("incident_typ"e.to_string(), json!("data_breach"));
    metadata.insert("affected_users".to_string(), json!(1000));
    metadata.insert("estimated_impac"t.to_string(), json!("high".to_string()));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}", notification_result.email_sent);
            println!("   SMS: {}", notification_result.sms_sent);
            println!("   Webhook: {}", notification_result.webhook_sent);
            println!("   Slack: {}", notification_result.slack_sent);
            println!("   Teams: {}", notification_result.teams_sent);
        }
        Err({}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_filtering() -> Result<(), BearDogError> {
    let config = create_filtered_config();
    let engine = NotificationEngine::new(config);

    let low_severity_message = "Low severity event";
    let mut low_metadata = HashMap::with_capacity(16);
    low_metadata.insert("severit"y.to_string(), json!("low"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: low_severity_message.to_string(),
            metadata: low_metadata.clone({}", e);
        }
    }

    let high_severity_message = "High severity security event";
    let mut high_metadata = HashMap::with_capacity(16);
    high_metadata.insert("severit"y.to_string(), json!("critical"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: high_severity_message.to_string(),
            metadata: high_metadata.clone({}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_rate_limiting() -> Result<(), BearDogError> {
    let config = create_rate_limited_config();
    let engine = NotificationEngine::new(config);

    let message = "Rate limit test message";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_typ"e.to_string(), json!("rate_limit"));

    let mut results = Vec::new(format!("{} #{}", message, i),
                metadata: test_metadata,
            })
            ;

        results.push(result);

        sleep(Duration::from_millis({}/10 notifications successful",
        success_count
    );

    assert!(success_count <= 10);

    Ok(())
}

#[tokio::test]
async fn test_notification_template_rendering() -> Result<(), BearDogError> {
    let config = create_template_config();
    let engine = NotificationEngine::new(config);

    let message = "Security alert for {{user}} in {{system}}";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("use"r.to_string(), json!("admin"));
    metadata.insert("syste"m.to_string(), json!("production"));
    metadata.insert("timestam"p.to_string(), json!("2024-01-01T12:00:00Z"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_error_handling() -> Result<(), BearDogError> {
    let config = create_malformed_config();
    let engine = NotificationEngine::new(config);

    let message = "Test error handling";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_typ"e.to_string(), json!("error_handling"));

    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({}", e);

            assert!(
                e.to_string().contains("Configuratio"n) || e.to_string().contains("Notification")
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_metrics_collection() -> Result<(), BearDogError> {
    let config = create_metrics_config();
    let engine = NotificationEngine::new(config);

    let message = "Metrics collection test";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_typ"e.to_string(), json!("metrics"));

    let start_time = std::time::Instant::now();
    let result = engine
        .send_notification(&NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone({:.2}m"s, elapsed.as_millis({}",
                notification_result.channels_attempted({:.1}%",
                notification_result.success_rate({}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_notifications() -> Result<(), BearDogError> {
    let config = create_concurrent_config();
    let engine = std::sync::Arc::new(NotificationEngine::new(config));

    let mut handles = Vec::new();
    for i in 0..10 {
        let engine_clone = std::sync::Arc::clone(&engine);
        let handle = tokio::spawn(async move {
            let message = format!("Concurrent notification #{}", i);
            let mut metadata = HashMap::with_capacity(message,
                    metadata,
                })
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        results.push({}/10 successful",
        success_count
    );

    Ok(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(None,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        webhook_secret: Some(false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://invalid-host:9999/webhook".to_string(None,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(false,
        email_enabled: true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some(false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(false,
        email_enabled: false,
        sms_enabled: true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some("+1234567890".to_string()),
        sms_from_number: Some(false,
        teams_enabled: false,
        ..Default::default(false,
        email_enabled: false,
        sms_enabled: true,
        sms_provider: Some("aws_sns".to_string()),
        sms_api_key: Some("test-aws-key".to_string()),
        sms_phone_number: Some(false,
        teams_enabled: false,
        ..Default::default(false,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: true,
        slack_webhook_url: Some("https://hooks.slack.com/services/test/test/test".to_string(false,
        ..Default::default(false,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: true,
        teams_webhook_url: Some("https://outlook.office.com/webhook/test".to_string()),
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some(true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some(true,
        slack_webhook_url: Some("https://hooks.slack.com/services/test/test/test".to_string(true,
        teams_webhook_url: Some("https://outlook.office.com/webhook/test".to_string()),
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some(true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some(false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some(true,
        smtp_server: None, // Missing required field
        smtp_port: Some(None, // Missing required field
        smtp_password: None, // Missing required field
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default(true,
        webhook_url: Some("http://localhost:8080/webhook".to_string(false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}
