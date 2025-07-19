//! SMS notification functionality
//!
//! This module handles SMS notifications through multiple providers
//! including Twilio and AWS SNS with proper configuration validation.

use super::super::types::*;
use beardog_errors::BearDogResult;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use tracing::{error, info};

impl NotificationEngine {
    /// Send SMS notification through configured provider
    pub(super) async fn send_sms_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if !self.config.sms_enabled {
            return Ok(());
        }

        let provider = self.config.sms_provider.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS provider not configured".to_string()
            }
        })?;

        let api_key = self.config.sms_api_key.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS API key not configured".to_string()
            }
        })?;

        let phone_number = self.config.sms_phone_number.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS phone number not configured".to_string()
            }
        })?;

        let sms_body = self.format_sms_body(message, metadata);

        match provider.as_str() {
            "twilio" => self.send_twilio_sms(api_key, phone_number, &sms_body).await,
            "aws_sns" => self.send_aws_sns_sms(api_key, phone_number, &sms_body).await,
            _ => {
                error!("❌ Unsupported SMS provider: {}", provider);
                Err(beardog_errors::BearDogError::Configuration {
                    message: format!("Unsupported SMS provider: {provider}")
                })
            }
        }
    }

    /// Test SMS configuration
    pub(super) async fn test_sms_config(&self) -> BearDogResult<()> {
        if !self.config.sms_enabled {
            return Ok(());
        }
        
        self.config.sms_provider.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS provider not configured".to_string(),
            }
        })?;
        
        self.config.sms_api_key.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS API key not configured".to_string(),
            }
        })?;
        
        self.config.sms_phone_number.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS phone number not configured".to_string(),
            }
        })?;

        Ok(())
    }

    // Private helper methods for SMS functionality

    fn format_sms_body(&self, message: &str, metadata: &HashMap<String, serde_json::Value>) -> String {
        format!(
            "BearDog Security: {}\nTime: {}\nRef: {}",
            message,
            chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"),
            metadata.get("incident_id").and_then(|v| v.as_str()).unwrap_or("N/A")
        )
    }

    /// Send SMS via Twilio API
    async fn send_twilio_sms(&self, api_key: &str, phone_number: &str, message: &str) -> BearDogResult<()> {
        let client = Client::new();
        let auth = general_purpose::STANDARD.encode(format!("{api_key}:{api_key}"));
        
        let payload = json!({
            "To": phone_number,
            "From": self.config.sms_from_number.as_ref().unwrap_or(&"+1234567890".to_string()),
            "Body": message
        });

        let response = client
            .post("https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json")
            .header("Authorization", format!("Basic {auth}"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&payload)
            .send()
            .await
            .map_err(|e| beardog_errors::BearDogError::Network {
                message: format!("Failed to connect to Twilio: {e}")
            })?;

        if response.status().is_success() {
            info!("✅ SMS sent successfully via Twilio");
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            error!("❌ Failed to send SMS via Twilio: {}", error_body);
            Err(beardog_errors::BearDogError::External {
                message: format!("Failed to send SMS via Twilio: {error_body}")
            })
        }
    }

    /// Send SMS via AWS SNS (placeholder implementation)
    async fn send_aws_sns_sms(&self, _api_key: &str, phone_number: &str, message: &str) -> BearDogResult<()> {
        // This is a placeholder implementation - in production would use AWS SDK
        info!("📱 SMS notification prepared for AWS SNS");
        info!("  Phone: {}", phone_number);
        info!("  Message: {}", message);
        
        // TODO: Implement AWS SNS integration using aws-sdk-sns
        // Example:
        // let config = aws_config::load_from_env().await;
        // let client = aws_sdk_sns::Client::new(&config);
        // let result = client.publish()
        //     .phone_number(phone_number)
        //     .message(message)
        //     .send()
        //     .await?;
        
        Ok(())
    }
} 