//! Type aliases for BearDog ecosystem
//!
//! This module provides convenient type aliases for commonly used types.
//! Result-based aliases have been removed in favor of using
//! Result<T, BearDogError> directly for better type clarity.

// BearDogError used in migration helper
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Core type aliases (non-deprecated)
pub type JsonValue = serde_json::Value;
pub type JsonMap = serde_json::Map<String, serde_json::Value>;
pub type StringMap = HashMap<String, String>;
pub type MetricsMap = HashMap<String, JsonValue>;

// Configuration aliases
pub type ConfigMap = HashMap<String, JsonValue>;
pub type EnvironmentMap = HashMap<String, String>;

// Network and service aliases
pub type ServiceId = String;
pub type EndpointUrl = String;
pub type ApiVersion = String;

// Security and crypto aliases
pub type KeyId = String;
pub type Signature = Vec<u8>;
pub type PublicKey = Vec<u8>;
pub type PrivateKey = Vec<u8>;

// Monitoring and health aliases
pub type HealthScore = f64;
pub type MetricValue = f64;
pub type Timestamp = chrono::DateTime<chrono::Utc>;

// Workflow and process aliases
pub type WorkflowId = String;
pub type ProcessId = String;
pub type TaskId = String;

// HSM and hardware aliases
pub type HsmId = String;
pub type DeviceId = String;
pub type ProviderId = String;

// Serialization helpers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedValue<T> {
    pub value: T,
    pub type_name: &'static str,
}

impl<T> TypedValue<T> {
    pub fn new(value: T, type_name: &'static str) -> Self {
        Self { value, type_name }
    }
}

// Migration utilities for deprecated types
pub struct TypeMigrationHelper;

impl TypeMigrationHelper {
    /// Check if a type alias is deprecated
    pub fn is_deprecated(alias_name: &str) -> bool {
        matches!(
            alias_name,
            "BearDogResult"
                | "SecurityResult"
                | "HsmResult"
                | "NetworkResult"
                | "ConfigResult"
                | "WorkflowResult"
                | "CryptoResult"
                | "MonitoringResult"
                | "ValidationResult"
        )
    }

    /// Get the canonical replacement for a deprecated alias
    pub fn get_canonical_replacement(deprecated_alias: &str) -> Option<&'static str> {
        match deprecated_alias {
            "BearDogResult" => Some("Result<T, BearDogError>"),
            "SecurityResult" => Some("Result<T, BearDogError>"),
            "HsmResult" => Some("Result<T, BearDogError>"),
            "NetworkResult" => Some("Result<T, BearDogError>"),
            "ConfigResult" => Some("Result<T, BearDogError>"),
            "WorkflowResult" => Some("Result<T, BearDogError>"),
            "CryptoResult" => Some("Result<T, BearDogError>"),
            "MonitoringResult" => Some("Result<T, BearDogError>"),
            "ValidationResult" => Some("Result<T, BearDogError>"),
            _ => None,
        }
    }

    /// Generate migration script for deprecated aliases
    pub fn generate_migration_script() -> String {
        let mut script = String::new();
        script.push_str("# BearDog Type Alias Migration Script\n");
        script.push_str("# Replace deprecated Result aliases with canonical forms\n\n");

        let deprecated_aliases = [
            "BearDogResult",
            "SecurityResult",
            "HsmResult",
            "NetworkResult",
            "ConfigResult",
            "WorkflowResult",
            "CryptoResult",
            "MonitoringResult",
            "ValidationResult",
        ];

        for alias in &deprecated_aliases {
            script.push_str(&format!("# Replace: {alias} -> Result<T, BearDogError>\n"));
        }

        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_migration_helper() {
        assert!(TypeMigrationHelper::is_deprecated("BearDogResult"));
        assert!(!TypeMigrationHelper::is_deprecated("JsonValue"));

        assert_eq!(
            TypeMigrationHelper::get_canonical_replacement("BearDogResult"),
            Some("Result<T, BearDogError>")
        );

        assert_eq!(
            TypeMigrationHelper::get_canonical_replacement("NonExistent"),
            None
        );
    }

    #[test]
    fn test_typed_value() {
        let typed_val = TypedValue::new(42, "i32");
        assert_eq!(typed_val.value, 42);
        assert_eq!(typed_val.type_name, "i32");
    }
}
