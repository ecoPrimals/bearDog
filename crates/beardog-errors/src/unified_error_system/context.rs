// SPDX-License-Identifier: AGPL-3.0-only

//! # Error Context Module
//!
//! This module contains error context types and contextual information for enhanced error handling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **ERROR CONTEXT** - Rich contextual information for errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Error occurrence timestamp
    pub timestamp: SystemTime,

    /// Component or module where error occurred
    pub component: String,

    /// Function or method where error occurred
    pub function: String,

    /// Source file and line number
    pub location: Option<ErrorLocation>,

    /// Error correlation ID for tracking
    pub correlation_id: String,

    /// Request ID if applicable
    pub request_id: Option<String>,

    /// User ID if applicable
    pub user_id: Option<String>,

    /// Session ID if applicable
    pub session_id: Option<String>,

    /// Additional context metadata
    pub metadata: HashMap<String, String>,

    /// Error cause chain
    pub cause_chain: Vec<String>,

    /// System state at time of error
    pub system_state: SystemState,
}

/// **ERROR LOCATION** - Source code location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    /// Source file path
    pub file: String,

    /// Line number in source file
    pub line: u32,

    /// Column number in source file
    pub column: Option<u32>,

    /// Function or method name
    pub function: Option<String>,

    /// Module path
    pub module: Option<String>,
}

/// **SYSTEM STATE** - System state information at error occurrence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemState {
    /// Available memory in bytes
    pub available_memory: Option<u64>,

    /// CPU usage percentage
    pub cpu_usage: Option<f64>,

    /// Active connections count
    pub active_connections: Option<u32>,

    /// System load average
    pub load_average: Option<f64>,

    /// Disk usage percentage
    pub disk_usage: Option<f64>,

    /// Network status
    pub network_status: NetworkStatus,

    /// Service health status
    pub service_health: ServiceHealth,

    /// Environment variables relevant to the error
    pub environment: HashMap<String, String>,
}

/// **NETWORK STATUS** - Network connectivity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Network connectivity available
    pub connected: bool,

    /// Network latency in milliseconds
    pub latency_ms: Option<f64>,

    /// Bandwidth utilization percentage
    pub bandwidth_usage: Option<f64>,

    /// Active network interfaces
    pub interfaces: Vec<String>,

    /// DNS resolution status
    pub dns_resolution: bool,
}

/// **SERVICE HEALTH** - Health status of related services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Overall health status
    pub status: HealthStatus,

    /// Individual service statuses
    pub services: HashMap<String, HealthStatus>,

    /// Last health check timestamp
    pub last_check: SystemTime,

    /// Health check interval
    pub check_interval: std::time::Duration,
}

/// **HEALTH STATUS** - Service health status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status unknown
    Unknown,
}

impl ErrorContext {
    /// Create a new error context with current timestamp
    #[must_use]
    pub fn new() -> Self {
        Self {
            timestamp: SystemTime::now(),
            component: "unknown".to_string(),
            function: "unknown".to_string(),
            location: None,
            correlation_id: generate_correlation_id(),
            request_id: None,
            user_id: None,
            session_id: None,
            metadata: HashMap::new(),
            cause_chain: Vec::new(),
            system_state: SystemState::new(),
        }
    }

    #[must_use]
    /// Create context with component information
    pub fn with_component(component: &str) -> Self {
        let mut context = Self::new();
        context.component = component.to_string();
        context
    }
 #[must_use]

    /// Create context with component and function information
    pub fn with_component_function(component: &str, function: &str) -> Self {
        let mut context = Self::with_component(component);
        context.function = function.to_string();
        context
    }

    /// Add metadata to the context
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Add to the error cause chain
    pub fn add_cause(&mut self, cause: String) {
        self.cause_chain.push(cause);
    }

    /// Set request ID
    #[must_use]
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Set user ID
    #[must_use]
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set session ID
    #[must_use]
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Set error location
    pub fn with_location(mut self, location: ErrorLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Update system state
    #[must_use]
    pub fn update_system_state(&mut self) {
        self.system_state = SystemState::collect_current();
    }
}

impl ErrorLocation {
    /// Create a new error location
    pub fn new(file: String, line: u32) -> Self {
        Self {
            file,
            line,
            column: None,
            function: None,
            module: None,
        }
    }

    /// Create location with full information
    pub fn with_details(
        file: String,
        line: u32,
        column: Option<u32>,
        function: Option<String>,
        module: Option<String>,
    ) -> Self {
        Self {
            file,
            line,
            column,
            function,
            module,
        }
    }
}

impl SystemState {
    /// Create a new system state with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Collect current system state information
    pub fn collect_current() -> Self {
        let mut state = Self::new();

        // Collect basic system information (simplified implementation)
        state.available_memory = get_available_memory();
        state.cpu_usage = get_cpu_usage();
        state.load_average = get_load_average();
        state.disk_usage = get_disk_usage();

        // Collect relevant environment variables
        for (key, value) in std::env::vars() {
            if key.starts_with("BEARDOG_") || key.starts_with("RUST_") {
                state.environment.insert(key, value);
            }
        }

        state
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for NetworkStatus {
    fn default() -> Self {
        Self {
            connected: true,
            latency_ms: None,
            bandwidth_usage: None,
            interfaces: Vec::new(),
            dns_resolution: true,
        }
    }
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            services: HashMap::new(),
            last_check: SystemTime::now(),
            check_interval: std::time::Duration::from_secs(30),
        }
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "HEALTHY"),
            Self::Degraded => write!(f, "DEGRADED"),
            Self::Unhealthy => write!(f, "UNHEALTHY"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

// Helper functions for system information collection
fn generate_correlation_id() -> String {
    format!(
        "corr-{}",
        SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    )
}

fn get_available_memory() -> Option<u64> {
    // Simplified implementation - in a real system, this would query actual memory
    None
}

fn get_cpu_usage() -> Option<f64> {
    // Simplified implementation - in a real system, this would query actual CPU usage
    None
}

fn get_load_average() -> Option<f64> {
    // Simplified implementation - in a real system, this would query actual load average
    None
}

fn get_disk_usage() -> Option<f64> {
    // Simplified implementation - in a real system, this would query actual disk usage
    None
}

/// Macro to create error location from current source location
#[macro_export]
macro_rules! error_location {
    () => {
        ErrorLocation::new(file!().to_string(), line!())
    };
}

/// Macro to create error context with current location
#[macro_export]
macro_rules! error_context {
    ($component:expr) => {
        ErrorContext::with_component($component).with_location(error_location!())
    };
    ($component:expr, $function:expr) => {
        ErrorContext::with_component_function($component, $function)
            .with_location(error_location!())
    };
}
