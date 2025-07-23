//! Security Provider Bridge for Universal Service Mesh Integration
//!
//! This bridge connects BearDog's SecurityProvider to the universal capability adapter,
//! allowing BearDog to provide security services to any service mesh that implements
//! the Universal Primal Architecture Standard.
//!
//! ## Core Principle
//!
//! BearDog knows only its security capabilities. It exposes them through this bridge
//! without knowing or caring which service mesh will consume them.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::universal::{
    capability_adapter::{ServiceCapability, UniversalServiceProvider},
    http_adapter::{ErrorInfo, ResponseStatus, UniversalRequest, UniversalResponse},
};
use crate::{BearDogError, BearDogResult};
use base64::{engine::general_purpose, Engine as _};
use beardog_security::{
    Action, ActionType, BearDogSecurityProvider, Resource, ResourceClassification, RiskLevel,
    SecurityAuditEvent, SecurityProvider, SecurityProviderConfig, SecuritySession, Subject,
    SubjectType,
};

/// Bridge between BearDog's SecurityProvider and Universal Service Mesh
pub struct SecurityProviderBridge {
    /// BearDog's security provider implementation
    security_provider: Arc<BearDogSecurityProvider>,
    /// Bridge configuration
    config: BridgeConfig,
    /// Active security sessions managed by the bridge
    sessions: Arc<RwLock<HashMap<String, SecuritySession>>>,
}

/// Configuration for the security provider bridge
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Enable authentication services
    pub enable_authentication: bool,
    /// Enable authorization services
    pub enable_authorization: bool,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Enable cryptographic operations
    pub enable_crypto_operations: bool,
    /// Enable key management
    pub enable_key_management: bool,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            enable_authentication: true,
            enable_authorization: true,
            enable_audit_logging: true,
            enable_crypto_operations: true,
            enable_key_management: true,
            max_concurrent_sessions: 10000,
        }
    }
}

impl SecurityProviderBridge {
    /// Create new security provider bridge
    pub async fn new(config: BridgeConfig) -> BearDogResult<Self> {
        // Initialize BearDog's security provider with default configuration
        let security_config = SecurityProviderConfig::default();
        let security_provider = Arc::new(
            BearDogSecurityProvider::new_with_config(security_config)
                .await
                .map_err(|e| BearDogError::Configuration {
                    message: format!("Failed to initialize security provider: {e}"),
                })?,
        );

        Ok(Self {
            security_provider,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create with existing security provider
    pub fn with_provider(
        security_provider: Arc<BearDogSecurityProvider>,
        config: BridgeConfig,
    ) -> Self {
        Self {
            security_provider,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle authentication request through universal interface
    async fn handle_authenticate_request(&self, request: UniversalRequest) -> UniversalResponse {
        if !self.config.enable_authentication {
            return self.error_response(
                request.request_id,
                "authentication_disabled",
                "Authentication services are not enabled",
            );
        }

        // Extract credentials from request parameters
        let credentials = match self.extract_credentials(&request.parameters) {
            Ok(creds) => creds,
            Err(e) => {
                return self.error_response(
                    request.request_id,
                    "invalid_credentials",
                    &format!("Failed to extract credentials: {e}"),
                )
            }
        };

        // Call BearDog's authentication
        match self.security_provider.authenticate(&credentials).await {
            Ok(auth_result) => {
                let success = auth_result.success && auth_result.authenticated;

                // Store session if authentication successful
                if success {
                    if let (Some(user_id), Some(session_id)) =
                        (&auth_result.user_id, &auth_result.session_id)
                    {
                        let session = SecuritySession {
                            id: session_id.clone(),
                            user_id: user_id.clone(),
                            is_active: true,
                            created_at: chrono::Utc::now(),
                            expires_at: auth_result.expires_at.unwrap_or_else(|| {
                                chrono::Utc::now() + chrono::Duration::hours(24)
                            }),
                            client_ip: None, // Can be extracted from request metadata if needed
                            user_agent: None,
                            permissions: vec![], // Would be populated from user info
                        };

                        let mut sessions = self.sessions.write().await;
                        sessions.insert(session_id.clone(), session);
                    }
                }

                UniversalResponse {
                    request_id: request.request_id,
                    status: if success {
                        ResponseStatus::Success
                    } else {
                        ResponseStatus::Error
                    },
                    data: Some(json!({
                        "authenticated": auth_result.authenticated,
                        "user_id": auth_result.user_id,
                        "session_id": auth_result.session_id,
                        "session_token": auth_result.session_token,
                        "mfa_required": auth_result.mfa_required,
                        "mfa_methods": auth_result.mfa_methods,
                        "expires_at": auth_result.expires_at,
                        "reason": auth_result.reason
                    })),
                    error: if success {
                        None
                    } else {
                        Some(ErrorInfo {
                            code: "authentication_failed".to_string(),
                            message: auth_result.reason,
                            details: auth_result.error.map(|e| json!({"error": e})),
                        })
                    },
                }
            }
            Err(e) => self.error_response(
                request.request_id,
                "authentication_error",
                &format!("Authentication failed: {e:?}"),
            ),
        }
    }

    /// Handle authorization request through universal interface
    async fn handle_authorize_request(&self, request: UniversalRequest) -> UniversalResponse {
        if !self.config.enable_authorization {
            return self.error_response(
                request.request_id,
                "authorization_disabled",
                "Authorization services are not enabled",
            );
        }

        // Extract authorization parameters
        let (subject, action, resource) =
            match self.extract_authorization_params(&request.parameters) {
                Ok(params) => params,
                Err(e) => {
                    return self.error_response(
                        request.request_id,
                        "invalid_auth_params",
                        &format!("Failed to extract authorization parameters: {e}"),
                    )
                }
            };

        // Call BearDog's authorization
        match self
            .security_provider
            .authorize(&subject, &action, &resource)
            .await
        {
            Ok(auth_result) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "authorized": auth_result.authorized,
                    "permitted": auth_result.permitted,
                    "reason": auth_result.reason,
                    "risk_level": auth_result.risk_level,
                    "additional_requirements": auth_result.additional_requirements,
                    "audit_id": auth_result.audit_id
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "authorization_error",
                &format!("Authorization failed: {e:?}"),
            ),
        }
    }

    /// Handle audit logging request through universal interface
    async fn handle_audit_request(&self, request: UniversalRequest) -> UniversalResponse {
        if !self.config.enable_audit_logging {
            return self.error_response(
                request.request_id,
                "audit_disabled",
                "Audit logging services are not enabled",
            );
        }

        // Extract audit event from parameters
        let audit_event = match self.extract_audit_event(&request.parameters) {
            Ok(event) => event,
            Err(e) => {
                return self.error_response(
                    request.request_id,
                    "invalid_audit_event",
                    &format!("Failed to extract audit event: {e}"),
                )
            }
        };

        // Log the audit event
        match self.security_provider.audit(audit_event).await {
            Ok(()) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "logged": true,
                    "timestamp": chrono::Utc::now()
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "audit_error",
                &format!("Audit logging failed: {e:?}"),
            ),
        }
    }

    /// Handle cryptographic operations (signing, verification, etc.)
    async fn handle_crypto_request(&self, request: UniversalRequest) -> UniversalResponse {
        if !self.config.enable_crypto_operations {
            return self.error_response(
                request.request_id,
                "crypto_disabled",
                "Cryptographic operations are not enabled",
            );
        }

        // Route to specific crypto operation based on operation type
        match request.operation.as_str() {
            "ed25519_sign" => self.handle_ed25519_sign(&request).await,
            "ed25519_verify" => self.handle_ed25519_verify(&request).await,
            "aes_encrypt" => self.handle_aes_encrypt(&request).await,
            "aes_decrypt" => self.handle_aes_decrypt(&request).await,
            _ => self.error_response(
                request.request_id,
                "unsupported_crypto_operation",
                &format!("Unsupported cryptographic operation: {}", request.operation),
            ),
        }
    }

    /// Handle key management operations
    async fn handle_key_management_request(&self, request: UniversalRequest) -> UniversalResponse {
        if !self.config.enable_key_management {
            return self.error_response(
                request.request_id,
                "key_management_disabled",
                "Key management services are not enabled",
            );
        }

        match request.operation.as_str() {
            "generate_key" => self.handle_generate_key(&request).await,
            "derive_key" => self.handle_derive_key(&request).await,
            "generate_address" => self.handle_generate_address(&request).await,
            _ => self.error_response(
                request.request_id,
                "unsupported_key_operation",
                &format!(
                    "Unsupported key management operation: {}",
                    request.operation
                ),
            ),
        }
    }

    // Private helper methods

    /// Extract credentials from request parameters
    fn extract_credentials(
        &self,
        params: &HashMap<String, Value>,
    ) -> BearDogResult<HashMap<String, String>> {
        let mut credentials = HashMap::new();

        if let Some(username) = params.get("username").and_then(|v| v.as_str()) {
            credentials.insert("username".to_string(), username.to_string());
        }

        if let Some(password) = params.get("password").and_then(|v| v.as_str()) {
            credentials.insert("password".to_string(), password.to_string());
        }

        if let Some(token) = params.get("token").and_then(|v| v.as_str()) {
            credentials.insert("token".to_string(), token.to_string());
        }

        if credentials.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "No valid credentials found in request".to_string(),
            });
        }

        Ok(credentials)
    }

    /// Extract authorization parameters from request
    fn extract_authorization_params(
        &self,
        params: &HashMap<String, Value>,
    ) -> BearDogResult<(Subject, Action, Resource)> {
        // Extract subject
        let subject_id = params
            .get("subject_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing subject_id parameter".to_string(),
            })?;

        let subject = Subject {
            id: subject_id.to_string(),
            name: subject_id.to_string(),    // Use ID as name for now
            subject_type: SubjectType::User, // Default to User
            roles: vec![],                   // Default empty roles
            clearance_level: None,
            metadata: HashMap::new(),
        };

        // Extract action
        let action_type = params
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing action parameter".to_string(),
            })?;

        let action = Action {
            action_type: ActionType::Read, // Would parse from action_type
            description: action_type.to_string(),
            risk_level: RiskLevel::Low, // Default risk level
            timestamp: chrono::Utc::now(),
        };

        // Extract resource
        let resource_id = params
            .get("resource_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing resource_id parameter".to_string(),
            })?;

        let resource = Resource {
            id: resource_id.to_string(),
            name: params
                .get("resource_name")
                .and_then(|v| v.as_str())
                .unwrap_or(resource_id)
                .to_string(),
            classification: ResourceClassification::Public, // Default
            metadata: HashMap::new(),
        };

        Ok((subject, action, resource))
    }

    /// Extract audit event from request parameters
    fn extract_audit_event(
        &self,
        params: &HashMap<String, Value>,
    ) -> BearDogResult<SecurityAuditEvent> {
        let event_type = params
            .get("event_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidInput {
                message: "Missing event_type parameter".to_string(),
            })?;

        let user_id = params
            .get("user_id")
            .and_then(|v| v.as_str())
            .unwrap_or("system");

        let resource_id = params
            .get("resource_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let message = params.get("message").and_then(|v| v.as_str()).unwrap_or("");

        // Create audit event (simplified)
        let audit_event = SecurityAuditEvent {
            id: Uuid::new_v4().to_string(),
            event_id: Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            subject: user_id.to_string(),
            resource: resource_id.to_string(),
            action: Action {
                action_type: ActionType::Read, // Default
                description: message.to_string(),
                risk_level: RiskLevel::Low,
                timestamp: chrono::Utc::now(),
            },
            success: true, // Default to success
            result: true,
            risk_level: RiskLevel::Low,
            details: HashMap::new(),
            metadata: HashMap::new(), // Convert from serde_json::Value to HashMap<String, String>
            timestamp: chrono::Utc::now(),
        };

        Ok(audit_event)
    }

    /// Create error response
    fn error_response(&self, request_id: Uuid, code: &str, message: &str) -> UniversalResponse {
        UniversalResponse {
            request_id,
            status: ResponseStatus::Error,
            data: None,
            error: Some(ErrorInfo {
                code: code.to_string(),
                message: message.to_string(),
                details: None,
            }),
        }
    }

    // Cryptographic operation handlers (stubs for now)
    async fn handle_ed25519_sign(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract private key and message from request parameters
        let private_key = match request
            .parameters
            .get("private_key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid private_key parameter",
                )
            }
        };

        let message = match request
            .parameters
            .get("message")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(msg) => msg,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid message parameter",
                )
            }
        };

        match BearDogCrypto::sign_ed25519(&private_key, &message) {
            Ok(signature) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "signature": general_purpose::STANDARD.encode(signature)
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("Ed25519 signing failed: {e:?}"),
            ),
        }
    }

    async fn handle_ed25519_verify(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract public key, message, and signature from request parameters
        let public_key = match request
            .parameters
            .get("public_key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid public_key parameter",
                )
            }
        };

        let message = match request
            .parameters
            .get("message")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(msg) => msg,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid message parameter",
                )
            }
        };

        let signature = match request
            .parameters
            .get("signature")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(sig) => sig,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid signature parameter",
                )
            }
        };

        match BearDogCrypto::verify_ed25519_signature(&public_key, &message, &signature) {
            Ok(is_valid) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "valid": is_valid
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("Ed25519 verification failed: {e:?}"),
            ),
        }
    }

    async fn handle_aes_encrypt(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract key and plaintext from request parameters
        let key = match request
            .parameters
            .get("key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(k) => k,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid key parameter",
                )
            }
        };

        let plaintext = match request
            .parameters
            .get("plaintext")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(pt) => pt,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid plaintext parameter",
                )
            }
        };

        match BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, None) {
            Ok((ciphertext, nonce)) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "ciphertext": general_purpose::STANDARD.encode(ciphertext),
                    "nonce": general_purpose::STANDARD.encode(nonce)
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("AES encryption failed: {e:?}"),
            ),
        }
    }

    async fn handle_aes_decrypt(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract key, ciphertext, and nonce from request parameters
        let key = match request
            .parameters
            .get("key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(k) => k,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid key parameter",
                )
            }
        };

        let ciphertext = match request
            .parameters
            .get("ciphertext")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(ct) => ct,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid ciphertext parameter",
                )
            }
        };

        let nonce = match request
            .parameters
            .get("nonce")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(n) => n,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid nonce parameter",
                )
            }
        };

        match BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce) {
            Ok(plaintext) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "plaintext": general_purpose::STANDARD.encode(plaintext)
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("AES decryption failed: {e:?}"),
            ),
        }
    }

    async fn handle_generate_key(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract key type from request parameters
        let key_type = match request.parameters.get("key_type").and_then(|v| v.as_str()) {
            Some(kt) => kt,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing key_type parameter",
                )
            }
        };

        match key_type {
            "ed25519" => match BearDogCrypto::generate_ed25519_keypair() {
                Ok((private_key, public_key)) => UniversalResponse {
                    request_id: request.request_id,
                    status: ResponseStatus::Success,
                    data: Some(json!({
                        "private_key": general_purpose::STANDARD.encode(private_key),
                        "public_key": general_purpose::STANDARD.encode(public_key)
                    })),
                    error: None,
                },
                Err(e) => self.error_response(
                    request.request_id,
                    "crypto_error",
                    &format!("Ed25519 key generation failed: {e:?}"),
                ),
            },
            "aes256" => match BearDogCrypto::secure_random_bytes(32) {
                Ok(key) => UniversalResponse {
                    request_id: request.request_id,
                    status: ResponseStatus::Success,
                    data: Some(json!({
                        "key": general_purpose::STANDARD.encode(key)
                    })),
                    error: None,
                },
                Err(e) => self.error_response(
                    request.request_id,
                    "crypto_error",
                    &format!("AES256 key generation failed: {e:?}"),
                ),
            },
            _ => self.error_response(
                request.request_id,
                "invalid_parameters",
                &format!("Unsupported key type: {key_type}"),
            ),
        }
    }

    async fn handle_derive_key(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::crypto_utils::BearDogCrypto;

        // Extract master key and derivation path from request parameters
        let master_key = match request
            .parameters
            .get("master_key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid master_key parameter",
                )
            }
        };

        let derivation_path = match request
            .parameters
            .get("derivation_path")
            .and_then(|v| v.as_str())
        {
            Some(path) => path,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing derivation_path parameter",
                )
            }
        };

        // Use path as salt for PBKDF2 key derivation
        let salt = derivation_path.as_bytes();
        match BearDogCrypto::derive_key_pbkdf2(&master_key, salt, 100_000, 32) {
            Ok(derived_key) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "derived_key": general_purpose::STANDARD.encode(derived_key)
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("Key derivation failed: {e:?}"),
            ),
        }
    }

    async fn handle_generate_address(&self, request: &UniversalRequest) -> UniversalResponse {
        use beardog_security::address_management::{AddressFormat, AddressManager};

        // Extract public key and address type from request parameters
        let public_key = match request
            .parameters
            .get("public_key")
            .and_then(|v| v.as_str())
            .and_then(|s| general_purpose::STANDARD.decode(s).ok())
        {
            Some(key) => key,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing or invalid public_key parameter",
                )
            }
        };

        let address_type = match request
            .parameters
            .get("address_type")
            .and_then(|v| v.as_str())
        {
            Some(at) => at,
            None => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    "Missing address_type parameter",
                )
            }
        };

        let format = match address_type {
            "bitcoin" => AddressFormat::BitcoinLegacy,
            "ethereum" => AddressFormat::Ethereum,
            "beardog" => AddressFormat::BearDogNative,
            _ => {
                return self.error_response(
                    request.request_id,
                    "invalid_parameters",
                    &format!("Unsupported address type: {address_type}"),
                )
            }
        };

        let mut address_manager = AddressManager::new();
        match address_manager.generate_address_from_ed25519(&public_key, format) {
            Ok(address) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: Some(json!({
                    "address": address
                })),
                error: None,
            },
            Err(e) => self.error_response(
                request.request_id,
                "crypto_error",
                &format!("Address generation failed: {e:?}"),
            ),
        }
    }
}

#[async_trait::async_trait]
impl UniversalServiceProvider for SecurityProviderBridge {
    /// Get BearDog's security capabilities
    fn get_capabilities(&self) -> Vec<ServiceCapability> {
        let mut capabilities = Vec::new();

        if self.config.enable_authentication || self.config.enable_authorization {
            capabilities.push(ServiceCapability::Security {
                functions: vec![
                    "authenticate".to_string(),
                    "authorize".to_string(),
                    "create_session".to_string(),
                    "validate_session".to_string(),
                    "revoke_session".to_string(),
                ],
                compliance: vec!["memory_safe".to_string(), "audit_logged".to_string()],
                trust_levels: vec!["high".to_string(), "maximum".to_string()],
            });
        }

        if self.config.enable_crypto_operations {
            capabilities.push(ServiceCapability::Security {
                functions: vec![
                    "ed25519_sign".to_string(),
                    "ed25519_verify".to_string(),
                    "aes_encrypt".to_string(),
                    "aes_decrypt".to_string(),
                ],
                compliance: vec!["cryptographically_secure".to_string()],
                trust_levels: vec!["maximum".to_string()],
            });
        }

        if self.config.enable_key_management {
            capabilities.push(ServiceCapability::Security {
                functions: vec![
                    "generate_key".to_string(),
                    "derive_key".to_string(),
                    "generate_address".to_string(),
                ],
                compliance: vec!["hierarchical_deterministic".to_string()],
                trust_levels: vec!["maximum".to_string()],
            });
        }

        if self.config.enable_audit_logging {
            capabilities.push(ServiceCapability::Security {
                functions: vec![
                    "audit_log".to_string(),
                    "security_event_logging".to_string(),
                ],
                compliance: vec!["tamper_resistant".to_string()],
                trust_levels: vec!["high".to_string()],
            });
        }

        capabilities
    }

    /// Handle universal security requests
    async fn handle_request(&self, request: UniversalRequest) -> UniversalResponse {
        match request.operation.as_str() {
            // Authentication and authorization
            "authenticate" => self.handle_authenticate_request(request).await,
            "authorize" => self.handle_authorize_request(request).await,

            // Audit logging
            "audit_log" => self.handle_audit_request(request).await,

            // Cryptographic operations
            "ed25519_sign" | "ed25519_verify" | "aes_encrypt" | "aes_decrypt" => {
                self.handle_crypto_request(request).await
            }

            // Key management
            "generate_key" | "derive_key" | "generate_address" => {
                self.handle_key_management_request(request).await
            }

            _ => self.error_response(
                request.request_id,
                "unsupported_operation",
                &format!(
                    "Operation '{}' not supported by security provider",
                    request.operation
                ),
            ),
        }
    }

    /// Health check - delegate to security provider
    async fn health_check(&self) -> bool {
        match self.security_provider.health_check().await {
            Ok(health) => health.overall_status == beardog_security::HealthStatus::Healthy,
            Err(_) => false,
        }
    }
}
