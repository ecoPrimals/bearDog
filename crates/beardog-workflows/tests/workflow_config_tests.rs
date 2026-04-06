// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Configuration tests for BearDog workflows
//!
//! These tests verify workflow configuration validation and serialization.

use beardog_workflows::WorkflowConfig;

#[test]
fn test_workflow_config_creation() {
    let config = WorkflowConfig {
        max_concurrent_workflows: 10,
        default_timeout_seconds: 300,
        retry_attempts: 3,
        enable_audit_logging: true,
        workflow_storage_path: "/var/lib/beardog/workflows".to_string(),
    };

    assert_eq!(config.max_concurrent_workflows, 10);
    assert_eq!(config.default_timeout_seconds, 300);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    assert_eq!(config.retry_attempts, 3);
    assert!(config.enable_audit_logging);
    assert_eq!(config.workflow_storage_path, "/var/lib/beardog/workflows");
}

#[test]
fn test_workflow_config_serialization() {
    let config = WorkflowConfig {
        max_concurrent_workflows: 10,
        default_timeout_seconds: 300,
        retry_attempts: 3,
        enable_audit_logging: true,
        workflow_storage_path: "/var/lib/beardog/workflows".to_string(),
    };
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal

    let json = serde_json::to_string(&config).expect("Serialization should succeed");
    assert!(json.contains("10"));
    assert!(json.contains("300"));
    assert!(json.contains('3'));
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal
#[test]
fn test_workflow_config_deserialization() {
    let json = r#"{
        "max_concurrent_workflows": 10,
        "default_timeout_seconds": 300,
        "retry_attempts": 3,
        "enable_audit_logging": true,
        "workflow_storage_path": "/var/lib/beardog/workflows"
    }"#;

    let config: WorkflowConfig =
        serde_json::from_str(json).expect("Deserialization should succeed");
    assert_eq!(config.max_concurrent_workflows, 10);
    assert_eq!(config.default_timeout_seconds, 300);
    assert_eq!(config.retry_attempts, 3);
    assert!(config.enable_audit_logging);
}

#[test]
fn test_workflow_config_high_concurrency() {
    let config = WorkflowConfig {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        max_concurrent_workflows: 1000,
        default_timeout_seconds: 60,
        retry_attempts: 5,
        enable_audit_logging: false,
        workflow_storage_path: "/tmp/workflows".to_string(),
    };

    assert_eq!(config.max_concurrent_workflows, 1000);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal
#[test]
fn test_workflow_config_no_retries() {
    let config = WorkflowConfig {
        max_concurrent_workflows: 5,
        default_timeout_seconds: 120,
        retry_attempts: 0,
        enable_audit_logging: true,
        workflow_storage_path: "/data/workflows".to_string(),
    };

    assert_eq!(config.retry_attempts, 0);
}
