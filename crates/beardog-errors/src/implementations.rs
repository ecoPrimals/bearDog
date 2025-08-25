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


/// Error implementation utilities and analysis
/// 
/// This module provides utility functions for analyzing and working with BearDogError instances.
use crate::*;

/// Error severity levels for analysis and reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - minor issues that can be deferred
    Low,
    /// Medium severity - moderate issues that should be addressed
    Medium,
    /// High severity - serious issues requiring prompt attention
    High,
    /// Critical severity - severe issues requiring immediate action
    Critical,
}

impl BearDogError {
    /// Get error severity level - updated for unified error system
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            BearDogError::Security { .. } => ErrorSeverity::High,
            BearDogError::System { .. } => ErrorSeverity::High,
            BearDogError::Business { .. } => ErrorSeverity::Medium,
            BearDogError::Network { .. } => ErrorSeverity::Medium,
            BearDogError::Configuration { .. } => ErrorSeverity::High,
            BearDogError::Initialization { .. } => ErrorSeverity::Critical,
            BearDogError::Hsm { .. } => ErrorSeverity::Critical,
            BearDogError::Api { .. } => ErrorSeverity::Medium,
            BearDogError::Workflow { .. } => ErrorSeverity::Medium,
            BearDogError::Genetics { .. } => ErrorSeverity::Low,
            BearDogError::Deployment { .. } => ErrorSeverity::High,
            BearDogError::Memory { .. } => ErrorSeverity::Critical,
            BearDogError::Adapter { .. } => ErrorSeverity::Medium,
            BearDogError::Authentication { .. } => ErrorSeverity::High,
            BearDogError::Authorization { .. } => ErrorSeverity::High,
            BearDogError::Cryptographic { .. } => ErrorSeverity::Critical,
            BearDogError::Monitoring { .. } => ErrorSeverity::Low,
            BearDogError::Compliance { .. } => ErrorSeverity::High,
        }
    }

    /// Get error category - updated for unified error system
    pub fn category(&self) -> ErrorCategory {
        match self {
            BearDogError::Security { .. } => ErrorCategory::Security,
            BearDogError::System { .. } => ErrorCategory::System,
            BearDogError::Business { .. } => ErrorCategory::Business,
            BearDogError::Network { .. } => ErrorCategory::Network,
            BearDogError::Configuration { .. } => ErrorCategory::Configuration,
            BearDogError::Initialization { .. } => ErrorCategory::Initialization,
            BearDogError::Hsm { .. } => ErrorCategory::Security,
            BearDogError::Api { .. } => ErrorCategory::Network,
            BearDogError::Workflow { .. } => ErrorCategory::Business,
            BearDogError::Genetics { .. } => ErrorCategory::Business,
            BearDogError::Deployment { .. } => ErrorCategory::System,
            BearDogError::Memory { .. } => ErrorCategory::System,
            BearDogError::Adapter { .. } => ErrorCategory::Network,
            BearDogError::Authentication { .. } => ErrorCategory::Security,
            BearDogError::Authorization { .. } => ErrorCategory::Security,
            BearDogError::Cryptographic { .. } => ErrorCategory::Security,
            BearDogError::Monitoring { .. } => ErrorCategory::System,
            BearDogError::Compliance { .. } => ErrorCategory::Security,
        }
    }

    /// Check if error is retryable - updated for unified error system
    pub fn is_retryable(&self) -> bool {
        match self {
            BearDogError::Security { .. } => false,
            BearDogError::System { .. } => true,
            BearDogError::Business { .. } => false,
            BearDogError::Network { .. } => true,
            BearDogError::Configuration { .. } => false,
            BearDogError::Initialization { .. } => false,
            BearDogError::Hsm { .. } => true, // HSM operations can be retried
            BearDogError::Api { .. } => true, // API calls can be retried
            BearDogError::Workflow { .. } => true, // Workflows can be retried
            BearDogError::Genetics { .. } => true, // Genetics operations can be retried
            BearDogError::Deployment { .. } => true, // Deployment can be retried
            BearDogError::Memory { .. } => false, // Memory errors usually aren't retryable
            BearDogError::Adapter { .. } => true, // Adapter operations can be retried
            BearDogError::Authentication { .. } => false, // Auth errors usually aren't retryable
            BearDogError::Authorization { .. } => false, // Authz errors usually aren't retryable
            BearDogError::Cryptographic { .. } => false, // Crypto errors usually aren't retryable
            BearDogError::Monitoring { .. } => true, // Monitoring operations can be retried
            BearDogError::Compliance { .. } => false, // Compliance errors usually aren't retryable
        }
    }

    /// Get contextual information about the error - updated for unified error system
    pub fn get_context(&self) -> ErrorContext {
        match self {
            BearDogError::Security { message, category } => {
                ErrorContext::Security(SecurityErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    recommendations: get_security_recommendations(category),
                })
            }
            BearDogError::System { message, category } => {
                ErrorContext::System(SystemErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    recommendations: get_system_recommendations(category),
                })
            }
            BearDogError::Business { message, category } => {
                ErrorContext::Business(BusinessErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    recommendations: get_business_recommendations(category),
                })
            }
            BearDogError::Network { message, category } => {
                ErrorContext::Network(NetworkErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    recommendations: get_network_recommendations(category),
                })
            }
            BearDogError::Configuration { message, category } => {
                ErrorContext::Configuration(ConfigurationErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    recommendations: get_configuration_recommendations(category),
                })
            }
            BearDogError::Initialization { message } => {
                ErrorContext::Initialization(InitializationErrorContext {
                    message: message.clone(),
                    recommendations: vec!["Check system requirements".to_string(), "Verify configuration".to_string()],
                })
            }
            BearDogError::Hsm { message, category, provider } => {
                ErrorContext::Hsm(HsmErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    provider: provider.clone(),
                    recommendations: get_hsm_recommendations(category),
                })
            }
            BearDogError::Api { message, category: _, status_code, endpoint } => {
                ErrorContext::Api(ApiErrorContext {
                    message: message.clone(),
                    status_code: *status_code,
                    endpoint: endpoint.clone(),
                    recommendations: get_api_recommendations(status_code),
                })
            }
            BearDogError::Workflow { message, workflow_id, category } => {
                ErrorContext::Workflow(WorkflowErrorContext {
                    message: message.clone(),
                    workflow_id: workflow_id.clone(),
                    category: category.clone(),
                    recommendations: get_workflow_recommendations(category),
                })
            }
            BearDogError::Genetics { message, operation } => {
                ErrorContext::Genetics(GeneticsErrorContext {
                    message: message.clone(),
                    operation: operation.clone(),
                    recommendations: vec!["Check genetics configuration".to_string()],
                })
            }
            BearDogError::Deployment { message, stage } => {
                ErrorContext::Deployment(DeploymentErrorContext {
                    message: message.clone(),
                    stage: stage.clone(),
                    recommendations: vec!["Check deployment configuration".to_string()],
                })
            }
            BearDogError::Memory { message, operation } => {
                ErrorContext::Memory(MemoryErrorContext {
                    message: message.clone(),
                    operation: operation.clone(),
                    recommendations: vec!["Check memory usage".to_string(), "Increase memory limits".to_string()],
                })
            }
            BearDogError::Adapter { message, adapter, target } => {
                ErrorContext::Adapter(AdapterErrorContext {
                    message: message.clone(),
                    adapter: adapter.clone(),
                    target: target.clone(),
                    recommendations: vec!["Check adapter configuration".to_string()],
                })
            }
            BearDogError::Authentication { message, method } => {
                ErrorContext::Authentication(AuthenticationErrorContext {
                    message: message.clone(),
                    method: method.clone(),
                    recommendations: vec!["Check credentials".to_string(), "Verify authentication method".to_string()],
                })
            }
            BearDogError::Authorization { message, required_permission } => {
                ErrorContext::Authorization(AuthorizationErrorContext {
                    message: message.clone(),
                    required_permission: required_permission.clone(),
                    recommendations: vec!["Check permissions".to_string(), "Contact administrator".to_string()],
                })
            }
            BearDogError::Cryptographic { message, operation, algorithm } => {
                ErrorContext::Cryptographic(CryptographicErrorContext {
                    message: message.clone(),
                    operation: operation.clone(),
                    algorithm: algorithm.clone(),
                    recommendations: vec!["Check cryptographic configuration".to_string()],
                })
            }
            BearDogError::Monitoring { message, metric } => {
                ErrorContext::Monitoring(MonitoringErrorContext {
                    message: message.clone(),
                    metric: metric.clone(),
                    recommendations: vec!["Check monitoring configuration".to_string()],
                })
            }
            BearDogError::Compliance { message, standard } => {
                ErrorContext::Compliance(ComplianceErrorContext {
                    message: message.clone(),
                    standard: standard.clone(),
                    recommendations: vec!["Check compliance configuration".to_string()],
                })
            }
        }
    }

    /// Get user-friendly error message - updated for unified error system
    pub fn user_message(&self) -> String {
        match self {
            BearDogError::Security { message, .. } => message.clone(),
            BearDogError::System { message, .. } => message.clone(),
            BearDogError::Business { message, .. } => message.clone(),
            BearDogError::Network { message, .. } => message.clone(),
            BearDogError::Configuration { message, .. } => message.clone(),
            BearDogError::Initialization { message } => message.clone(),
            BearDogError::Hsm { message, .. } => message.clone(),
            BearDogError::Api { message, .. } => message.clone(),
            BearDogError::Workflow { message, .. } => message.clone(),
            BearDogError::Genetics { message, .. } => message.clone(),
            BearDogError::Deployment { message, .. } => message.clone(),
            BearDogError::Memory { message, .. } => message.clone(),
            BearDogError::Adapter { message, .. } => message.clone(),
            BearDogError::Authentication { message, .. } => message.clone(),
            BearDogError::Authorization { message, .. } => message.clone(),
            BearDogError::Cryptographic { message, .. } => message.clone(),
            BearDogError::Monitoring { message, .. } => message.clone(),
            BearDogError::Compliance { message, .. } => message.clone(),
        }
    }
}

/// Error categories for classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Security-related errors
    Security,
    /// System and infrastructure errors
    System,
    /// Business logic errors
    Business,
    /// Network connectivity errors
    Network,
    /// Configuration errors
    Configuration,
    /// System initialization errors
    Initialization,
}

/// Contextual error information with recommendations
#[derive(Debug, Clone)]
pub enum ErrorContext {
    /// Security error context
    Security(SecurityErrorContext),
    /// System error context
    System(SystemErrorContext),
    /// Business error context
    Business(BusinessErrorContext),
    /// Network error context
    Network(NetworkErrorContext),
    /// Configuration error context
    Configuration(ConfigurationErrorContext),
    /// Initialization error context
    Initialization(InitializationErrorContext),
    /// HSM error context
    Hsm(HsmErrorContext),
    /// API error context
    Api(ApiErrorContext),
    /// Workflow error context
    Workflow(WorkflowErrorContext),
    /// Genetics error context
    Genetics(GeneticsErrorContext),
    /// Deployment error context
    Deployment(DeploymentErrorContext),
    /// Memory error context
    Memory(MemoryErrorContext),
    /// Adapter error context
    Adapter(AdapterErrorContext),
    /// Authentication error context
    Authentication(AuthenticationErrorContext),
    /// Authorization error context
    Authorization(AuthorizationErrorContext),
    /// Cryptographic error context
    Cryptographic(CryptographicErrorContext),
    /// Monitoring error context
    Monitoring(MonitoringErrorContext),
    /// Compliance error context
    Compliance(ComplianceErrorContext),
}

// Context structures for different error types
/// Security error context with detailed information
#[derive(Debug, Clone)]
pub struct SecurityErrorContext {
    /// Error message
    pub message: String,
    /// Security error category
    pub category: SecurityErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// System error context with diagnostic information
#[derive(Debug, Clone)]
pub struct SystemErrorContext {
    /// Error message
    pub message: String,
    /// System error category
    pub category: SystemErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Business error context with validation information
#[derive(Debug, Clone)]
pub struct BusinessErrorContext {
    /// Error message
    pub message: String,
    /// Business error category
    pub category: BusinessErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Network error context with connectivity information
#[derive(Debug, Clone)]
pub struct NetworkErrorContext {
    /// Error message
    pub message: String,
    /// Network error category
    pub category: NetworkErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Configuration error context with setup information
#[derive(Debug, Clone)]
pub struct ConfigurationErrorContext {
    /// Error message
    pub message: String,
    /// Configuration error category
    pub category: ConfigurationErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Initialization error context with startup information
#[derive(Debug, Clone)]
pub struct InitializationErrorContext {
    /// Error message
    pub message: String,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// HSM error context with hardware security module information
#[derive(Debug, Clone)]
pub struct HsmErrorContext {
    /// Error message
    pub message: String,
    /// HSM error category
    pub category: HsmErrorCategory,
    /// HSM provider name if available
    pub provider: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// API error context with HTTP information
#[derive(Debug, Clone)]
pub struct ApiErrorContext {
    /// Error message
    pub message: String,
    /// HTTP status code if available
    pub status_code: Option<u16>,
    /// API endpoint if available
    pub endpoint: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Workflow error context with execution information
#[derive(Debug, Clone)]
pub struct WorkflowErrorContext {
    /// Error message
    pub message: String,
    /// Workflow ID if available
    pub workflow_id: Option<String>,
    /// Workflow error category
    pub category: WorkflowErrorCategory,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Genetics error context with organism information
#[derive(Debug, Clone)]
pub struct GeneticsErrorContext {
    /// Error message
    pub message: String,
    /// Genetic operation if available
    pub operation: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Deployment error context with target information
#[derive(Debug, Clone)]
pub struct DeploymentErrorContext {
    /// Error message
    pub message: String,
    /// Deployment stage if available
    pub stage: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Memory error context with operation information
#[derive(Debug, Clone)]
pub struct MemoryErrorContext {
    /// Error message
    pub message: String,
    /// Memory operation if available
    pub operation: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Adapter error context with adapter information
#[derive(Debug, Clone)]
pub struct AdapterErrorContext {
    /// Error message
    pub message: String,
    /// Adapter name if available
    pub adapter: Option<String>,
    /// Target system if available
    pub target: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Authentication error context with method information
#[derive(Debug, Clone)]
pub struct AuthenticationErrorContext {
    /// Error message
    pub message: String,
    /// Authentication method if available
    pub method: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Authorization error context with permission information
#[derive(Debug, Clone)]
pub struct AuthorizationErrorContext {
    /// Error message
    pub message: String,
    /// Required permission if available
    pub required_permission: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Cryptographic error context with algorithm information
#[derive(Debug, Clone)]
pub struct CryptographicErrorContext {
    /// Error message
    pub message: String,
    /// Cryptographic operation if available
    pub operation: Option<String>,
    /// Cryptographic algorithm if available
    pub algorithm: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Monitoring error context with metric information
#[derive(Debug, Clone)]
pub struct MonitoringErrorContext {
    /// Error message
    pub message: String,
    /// Metric name if available
    pub metric: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

/// Compliance error context with standard information
#[derive(Debug, Clone)]
pub struct ComplianceErrorContext {
    /// Error message
    pub message: String,
    /// Compliance standard if available
    pub standard: Option<String>,
    /// Recommended remediation actions
    pub recommendations: Vec<String>,
}

// Helper functions for generating recommendations
fn get_security_recommendations(category: &SecurityErrorCategory) -> Vec<String> {
    match category {
        SecurityErrorCategory::Authentication => vec![
            "Check credentials".to_string(),
            "Verify authentication configuration".to_string(),
        ],
        SecurityErrorCategory::Authorization => vec![
            "Check permissions".to_string(),
            "Contact system administrator".to_string(),
        ],
        SecurityErrorCategory::Encryption => vec![
            "Check encryption configuration".to_string(),
            "Verify key management".to_string(),
        ],
        _ => vec!["Review security configuration".to_string()],
    }
}

fn get_system_recommendations(category: &SystemErrorCategory) -> Vec<String> {
    match category {
        SystemErrorCategory::FileSystem => vec![
            "Check file permissions".to_string(),
            "Verify disk space".to_string(),
        ],
        SystemErrorCategory::Resource => vec![
            "Check system resources".to_string(),
            "Monitor memory usage".to_string(),
        ],
        _ => vec!["Check system configuration".to_string()],
    }
}

fn get_business_recommendations(category: &BusinessErrorCategory) -> Vec<String> {
    match category {
        BusinessErrorCategory::Validation => vec![
            "Check input data".to_string(),
            "Verify data format".to_string(),
        ],
        _ => vec!["Review business logic".to_string()],
    }
}

fn get_network_recommendations(category: &NetworkErrorCategory) -> Vec<String> {
    match category {
        NetworkErrorCategory::Connection => vec![
            "Check network connectivity".to_string(),
            "Verify endpoint configuration".to_string(),
        ],
        NetworkErrorCategory::Timeout => vec![
            "Increase timeout values".to_string(),
            "Check network latency".to_string(),
        ],
        _ => vec!["Check network configuration".to_string()],
    }
}

fn get_configuration_recommendations(category: &ConfigurationErrorCategory) -> Vec<String> {
    match category {
        ConfigurationErrorCategory::Parsing => vec![
            "Check configuration file syntax".to_string(),
            "Verify configuration format".to_string(),
        ],
        ConfigurationErrorCategory::Missing => vec![
            "Provide required configuration".to_string(),
            "Check configuration completeness".to_string(),
        ],
        _ => vec!["Review configuration settings".to_string()],
    }
}

fn get_hsm_recommendations(category: &HsmErrorCategory) -> Vec<String> {
    match category {
        HsmErrorCategory::KeyGeneration => vec![
            "Check HSM capacity".to_string(),
            "Verify key generation parameters".to_string(),
        ],
        HsmErrorCategory::Hardware => vec![
            "Check HSM hardware status".to_string(),
            "Verify HSM connectivity".to_string(),
        ],
        _ => vec!["Check HSM configuration".to_string()],
    }
}

fn get_api_recommendations(status_code: &Option<u16>) -> Vec<String> {
    match status_code {
        Some(401) => vec!["Check authentication".to_string()],
        Some(403) => vec!["Check authorization".to_string()],
        Some(404) => vec!["Check endpoint URL".to_string()],
        Some(500) => vec!["Check server status".to_string()],
        _ => vec!["Check API configuration".to_string()],
    }
}

fn get_workflow_recommendations(category: &WorkflowErrorCategory) -> Vec<String> {
    match category {
        WorkflowErrorCategory::Execution => vec![
            "Check workflow configuration".to_string(),
            "Verify workflow parameters".to_string(),
        ],
        WorkflowErrorCategory::Approval => vec![
            "Check approval requirements".to_string(),
            "Verify approver availability".to_string(),
        ],
        _ => vec!["Check workflow settings".to_string()],
    }
}
