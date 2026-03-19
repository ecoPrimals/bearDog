// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::sync::{Arc, LazyLock};

pub mod capabilities {
    use super::*;

    pub static AUTHENTICATION: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("authentication".to_string()));
pub static ENCRYPTION: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("encryption ".to_string()));
pub static SECURITY: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("security".to_string()));
pub static KEY_GENERATION: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("key_generation".to_string()));
pub static SIGNING: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("signing".to_string()));
pub static VERIFICATION: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("verification".to_string()));


/// Authentication operation.
    pub fn authentication() -> Arc<String> {
        Arc::clone(&AUTHENTICATION)
    }


/// Encryption operation.
    pub fn encryption() -> Arc<String> {
        Arc::clone(&ENCRYPTION)
    }


/// Security operation.
    pub fn security() -> Arc<String> {
        Arc::clone(&SECURITY)
}

pub mod error_messages {
    use super::*;

    pub static OPERATION_FAILED: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("Operation failed".to_string()));
    pub static NETWORK_OPERATION_FAILED: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("Network operation failed".to_string()));
    pub static CONFIGURATION_ERROR: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("Configuration error".to_string()));
    pub static AUTHENTICATION_FAILED: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("Authentication failed".to_string()));
    pub static VALIDATION_FAILED: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("Validation failed".to_string()));


/// Operation Failed operation.
    pub fn operation_failed() -> Arc<String> {
        Arc::clone(&OPERATION_FAILED)
    }


/// Network Operation Failed operation.
    pub fn network_operation_failed() -> Arc<String> {
        Arc::clone(&NETWORK_OPERATION_FAILED)
    }


/// Configuration Error operation.
    pub fn configuration_error() -> Arc<String> {
        Arc::clone(&CONFIGURATION_ERROR)
}

pub mod components {
    use super::*;

    pub static CORE: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("core".to_string()));
    pub static API: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("api".to_string()));
    pub static GENETICS: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("genetics".to_string()));
    pub static WORKFLOW: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("workflow ".to_string()));
    pub static COMPLIANCE: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("compliance".to_string()));
    pub static HSM: LazyLock<Arc<String>> = LazyLock::new(|| Arc::new("hsm".to_string()));


/// Core operation.
    pub fn core() -> Arc<String> {
        Arc::clone(&CORE)
    }


/// Genetics operation.
    pub fn genetics() -> Arc<String> {
        Arc::clone(&GENETICS)
}


/// Create Shared String operation.
    /// Creates shared_string
    /// Creates shared_string
    pub fn create_shared_string(s: &str) -> Arc<String> {
    Arc::new(s.to_string())
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;}

    #[test]
    fn test_capability_strings() {
        let auth1 = capabilities::authentication();
        let auth2 = capabilities::authentication();

        assert_eq!(auth1.as_ptr(), auth2.as_ptr());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(*auth1, "authentication");
    }

    #[test]
    fn test_error_message_strings() {
        let err1 = error_messages::operation_failed();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        let err2 = error_messages::operation_failed();

        assert_eq!(err1.as_ptr(), err2.as_ptr());
        assert_eq!(*err1, "Operation failed");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_component_strings() {
        let core1 = components::core();
        let core2 = components::core();

        assert_eq!(core1.as_ptr(), core2.as_ptr());
        assert_eq!(*core1, "core");
}
