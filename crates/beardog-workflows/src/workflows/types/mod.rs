// SPDX-License-Identifier: AGPL-3.0-only

//! Shared workflow configuration, API DTOs, and taxonomy enums for orchestration.
//!
//! [`WorkflowRequest`] / [`WorkflowResponse`] model RPC-style workflow submission and status.
//! [`enums`](crate::workflows::types::enums) defines lifecycle, audit, approval, and execution states used by policies and logs.

// Define WorkflowConfig locally until it's available in beardog_types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Local mirror of workflow runtime limits until a single source exists in `beardog_types`.
pub struct WorkflowConfig {
    /// Number of `max_concurrent_workflows`
    pub max_concurrent_workflows: usize,
    /// Default timeout applied to workflow operations when none is specified, in seconds.
    pub default_timeout_seconds: u64,
    /// Whether `enable_logging` is enabled
    pub enable_logging: bool,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workflows: 100,
            default_timeout_seconds: 300,
            enable_logging: true,
        }
    }
}

// Re-export WorkflowType from enums module
pub use enums::WorkflowType;

/// Enumerations and structs for workflow classification, execution telemetry, and approvals.
pub mod enums;

pub use enums::{
    ApprovalStatus, AuditAction, ExecutionStatus, WorkflowExecutionState, WorkflowPriority,
    WorkflowTarget,
};

/// Client- or gateway-submitted workflow invocation: correlation id, logical type, and opaque payload.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRequest {
    /// Correlation or request identifier (often echoed in [`WorkflowResponse`]).
    pub id: String,
    /// The workflow type value
    pub workflow_type: String,
    /// The data value
    pub data: serde_json::Value,
}

/// Outcome envelope for a workflow request: same correlation id, human-readable status, optional structured result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowResponse {
    /// Correlation id matching the originating [`WorkflowRequest::id`].
    pub id: String,
    /// Current status of the component
    pub status: String,
    /// Optional result
    pub result: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_config_default() {
        let config = WorkflowConfig::default();
        assert_eq!(config.max_concurrent_workflows, 100);
        assert_eq!(config.default_timeout_seconds, 300);
        assert!(config.enable_logging);
    }

    #[test]
    fn test_workflow_config_custom() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 50,
            default_timeout_seconds: 600,
            enable_logging: false,
        };
        assert_eq!(config.max_concurrent_workflows, 50);
        assert_eq!(config.default_timeout_seconds, 600);
        assert!(!config.enable_logging);
    }

    #[test]
    fn test_workflow_config_serialization() {
        let config = WorkflowConfig::default();
        let json = serde_json::to_string(&config).expect("Serialization should succeed");
        assert!(json.contains("max_concurrent_workflows"));
        assert!(json.contains("100"));

        let deserialized: WorkflowConfig =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(
            deserialized.max_concurrent_workflows,
            config.max_concurrent_workflows
        );
        assert_eq!(
            deserialized.default_timeout_seconds,
            config.default_timeout_seconds
        );
        assert_eq!(deserialized.enable_logging, config.enable_logging);
    }

    #[test]
    fn test_workflow_config_clone() {
        let config = WorkflowConfig::default();
        let cloned = config.clone();
        assert_eq!(
            cloned.max_concurrent_workflows,
            config.max_concurrent_workflows
        );
        assert_eq!(
            cloned.default_timeout_seconds,
            config.default_timeout_seconds
        );
        assert_eq!(cloned.enable_logging, config.enable_logging);
    }

    #[test]
    fn test_workflow_request_creation() {
        let request = WorkflowRequest {
            id: "test-id-123".to_string(),
            workflow_type: "deployment".to_string(),
            data: serde_json::json!({"key": "value"}),
        };
        assert_eq!(request.id, "test-id-123");
        assert_eq!(request.workflow_type, "deployment");
        assert_eq!(request.data["key"], "value");
    }

    #[test]
    fn test_workflow_request_serialization() {
        let request = WorkflowRequest {
            id: "test-id-456".to_string(),
            workflow_type: "validation".to_string(),
            data: serde_json::json!({"count": 42}),
        };

        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        assert!(json.contains("test-id-456"));
        assert!(json.contains("validation"));

        let deserialized: WorkflowRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(deserialized.id, request.id);
        assert_eq!(deserialized.workflow_type, request.workflow_type);
        assert_eq!(deserialized.data, request.data);
    }

    #[test]
    fn test_workflow_request_with_complex_data() {
        let complex_data = serde_json::json!({
            "nested": {
                "field1": "value1",
                "field2": 100
            },
            "array": [1, 2, 3],
            "boolean": true
        });

        let request = WorkflowRequest {
            id: "complex-test".to_string(),
            workflow_type: "complex".to_string(),
            data: complex_data,
        };

        assert_eq!(request.data["nested"]["field1"], "value1");
        assert_eq!(request.data["array"][0], 1);
        assert_eq!(request.data["boolean"], true);
    }

    #[test]
    fn test_workflow_request_clone() {
        let request = WorkflowRequest {
            id: "clone-test".to_string(),
            workflow_type: "test".to_string(),
            data: serde_json::json!({"test": true}),
        };

        let cloned = request.clone();
        assert_eq!(cloned.id, request.id);
        assert_eq!(cloned.workflow_type, request.workflow_type);
        assert_eq!(cloned.data, request.data);
    }

    #[test]
    fn test_workflow_response_creation() {
        let response = WorkflowResponse {
            id: "response-id-123".to_string(),
            status: "completed".to_string(),
            result: Some(serde_json::json!({"success": true})),
        };
        assert_eq!(response.id, "response-id-123");
        assert_eq!(response.status, "completed");
        assert!(response.result.is_some());
    }

    #[test]
    fn test_workflow_response_without_result() {
        let response = WorkflowResponse {
            id: "response-id-456".to_string(),
            status: "pending".to_string(),
            result: None,
        };
        assert_eq!(response.id, "response-id-456");
        assert_eq!(response.status, "pending");
        assert!(response.result.is_none());
    }

    #[test]
    fn test_workflow_response_serialization() {
        let response = WorkflowResponse {
            id: "ser-test".to_string(),
            status: "running".to_string(),
            result: Some(serde_json::json!({"progress": 50})),
        };

        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        assert!(json.contains("ser-test"));
        assert!(json.contains("running"));

        let deserialized: WorkflowResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(deserialized.id, response.id);
        assert_eq!(deserialized.status, response.status);
        assert_eq!(deserialized.result, response.result);
    }

    #[test]
    fn test_workflow_response_clone() {
        let response = WorkflowResponse {
            id: "clone-response".to_string(),
            status: "failed".to_string(),
            result: Some(serde_json::json!({"error": "test error"})),
        };

        let cloned = response.clone();
        assert_eq!(cloned.id, response.id);
        assert_eq!(cloned.status, response.status);
        assert_eq!(cloned.result, response.result);
    }

    #[test]
    fn test_workflow_request_debug() {
        let request = WorkflowRequest {
            id: "debug-test".to_string(),
            workflow_type: "debug".to_string(),
            data: serde_json::json!({}),
        };
        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("WorkflowRequest"));
        assert!(debug_str.contains("debug-test"));
    }

    #[test]
    fn test_workflow_response_debug() {
        let response = WorkflowResponse {
            id: "debug-response".to_string(),
            status: "test".to_string(),
            result: None,
        };
        let debug_str = format!("{response:?}");
        assert!(debug_str.contains("WorkflowResponse"));
        assert!(debug_str.contains("debug-response"));
    }

    #[test]
    fn test_workflow_config_debug() {
        let config = WorkflowConfig::default();
        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("WorkflowConfig"));
        assert!(debug_str.contains("100"));
    }

    #[test]
    fn test_workflow_config_zero_values() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 0,
            default_timeout_seconds: 0,
            enable_logging: false,
        };
        assert_eq!(config.max_concurrent_workflows, 0);
        assert_eq!(config.default_timeout_seconds, 0);
    }

    #[test]
    fn test_workflow_config_large_values() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 10000,
            default_timeout_seconds: 86400, // 24 hours
            enable_logging: true,
        };
        assert_eq!(config.max_concurrent_workflows, 10000);
        assert_eq!(config.default_timeout_seconds, 86400);
    }

    #[test]
    fn test_workflow_request_empty_id() {
        let request = WorkflowRequest {
            id: String::new(),
            workflow_type: "test".to_string(),
            data: serde_json::json!({}),
        };
        assert!(request.id.is_empty());
    }

    #[test]
    fn test_workflow_response_various_statuses() {
        let statuses = vec!["pending", "running", "completed", "failed", "cancelled"];

        for status in statuses {
            let response = WorkflowResponse {
                id: format!("test-{status}"),
                status: status.to_string(),
                result: None,
            };
            assert_eq!(response.status, status);
        }
    }
}
