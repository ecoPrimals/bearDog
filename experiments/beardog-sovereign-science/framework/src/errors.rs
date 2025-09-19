//! Error types for the BearDog Sovereign Science Framework

/// Sovereign science framework errors
#[derive(Debug, thiserror::Error)]
pub enum SovereignScienceError {
    #[error("Cryptographic validation failed: {0}")]
    CryptographicValidationFailed(String),
    
    #[error("Performance validation failed: {0}")]
    PerformanceValidationFailed(String),
    
    #[error("Distributed security validation failed: {0}")]
    DistributedSecurityValidationFailed(String),
    
    #[error("Human dignity validation failed: {0}")]
    HumanDignityValidationFailed(String),
    
    #[error("Enterprise validation failed: {0}")]
    EnterpriseValidationFailed(String),
    
    #[error("Infrastructure setup failed: {0}")]
    InfrastructureSetupFailed(String),
    
    #[error("Statistical analysis failed: {0}")]
    StatisticalAnalysisFailed(String),
    
    #[error("Telemetry error: {0}")]
    TelemetryError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
} 