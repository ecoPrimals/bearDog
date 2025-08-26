

use crate::{BearDogError, *};

impl BearDogError {
    /// Creates a security-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the security error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and general subcategory
    pub fn security(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::General,
        }
    }

    /// Creates a security error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the security error
    /// * `category` - Specific security error category
    /// 
    /// # Returns
    /// A `BearDogError` with security category and specified subcategory
    pub fn security_with_category(message: impl Into<String>, category: SecurityErrorCategory) -> Self {
        BearDogError::Security {
            message: message.into(),
            category,
        }
    }

    /// Creates a system-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the system error
    /// 
    /// # Returns
    /// A `BearDogError` with system category and general subcategory
    pub fn system(message: impl Into<String>) -> Self {
        BearDogError::System {
            message: message.into(),
            category: SystemErrorCategory::General,
        }
    }

    /// Creates a system error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the system error
    /// * `category` - Specific system error category
    /// 
    /// # Returns
    /// A `BearDogError` with system category and specified subcategory
    pub fn system_with_category(message: impl Into<String>, category: SystemErrorCategory) -> Self {
        BearDogError::System {
            message: message.into(),
            category,
        }
    }

    /// Creates a business logic error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the business error
    /// 
    /// # Returns
    /// A `BearDogError` with business category and general subcategory
    pub fn business(message: impl Into<String>) -> Self {
        BearDogError::Business {
            message: message.into(),
            category: BusinessErrorCategory::General,
        }
    }

    /// Creates an invalid input error (business validation)
    /// 
    /// # Arguments
    /// * `message` - Description of the invalid input
    /// 
    /// # Returns
    /// A `BearDogError` with business category and validation subcategory
    pub fn invalid_input(message: impl Into<String>) -> Self {
        BearDogError::Business {
            message: message.into(),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Creates a business error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the business error
    /// * `category` - Specific business error category
    /// 
    /// # Returns
    /// A `BearDogError` with business category and specified subcategory
    pub fn business_with_category(message: impl Into<String>, category: BusinessErrorCategory) -> Self {
        BearDogError::Business {
            message: message.into(),
            category,
        }
    }

    /// Creates a network-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the network error
    /// 
    /// # Returns
    /// A `BearDogError` with network category and general subcategory
    pub fn network(message: impl Into<String>) -> Self {
        BearDogError::Network {
            message: message.into(),
            category: NetworkErrorCategory::General,
        }
    }

    /// Creates a configuration-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the configuration error
    /// 
    /// # Returns
    /// A `BearDogError` with configuration category and general subcategory
    pub fn configuration(message: impl Into<String>) -> Self {
        BearDogError::Configuration {
            message: message.into(),
            category: ConfigurationErrorCategory::General,
        }
    }

    /// Creates an initialization error
    /// 
    /// # Arguments
    /// * `message` - Description of the initialization error
    /// 
    /// # Returns
    /// A `BearDogError` with initialization category
    pub fn initialization(message: impl Into<String>) -> Self {
        BearDogError::Initialization {
            message: message.into(),
        }
    }

    /// Creates an API error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the API error
    /// * `category` - Specific API error category
    /// 
    /// # Returns
    /// A `BearDogError` with API category and specified subcategory
    pub fn api(message: impl Into<String>, category: ApiErrorCategory) -> Self {
        BearDogError::Api {
            message: message.into(),
            category,
            status_code: None,
            endpoint: None,
        }
    }

    /// Creates an HSM-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the HSM error
    /// 
    /// # Returns
    /// A `BearDogError` with HSM category and general subcategory
    pub fn hsm(message: impl Into<String>) -> Self {
        BearDogError::Hsm {
            message: message.into(),
            category: HsmErrorCategory::General,
        }
    }

    /// Creates a workflow-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the workflow error
    /// 
    /// # Returns
    /// A `BearDogError` with workflow category and general subcategory
    pub fn workflow(message: impl Into<String>) -> Self {
        BearDogError::Workflow {
            message: message.into(),
            category: WorkflowErrorCategory::General,
        }
    }

    /// Creates a genetics-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the genetics error
    /// 
    /// # Returns
    /// A `BearDogError` with genetics category
    pub fn genetics(message: impl Into<String>) -> Self {
        BearDogError::Genetics {
            message: message.into(),
        }
    }

    /// Creates a deployment-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the deployment error
    /// 
    /// # Returns
    /// A `BearDogError` with deployment category
    pub fn deployment(message: impl Into<String>) -> Self {
        BearDogError::Deployment {
            message: message.into(),
        }
    }

    /// Creates a memory-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the memory error
    /// 
    /// # Returns
    /// A `BearDogError` with memory category
    pub fn memory(message: impl Into<String>) -> Self {
        BearDogError::Memory {
            message: message.into(),
        }
    }

    /// Creates a monitoring-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the monitoring error
    /// 
    /// # Returns
    /// A `BearDogError` with monitoring category
    pub fn monitoring(message: impl Into<String>) -> Self {
        BearDogError::Monitoring {
            message: message.into(),
        }
    }

    /// Creates a compliance-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the compliance error
    /// 
    /// # Returns
    /// A `BearDogError` with compliance category
    pub fn compliance(message: impl Into<String>) -> Self {
        BearDogError::Compliance {
            message: message.into(),
        }
    }

    /// Creates a cryptographic error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the cryptographic error
    /// 
    /// # Returns
    /// A `BearDogError` with cryptographic category
    pub fn cryptographic(message: impl Into<String>) -> Self {
        BearDogError::Cryptographic {
            message: message.into(),
        }
    }

    /// Creates a tunnel-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the tunnel error
    /// 
    /// # Returns
    /// A `BearDogError` with tunnel category
    pub fn tunnel(message: impl Into<String>) -> Self {
        BearDogError::Tunnel {
            message: message.into(),
        }
    }

    /// Creates an adapter-related error with the given message
    /// 
    /// # Arguments
    /// * `message` - Description of the adapter error
    /// 
    /// # Returns
    /// A `BearDogError` with adapter category
    pub fn adapter(message: impl Into<String>) -> Self {
        BearDogError::Adapter {
            message: message.into(),
        }
    }

    /// Creates an authentication error
    /// 
    /// # Arguments
    /// * `message` - Description of the authentication error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and authentication subcategory
    pub fn authentication(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Authentication,
        }
    }

    /// Creates an authorization error
    /// 
    /// # Arguments
    /// * `message` - Description of the authorization error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and authorization subcategory
    pub fn authorization(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Authorization,
        }
    }

    /// Creates a validation error
    /// 
    /// # Arguments
    /// * `message` - Description of the validation error
    /// 
    /// # Returns
    /// A `BearDogError` with business category and validation subcategory
    pub fn validation(message: impl Into<String>) -> Self {
        BearDogError::Business {
            message: message.into(),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Creates an internal error
    /// 
    /// # Arguments
    /// * `message` - Description of the internal error
    /// 
    /// # Returns
    /// A `BearDogError` with system category and general subcategory
    pub fn internal(message: impl Into<String>) -> Self {
        BearDogError::System {
            message: message.into(),
            category: SystemErrorCategory::General,
        }
    }

    /// Creates a timeout error
    /// 
    /// # Arguments
    /// * `message` - Description of the timeout error
    /// 
    /// # Returns
    /// A `BearDogError` with network category and timeout subcategory
    pub fn timeout(message: impl Into<String>) -> Self {
        BearDogError::Network {
            message: message.into(),
            category: NetworkErrorCategory::Timeout,
        }
    }

    /// Creates a connection error
    /// 
    /// # Arguments
    /// * `message` - Description of the connection error
    /// 
    /// # Returns
    /// A `BearDogError` with network category and connection subcategory
    pub fn connection(message: impl Into<String>) -> Self {
        BearDogError::Network {
            message: message.into(),
            category: NetworkErrorCategory::Connection,
        }
    }

    /// Creates a parsing error
    /// 
    /// # Arguments
    /// * `message` - Description of the parsing error
    /// 
    /// # Returns
    /// A `BearDogError` with configuration category and parsing subcategory
    pub fn parsing(message: impl Into<String>) -> Self {
        BearDogError::Configuration {
            message: message.into(),
            category: ConfigurationErrorCategory::Parsing,
        }
    }

    /// Creates a not found error
    /// 
    /// # Arguments
    /// * `message` - Description of what was not found
    /// 
    /// # Returns
    /// A `BearDogError` with API category and not found subcategory
    pub fn not_found(message: impl Into<String>) -> Self {
        BearDogError::Api {
            message: message.into(),
            category: ApiErrorCategory::NotFound,
            status_code: Some(404),
            endpoint: None,
        }
    }

    /// Creates an already exists error
    /// 
    /// # Arguments
    /// * `message` - Description of what already exists
    /// 
    /// # Returns
    /// A `BearDogError` with API category and conflict subcategory
    pub fn already_exists(message: impl Into<String>) -> Self {
        BearDogError::Api {
            message: message.into(),
            category: ApiErrorCategory::Conflict,
            status_code: Some(409),
            endpoint: None,
        }
    }

    /// Creates an API error with detailed information
    /// 
    /// # Arguments
    /// * `message` - Description of the API error
    /// * `category` - Specific API error category
    /// * `status_code` - HTTP status code, if applicable
    /// * `endpoint` - API endpoint where the error occurred
    /// 
    /// # Returns
    /// A `BearDogError` with API category and detailed information
    pub fn api_with_details(
        message: impl Into<String>,
        category: ApiErrorCategory,
        status_code: Option<u16>,
        endpoint: Option<&str>,
    ) -> Self {
        BearDogError::Api {
            message: message.into(),
            category,
            status_code,
            endpoint: endpoint.map(|s| s.to_string()),
        }
    }

    /// Creates an HSM error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the HSM error
    /// * `category` - Specific HSM error category
    /// 
    /// # Returns
    /// A `BearDogError` with HSM category and specified subcategory
    pub fn hsm_with_category(message: impl Into<String>, category: HsmErrorCategory) -> Self {
        BearDogError::Hsm {
            message: message.into(),
            category,
        }
    }

    /// Creates a workflow error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the workflow error
    /// * `category` - Specific workflow error category
    /// 
    /// # Returns
    /// A `BearDogError` with workflow category and specified subcategory
    pub fn workflow_with_category(message: impl Into<String>, category: WorkflowErrorCategory) -> Self {
        BearDogError::Workflow {
            message: message.into(),
            category,
        }
    }

    /// Creates a network error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the network error
    /// * `category` - Specific network error category
    /// 
    /// # Returns
    /// A `BearDogError` with network category and specified subcategory
    pub fn network_with_category(message: impl Into<String>, category: NetworkErrorCategory) -> Self {
        BearDogError::Network {
            message: message.into(),
            category,
        }
    }

    /// Creates a configuration error with specific category
    /// 
    /// # Arguments
    /// * `message` - Description of the configuration error
    /// * `category` - Specific configuration error category
    /// 
    /// # Returns
    /// A `BearDogError` with configuration category and specified subcategory
    pub fn configuration_with_category(message: impl Into<String>, category: ConfigurationErrorCategory) -> Self {
        BearDogError::Configuration {
            message: message.into(),
            category,
        }
    }

    /// Creates a permission denied error
    /// 
    /// # Arguments
    /// * `message` - Description of the permission denied error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and authorization subcategory
    pub fn permission_denied(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Authorization,
        }
    }

    /// Creates an invalid credentials error
    /// 
    /// # Arguments
    /// * `message` - Description of the invalid credentials error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and authentication subcategory
    pub fn invalid_credentials(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Authentication,
        }
    }

    /// Creates a service unavailable error
    /// 
    /// # Arguments
    /// * `message` - Description of the service unavailable error
    /// 
    /// # Returns
    /// A `BearDogError` with API category and service unavailable subcategory
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        BearDogError::Api {
            message: message.into(),
            category: ApiErrorCategory::ServiceUnavailable,
            status_code: Some(503),
            endpoint: None,
        }
    }

    /// Creates a database error
    /// 
    /// # Arguments
    /// * `message` - Description of the database error
    /// 
    /// # Returns
    /// A `BearDogError` with system category and storage subcategory
    pub fn database(message: impl Into<String>) -> Self {
        BearDogError::System {
            message: message.into(),
            category: SystemErrorCategory::Storage,
        }
    }

    /// Creates a filesystem error
    /// 
    /// # Arguments
    /// * `message` - Description of the filesystem error
    /// 
    /// # Returns
    /// A `BearDogError` with system category and filesystem subcategory
    pub fn filesystem(message: impl Into<String>) -> Self {
        BearDogError::System {
            message: message.into(),
            category: SystemErrorCategory::FileSystem,
        }
    }

    /// Creates an invalid format error
    /// 
    /// # Arguments
    /// * `message` - Description of the invalid format error
    /// 
    /// # Returns
    /// A `BearDogError` with configuration category and format subcategory
    pub fn invalid_format(message: impl Into<String>) -> Self {
        BearDogError::Configuration {
            message: message.into(),
            category: ConfigurationErrorCategory::Format,
        }
    }

    /// Creates a rate limit exceeded error
    /// 
    /// # Arguments
    /// * `message` - Description of the rate limit exceeded error
    /// 
    /// # Returns
    /// A `BearDogError` with API category and rate limit subcategory
    pub fn rate_limit_exceeded(message: impl Into<String>) -> Self {
        BearDogError::Api {
            message: message.into(),
            category: ApiErrorCategory::RateLimit,
            status_code: Some(429),
            endpoint: None,
        }
    }

    /// Creates a certificate error
    /// 
    /// # Arguments
    /// * `message` - Description of the certificate error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and certificate subcategory
    pub fn certificate_error(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Certificate,
        }
    }

    /// Creates an encryption error
    /// 
    /// # Arguments
    /// * `message` - Description of the encryption error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and encryption subcategory
    pub fn encryption_error(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::Encryption,
        }
    }

    /// Creates a key management error
    /// 
    /// # Arguments
    /// * `message` - Description of the key management error
    /// 
    /// # Returns
    /// A `BearDogError` with security category and key management subcategory
    pub fn key_management_error(message: impl Into<String>) -> Self {
        BearDogError::Security {
            message: message.into(),
            category: SecurityErrorCategory::KeyManagement,
        }
    }
} 