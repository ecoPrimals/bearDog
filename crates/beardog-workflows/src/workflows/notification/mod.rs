//! Universal notification system for workflow events
//!
//! Supports any communication method users have available through a plugin-based adapter system

use async_trait::async_trait;
use beardog_config::integration::NotificationConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod adapters;
pub mod universal;

/// Universal notification adapter trait
/// This allows any communication service to be plugged in
#[async_trait]
pub trait NotificationAdapter: Send + Sync {
    /// Get the adapter name (e.g., "discord", "sms", "email")
    fn name(&self) -> &str;

    /// Test if this adapter is properly configured
    async fn test_connection(&self) -> BearDogResult<()>;

    /// Send a notification through this adapter
    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult>;

    /// Get supported message formats for this adapter
    fn supported_formats(&self) -> Vec<MessageFormat>;

    /// Get adapter capabilities
    fn capabilities(&self) -> AdapterCapabilities;
}

/// Universal notification message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {
    /// Message title/subject
    pub title: String,
    /// Message content/body
    pub content: String,
    /// Priority level
    pub priority: NotificationPriority,
    /// Target recipients (phone numbers, email addresses, user IDs, etc.)
    pub recipients: Vec<String>,
    /// Additional metadata for formatting
    pub metadata: HashMap<String, serde_json::Value>,
    /// Preferred format for this message
    pub format: MessageFormat,
}

/// Message format types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageFormat {
    /// Plain text message
    PlainText,
    /// Rich text with basic formatting
    RichText,
    /// Markdown formatting
    Markdown,
    /// HTML formatting
    Html,
    /// JSON structured data
    Json,
}

/// Notification priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationPriority {
    /// Low priority, can be delayed
    Low,
    /// Normal priority
    Normal,
    /// High priority, send immediately
    High,
    /// Critical priority, use all available channels
    Critical,
}

/// Notification delivery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    /// Unique delivery ID
    pub delivery_id: String,
    /// Whether delivery was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Delivery timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Retry count if this was a retry
    pub retry_count: u32,
}

/// Adapter capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    /// Maximum message length
    pub max_message_length: Option<usize>,
    /// Supports rich formatting
    pub supports_rich_formatting: bool,
    /// Supports attachments
    pub supports_attachments: bool,
    /// Supports group messaging
    pub supports_groups: bool,
    /// Rate limit (messages per minute)
    pub rate_limit: Option<u32>,
}

/// Universal notification engine
pub struct NotificationEngine {
    /// Registered adapters by name
    adapters: HashMap<String, Box<dyn NotificationAdapter>>,
    /// Default adapter configuration
    config: NotificationConfig,
    /// Retry configuration
    retry_config: RetryConfig,
}

/// Retry configuration for failed deliveries
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl NotificationEngine {
    /// Create a new universal notification engine
    pub fn new(config: NotificationConfig) -> Self {
        let mut engine = Self {
            adapters: HashMap::new(),
            config,
            retry_config: RetryConfig::default(),
        };

        // Auto-register available adapters based on configuration
        engine.auto_register_adapters();
        engine
    }

    /// Register a notification adapter
    pub fn register_adapter(&mut self, adapter: Box<dyn NotificationAdapter>) {
        let name = adapter.name().to_string();
        tracing::info!("📢 Registered notification adapter: {}", name);
        self.adapters.insert(name, adapter);
    }

    /// Send notification through all compatible adapters
    pub async fn send_universal_notification(
        &self,
        message: NotificationMessage,
    ) -> BearDogResult<Vec<NotificationResult>> {
        let mut results = Vec::new();

        if !self.config.enabled {
            tracing::debug!("Notifications disabled, skipping");
            return Ok(results);
        }

        // Find compatible adapters for this message
        let compatible_adapters = self.find_compatible_adapters(&message);

        if compatible_adapters.is_empty() {
            tracing::warn!("No compatible notification adapters found for message");
            return Err(BearDogError::ConfigurationError {
                message: "No notification adapters available".to_string(),
            });
        }

        // Send through compatible adapters
        for adapter_name in compatible_adapters {
            if let Some(adapter) = self.adapters.get(&adapter_name) {
                match self.send_with_retry(adapter.as_ref(), &message).await {
                    Ok(result) => {
                        tracing::info!(
                            "✅ Notification sent via {}: {}",
                            adapter_name,
                            result.delivery_id
                        );
                        results.push(result);
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to send via {}: {}", adapter_name, e);
                        results.push(NotificationResult {
                            delivery_id: uuid::Uuid::new_v4().to_string(),
                            success: false,
                            message: e.to_string(),
                            timestamp: chrono::Utc::now(),
                            retry_count: self.retry_config.max_retries,
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    /// Find adapters compatible with the message format and priority
    fn find_compatible_adapters(&self, message: &NotificationMessage) -> Vec<String> {
        let mut compatible = Vec::new();

        for (name, adapter) in &self.adapters {
            // Check if adapter supports the message format
            if adapter.supported_formats().contains(&message.format) {
                // For critical messages, use all available adapters
                if message.priority == NotificationPriority::Critical {
                    compatible.push(name.clone());
                    continue;
                }

                // For other priorities, use based on adapter type
                match message.priority {
                    NotificationPriority::High => {
                        // Use fast adapters for high priority
                        if matches!(name.as_str(), "sms" | "discord" | "slack" | "webhook") {
                            compatible.push(name.clone());
                        }
                    }
                    NotificationPriority::Normal | NotificationPriority::Low => {
                        // Use any available adapter
                        compatible.push(name.clone());
                    }
                    NotificationPriority::Critical => {
                        // Already handled above
                        compatible.push(name.clone());
                    }
                }
            }
        }

        compatible
    }

    /// Send notification with retry logic
    async fn send_with_retry(
        &self,
        adapter: &dyn NotificationAdapter,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        let mut last_error = None;
        let mut delay_ms = self.retry_config.initial_delay_ms;

        for attempt in 0..=self.retry_config.max_retries {
            match adapter.send_notification(message).await {
                Ok(mut result) => {
                    result.retry_count = attempt;
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);

                    // Don't sleep after the last attempt
                    if attempt < self.retry_config.max_retries {
                        tracing::warn!(
                            "Notification attempt {} failed via {}, retrying in {}ms",
                            attempt + 1,
                            adapter.name(),
                            delay_ms
                        );

                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                        delay_ms = std::cmp::min(
                            (delay_ms as f32 * self.retry_config.backoff_multiplier) as u64,
                            self.retry_config.max_delay_ms,
                        );
                    }
                }
            }
        }

        Err(
            last_error.unwrap_or_else(|| BearDogError::NotificationError {
                message: "All retry attempts failed".to_string(),
            }),
        )
    }

    /// Auto-register adapters based on configuration
    fn auto_register_adapters(&mut self) {
        // This will be implemented to auto-detect available services
        tracing::info!("🔍 Auto-detecting available notification services...");

        // Register universal webhook adapter if webhook config exists
        if let Some(webhook_config) = &self.config.webhook {
            let adapter = universal::WebhookAdapter::new(webhook_config.clone());
            self.register_adapter(Box::new(adapter));
        }

        // Register email adapter if SMTP config exists
        if let Some(email_config) = &self.config.email {
            let adapter = universal::EmailAdapter::new(email_config.clone());
            self.register_adapter(Box::new(adapter));
        }

        // Register SMS adapter if SMS config exists
        if let Some(sms_config) = &self.config.slack {
            // Note: Using slack config as example - this would be proper SMS config
            let adapter = universal::SlackAdapter::new(sms_config.clone());
            self.register_adapter(Box::new(adapter));
        }
    }

    /// Test all registered adapters
    pub async fn test_all_adapters(&self) -> BearDogResult<HashMap<String, bool>> {
        let mut results = HashMap::new();

        for (name, adapter) in &self.adapters {
            match adapter.test_connection().await {
                Ok(_) => {
                    tracing::info!("✅ Adapter {} test passed", name);
                    results.insert(name.clone(), true);
                }
                Err(e) => {
                    tracing::warn!("❌ Adapter {} test failed: {}", name, e);
                    results.insert(name.clone(), false);
                }
            }
        }

        Ok(results)
    }

    /// Get list of registered adapters
    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Legacy workflow notification methods for backward compatibility
    pub async fn notify_workflow_initiated(
        &self,
        workflow: &crate::workflows::types::Workflow,
    ) -> BearDogResult<()> {
        let message = NotificationMessage {
            title: "Workflow Initiated".to_string(),
            content: format!("Workflow {} has been initiated", workflow.id),
            priority: NotificationPriority::Normal,
            recipients: vec![], // Will be filled from workflow configuration
            metadata: HashMap::new(),
            format: MessageFormat::PlainText,
        };

        let _results = self.send_universal_notification(message).await?;
        Ok(())
    }

    pub async fn notify_workflow_completed(
        &self,
        workflow: &crate::workflows::types::Workflow,
    ) -> BearDogResult<()> {
        let message = NotificationMessage {
            title: "Workflow Completed".to_string(),
            content: format!(
                "Workflow {} has been completed with status: {}",
                workflow.id, workflow.status
            ),
            priority: NotificationPriority::Normal,
            recipients: vec![],
            metadata: HashMap::new(),
            format: MessageFormat::PlainText,
        };

        let _results = self.send_universal_notification(message).await?;
        Ok(())
    }

    pub async fn notify_approval_submitted(
        &self,
        workflow: &crate::workflows::types::Workflow,
        approval: &crate::workflows::types::ApprovalRecord,
    ) -> BearDogResult<()> {
        let message = NotificationMessage {
            title: "Approval Submitted".to_string(),
            content: format!(
                "Approval {} submitted for workflow {}",
                approval.decision, workflow.id
            ),
            priority: NotificationPriority::Normal,
            recipients: vec![],
            metadata: HashMap::new(),
            format: MessageFormat::PlainText,
        };

        let _results = self.send_universal_notification(message).await?;
        Ok(())
    }
}
