use beardog_errors::BearDogError;
use std::collections::HashMap;

pub trait Authentication: Send + Sync {
    fn method(&self) -> &str;

    fn validate(&self, params: &HashMap<&str, &str>) -> Result<(), BearDogError>;

    fn get_headers(&self) -> HashMap<String, String>;
}

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

    fn validate(&self, _params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::with_capacity(16)
    }
}

pub struct ApiKeyAuthentication {
    api_key: String,
    header_name: String,
}

impl ApiKeyAuthentication {
    pub fn new(api_key: &str, header_name: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            header_name: header_name.unwrap_or("Authorization").to_string(),
        }
    }
}

impl Authentication for ApiKeyAuthentication {
    fn method(&self) -> &str {
        "api_key"
    }

    fn validate(&self, _params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        if self.api_key.is_empty() {
            return Err(BearDogError::configuration(
                "API key cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::with_capacity(16);
        headers.insert(self.header_name.clone(), format!("Bearer {}", self.api_key));
        headers
    }
}

pub struct BearerTokenAuthentication {
    token: String,
}

impl BearerTokenAuthentication {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

impl Authentication for BearerTokenAuthentication {
    fn method(&self) -> &str {
        "bearer_token"
    }

    fn validate(&self, _params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        if self.token.is_empty() {
            return Err(BearDogError::configuration(
                "Bearer token cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::with_capacity(16);
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.token),
        );
        headers
    }
}

#[derive(Debug)]
pub struct KubeconfigAuthentication {
    pub config_path: String,
}

impl Default for KubeconfigAuthentication {
    fn default() -> Self {
        Self {
            config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
        }
    }
}

impl KubeconfigAuthentication {
    pub fn with_path(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
        }
    }
}

impl Authentication for KubeconfigAuthentication {
    fn method(&self) -> &str {
        "kubeconfig"
    }

    fn validate(&self, _params: &HashMap<&str, &str>) -> Result<(), BearDogError> {
        Ok(())
    }

    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::with_capacity(16)
    }
}
