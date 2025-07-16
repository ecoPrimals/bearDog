//! Request handlers for BearDog PrimalProvider
//!
//! This module contains all request handling logic for different security operations
//! including encryption, decryption, authentication, and authorization.

use std::collections::HashMap;
use serde_json::json;

use super::super::traits::*;
use super::super::request_types;
use super::core::BearDogPrimalProvider;
use crate::security::types::SecurityProvider;
use crate::{BearDogError, BearDogResult};

impl BearDogPrimalProvider {
    /// Handle security-specific requests
    pub async fn handle_security_request(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        match request.request_type.as_str() {
            request_types::SECURITY_ENCRYPT => self.handle_encrypt_request(request).await,
            request_types::SECURITY_DECRYPT => self.handle_decrypt_request(request).await,
            request_types::SECURITY_AUTHENTICATE => self.handle_auth_request(request).await,
            request_types::SECURITY_AUTHORIZE => self.handle_authz_request(request).await,
            _ => Err(BearDogError::internal(format!(
                "Unsupported security request type: {}",
                request.request_type
            ))),
        }
    }

    /// Handle encryption requests
    pub async fn handle_encrypt_request(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        // Extract data from request
        let data = request
            .payload
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "data",
                    "Missing 'data' field in encryption request",
                )
            })?;

        // Use actual BearDog encryption engine
        let result = self.core.encryption_engine()
            .encrypt(data.as_bytes(), Some(self.get_encryption_context().await?))
            .await?;
        
        Ok(ServiceResponse {
            request_id: request.request_id,
            success: true,
            payload: json!({
                "encrypted_data": result.ciphertext,
                "algorithm": result.algorithm,
                "key_id": result.key_id.unwrap_or_else(|| "default".to_string())
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            error: None,
        })
    }

    /// Handle decryption requests
    pub async fn handle_decrypt_request(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        // Extract encrypted data from request
        let encrypted_data = request
            .payload
            .get("encrypted_data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "encrypted_data",
                    "Missing 'encrypted_data' field in decryption request",
                )
            })?;

        // Use actual BearDog decryption engine
        let encrypted_data_struct = crate::encryption::EncryptedData {
            ciphertext: encrypted_data.as_bytes().to_vec(),
            nonce: self.get_current_nonce().await?,
            algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
            key_id: Some("default".to_string()),
            tag: Some(vec![0u8; 16]), // Placeholder tag
            metadata: HashMap::new(),
        };
        
        let result = self.core.encryption_engine()
            .decrypt(&encrypted_data_struct)
            .await?;
        
        Ok(ServiceResponse {
            request_id: request.request_id,
            success: true,
            payload: json!({
                "decrypted_data": String::from_utf8(result).unwrap_or_else(|_| "invalid_utf8".to_string()),
                "algorithm": "AES-256-GCM"
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            error: None,
        })
    }

    /// Handle authentication requests
    pub async fn handle_auth_request(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        // Extract credentials from request
        let username = request
            .payload
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "username",
                    "Missing 'username' field in authentication request",
                )
            })?;

        let password = request
            .payload
            .get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "password",
                    "Missing 'password' field in authentication request",
                )
            })?;

        // Use actual BearDog authentication
        let result = self.core.security_provider()
            .authenticate(username, password)
            .await?;
        
        Ok(ServiceResponse {
            request_id: request.request_id,
            success: true,
            payload: json!({
                "authenticated": result.success,
                "user_id": result.user_id,
                "token": result.session_id,
                "expires_at": result.expires_at.map(|dt| dt.to_rfc3339()).unwrap_or_else(|| "".to_string())
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            error: None,
        })
    }

    /// Handle authorization requests
    pub async fn handle_authz_request(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        // Extract authorization data from request
        let resource = request
            .payload
            .get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "resource",
                    "Missing 'resource' field in authorization request",
                )
            })?;

        let action = request
            .payload
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "action",
                    "Missing 'action' field in authorization request",
                )
            })?;

        // Create authorization context
        let subject = crate::security::types::Subject {
            id: "token-123".to_string(), // Placeholder token
            subject_type: crate::security::types::SubjectType::User,
            roles: vec![], // Would be populated from token
            attributes: HashMap::new(),
            clearance_level: None,
        };
        
        let resource_obj = crate::security::types::Resource {
            id: resource.to_string(),
            resource_type: "data".to_string(),
            classification: crate::security::types::ResourceClassification::Internal,
            attributes: HashMap::new(),
            owner: Some("system".to_string()),
        };
        
        let action_obj = crate::security::types::Action {
            action_type: match action {
                "read" => crate::security::types::ActionType::Read,
                "write" => crate::security::types::ActionType::Write,
                "delete" => crate::security::types::ActionType::Delete,
                "execute" => crate::security::types::ActionType::Execute,
                "admin" => crate::security::types::ActionType::Admin,
                _ => crate::security::types::ActionType::Read,
            },
            context: HashMap::new(),
            timestamp: chrono::Utc::now(),
            source_ip: None,
        };
        
        // Use actual BearDog authorization
        let result = self.core.security_provider()
            .authorize(&subject, &resource_obj, &action_obj)
            .await?;
        
        Ok(ServiceResponse {
            request_id: request.request_id,
            success: true,
            payload: json!({
                "authorized": result.permitted,
                "resource": resource,
                "action": action,
                "reason": result.reason
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            error: None,
        })
    }
} 