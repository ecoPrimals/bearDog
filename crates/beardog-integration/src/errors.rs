// SPDX-License-Identifier: AGPL-3.0-only

//! Integration-specific error types
//!
//! **EVOLVED**: Removed reqwest dependency (Tower Atomic uses Unix sockets, not HTTP)

use std::fmt;

/// Integration error type
#[derive(Debug)]
pub enum IntegrationError {
    /// Network/IPC error (Tower Atomic)
    Network(String),
    /// Configuration error
    Config(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationError::Network(msg) => write!(f, "Network error: {}", msg),
            IntegrationError::Config(msg) => write!(f, "Configuration error: {}", msg),
            IntegrationError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for IntegrationError {}

// Fossil Record: reqwest::Error conversion removed (Tower Atomic evolution)
// impl From<reqwest::Error> for IntegrationError {
//     fn from(e: reqwest::Error) -> Self {
//         IntegrationError::Network(e.to_string())
//     }
// }
