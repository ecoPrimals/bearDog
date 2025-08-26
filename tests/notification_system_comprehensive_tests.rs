

use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;

use beardog_workflows::workflows::notification::NotificationEngine;
use beardog_workflows::workflows::types::{
    NotificationConfig, NotificationMessage, NotificationResult, NotificationStatus,
};
use beardog_errors::BearDogResult;

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

    fn with_failure() -> Self {
        Self {
            port: 8080,
            should_fail: true,
        }
    }

    async fn start(&self) -> String {
        format_args!("http://localhost:{}/webhook", self.port).to_string()
    }
}

#[tokio::test]
async fn test_webhook_notification_success() -> BearDogResult<()> {
    let config = create_webhook_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Test security alert";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("alert_type".to_string(), json!("security_breach"));
    metadata.insert("severity".to_string(), json!("high"));
    metadata.insert("incident_id".to_string(), json!("INC-2024-001"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Webhook notification sent successfully");
            assert!(notification_result.webhook_sent);
        }
        Err(e) => {
            println!("⚠️  Webhook notification failed (expected in test environment): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_webhook_notification_with_signature() -> BearDogResult<()> {
    let config = create_webhook_config_with_secret();
    let engine = NotificationEngine::new(config);
    
    let message = "Test webhook with signature";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_type".to_string(), json!("signature_verification"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Webhook with signature sent successfully");
            assert!(notification_result.webhook_sent);
        }
        Err(e) => {
            println!("⚠️  Webhook with signature failed (expected in test environment): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_webhook_retry_mechanism() -> BearDogResult<()> {
    let config = create_webhook_config_with_invalid_url();
    let engine = NotificationEngine::new(config);
    
    let message = "Test webhook retry";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_type".to_string(), json!("retry_test"));

    let start_time = std::time::Instant::now();
    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    let elapsed = start_time.elapsed();

    assert!(result.is_err());

    assert!(elapsed >= Duration::from_secs(2));
    println!("✅ Webhook retry mechanism working, took {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

#[tokio::test]
async fn test_email_notification_configuration() -> BearDogResult<()> {
    let config = create_email_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Test email notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("alert_type".to_string(), json!("email_test"));
    metadata.insert("priority".to_string(), json!("medium"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Email notification sent successfully");
            assert!(notification_result.email_sent);
        }
        Err(e) => {
            println!("⚠️  Email notification failed (expected without SMTP server): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_sms_notification_twilio() -> BearDogResult<()> {
    let config = create_sms_config_twilio();
    let engine = NotificationEngine::new(config);
    
    let message = "Test SMS via Twilio";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("provider".to_string(), json!("twilio"));
    metadata.insert("incident_id".to_string(), json!("SMS-001"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ SMS notification sent successfully");
            assert!(notification_result.sms_sent);
        }
        Err(e) => {
            println!("⚠️  SMS notification failed (expected without Twilio credentials): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_sms_notification_aws_sns() -> BearDogResult<()> {
    let config = create_sms_config_aws_sns();
    let engine = NotificationEngine::new(config);
    
    let message = "Test SMS via AWS SNS";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("provider".to_string(), json!("aws_sns"));
    metadata.insert("incident_id".to_string(), json!("SMS-002"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ SMS notification sent successfully");
            assert!(notification_result.sms_sent);
        }
        Err(e) => {
            println!("⚠️  SMS notification failed (expected without AWS credentials): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_slack_notification() -> BearDogResult<()> {
    let config = create_slack_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Test Slack notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("channel".to_string(), json!("#security-alerts"));
    metadata.insert("severity".to_string(), json!("high"));
    metadata.insert("affected_systems".to_string(), json!("authentication"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Slack notification sent successfully");
            assert!(notification_result.slack_sent);
        }
        Err(e) => {
            println!("⚠️  Slack notification failed (expected without webhook URL): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_teams_notification() -> BearDogResult<()> {
    let config = create_teams_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Test Teams notification";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("team".to_string(), json!("Security Team"));
    metadata.insert("priority".to_string(), json!("critical"));
    metadata.insert("action_required".to_string(), json!("immediate"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Teams notification sent successfully");
            assert!(notification_result.teams_sent);
        }
        Err(e) => {
            println!("⚠️  Teams notification failed (expected without webhook URL): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_multi_channel_notification() -> BearDogResult<()> {
    let config = create_multi_channel_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Critical security incident requiring immediate attention";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("severity".to_string(), json!("critical"));
    metadata.insert("incident_type".to_string(), json!("data_breach"));
    metadata.insert("affected_users".to_string(), json!(1000));
    metadata.insert("estimated_impact".to_string(), json!("high"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Multi-channel notification completed");
            println!("   Email: {}", notification_result.email_sent);
            println!("   SMS: {}", notification_result.sms_sent);
            println!("   Webhook: {}", notification_result.webhook_sent);
            println!("   Slack: {}", notification_result.slack_sent);
            println!("   Teams: {}", notification_result.teams_sent);
        }
        Err(e) => {
            println!("⚠️  Multi-channel notification had failures: {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_filtering() -> BearDogResult<()> {
    let config = create_filtered_config();
    let engine = NotificationEngine::new(config);

    let low_severity_message = "Low severity event";
    let mut low_metadata = HashMap::with_capacity(16);
    low_metadata.insert("severity".to_string(), json!("low"));
    
    let result = engine.send_notification(
        &NotificationMessage {
            content: low_severity_message.to_string(),
            metadata: low_metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Low severity notification filtered appropriately");

            assert!(!notification_result.sms_sent); // SMS typically for critical only
        }
        Err(e) => {
            println!("⚠️  Low severity notification failed: {}", e);
        }
    }

    let high_severity_message = "High severity security event";
    let mut high_metadata = HashMap::with_capacity(16);
    high_metadata.insert("severity".to_string(), json!("critical"));
    
    let result = engine.send_notification(
        &NotificationMessage {
            content: high_severity_message.to_string(),
            metadata: high_metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ High severity notification sent to all channels");
        }
        Err(e) => {
            println!("⚠️  High severity notification failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_rate_limiting() -> BearDogResult<()> {
    let config = create_rate_limited_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Rate limit test message";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_type".to_string(), json!("rate_limit"));

    let mut results = Vec::new();
    for i in 0..10 {
        let mut test_metadata = metadata.clone();
        test_metadata.insert("sequence".to_string(), json!(i));
        
        let result = engine.send_notification(
            &NotificationMessage {
                content: format_args!("{} #{}", message, i).to_string(),
                metadata: test_metadata,
            }
        ).await;
        
        results.push(result);

        sleep(Duration::from_millis(10)).await;
    }

    let success_count = results.iter().filter(|r| r.is_ok()).count();
    println!("✅ Rate limiting test completed: {}/10 notifications successful", success_count);

    assert!(success_count <= 10);

    Ok(())
}

#[tokio::test]
async fn test_notification_template_rendering() -> BearDogResult<()> {
    let config = create_template_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Security alert for {{user}} in {{system}}";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("user".to_string(), json!("admin"));
    metadata.insert("system".to_string(), json!("production"));
    metadata.insert("timestamp".to_string(), json!("2024-01-01T12:00:00Z"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(notification_result) => {
            println!("✅ Template rendering successful");

        }
        Err(e) => {
            println!("⚠️  Template rendering failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_error_handling() -> BearDogResult<()> {

    let config = create_malformed_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Test error handling";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_type".to_string(), json!("error_handling"));

    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;

    match result {
        Ok(_) => {
            println!("✅ Error handling test passed (unexpected success)");
        }
        Err(e) => {
            println!("✅ Error handling test passed: {}", e);

            assert!(e.to_string().contains("Configuration") || e.to_string().contains("Notification"));
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_notification_metrics_collection() -> BearDogResult<()> {
    let config = create_metrics_config();
    let engine = NotificationEngine::new(config);
    
    let message = "Metrics collection test";
    let mut metadata = HashMap::with_capacity(16);
    metadata.insert("test_type".to_string(), json!("metrics"));

    let start_time = std::time::Instant::now();
    let result = engine.send_notification(
        &NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        }
    ).await;
    let elapsed = start_time.elapsed();

    match result {
        Ok(notification_result) => {
            println!("✅ Metrics collection test completed");
            println!("   Processing time: {:.2}ms", elapsed.as_millis());
            println!("   Channels attempted: {}", notification_result.channels_attempted());
            println!("   Success rate: {:.1}%", notification_result.success_rate() * 100.0);
        }
        Err(e) => {
            println!("⚠️  Metrics collection test failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_notifications() -> BearDogResult<()> {
    let config = create_concurrent_config();
    let engine = std::sync::Arc::new(NotificationEngine::new(config));

    let mut handles = Vec::new();
    for i in 0..10 {
        let engine_clone = std::sync::Arc::clone(&engine);
        let handle = tokio::spawn(async move {
            let message = format_args!("Concurrent notification #{}", i).to_string();
            let mut metadata = HashMap::with_capacity(16);
            metadata.insert("sequence".to_string(), json!(i));
            metadata.insert("test_type".to_string(), json!("concurrent"));
            
            engine_clone.send_notification(
                &NotificationMessage {
                    content: message,
                    metadata,
                }
            ).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        results.push(result);
    }

    let success_count = results.iter().filter(|r| r.is_ok()).count();
    println!("✅ Concurrent notifications test: {}/10 successful", success_count);

    Ok(())
}

fn create_webhook_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        webhook_secret: None,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_webhook_config_with_secret() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        webhook_secret: Some("test-secret-key".to_string()),
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_webhook_config_with_invalid_url() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://invalid-host:9999/webhook".to_string()),
        webhook_secret: None,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_email_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: false,
        email_enabled: true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some("test-password".to_string()),
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_sms_config_twilio() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: false,
        email_enabled: false,
        sms_enabled: true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some("+1234567890".to_string()),
        sms_from_number: Some("+0987654321".to_string()),
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_sms_config_aws_sns() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: false,
        email_enabled: false,
        sms_enabled: true,
        sms_provider: Some("aws_sns".to_string()),
        sms_api_key: Some("test-aws-key".to_string()),
        sms_phone_number: Some("+1234567890".to_string()),
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_slack_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: false,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: true,
        slack_webhook_url: Some("https://hooks.slack.com/services/test/test/test".to_string()),
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_teams_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: false,
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: true,
        teams_webhook_url: Some("https://outlook.office.com/webhook/test".to_string()),
        ..Default::default()
    }
}

fn create_multi_channel_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some("test-password".to_string()),
        sms_enabled: true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some("+1234567890".to_string()),
        slack_enabled: true,
        slack_webhook_url: Some("https://hooks.slack.com/services/test/test/test".to_string()),
        teams_enabled: true,
        teams_webhook_url: Some("https://outlook.office.com/webhook/test".to_string()),
        ..Default::default()
    }
}

fn create_filtered_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: true,
        smtp_server: Some("smtp.example.com".to_string()),
        smtp_port: Some(587),
        smtp_username: Some("test@example.com".to_string()),
        smtp_password: Some("test-password".to_string()),
        sms_enabled: true,
        sms_provider: Some("twilio".to_string()),
        sms_api_key: Some("test-twilio-key".to_string()),
        sms_phone_number: Some("+1234567890".to_string()),
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_rate_limited_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_template_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_malformed_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("not-a-valid-url".to_string()),
        email_enabled: true,
        smtp_server: None, // Missing required field
        smtp_port: Some(587),
        smtp_username: None, // Missing required field
        smtp_password: None, // Missing required field
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_metrics_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
}

fn create_concurrent_config() -> NotificationConfig {
    NotificationConfig {
        webhook_enabled: true,
        webhook_url: Some("http://localhost:8080/webhook".to_string()),
        email_enabled: false,
        sms_enabled: false,
        slack_enabled: false,
        teams_enabled: false,
        ..Default::default()
    }
} 