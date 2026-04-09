// SPDX-License-Identifier: AGPL-3.0-or-later

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
            Self::Network(msg) => write!(f, "Network error: {msg}"),
            Self::Config(msg) => write!(f, "Configuration error: {msg}"),
            Self::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for IntegrationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_error_display_network() {
        let e = IntegrationError::Network("peer reset".to_string());
        assert_eq!(e.to_string(), "Network error: peer reset");
    }

    #[test]
    fn integration_error_display_config() {
        let e = IntegrationError::Config("bad port".to_string());
        assert_eq!(e.to_string(), "Configuration error: bad port");
    }

    #[test]
    fn integration_error_display_internal() {
        let e = IntegrationError::Internal("panic avoided".to_string());
        assert_eq!(e.to_string(), "Internal error: panic avoided");
    }

    #[test]
    fn integration_error_debug_includes_variant() {
        let e = IntegrationError::Config("x".to_string());
        let s = format!("{e:?}");
        assert!(s.contains("Config"));
    }

    #[test]
    fn integration_error_source_is_none() {
        let e = IntegrationError::Network("n".to_string());
        assert!(std::error::Error::source(&e).is_none());
    }
}

// `reqwest::Error` conversion removed in Tower Atomic evolution; see git history if needed.
