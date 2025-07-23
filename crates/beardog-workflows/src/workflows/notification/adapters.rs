//! Notification adapter plugin system
//!
//! This module provides a framework for users to create custom notification adapters
//! for any service they use - Matrix, Signal, custom internal systems, etc.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin-based adapter registry
pub struct AdapterRegistry {
    /// Registered adapter factories
    factories: HashMap<String, Box<dyn AdapterFactory>>,
    /// Configuration templates for each adapter type
    config_templates: HashMap<String, AdapterConfigTemplate>,
}

/// Factory trait for creating adapters from configuration
pub trait AdapterFactory: Send + Sync {
    /// Create an adapter from configuration
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>>;

    /// Get the adapter type name
    fn adapter_type(&self) -> &str;

    /// Get configuration schema for this adapter
    fn config_schema(&self) -> AdapterConfigTemplate;
}

/// Configuration template for an adapter type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfigTemplate {
    /// Adapter type name
    pub adapter_type: String,
    /// Human-readable description
    pub description: String,
    /// Required configuration fields
    pub required_fields: Vec<ConfigField>,
    /// Optional configuration fields
    pub optional_fields: Vec<ConfigField>,
    /// Example configuration
    pub example_config: HashMap<String, String>,
}

/// Configuration field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    /// Field name
    pub name: String,
    /// Field description
    pub description: String,
    /// Field type
    pub field_type: ConfigFieldType,
    /// Whether this field is sensitive (password, token, etc.)
    pub sensitive: bool,
    /// Default value if any
    pub default_value: Option<String>,
}

/// Configuration field types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigFieldType {
    String,
    Integer,
    Boolean,
    Url,
    PhoneNumber,
    Email,
    Token,
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
            config_templates: HashMap::new(),
        };

        // Register built-in adapter factories
        registry.register_builtin_factories();
        registry
    }

    /// Register a custom adapter factory
    pub fn register_factory(&mut self, factory: Box<dyn AdapterFactory>) {
        let adapter_type = factory.adapter_type().to_string();
        let config_template = factory.config_schema();

        self.config_templates
            .insert(adapter_type.clone(), config_template);
        self.factories.insert(adapter_type, factory);
    }

    /// Create an adapter from configuration
    pub fn create_adapter(
        &self,
        adapter_type: &str,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        match self.factories.get(adapter_type) {
            Some(factory) => factory.create_adapter(config),
            None => Err(BearDogError::ConfigurationError {
                message: format!("Unknown adapter type: {adapter_type}"),
            }),
        }
    }

    /// Get available adapter types
    pub fn available_adapters(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }

    /// Get configuration template for an adapter type
    pub fn get_config_template(&self, adapter_type: &str) -> Option<&AdapterConfigTemplate> {
        self.config_templates.get(adapter_type)
    }

    /// Register built-in adapter factories
    fn register_builtin_factories(&mut self) {
        self.register_factory(Box::new(DiscordFactory));
        self.register_factory(Box::new(SlackFactory));
        self.register_factory(Box::new(TeamsFactory));
        self.register_factory(Box::new(SmsFactory));
        self.register_factory(Box::new(EmailFactory));
        self.register_factory(Box::new(GenericWebhookFactory));
        self.register_factory(Box::new(MatrixFactory));
        self.register_factory(Box::new(SignalFactory));
    }
}

/// Discord adapter factory
pub struct DiscordFactory;

impl AdapterFactory for DiscordFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let webhook_url = config
            .get("webhook_url")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Discord webhook_url is required".to_string(),
            })?
            .clone();

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::discord(webhook_url),
        ))
    }

    fn adapter_type(&self) -> &str {
        "discord"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "discord".to_string(),
            description: "Discord webhook notifications".to_string(),
            required_fields: vec![ConfigField {
                name: "webhook_url".to_string(),
                description: "Discord webhook URL from channel settings".to_string(),
                field_type: ConfigFieldType::Url,
                sensitive: true,
                default_value: None,
            }],
            optional_fields: vec![],
            example_config: {
                let mut config = HashMap::new();
                config.insert(
                    "webhook_url".to_string(),
                    "https://discord.com/api/webhooks/123456789/abcdef...".to_string(),
                );
                config
            },
        }
    }
}

/// Slack adapter factory
pub struct SlackFactory;

impl AdapterFactory for SlackFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let webhook_url = config
            .get("webhook_url")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Slack webhook_url is required".to_string(),
            })?
            .clone();

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::slack(webhook_url),
        ))
    }

    fn adapter_type(&self) -> &str {
        "slack"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "slack".to_string(),
            description: "Slack webhook notifications".to_string(),
            required_fields: vec![ConfigField {
                name: "webhook_url".to_string(),
                description: "Slack incoming webhook URL".to_string(),
                field_type: ConfigFieldType::Url,
                sensitive: true,
                default_value: None,
            }],
            optional_fields: vec![],
            example_config: {
                let mut config = HashMap::new();
                config.insert(
                    "webhook_url".to_string(),
                    "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"
                        .to_string(),
                );
                config
            },
        }
    }
}

/// Microsoft Teams adapter factory
pub struct TeamsFactory;

impl AdapterFactory for TeamsFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let webhook_url = config
            .get("webhook_url")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Teams webhook_url is required".to_string(),
            })?
            .clone();

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::teams(webhook_url),
        ))
    }

    fn adapter_type(&self) -> &str {
        "teams"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "teams".to_string(),
            description: "Microsoft Teams webhook notifications".to_string(),
            required_fields: vec![ConfigField {
                name: "webhook_url".to_string(),
                description: "Microsoft Teams incoming webhook URL".to_string(),
                field_type: ConfigFieldType::Url,
                sensitive: true,
                default_value: None,
            }],
            optional_fields: vec![],
            example_config: {
                let mut config = HashMap::new();
                config.insert(
                    "webhook_url".to_string(),
                    "https://outlook.office.com/webhook/...".to_string(),
                );
                config
            },
        }
    }
}

/// SMS adapter factory
pub struct SmsFactory;

impl AdapterFactory for SmsFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let provider = config
            .get("provider")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "SMS provider is required".to_string(),
            })?;

        let api_key = config
            .get("api_key")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "SMS api_key is required".to_string(),
            })?
            .clone();

        let from_number = config.get("from_number").cloned().unwrap_or_default();

        let adapter = match provider.as_str() {
            "twilio" => {
                crate::workflows::notification::universal::SmsAdapter::twilio(api_key, from_number)
            }
            "aws_sns" => crate::workflows::notification::universal::SmsAdapter::aws_sns(api_key),
            _ => crate::workflows::notification::universal::SmsAdapter::custom(
                provider.clone(),
                api_key,
                from_number,
            ),
        };

        Ok(Box::new(adapter))
    }

    fn adapter_type(&self) -> &str {
        "sms"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "sms".to_string(),
            description: "SMS notifications via various providers".to_string(),
            required_fields: vec![
                ConfigField {
                    name: "provider".to_string(),
                    description: "SMS provider (twilio, aws_sns, custom)".to_string(),
                    field_type: ConfigFieldType::String,
                    sensitive: false,
                    default_value: Some("twilio".to_string()),
                },
                ConfigField {
                    name: "api_key".to_string(),
                    description: "API key for SMS provider".to_string(),
                    field_type: ConfigFieldType::Token,
                    sensitive: true,
                    default_value: None,
                },
            ],
            optional_fields: vec![ConfigField {
                name: "from_number".to_string(),
                description: "Sender phone number (required for some providers)".to_string(),
                field_type: ConfigFieldType::PhoneNumber,
                sensitive: false,
                default_value: None,
            }],
            example_config: {
                let mut config = HashMap::new();
                config.insert("provider".to_string(), "twilio".to_string());
                config.insert("api_key".to_string(), "your_twilio_api_key".to_string());
                config.insert("from_number".to_string(), "+1234567890".to_string());
                config
            },
        }
    }
}

/// Email adapter factory
pub struct EmailFactory;

impl AdapterFactory for EmailFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let smtp_server = config
            .get("smtp_server")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Email smtp_server is required".to_string(),
            })?
            .clone();

        let smtp_port = config
            .get("smtp_port")
            .and_then(|p| p.parse().ok())
            .unwrap_or(587);

        let username = config.get("username").cloned().unwrap_or_default();
        let password = config.get("password").cloned().unwrap_or_default();
        let from_address = config
            .get("from_address")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Email from_address is required".to_string(),
            })?
            .clone();

        let use_tls = config
            .get("use_tls")
            .and_then(|t| t.parse().ok())
            .unwrap_or(true);

        let email_config = beardog_config::integration::EmailConfig {
            enabled: true,
            smtp_server,
            smtp_port,
            username,
            password,
            from_address,
            use_tls,
        };

        Ok(Box::new(
            crate::workflows::notification::universal::EmailAdapter::new(email_config),
        ))
    }

    fn adapter_type(&self) -> &str {
        "email"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "email".to_string(),
            description: "Email notifications via SMTP".to_string(),
            required_fields: vec![
                ConfigField {
                    name: "smtp_server".to_string(),
                    description: "SMTP server hostname".to_string(),
                    field_type: ConfigFieldType::String,
                    sensitive: false,
                    default_value: Some("smtp.gmail.com".to_string()),
                },
                ConfigField {
                    name: "from_address".to_string(),
                    description: "Sender email address".to_string(),
                    field_type: ConfigFieldType::Email,
                    sensitive: false,
                    default_value: None,
                },
            ],
            optional_fields: vec![
                ConfigField {
                    name: "smtp_port".to_string(),
                    description: "SMTP server port".to_string(),
                    field_type: ConfigFieldType::Integer,
                    sensitive: false,
                    default_value: Some("587".to_string()),
                },
                ConfigField {
                    name: "username".to_string(),
                    description: "SMTP username".to_string(),
                    field_type: ConfigFieldType::String,
                    sensitive: false,
                    default_value: None,
                },
                ConfigField {
                    name: "password".to_string(),
                    description: "SMTP password".to_string(),
                    field_type: ConfigFieldType::Token,
                    sensitive: true,
                    default_value: None,
                },
                ConfigField {
                    name: "use_tls".to_string(),
                    description: "Use TLS encryption".to_string(),
                    field_type: ConfigFieldType::Boolean,
                    sensitive: false,
                    default_value: Some("true".to_string()),
                },
            ],
            example_config: {
                let mut config = HashMap::new();
                config.insert("smtp_server".to_string(), "smtp.gmail.com".to_string());
                config.insert("smtp_port".to_string(), "587".to_string());
                config.insert(
                    "from_address".to_string(),
                    "notifications@yourcompany.com".to_string(),
                );
                config.insert("username".to_string(), "your_email@gmail.com".to_string());
                config.insert("password".to_string(), "your_app_password".to_string());
                config.insert("use_tls".to_string(), "true".to_string());
                config
            },
        }
    }
}

/// Generic webhook adapter factory for any webhook service
pub struct GenericWebhookFactory;

impl AdapterFactory for GenericWebhookFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let url = config
            .get("url")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Webhook URL is required".to_string(),
            })?
            .clone();

        let auth_token = config.get("auth_token").cloned();
        let timeout_ms = config
            .get("timeout_ms")
            .and_then(|t| t.parse().ok())
            .unwrap_or(10000);

        let webhook_config = beardog_config::integration::WebhookConfig {
            enabled: true,
            url,
            auth_token,
            timeout: std::time::Duration::from_millis(timeout_ms),
        };

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::new(webhook_config),
        ))
    }

    fn adapter_type(&self) -> &str {
        "webhook"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "webhook".to_string(),
            description: "Generic webhook notifications for any service".to_string(),
            required_fields: vec![ConfigField {
                name: "url".to_string(),
                description: "Webhook URL endpoint".to_string(),
                field_type: ConfigFieldType::Url,
                sensitive: false,
                default_value: None,
            }],
            optional_fields: vec![
                ConfigField {
                    name: "auth_token".to_string(),
                    description: "Authorization token (Bearer token)".to_string(),
                    field_type: ConfigFieldType::Token,
                    sensitive: true,
                    default_value: None,
                },
                ConfigField {
                    name: "timeout_ms".to_string(),
                    description: "Request timeout in milliseconds".to_string(),
                    field_type: ConfigFieldType::Integer,
                    sensitive: false,
                    default_value: Some("10000".to_string()),
                },
            ],
            example_config: {
                let mut config = HashMap::new();
                config.insert(
                    "url".to_string(),
                    "https://your-service.com/webhook".to_string(),
                );
                config.insert(
                    "auth_token".to_string(),
                    "Bearer your_token_here".to_string(),
                );
                config.insert("timeout_ms".to_string(), "10000".to_string());
                config
            },
        }
    }
}

/// Matrix adapter factory (for Matrix/Element users)
pub struct MatrixFactory;

impl AdapterFactory for MatrixFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let homeserver = config
            .get("homeserver")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Matrix homeserver is required".to_string(),
            })?
            .clone();

        let access_token = config
            .get("access_token")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Matrix access_token is required".to_string(),
            })?
            .clone();

        // Create webhook URL for Matrix API
        let webhook_url =
            format!("{homeserver}/_matrix/client/r0/rooms/%room_id%/send/m.room.message");

        let webhook_config = beardog_config::integration::WebhookConfig {
            enabled: true,
            url: webhook_url,
            auth_token: Some(format!("Bearer {access_token}")),
            timeout: std::time::Duration::from_secs(10),
        };

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::new(webhook_config),
        ))
    }

    fn adapter_type(&self) -> &str {
        "matrix"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "matrix".to_string(),
            description: "Matrix/Element notifications".to_string(),
            required_fields: vec![
                ConfigField {
                    name: "homeserver".to_string(),
                    description: "Matrix homeserver URL".to_string(),
                    field_type: ConfigFieldType::Url,
                    sensitive: false,
                    default_value: Some("https://matrix.org".to_string()),
                },
                ConfigField {
                    name: "access_token".to_string(),
                    description: "Matrix access token".to_string(),
                    field_type: ConfigFieldType::Token,
                    sensitive: true,
                    default_value: None,
                },
            ],
            optional_fields: vec![],
            example_config: {
                let mut config = HashMap::new();
                config.insert("homeserver".to_string(), "https://matrix.org".to_string());
                config.insert(
                    "access_token".to_string(),
                    "your_matrix_access_token".to_string(),
                );
                config
            },
        }
    }
}

/// Signal adapter factory (for Signal messenger users)
pub struct SignalFactory;

impl AdapterFactory for SignalFactory {
    fn create_adapter(
        &self,
        config: HashMap<String, String>,
    ) -> BearDogResult<Box<dyn NotificationAdapter>> {
        let api_url = config
            .get("api_url")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Signal API URL is required".to_string(),
            })?
            .clone();

        let auth_token = config.get("auth_token").cloned();

        let webhook_config = beardog_config::integration::WebhookConfig {
            enabled: true,
            url: api_url,
            auth_token,
            timeout: std::time::Duration::from_secs(15),
        };

        Ok(Box::new(
            crate::workflows::notification::universal::WebhookAdapter::new(webhook_config),
        ))
    }

    fn adapter_type(&self) -> &str {
        "signal"
    }

    fn config_schema(&self) -> AdapterConfigTemplate {
        AdapterConfigTemplate {
            adapter_type: "signal".to_string(),
            description: "Signal messenger notifications (requires Signal API)".to_string(),
            required_fields: vec![ConfigField {
                name: "api_url".to_string(),
                description: "Signal API endpoint URL".to_string(),
                field_type: ConfigFieldType::Url,
                sensitive: false,
                default_value: None,
            }],
            optional_fields: vec![ConfigField {
                name: "auth_token".to_string(),
                description: "Signal API authentication token".to_string(),
                field_type: ConfigFieldType::Token,
                sensitive: true,
                default_value: None,
            }],
            example_config: {
                let mut config = HashMap::new();
                config.insert(
                    "api_url".to_string(),
                    std::env::var("BEARDOG_SMS_API_URL").unwrap_or_else(|_| {
                        beardog_config::constants::endpoints::get_api_endpoint()
                    }),
                );
                config.insert("provider".to_string(), "custom".to_string());
                config
            },
        }
    }
}
