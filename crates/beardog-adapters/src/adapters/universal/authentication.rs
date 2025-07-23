//! Authentication Implementations for Universal Adapter
//!
//! Authentication-agnostic layer for external systems

use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;

/// Authentication trait for universal adapter
pub trait Authentication: Send + Sync {
    /// Authentication method name
    fn method(&self) -> &str;

    /// Validate authentication parameters
    fn validate(&self, params: &HashMap<String, String>) -> BearDogResult<()>;

    /// Get authentication headers for HTTP requests
    fn get_headers(&self) -> HashMap<String, String>;
}

/// No authentication - for systems that don't require auth
pub struct NoAuthentication;

impl Default for NoAuthentication {
    fn default() -> Self {
        Self::new()
    }
}

impl NoAuthentication {
    pub fn new() -> Self {
        Self
    }
}

impl Authentication for NoAuthentication {
    fn method(&self) -> &str {
        "none"
    }

    fn validate(&self, _params: &HashMap<String, String>) -> BearDogResult<()> {
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

/// API Key authentication
pub struct ApiKeyAuthentication {
    api_key: String,
    header_name: String,
}

impl ApiKeyAuthentication {
    pub fn new(api_key: String, header_name: Option<String>) -> Self {
        Self {
            api_key,
            header_name: header_name.unwrap_or_else(|| "Authorization".to_string()),
        }
    }
}

impl Authentication for ApiKeyAuthentication {
    fn method(&self) -> &str {
        "api_key"
    }

    fn validate(&self, _params: &HashMap<String, String>) -> BearDogResult<()> {
        if self.api_key.is_empty() {
            return Err(BearDogError::ConfigurationError {
                message: "API key cannot be empty".to_string(),
            });
        }
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(self.header_name.clone(), format!("Bearer {}", self.api_key));
        headers
    }
}

/// Bearer token authentication
pub struct BearerTokenAuthentication {
    token: String,
}

impl BearerTokenAuthentication {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Authentication for BearerTokenAuthentication {
    fn method(&self) -> &str {
        "bearer_token"
    }

    fn validate(&self, _params: &HashMap<String, String>) -> BearDogResult<()> {
        if self.token.is_empty() {
            return Err(BearDogError::ConfigurationError {
                message: "Bearer token cannot be empty".to_string(),
            });
        }
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.token),
        );
        headers
    }
}

/// Kubernetes config-based authentication
pub struct KubeconfigAuthentication {
    #[allow(dead_code)] // Will be used when kubeconfig authentication is fully implemented
    config_path: String,
}

impl Default for KubeconfigAuthentication {
    fn default() -> Self {
        Self::new()
    }
}

impl KubeconfigAuthentication {
    pub fn new() -> Self {
        Self {
            config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
        }
    }

    pub fn with_path(config_path: String) -> Self {
        Self { config_path }
    }
}

impl Authentication for KubeconfigAuthentication {
    fn method(&self) -> &str {
        "kubeconfig"
    }

    fn validate(&self, _params: &HashMap<String, String>) -> BearDogResult<()> {
        // Could validate kubeconfig exists and is readable
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        // kubectl handles authentication internally
        HashMap::new()
    }
}
