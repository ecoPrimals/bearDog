// SPDX-License-Identifier: AGPL-3.0-only

//! # External FFI Types
//!
//! Type definitions for the External Function Interface (FFI) system.
//! Provides safe abstractions for calling external libraries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ============================================================
// Registry Configuration
// ============================================================

/// Configuration for the FFI registry
/// Renamed from RegistryConfig to FfiRegistryConfig for clarity
#[derive(Debug, Clone)]
pub struct FfiRegistryConfig {
    /// Maximum number of loaded libraries
    pub max_libraries: usize,

    /// Enable function caching
    pub enable_caching: bool,

    /// Enable safety checks
    pub enable_safety_checks: bool,

    /// Allowed library directories
    pub allowed_directories: Vec<PathBuf>,

    /// Function call timeout in milliseconds
    pub function_timeout_ms: u64,

    /// Enable logging of FFI calls
    pub enable_logging: bool,
}

/// Type alias for backward compatibility
pub type RegistryConfig = FfiRegistryConfig;

impl Default for FfiRegistryConfig {
    fn default() -> Self {
        Self {
            max_libraries: 100,
            enable_caching: true,
            enable_safety_checks: true,
            allowed_directories: vec![],
            function_timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
            enable_logging: true,
        }
    }
}

// ============================================================
// Library Types
// ============================================================

/// Handle to a loaded library
#[derive(Debug, Clone)]
pub struct LibraryHandle {
    /// Library identifier
    pub id: String,

    /// Library name
    pub name: String,

    /// Library path
    pub path: PathBuf,

    /// Library metadata
    pub metadata: LibraryMetadata,

    /// Load timestamp
    pub loaded_at: DateTime<Utc>,

    /// Library status
    pub status: LibraryStatus,
}

/// Library metadata
#[derive(Debug, Clone, Default)]
pub struct LibraryMetadata {
    /// Library version
    pub version: String,

    /// Library description
    pub description: String,

    /// Library author
    pub author: String,

    /// Library license
    pub license: String,

    /// Exported functions
    pub exported_functions: Vec<String>,

    /// Library dependencies
    pub dependencies: Vec<String>,

    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Library status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LibraryStatus {
    /// Library is loaded and ready
    #[default]
    Loaded,

    /// Library is loading
    Loading,

    /// Library failed to load
    Failed,

    /// Library is unloaded
    Unloaded,
}

// ============================================================
// Safety Types
// ============================================================

/// Safety level for function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SafetyLevel {
    /// Safe to call without restrictions
    #[default]
    Safe,

    /// Potentially unsafe but verified
    Verified,

    /// Unsafe but sandboxed
    Sandboxed,

    /// Unsafe and unsandboxed - use with caution
    Unsafe,
}

// ============================================================
// Function Types
// ============================================================

/// Handle to a cached function
#[derive(Debug, Clone)]
pub struct FunctionHandle {
    /// Function identifier
    pub id: String,

    /// Function name
    pub name: String,

    /// Library this function belongs to
    pub library_id: String,

    /// Function signature
    pub signature: FunctionSignature,

    /// Function call count
    pub call_count: u64,

    /// Last time this function was called
    pub last_called: Option<DateTime<Utc>>,

    /// Function metadata
    pub metadata: FunctionMetadata,
}

/// Function signature
#[derive(Debug, Clone, Default)]
pub struct FunctionSignature {
    /// Parameter types
    pub parameters: Vec<ParameterType>,

    /// Return type
    pub return_type: ReturnType,

    /// Calling convention
    pub calling_convention: CallingConvention,

    /// Function attributes
    pub attributes: Vec<FunctionAttribute>,
}

/// Parameter type for function parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterType {
    /// Void type
    Void,

    /// 8-bit signed integer
    Int8,

    /// 16-bit signed integer
    Int16,

    /// 32-bit signed integer
    Int32,

    /// 64-bit signed integer
    Int64,

    /// 8-bit unsigned integer
    UInt8,

    /// 16-bit unsigned integer
    UInt16,

    /// 32-bit unsigned integer
    UInt32,

    /// 64-bit unsigned integer
    UInt64,

    /// 32-bit floating point
    Float32,

    /// 64-bit floating point
    Float64,

    /// Pointer to another type
    Pointer(Box<ParameterType>),

    /// C-style string
    CString,

    /// Array of a type with size
    Array(Box<ParameterType>, usize),

    /// Custom/opaque type
    Custom(String),
}

impl Default for ParameterType {
    fn default() -> Self {
        Self::Void
    }
}

/// Return type for functions
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ReturnType {
    /// Void return
    #[default]
    Void,

    /// Typed return
    Type(ParameterType),

    /// Result type with success and error types
    Result(Box<ReturnType>, Box<ReturnType>),
}

/// Calling convention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallingConvention {
    /// C calling convention
    #[default]
    C,

    /// Standard calling convention
    Std,

    /// Fast calling convention
    Fast,

    /// System calling convention
    System,
}

/// Function attributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionAttribute {
    /// Pure function (no side effects)
    Pure,

    /// Thread-safe function
    ThreadSafe,

    /// Deprecated function
    Deprecated,

    /// Experimental function
    Experimental,

    /// Custom attribute
    Custom(String),
}

// ============================================================
// Function Metadata
// ============================================================

/// Function metadata
#[derive(Debug, Clone, Default)]
pub struct FunctionMetadata {
    /// Function description
    pub description: String,

    /// Safety level
    pub safety_level: SafetyLevel,

    /// Performance information
    pub performance: PerformanceInfo,

    /// Security considerations
    pub security: SecurityInfo,

    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Performance information
#[derive(Debug, Clone, Default)]
pub struct PerformanceInfo {
    /// Expected execution time in microseconds
    pub expected_duration_us: Option<u64>,

    /// Memory usage estimate in bytes
    pub memory_usage_bytes: Option<usize>,

    /// CPU intensity level
    pub cpu_intensity: CpuIntensity,

    /// Whether I/O operations are expected
    pub io_operations: bool,
}

/// CPU intensity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CpuIntensity {
    /// Low CPU usage
    #[default]
    Low,

    /// Medium CPU usage
    Medium,

    /// High CPU usage
    High,

    /// Critical CPU usage
    Critical,
}

/// Security information
#[derive(Debug, Clone, Default)]
pub struct SecurityInfo {
    /// Security clearance required
    pub clearance_required: SecurityClearance,

    /// Audit logging required
    pub audit_required: bool,

    /// Sandbox required
    pub sandbox_required: bool,

    /// Access restrictions
    pub access_restrictions: Vec<AccessRestriction>,
}

/// Security clearance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum SecurityClearance {
    /// Public access
    #[default]
    Public,

    /// Internal use only
    Internal,

    /// Restricted access
    Restricted,

    /// Confidential access
    Confidential,

    /// Secret access
    Secret,

    /// Top secret access
    TopSecret,
}

/// Access restrictions
#[derive(Debug, Clone)]
pub enum AccessRestriction {
    /// Time-based restriction
    TimeWindow(DateTime<Utc>, DateTime<Utc>),

    /// User-based restriction
    User(String),

    /// Role-based restriction
    Role(String),

    /// IP-based restriction
    IpAddress(String),

    /// Custom restriction with name and description
    Custom(String, String),
}

// ============================================================
// Function Values
// ============================================================

/// Function parameter/return value
#[derive(Debug, Clone)]
pub enum FunctionValue {
    /// String value
    String(String),

    /// 64-bit integer value
    Integer(i64),

    /// 64-bit floating point value
    Float(f64),

    /// Boolean value
    Boolean(bool),

    /// Binary data
    Binary(Vec<u8>),

    /// Array of values
    Array(Vec<FunctionValue>),

    /// Custom binary data
    Custom(Vec<u8>),

    /// Null/empty value
    Null,
}

impl Default for FunctionValue {
    fn default() -> Self {
        Self::Null
    }
}

// ============================================================
// External Function Types
// ============================================================

/// External function representation
#[derive(Debug, Clone)]
pub struct ExternalFunction {
    /// Unique identifier
    pub id: String,

    /// Function name
    pub name: String,

    /// Function signature
    pub signature: FunctionSignature,

    /// Function metadata
    pub metadata: FunctionMetadata,

    /// Library identifier
    pub library_id: String,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct FunctionParameter {
    /// Parameter name
    pub name: String,

    /// Parameter type
    pub param_type: ParameterType,

    /// Whether the parameter is required
    pub required: bool,

    /// Optional default value
    pub default_value: Option<FunctionValue>,
}

/// Function result
#[derive(Debug, Clone)]
pub struct FunctionResult {
    /// Whether the call was successful
    pub success: bool,

    /// Return value
    pub value: Option<FunctionValue>,

    /// Error message if failed
    pub error: Option<String>,

    /// Execution time in microseconds
    pub execution_time_us: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_config_default() {
        let config = FfiRegistryConfig::default();
        assert_eq!(config.max_libraries, 100);
        assert!(config.enable_caching);
        assert!(config.enable_safety_checks);
    }

    #[test]
    fn test_library_status_default() {
        assert_eq!(LibraryStatus::default(), LibraryStatus::Loaded);
    }

    #[test]
    fn test_safety_level_default() {
        assert_eq!(SafetyLevel::default(), SafetyLevel::Safe);
    }

    #[test]
    fn test_security_clearance_ordering() {
        assert!(SecurityClearance::TopSecret > SecurityClearance::Secret);
        assert!(SecurityClearance::Secret > SecurityClearance::Confidential);
        assert!(SecurityClearance::Confidential > SecurityClearance::Restricted);
    }

    #[test]
    fn test_function_value_default() {
        assert!(matches!(FunctionValue::default(), FunctionValue::Null));
    }
}
