// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use std::collections::HashMap;

pub trait Protocol: Send + Sync {
    fn name(&HashMap<&str, &str>) -> Result<(), BearDogError>;
}

#[derive(Debug, Clone)]
    pub timeout_seconds: u64,
    /// Number of max_retries
    pub max_retries: u32,
    /// The user agent value
    pub user_agent: String,
    /// Mapping of headers
    pub headers: HashMap<String, String>,
}

impl Default for HttpProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProtocol {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            version: "1.1".to_string(),
            max_retries: 3,
            user_agent: "Beardog/1.0".to_string(),
            headers: HashMap::with_capacity(&HashMap<&str, &str>) -> Result<(), BearDogError> {
        let _endpoint = params.get("endpoint").ok_or_else(|| {
            BearDogError::configuration(String,
}

impl Default for WebSocketProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketProtocol {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            version: "13".to_string() -> Self {
        Self {
            version: "2.0".to_string()?;
        Ok(())
    }
}
