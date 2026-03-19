// SPDX-License-Identifier: AGPL-3.0-only

use crate::BearDogError;

impl BearDogError {
    /// Internal operation error constructor.
    pub fn internal<T: std::fmt::Display>(message: T) -> Self {
        Self::Internal {
            message: message.to_string(),
        }
    }

    /// Internal operation with error context.
    pub fn internal_with_error<E: std::error::Error>(error: E, context: &str) -> Self {
        Self::Internal {
            message: format!("{}: {}", context, error),
        }
    }

    /// Validation operation error constructor.
    pub fn validation<T: std::fmt::Display>(message: T) -> Self {
        Self::Validation {
            message: message.to_string(),
        }
    }

    /// Config operation error constructor.
    pub fn config<T: std::fmt::Display>(message: T) -> Self {
        Self::Config {
            message: message.to_string(),
        }
    }

    /// Auth operation error constructor.
    pub fn auth<T: std::fmt::Display>(message: T) -> Self {
        Self::Auth {
            message: message.to_string(),
        }
    }

    /// Network operation error constructor.
    pub fn network<T: std::fmt::Display>(message: T) -> Self {
        Self::Network {
            message: message.to_string(),
        }
    }

    /// Timeout operation error constructor.
    pub fn timeout<T: std::fmt::Display>(message: T) -> Self {
        Self::Timeout {
            message: message.to_string(),
        }
    }

    /// Serialization operation error constructor.
    pub fn serialization<T: std::fmt::Display>(message: T) -> Self {
        Self::Serialization {
            message: message.to_string(),
        }
    }

    /// Invalid Input operation error constructor.
    pub fn invalid_input<T: std::fmt::Display>(message: T) -> Self {
        Self::InvalidInput {
            message: message.to_string(),
        }
    }

    /// Cryptographic operation error constructor.
    pub fn cryptographic<T: std::fmt::Display>(operation: T) -> Self {
        Self::Cryptographic {
            operation: operation.to_string(),
        }
    }

    /// Protocol operation error constructor.
    pub fn protocol<T: std::fmt::Display>(message: T) -> Self {
        Self::Protocol {
            message: message.to_string(),
        }
    }

    /// Database operation error constructor.
    pub fn database<T: std::fmt::Display>(message: T) -> Self {
        Self::Database {
            message: message.to_string(),
        }
    }

    /// Monitoring operation error constructor.
    pub fn monitoring<T: std::fmt::Display>(message: T) -> Self {
        Self::Monitoring {
            message: message.to_string(),
        }
    }

    /// Health Check operation error constructor.
    pub fn health_check<T: std::fmt::Display>(message: T) -> Self {
        Self::HealthCheck {
            message: message.to_string(),
        }
    }

    /// Caching operation error constructor.
    pub fn caching<T: std::fmt::Display>(message: T) -> Self {
        Self::Caching {
            message: message.to_string(),
        }
    }
}
