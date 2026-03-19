// SPDX-License-Identifier: AGPL-3.0-only

//! # Enhanced Error Context System
//!
//! **PEDANTIC ERROR HANDLING** - Comprehensive error context and recovery strategies
//! 
//! This module provides enhanced error handling capabilities with detailed context,
//! recovery suggestions, and operational guidance for production environments.

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **Enhanced Error Context**
///
/// Provides comprehensive error context including operational guidance,
/// recovery strategies, and detailed diagnostic information for production
/// incident response and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedErrorContext {
    /// Original error information
    pub error: ErrorInfo,
    
    /// Operational context and environment
    pub context: OperationalContext,
    
    /// Recovery strategies and recommendations
    pub recovery: RecoveryGuidance,
    
    /// Diagnostic information for troubleshooting
    pub diagnostics: DiagnosticInfo,
    
    /// Related errors and causation chain
    pub related_errors: Vec<String>,
}

/// Core error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error code for programmatic handling
    pub code: String,
    
    /// Human-readable error message
    pub message: String,
    
    /// Error severity level
    pub severity: ErrorSeverity,
    
    /// Error category for classification
    pub category: ErrorCategory,
    
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
    
    /// Source component or module
    pub source: String,
}

/// Operational context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalContext {
    /// System component where error occurred
    pub component: String,
    
    /// Operation being performed when error occurred
    pub operation: String,
    
    /// Request ID for correlation
    pub request_id: Option<String>,
    
    /// User context (if applicable)
    pub user_context: Option<String>,
    
    /// System resource state
    pub resource_state: HashMap<String, String>,
    
    /// Configuration values relevant to the error
    pub relevant_config: HashMap<String, String>,
}

/// Recovery guidance and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryGuidance {
    /// Immediate actions to take
    pub immediate_actions: Vec<String>,
    
    /// Long-term preventive measures
    pub preventive_measures: Vec<String>,
    
    /// Whether automatic recovery is possible
    pub auto_recovery_possible: bool,
    
    /// Estimated recovery time
    pub estimated_recovery_time: Option<String>,
    
    /// Required permissions or access for recovery
    pub required_access: Vec<String>,
    
    /// Escalation path if recovery fails
    pub escalation_path: Vec<String>,
}

/// Diagnostic information for troubleshooting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticInfo {
    /// System metrics at time of error
    pub system_metrics: HashMap<String, f64>,
    
    /// Relevant log entries
    pub log_entries: Vec<String>,
    
    /// Configuration values that may be relevant
    pub config_snapshot: HashMap<String, String>,
    
    /// Network connectivity status
    pub network_status: Option<String>,
    
    /// Resource availability
    pub resource_availability: HashMap<String, bool>,
    
    /// Performance indicators
    pub performance_indicators: HashMap<String, f64>,
}

/// Error severity levels with operational implications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical - System unusable, immediate action required
    Critical,
    /// High - Major functionality affected, urgent action needed
    High,
    /// Medium - Significant impact, action needed within hours
    Medium,
    /// Low - Minor impact, can be addressed during maintenance
    Low,
    /// Info - Informational, no immediate action required
    Info,
}

/// Error categories for systematic handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Configuration errors
    Configuration,
    /// Authentication/authorization errors
    Security,
    /// Network connectivity errors
    Network,
    /// Hardware/resource errors
    Hardware,
    /// Software/logic errors
    Software,
    /// External service errors
    External,
    /// Data validation errors
    Validation,
    /// Performance/timeout errors
    Performance,
}

impl EnhancedErrorContext {
    /// Create enhanced context for configuration errors
    pub fn configuration_error(
        message: &str,
        config_key: &str,
        config_value: &str,
        component: &str,
    ) -> Self {
        let mut context = HashMap::new();
        context.insert("config_key".to_string(), config_key.to_string());
        context.insert("config_value".to_string(), config_value.to_string());
        
        let mut relevant_config = HashMap::new();
        relevant_config.insert(config_key.to_string(), config_value.to_string());
        
        Self {
            error: ErrorInfo {
                code: "CONFIG_ERROR".to_string(),
                message: message.to_string(),
                severity: ErrorSeverity::High,
                category: ErrorCategory::Configuration,
                timestamp: SystemTime::now(),
                source: component.to_string(),
            },
            context: OperationalContext {
                component: component.to_string(),
                operation: "configuration_validation".to_string(),
                request_id: None,
                user_context: None,
                resource_state: HashMap::new(),
                relevant_config,
            },
            recovery: RecoveryGuidance {
                immediate_actions: vec![
                    format!("Review configuration value for key: {}", config_key),
                    "Check configuration documentation for valid values".to_string(),
                    "Validate configuration syntax".to_string(),
                ],
                preventive_measures: vec![
                    "Implement configuration validation in CI/CD pipeline".to_string(),
                    "Use configuration schema validation".to_string(),
                    "Add configuration tests".to_string(),
                ],
                auto_recovery_possible: false,
                estimated_recovery_time: Some("5-15 minutes".to_string()),
                required_access: vec!["configuration_write".to_string()],
                escalation_path: vec![
                    "System Administrator".to_string(),
                    "DevOps Team".to_string(),
                ],
            },
            diagnostics: DiagnosticInfo {
                system_metrics: HashMap::new(),
                log_entries: Vec::new(),
                config_snapshot: relevant_config.clone(),
                network_status: None,
                resource_availability: HashMap::new(),
                performance_indicators: HashMap::new(),
            },
            related_errors: Vec::new(),
        }
    }
    
    /// Create enhanced context for security errors
    pub fn security_error(
        message: &str,
        operation: &str,
        user_context: Option<&str>,
        component: &str,
    ) -> Self {
        Self {
            error: ErrorInfo {
                code: "SECURITY_ERROR".to_string(),
                message: message.to_string(),
                severity: ErrorSeverity::Critical,
                category: ErrorCategory::Security,
                timestamp: SystemTime::now(),
                source: component.to_string(),
            },
            context: OperationalContext {
                component: component.to_string(),
                operation: operation.to_string(),
                request_id: None,
                user_context: user_context.map(|s| s.to_string()),
                resource_state: HashMap::new(),
                relevant_config: HashMap::new(),
            },
            recovery: RecoveryGuidance {
                immediate_actions: vec![
                    "Review security logs for suspicious activity".to_string(),
                    "Verify user permissions and access rights".to_string(),
                    "Check authentication token validity".to_string(),
                    "Consider temporary access restriction".to_string(),
                ],
                preventive_measures: vec![
                    "Implement multi-factor authentication".to_string(),
                    "Regular access review and cleanup".to_string(),
                    "Enhanced monitoring for security events".to_string(),
                    "Security awareness training".to_string(),
                ],
                auto_recovery_possible: false,
                estimated_recovery_time: Some("immediate to 1 hour".to_string()),
                required_access: vec!["security_admin".to_string()],
                escalation_path: vec![
                    "Security Team".to_string(),
                    "CISO".to_string(),
                    "Incident Response Team".to_string(),
                ],
            },
            diagnostics: DiagnosticInfo {
                system_metrics: HashMap::new(),
                log_entries: Vec::new(),
                config_snapshot: HashMap::new(),
                network_status: None,
                resource_availability: HashMap::new(),
                performance_indicators: HashMap::new(),
            },
            related_errors: Vec::new(),
        }
    }
    
    /// Create enhanced context for network errors
    pub fn network_error(
        message: &str,
        endpoint: &str,
        operation: &str,
        component: &str,
    ) -> Self {
        let mut resource_state = HashMap::new();
        resource_state.insert("endpoint".to_string(), endpoint.to_string());
        
        Self {
            error: ErrorInfo {
                code: "NETWORK_ERROR".to_string(),
                message: message.to_string(),
                severity: ErrorSeverity::Medium,
                category: ErrorCategory::Network,
                timestamp: SystemTime::now(),
                source: component.to_string(),
            },
            context: OperationalContext {
                component: component.to_string(),
                operation: operation.to_string(),
                request_id: None,
                user_context: None,
                resource_state,
                relevant_config: HashMap::new(),
            },
            recovery: RecoveryGuidance {
                immediate_actions: vec![
                    format!("Verify network connectivity to: {}", endpoint),
                    "Check DNS resolution".to_string(),
                    "Verify firewall rules".to_string(),
                    "Test endpoint availability".to_string(),
                ],
                preventive_measures: vec![
                    "Implement circuit breaker pattern".to_string(),
                    "Add retry logic with exponential backoff".to_string(),
                    "Monitor network health continuously".to_string(),
                    "Implement failover endpoints".to_string(),
                ],
                auto_recovery_possible: true,
                estimated_recovery_time: Some("1-30 minutes".to_string()),
                required_access: vec!["network_admin".to_string()],
                escalation_path: vec![
                    "Network Team".to_string(),
                    "Infrastructure Team".to_string(),
                ],
            },
            diagnostics: DiagnosticInfo {
                system_metrics: HashMap::new(),
                log_entries: Vec::new(),
                config_snapshot: HashMap::new(),
                network_status: Some("checking".to_string()),
                resource_availability: HashMap::new(),
                performance_indicators: HashMap::new(),
            },
            related_errors: Vec::new(),
        }
    }
    
    /// Convert to BearDogError with enhanced context
    pub fn to_beardog_error(&self) -> BearDogError {
        BearDogError::system(format!(
            "{}: {} (Component: {}, Operation: {})",
            self.error.code,
            self.error.message,
            self.context.component,
            self.context.operation
        ))
    }
    
    /// Add diagnostic information
    pub fn with_diagnostics(mut self, diagnostics: DiagnosticInfo) -> Self {
        self.diagnostics = diagnostics;
        self
    }
    
    /// Add related error information
    pub fn with_related_error(mut self, error_id: String) -> Self {
        self.related_errors.push(error_id);
        self
    }
    
    /// Get recovery time estimate in minutes
    pub fn recovery_time_minutes(&self) -> Option<u32> {
        // Parse recovery time estimate and return minutes
        // This is a simplified implementation
        match self.recovery.estimated_recovery_time.as_ref()? {
            s if s.contains("immediate") => Some(0),
            s if s.contains("5-15 minutes") => Some(10),
            s if s.contains("1-30 minutes") => Some(15),
            s if s.contains("1 hour") => Some(60),
            _ => None,
        }
    }
    
    /// Check if error requires immediate escalation
    pub fn requires_immediate_escalation(&self) -> bool {
        matches!(self.error.severity, ErrorSeverity::Critical) ||
        matches!(self.error.category, ErrorCategory::Security)
    }
}

/// **PEDANTIC ERROR HANDLING UTILITIES**
///
/// Utility functions for creating enhanced error contexts with comprehensive
/// diagnostic information and recovery guidance.
pub struct ErrorContextBuilder;

impl ErrorContextBuilder {
    /// Create comprehensive validation error context
    pub fn validation_error(
        field: &str,
        value: &str,
        constraint: &str,
        component: &str,
    ) -> EnhancedErrorContext {
        EnhancedErrorContext::configuration_error(
            &format!("Validation failed for field '{}': value '{}' violates constraint '{}'", 
                    field, value, constraint),
            field,
            value,
            component,
        )
    }
    
    /// Create performance error context with metrics
    pub fn performance_error(
        operation: &str,
        duration_ms: u64,
        threshold_ms: u64,
        component: &str,
    ) -> EnhancedErrorContext {
        let mut performance_indicators = HashMap::new();
        performance_indicators.insert("duration_ms".to_string(), duration_ms as f64);
        performance_indicators.insert("threshold_ms".to_string(), threshold_ms as f64);
        performance_indicators.insert("performance_ratio".to_string(), 
                                    duration_ms as f64 / threshold_ms as f64);
        
        let context = EnhancedErrorContext {
            error: ErrorInfo {
                code: "PERFORMANCE_ERROR".to_string(),
                message: format!("Operation '{}' took {}ms, exceeding threshold of {}ms", 
                                operation, duration_ms, threshold_ms),
                severity: ErrorSeverity::Medium,
                category: ErrorCategory::Performance,
                timestamp: SystemTime::now(),
                source: component.to_string(),
            },
            context: OperationalContext {
                component: component.to_string(),
                operation: operation.to_string(),
                request_id: None,
                user_context: None,
                resource_state: HashMap::new(),
                relevant_config: HashMap::new(),
            },
            recovery: RecoveryGuidance {
                immediate_actions: vec![
                    "Check system resource utilization".to_string(),
                    "Review recent configuration changes".to_string(),
                    "Verify database performance".to_string(),
                    "Check network latency".to_string(),
                ],
                preventive_measures: vec![
                    "Implement performance monitoring".to_string(),
                    "Add performance testing to CI/CD".to_string(),
                    "Optimize database queries".to_string(),
                    "Consider caching strategies".to_string(),
                ],
                auto_recovery_possible: true,
                estimated_recovery_time: Some("5-30 minutes".to_string()),
                required_access: vec!["performance_admin".to_string()],
                escalation_path: vec![
                    "Performance Team".to_string(),
                    "Architecture Team".to_string(),
                ],
            },
            diagnostics: DiagnosticInfo {
                system_metrics: HashMap::new(),
                log_entries: Vec::new(),
                config_snapshot: HashMap::new(),
                network_status: None,
                resource_availability: HashMap::new(),
                performance_indicators,
            },
            related_errors: Vec::new(),
        };
        
        context
    }
}

/// **SAFETY AND INVARIANT DOCUMENTATION**
///
/// This enhanced error handling system ensures:
/// 
/// - **Comprehensive Context**: Every error includes full operational context
/// - **Recovery Guidance**: Clear steps for error resolution and prevention  
/// - **Escalation Paths**: Defined escalation procedures for critical errors
/// - **Diagnostic Information**: Detailed system state for troubleshooting
/// - **Performance Tracking**: Error timing and performance impact analysis
/// - **Security Awareness**: Special handling for security-related errors
/// - **Operational Excellence**: Production-ready error handling patterns 