// SPDX-License-Identifier: AGPL-3.0-only

use crate::{
    categories::{
        BusinessErrorCategory, ConfigurationErrorCategory, HsmErrorCategory, NetworkErrorCategory,
        SecurityErrorCategory, SystemErrorCategory,
    },
    BearDogError,
};
use std::fmt::Display;

pub trait BearDogErrorExt<T> {
    /// Creates instance with business context
    fn with_business_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with system context
    fn with_system_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with network context
    fn with_network_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with security context
    fn with_security_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with configuration context
    fn with_configuration_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with hsm context
    fn with_hsm_context(self, context: &str) -> Result<T, BearDogError>;
    fn chain_context<F>(self, f: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String;
}

impl<T, E> BearDogErrorExt<T> for Result<T, E>
where
    E: Display + Send + Sync + 'static,
{
    /// Creates instance with business context
    fn with_business_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::Business {
            category: BusinessErrorCategory::Validation,
            message: format!("{context}: {e}"),
        })
    }

    /// Creates instance with system context
    fn with_system_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::System {
            category: SystemErrorCategory::General,
            message: format!("{context}: {e}"),
        })
    }

    /// Creates instance with network context
    fn with_network_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::Network {
            category: NetworkErrorCategory::General,
            message: format!("{context}: {e}"),
        })
    }

    /// Creates instance with security context
    fn with_security_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::Security {
            category: SecurityErrorCategory::General,
            message: format!("{context}: {e}"),
        })
    }

    /// Creates instance with configuration context
    fn with_configuration_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::Configuration {
            category: ConfigurationErrorCategory::General,
            message: format!("{context}: {e}"),
        })
    }

    /// Creates instance with hsm context
    fn with_hsm_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::Hsm {
            category: HsmErrorCategory::General,
            message: format!("{context}: {e}"),
        })
    }


    fn chain_context<F>(self, f: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| BearDogError::internal(format!("{}: {}", f(), e)))
    }
}

pub trait IntoBearDogError {
    fn into_beardog_error(self) -> BearDogError;
    fn into_beardog_error_with_category(self, category: &str) -> BearDogError;
}

impl<E> IntoBearDogError for E
where
    E: Display + Send + Sync + 'static,
{
    fn into_beardog_error(self) -> BearDogError {
        BearDogError::internal(self.to_string())
    }


    fn into_beardog_error_with_category(self, category: &str) -> BearDogError {
        BearDogError::System {
            category: SystemErrorCategory::General,
            message: format!("[{category}] {self}"),
        }
    }
}
