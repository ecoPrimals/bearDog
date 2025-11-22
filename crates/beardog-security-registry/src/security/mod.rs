//! Security module for BearDog security registry
//!
//! Provides security entry management and token handling.

use std::collections::HashMap;

mod crypto_keys;

// pub use crypto_keys::*; // Unused - commented out

/// Security entry containing trust information
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used in future implementation
pub struct SecurityEntry {
    /// Trust level for this entry
    pub trust_level: crate::TrustLevel,
}

/// Security tokens for authentication
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used in future implementation
pub struct SecurityTokens {
    tokens: HashMap<String, String>,
}

#[allow(dead_code)] // Used in future implementation
impl SecurityTokens {
    /// Creates a new security tokens instance
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Adds a token
    pub fn add_token(&mut self, key: String, value: String) {
        self.tokens.insert(key, value);
    }

    /// Gets a token
    pub fn get_token(&self, key: &str) -> Option<&String> {
        self.tokens.get(key)
    }
}

impl Default for SecurityTokens {
    fn default() -> Self {
        Self::new()
    }
}
