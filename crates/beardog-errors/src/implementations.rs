// SPDX-License-Identifier: AGPL-3.0-only

use beardog_errors::BearDogError;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,
}

impl BearDogError {

/// Severity operation.
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

            BearDogError::Cryptographic { .. } => ErrorSeverity::Critical,
            BearDogError::Monitoring { .. } => ErrorSeverity::Low,
            BearDogError::Compliance { .. } => ErrorSeverity::High,
            BearDogError::Testing { .. } => ErrorSeverity::Low,
        }
    }

/// Category operation.
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

            BearDogError::Cryptographic { .. } => ErrorCategory::Security,
            BearDogError::Monitoring { .. } => ErrorCategory::System,
            BearDogError::Compliance { .. } => ErrorCategory::Security,
            BearDogError::Testing { .. } => ErrorCategory::System,
        }
    }

/// Is Retryable operation.
    /// Checks if retryable
    /// Checks if retryable
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

            BearDogError::Cryptographic { .. } => false, // Crypto errors usually aren't retryable
            BearDogError::Monitoring { .. } => true, // Monitoring operations can be retried
            BearDogError::Compliance { .. } => false, // Compliance errors usually aren't retryable
            BearDogError::Testing { .. } => true, // Test operations can be retried
        }
    }

/// Get Context operation.
    /// Gets context
    /// Gets context
    pub fn get_context(&self) -> ErrorContext {
        match self {
            BearDogError::Security { message, category } => {
                ErrorContext::Security(message,
                    category: category.clone(),
                    recommendations: get_security_recommendations(category),
                })
            }
            BearDogError::System { message, category } => {
                ErrorContext::System(message,
                    category: category.clone(),
                    recommendations: get_system_recommendations(category),
                })
            }
            BearDogError::Business { message, category } => {
                ErrorContext::Business(message,
                    category: category.clone(),
                    recommendations: get_business_recommendations(category),
                })
            }
            BearDogError::Network { message, category } => {
                ErrorContext::Network(message,
                    category: category.clone(),
                    recommendations: get_network_recommendations(category),
                })
            }
            BearDogError::Configuration { message, category } => {
                ErrorContext::Configuration(message,
                    category: category.clone(),
                    recommendations: get_configuration_recommendations(category),
                })
            }
            BearDogError::Initialization { message } => {
                ErrorContext::Initialization(message,
                    recommendations: vec!["Check system requirements".to_string(), "Verify configuration".to_string()],
                })
            }
            BearDogError::Hsm { message, category } => {
                ErrorContext::Hsm(message,
                    category: category.clone(),
                    provider: "unknown".to_string(),
                    recommendations: get_hsm_recommendations(category),
                })
            }
            BearDogError::Api { message, category: _, status_code, endpoint } => {
                ErrorContext::Api(message,
                    status_code: *status_code,
                    endpoint: &endpoint,
                    recommendations: get_api_recommendations(status_code),
                })
            }
            BearDogError::Workflow { message, category } => {
                ErrorContext::Workflow(message,
                    workflow_id: "unknown".to_string(),
                    category: category.clone(),
                    recommendations: get_workflow_recommendations(category),
                })
            }
            BearDogError::Genetics { message } => {
                ErrorContext::Genetics(message,
                    operation: "unknown".to_string(),
                    recommendations: vec!["Check genetics configuration".to_string()],
                })
            }
            BearDogError::Deployment { message } => {
                ErrorContext::Deployment(message,
                    stage: "unknown".to_string(),
                    recommendations: vec!["Check deployment configuration".to_string()],
                })
            }
            BearDogError::Memory { message } => {
                ErrorContext::Memory(message,
                    operation: operation.clone(),
                    recommendations: vec!["Check memory usage".to_string(), "Increase memory limits".to_string()],
                })
            }
            BearDogError::Adapter { message, adapter, target } => {
                ErrorContext::Adapter(message,
                    adapter: adapter.clone(),
                    target: target.clone(),
                    recommendations: vec!["Check adapter configuration".to_string()],
                })
            }

            BearDogError::Cryptographic { message, operation, algorithm } => {
                ErrorContext::Cryptographic(message,
                    operation: operation.clone(),
                    algorithm: algorithm.clone(),
                    recommendations: vec!["Check cryptographic configuration".to_string()],
                })
            }
            BearDogError::Monitoring { message, metric } => {
                ErrorContext::Monitoring(message,
                    metric: metric.clone(),
                    recommendations: vec!["Check monitoring configuration".to_string()],
                })
            }
            BearDogError::Compliance { message, standard } => {
                ErrorContext::Compliance(message,
                    standard: standard.clone(),
                    recommendations: vec!["Check compliance configuration".to_string()],
                })
            }
        }
    }

/// User Message operation.
    pub fn user_message(&self) -> &str {
        match self {
            BearDogError::Security { message, .. } => message,
            BearDogError::System { message, .. } => message,
            BearDogError::Business { message, .. } => message,
            BearDogError::Network { message, .. } => message,
            BearDogError::Configuration { message, .. } => message,
            BearDogError::Initialization { message } => message,
            BearDogError::Hsm { message, .. } => message,
            BearDogError::Api { message, .. } => message,
            BearDogError::Workflow { message, .. } => message,
            BearDogError::Genetics { message, .. } => message,
            BearDogError::Deployment { message, .. } => message,
            BearDogError::Memory { message, .. } => message,
            BearDogError::Adapter { message, .. } => message,

            BearDogError::Cryptographic { message, .. } => message,
            BearDogError::Monitoring { message, .. } => message,
            BearDogError::Compliance { message, .. } => message,
            BearDogError::Testing { message, .. } => message,
        }
    }
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: SecurityErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: SystemErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: BusinessErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: NetworkErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: ConfigurationErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The category value
    pub category: HsmErrorCategory,


    pub provider: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Current status of the component_code
    pub status_code: Option<u16>,

    /// Optional endpoint
    pub endpoint: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    pub workflow_id: Option<String>,

    /// The category value
    pub category: WorkflowErrorCategory,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional operation
    pub operation: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional stage
    pub stage: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional operation
    pub operation: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional adapter
    pub adapter: Option<String>,

    /// Optional target
    pub target: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional method
    pub method: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional required permission
    pub required_permission: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional operation
    pub operation: Option<String>,

    /// Optional algorithm
    pub algorithm: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional metric
    pub metric: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
    /// Optional standard
    pub standard: Option<String>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

/// Gets security_recommendations
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

/// Gets system_recommendations
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

/// Gets business_recommendations
fn get_business_recommendations(category: &BusinessErrorCategory) -> Vec<String> {
    match category {
        BusinessErrorCategory::Validation => vec![
            "Check input data".to_string(),
            "Verify data format".to_string(),
        ],
        _ => vec!["Review business logic".to_string()],
    }
}

/// Gets network_recommendations
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

/// Gets configuration_recommendations
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

/// Gets hsm_recommendations
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

/// Gets api_recommendations
fn get_api_recommendations(status_code: &Option<u16>) -> Vec<String> {
    match status_code {
        Some(401) => vec!["Check authentication".to_string()],
        Some(403) => vec!["Check authorization".to_string()],
        Some(404) => vec!["Check endpoint URL".to_string()],
        Some(500) => vec!["Check server status".to_string()],
        _ => vec!["Check API configuration".to_string()],
    }
}

/// Gets workflow_recommendations
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
