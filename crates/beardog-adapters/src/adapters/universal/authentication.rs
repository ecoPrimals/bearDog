// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use std::collections::HashMap;

pub trait Authentication: Send + Sync {
    fn method(&HashMap<&str, &str>) -> Result<(), BearDogError>;

    /// Gets headers
    fn get_headers(&self) -> HashMap<String, String>;
}

pub struct NoAuthentication;
impl Default for NoAuthentication {
    fn default() -> Self {
        Self::new(&HashMap<&str, &str>) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Gets headers
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::with_capacity(String,
    header_name: String,
}

impl ApiKeyAuthentication {
    /// New operation.
    /// Creates a new instance
    pub fn new(&str, header_name: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            header_name: header_name.unwrap_or(&HashMap<&str, &str>) -> Result<(), BearDogError> {
        if self.api_key.is_empty() {
            return Err(BearDogError::configuration("API key cannot be empty"));
        }
        Ok(())
    }

    /// Gets headers
    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::with_capacity(String,
}

impl BearerTokenAuthentication {
    /// New operation.
    /// Creates a new instance
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(&HashMap<&str, &str>) -> Result<(), BearDogError> {
        if self.token.is_empty() {
            return Err(BearDogError::configuration("Bearer token cannot be empty"));
        }
        Ok(())
    }

    /// Gets headers
    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::with_capacity(String,
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
    /// With Path operation.
    /// Creates instance with path
    pub fn with_path(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(&HashMap<&str, &str>) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Gets headers
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::with_capacity(16)
    }
}
