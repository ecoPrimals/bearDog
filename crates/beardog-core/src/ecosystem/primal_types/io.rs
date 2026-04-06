// SPDX-License-Identifier: AGPL-3.0-or-later

//! RPC and error types for primal-to-primal communication.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

///
/// Represents errors that occur during primal component operations,
/// including initialization failures, communication errors, and
/// with error codes, messages, and additional context details.
#[derive(Debug, Clone)]
pub struct PrimalError {
    /// Error code identifying the type of error
    /// The code value
    pub code: String,
    /// Human-readable error message
    /// The message value
    pub message: String,
    /// Mapping of details
    pub details: HashMap<String, serde_json::Value>,
}

impl PrimalError {
    /// Create a new primal error
    ///
    /// Creates a new error with the specified error code and message.
    /// The details map is initialized as empty and can be populated later.
    ///
    /// # Arguments
    /// * `code` - Error code identifying the type of error
    /// * `message` - Human-readable error message
    ///
    /// # Returns
    /// A new `PrimalError` instance
    /// Creates a new instance
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: HashMap::new(),
        }
    }

    /// Create an initialization failure error
    ///
    ///
    /// # Arguments
    /// * `message` - Description of the initialization failure
    ///
    /// # Returns
    /// A `PrimalError` with "`InitializationFailed`" error code
    /// Initializes `componentialization_failed`
    pub fn initialization_failed(message: impl Into<String>) -> Self {
        Self::new("InitializationFailed", message)
    }

    /// Create a health check failure error
    ///
    ///
    /// # Arguments
    /// * `message` - Description of the health check failure
    ///
    /// # Returns
    /// A `PrimalError` with "`HealthCheckFailed`" error code
    pub fn health_check_failed(message: impl Into<String>) -> Self {
        Self::new("HealthCheckFailed", message)
    }

    /// Create an unsupported operation error
    ///
    /// by the current primal component configuration.
    ///
    /// # Arguments
    /// * `message` - Description of the unsupported operation
    ///
    /// # Returns
    /// A `PrimalError` with "`UnsupportedOperation`" error code
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        Self::new("UnsupportedOperation", message)
    }
}

impl From<beardog_errors::BearDogError> for PrimalError {
    fn from(err: beardog_errors::BearDogError) -> Self {
        Self::new("BearDogError", format!("{err}"))
    }
}

/// Request to a primal service
///
/// Represents a standardized request format for primal-to-primal communication,
/// including operation specification, parameters, and contextual metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    /// Unique identifier for this primal instance
    pub id: String,
    /// Unique identifier for this specific request
    pub request_id: String,
    /// Type of operation being requested
    pub operation_type: String,
    /// Operation parameters as JSON value
    pub params: serde_json::Value,
    /// Additional contextual metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Request creation timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalRequest {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
            operation_type: "default".to_string(),
            params: serde_json::Value::Null,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Response from a primal service
///
/// Represents a standardized response format for primal-to-primal communication,
/// including status, success indication, result data, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    /// Unique identifier for the responding primal
    pub id: String,
    /// Identifier matching the original request
    pub request_id: String,
    /// Status description of the operation
    pub status: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Operation result data as JSON value
    pub data: serde_json::Value,
    /// Additional response metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalResponse {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
            status: "pending".to_string(),
            success: false,
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Helper for migrating from hardcoded primal types to capability-based discovery
///
/// Deprecated: Use `CapabilityBasedEcosystem` for capability-based service discovery.
#[deprecated = "Use CapabilityBasedEcosystem instead"]
pub struct PrimalTypeMigrationHelper;

impl PrimalTypeMigrationHelper {
    /// Gets `migration_guidance`
    #[must_use]
    pub const fn get_migration_guidance() -> &'static str {
        r#"
🔄 PRIMAL SOVEREIGNTY MIGRATION GUIDE

STEP 1: Replace hardcoded primal types with capability discovery
❌ OLD (VIOLATES SOVEREIGNTY):
    let primal = PrimalType::SpecificPrimal;
    let client = create_hardcoded_client();

✅ NEW (ACHIEVES SOVEREIGNTY):
    let universal_adapter = UniversalAdapter::new()?;
    let compute_providers = universal_adapter
        .discover_capability(ServiceCapabilityType::ComputeIntelligence)
        ?;

STEP 2: Use discovered capabilities instead of hardcoded names
❌ OLD (VIOLATES SOVEREIGNTY):
    if primal == PrimalType::SpecificPrimal {
        // Hardcoded primal-specific logic
    }

✅ NEW (ACHIEVES SOVEREIGNTY):
    for provider in compute_providers {
        if provider.capabilities.contains(&ServiceCapabilityType::ComputeIntelligence) {
            // Universal capability-based logic
        }
    }

STEP 3: Remove hardcoded endpoint assumptions
❌ OLD (VIOLATES SOVEREIGNTY):
    let endpoint = "http://hardcoded-service:8081";

✅ NEW (ACHIEVES SOVEREIGNTY):
    let endpoint = provider.endpoint.url; // Discovered dynamically

✅ BENEFITS:
- Works with ANY compute provider (no hardcoded assumptions)
- Scales to infinite ecosystem size (no 2^n hardcoding problem)
- True primal sovereignty (each knows only itself)
- Automatic failover and load balancing
- Zero vendor lock-in

📖 Full guide: HARDCODING_ELIMINATION_PLAN_2025.md
"#
    }
}
