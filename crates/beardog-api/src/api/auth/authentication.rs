use beardog_auth::auth::{AuthenticationRequest, AuthenticationResult};
use beardog_errors::BearDogError;
use beardog_types::canonical::{SecurityContext, SessionConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthRequest {
    pub username: String,
    pub password: String,
    pub session_config: Option<SessionConfig>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthResponse {
    pub session_token: String,
    pub expires_at: String,
    pub security_context: SecurityContext,
    pub permissions: Vec<String>,
}

pub struct AuthenticationHandler {
    auth_service: beardog_auth::auth::handlers::AuthHandler,
}

impl AuthenticationHandler {
    pub fn new() -> Self {
        Self {
            auth_service: beardog_auth::auth::handlers::AuthHandler::new(),
        }
    }

    pub async fn authenticate(&self, request: ApiAuthRequest) -> Result<ApiAuthResponse, BearDogError> {
        let auth_request = AuthenticationRequest {
            username: request.username,
            password: request.password,
            session_config: request.session_config.unwrap_or_default(),
        };

        let auth_result = self.auth_service.authenticate(auth_request).await?;

        match auth_result {
            AuthenticationResult::Success { session_token, security_context, expires_at } => {
                Ok(ApiAuthResponse {
                    session_token,
                    expires_at: expires_at.to_rfc3339(),
                    security_context,
                    permissions: vec!["read".to_string(), "write".to_string()],
                })
            }
            AuthenticationResult::Failure { reason } => {
                Err(BearDogError::authentication(format!("Authentication failed: {}", reason)))
            }
        }
    }

    pub async fn validate_session(&self, token: &str) -> Result<SecurityContext, BearDogError> {
        self.auth_service.validate_session(token).await
    }

    pub async fn refresh_session(&self, token: &str) -> Result<ApiAuthResponse, BearDogError> {
        let new_token = self.auth_service.refresh_session(token).await?;
        
        Ok(ApiAuthResponse {
            session_token: new_token,
            expires_at: chrono::Utc::now().to_rfc3339(),
            security_context: SecurityContext::default(),
            permissions: vec!["read".to_string(), "write".to_string()],
        })
    }
}

impl Default for AuthenticationHandler {
    fn default() -> Self {
        Self::new()
    }
}
