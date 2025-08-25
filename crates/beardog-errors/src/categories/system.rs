// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// System Error Categories
///
/// **SYSTEM AND INFRASTRUCTURE ERROR TYPES**
/// This module contains all error types related to system operations including
/// network, storage, configuration, and general system errors.
use thiserror::Error;
pub use crate::error_types::ErrorSeverity;

/// System and infrastructure error types
#[derive(Error, Debug, Clone)]
pub enum SystemError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Configuration {
        /// Error message describing the configuration issue
        message: String,
        /// Additional context about the configuration error
        context: Vec<String>,
    },
    
    /// System errors
    #[error("System error: {message}")]
    System {
        /// Error message describing the system issue
        message: String,
        /// Additional context about the system error
        context: Vec<String>,
    },
    
    /// Network-related errors
    #[error("Network error: {message}")]
    Network {
        /// Error message describing the network issue
        message: String,
        /// Additional context about the network error
        context: Vec<String>,
    },
    
    /// Storage-related errors
    #[error("Storage error: {message}")]
    Storage {
        /// Error message describing the storage issue
        message: String,
        /// Additional context about the storage error
        context: Vec<String>,
    },
    
    /// I/O operation errors
    #[error("I/O error: {message}")]
    IoError {
        /// Error message describing the I/O issue
        message: String,
        /// Additional context about the I/O error
        context: Vec<String>,
    },
    
    /// External service errors
    #[error("External service error: {message}")]
    ExternalService {
        /// Error message describing the external service issue
        message: String,
        /// Additional context about the external service error
        context: Vec<String>,
    },
    
    /// Resource not found errors
    #[error("Resource not found: {message}")]
    NotFound {
        /// Error message describing what was not found
        message: String,
        /// Additional context about the not found error
        context: Vec<String>,
    },
}

impl SystemError {
    /// Create a new configuration error}


    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new system error
    pub fn system(message: impl Into<String>) -> Self {
        Self::System {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new storage error
    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new I/O error
    pub fn io(message: impl Into<String>) -> Self {
        Self::IoError {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new external service error
    pub fn external_service(message: impl Into<String>) -> Self {
        Self::ExternalService {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Create a new not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            SystemError::Network { .. }
                | SystemError::ExternalService { .. }
                | SystemError::IoError { .. }
        )
    }
    
    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            SystemError::Configuration { .. } => "configuration",
            SystemError::System { .. } => "system",
            SystemError::Network { .. } => "network",
            SystemError::Storage { .. } => "storage",
            SystemError::IoError { .. } => "io",
            SystemError::ExternalService { .. } => "external_service",
            SystemError::NotFound { .. } => "not_found",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_error_creation() {
        let error = SystemError::network("Connection failed");
        assert_eq!(error.category(), "network");
    }
    
    #[test]
    fn test_error_recoverability() {
        let network_error = SystemError::network("Connection failed");
        assert!(network_error.is_recoverable());
        
        let config_error = SystemError::configuration("Missing config file");
        assert!(!config_error.is_recoverable());
    }
    
    #[test]
    fn test_error_display() {
        let error = SystemError::storage("Disk full");
        let error_string = format!("{}", error);
        assert!(error_string.contains("Storage error"));
        assert!(error_string.contains("Disk full"));
    }
}
