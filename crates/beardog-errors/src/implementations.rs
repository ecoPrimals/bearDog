use beardog_errors::BearDogError;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {

    Low,

    Medium,

    High,

    Critical,
}

impl BearDogError {

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
        }
    }

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
        }
    }

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
        }
    }

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
            BearDogError::Hsm { message, category } => {
                ErrorContext::Hsm(HsmErrorContext {
                    message: message.clone(),
                    category: category.clone(),
                    provider: "unknown".to_string(),
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
            BearDogError::Workflow { message, category } => {
                ErrorContext::Workflow(WorkflowErrorContext {
                    message: message.clone(),
                    workflow_id: "unknown".to_string(),
                    category: category.clone(),
                    recommendations: get_workflow_recommendations(category),
                })
            }
            BearDogError::Genetics { message } => {
                ErrorContext::Genetics(GeneticsErrorContext {
                    message: message.clone(),
                    operation: "unknown".to_string(),
                    recommendations: vec!["Check genetics configuration".to_string()],
                })
            }
            BearDogError::Deployment { message } => {
                ErrorContext::Deployment(DeploymentErrorContext {
                    message: message.clone(),
                    stage: "unknown".to_string(),
                    recommendations: vec!["Check deployment configuration".to_string()],
                })
            }
            BearDogError::Memory { message } => {
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

            BearDogError::Cryptographic { message, .. } => message.clone(),
            BearDogError::Monitoring { message, .. } => message.clone(),
            BearDogError::Compliance { message, .. } => message.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {

    Security,

    System,

    Business,

    Network,

    Configuration,

    Initialization,
}

#[derive(Debug, Clone)]
pub enum ErrorContext {

    Security(SecurityErrorContext),

    System(SystemErrorContext),

    Business(BusinessErrorContext),

    Network(NetworkErrorContext),

    Configuration(ConfigurationErrorContext),

    Initialization(InitializationErrorContext),

    Hsm(HsmErrorContext),

    Api(ApiErrorContext),

    Workflow(WorkflowErrorContext),

    Genetics(GeneticsErrorContext),

    Deployment(DeploymentErrorContext),

    Memory(MemoryErrorContext),

    Adapter(AdapterErrorContext),

    Authentication(AuthenticationErrorContext),

    Authorization(AuthorizationErrorContext),

    Cryptographic(CryptographicErrorContext),

    Monitoring(MonitoringErrorContext),

    Compliance(ComplianceErrorContext),
}

#[derive(Debug, Clone)]
pub struct SecurityErrorContext {

    pub message: String,

    pub category: SecurityErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SystemErrorContext {

    pub message: String,

    pub category: SystemErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BusinessErrorContext {

    pub message: String,

    pub category: BusinessErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkErrorContext {

    pub message: String,

    pub category: NetworkErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationErrorContext {

    pub message: String,

    pub category: ConfigurationErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InitializationErrorContext {

    pub message: String,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HsmErrorContext {

    pub message: String,

    pub category: HsmErrorCategory,

    pub provider: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ApiErrorContext {

    pub message: String,

    pub status_code: Option<u16>,

    pub endpoint: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowErrorContext {

    pub message: String,

    pub workflow_id: Option<String>,

    pub category: WorkflowErrorCategory,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GeneticsErrorContext {

    pub message: String,

    pub operation: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeploymentErrorContext {

    pub message: String,

    pub stage: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MemoryErrorContext {

    pub message: String,

    pub operation: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AdapterErrorContext {

    pub message: String,

    pub adapter: Option<String>,

    pub target: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthenticationErrorContext {

    pub message: String,

    pub method: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthorizationErrorContext {

    pub message: String,

    pub required_permission: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CryptographicErrorContext {

    pub message: String,

    pub operation: Option<String>,

    pub algorithm: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MonitoringErrorContext {

    pub message: String,

    pub metric: Option<String>,

    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ComplianceErrorContext {

    pub message: String,

    pub standard: Option<String>,

    pub recommendations: Vec<String>,
}

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
