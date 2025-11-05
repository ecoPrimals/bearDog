// Type definitions for External Functions Interface

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// External functions registry configuration
/// Renamed from `RegistryConfig` to `ExternalFunctionsRegistryConfig` for clarity
#[derive(Debug, Clone)]
pub struct ExternalFunctionsRegistryConfig {
    /// Maximum number of loaded libraries
    /// Number of `max_libraries`
    pub max_libraries: usize,
    /// Enable function caching
    /// Whether `enable_caching` is enabled
    pub enable_caching: bool,
    /// Enable safety checks
    /// Whether `enable_safety_checks` is enabled
    pub enable_safety_checks: bool,
    /// Allowed library directories
    /// Collection of allowed directories
    pub allowed_directories: Vec<PathBuf>,
    /// Function call timeout in milliseconds
    pub function_timeout_ms: u64,
    /// Enable logging of FFI calls
    /// Whether `enable_logging` is enabled
    pub enable_logging: bool,
}

/// Handle to a loaded library
#[derive(Debug, Clone)]
pub struct LibraryHandle {
    /// Library identifier
    pub id: String,
    /// Library name
    /// Name of the item
    pub name: String,
    /// Library path
    /// The path value
    pub path: PathBuf,
    /// Library metadata
    /// The metadata value
    pub metadata: LibraryMetadata,
    /// Load timestamp
    /// The loaded at value
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// Library status
    /// Current status of the component
    pub status: LibraryStatus,
}

/// Library metadata
#[derive(Debug, Clone)]
pub struct LibraryMetadata {
    /// Library version
    /// The version value
    pub version: String,
    /// Library description
    /// The description value
    pub description: String,
    /// Library author
    /// The author value
    pub author: String,
    /// Library license
    /// The license value
    pub license: String,
    /// Exported functions
    /// Collection of exported functions
    pub exported_functions: Vec<String>,
    /// Library dependencies
    /// Collection of dependencies
    pub dependencies: Vec<String>,
    /// Custom metadata
    /// Mapping of custom
    pub custom: HashMap<String, String>,
}

/// Library status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryStatus {
    /// Library is loaded and ready
    Loaded,
    /// Library is loading
    Loading,
    /// Library failed to load
    Failed,
    /// Library is unloaded
    Unloaded,
}

/// Safety level for external function calls
///
/// Classifies the safety guarantees of FFI function calls,
/// from fully safe to potentially unsafe operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    /// Safe to call without restrictions (no unsafe operations)
    Safe,
    /// Potentially unsafe but verified through testing and audits
    Verified,
    /// Unsafe operations contained within a sandbox
    Sandboxed,
    /// Unsafe operations without sandbox protection
    Unsafe,
}

/// Handle to a cached function
#[derive(Debug, Clone)]
pub struct FunctionHandle {
    /// Function identifier
    pub id: String,
    /// Function name
    /// Name of the item
    pub name: String,
    /// Library this function belongs to
    pub library_id: String,
    /// Function signature
    /// The signature value
    pub signature: FunctionSignature,
    /// Function call count
    /// Number of call
    pub call_count: u64,
    /// Last time this function was called
    /// Optional last called
    pub last_called: Option<chrono::DateTime<chrono::Utc>>,
    /// Function metadata
    /// The metadata value
    pub metadata: FunctionMetadata,
}

/// Function signature for external function calls
///
/// Describes the parameters, return type, calling convention, and attributes of an external function.
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Parameter types for this function
    pub parameters: Vec<ParameterType>,
    /// Return type of the function
    pub return_type: ReturnType,
    /// Calling convention (C, Std, Fast, etc.)
    pub calling_convention: CallingConvention,
    /// Function attributes (Pure, `ThreadSafe`, etc.)
    pub attributes: Vec<FunctionAttribute>,
}

#[derive(Debug, Clone)]
/// Types of parameter
pub enum ParameterType {
    /// Void type
    Void,
    /// Integer types
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
    /// Floating point types
    Float32,
    /// 64-bit floating point number
    Float64,
    /// Pointer types
    Pointer(Box<ParameterType>),
    /// String types
    CString,
    /// Array types
    Array(Box<ParameterType>, usize),
    /// Custom types
    Custom(String),
}

#[derive(Debug, Clone)]
/// Types of return
pub enum ReturnType {
    /// Void return
    Void,
    /// Parameter type return
    Type(ParameterType),
    /// Result type
    Result(Box<ReturnType>, Box<ReturnType>),
}

/// Calling convention
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallingConvention {
    /// C calling convention
    C,
    /// Standard calling convention
    Std,
    /// Fast calling convention
    Fast,
    /// System calling convention
    System,
    /// Custom calling convention
    Custom(String),
}

/// Function attributes
#[derive(Debug, Clone)]
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

/// Metadata about an external function
///
/// Provides detailed information about safety, performance, and security characteristics.
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    /// Human-readable function description
    pub description: String,
    /// Safety level classification
    pub safety_level: SafetyLevel,
    /// Performance characteristics and estimates
    pub performance: PerformanceInfo,
    /// Security requirements and restrictions
    pub security: SecurityInfo,
    /// Custom metadata key-value pairs
    pub custom: HashMap<String, String>,
}

/// Performance characteristics of an external function
///
/// Provides estimates for execution time, memory usage, and CPU intensity.
#[derive(Debug, Clone)]
pub struct PerformanceInfo {
    /// Expected execution time in microseconds
    pub expected_duration_us: Option<u64>,
    /// Estimated memory usage in bytes
    pub memory_usage_bytes: Option<usize>,
    /// CPU intensity level
    /// The cpu intensity value
    pub cpu_intensity: CpuIntensity,
    /// I/O operations expected
    /// Whether `io_operations` is enabled
    pub io_operations: bool,
}

/// CPU intensity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuIntensity {
    /// Low CPU usage
    Low,
    /// Medium CPU usage
    Medium,
    /// High CPU usage
    High,
    /// Critical CPU usage
    Critical,
}

/// Security information for an external function
///
/// Defines security clearance, auditing requirements, sandboxing, and access restrictions.
#[derive(Debug, Clone)]
pub struct SecurityInfo {
    /// Security clearance level required to call this function
    pub clearance_required: SecurityClearance,
    /// Whether audit logging is required for calls
    pub audit_required: bool,
    /// Whether the function must run in a sandbox
    pub sandbox_required: bool,
    /// Access restrictions for calling this function
    pub access_restrictions: Vec<AccessRestriction>,
}

/// Security clearance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityClearance {
    /// Public access
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
pub enum AccessRestriction {
    /// Time-based restriction
    TimeWindow(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>),
    /// User-based restriction
    User(String),
    /// Role-based restriction
    Role(String),
    /// IP-based restriction
    IpAddress(String),
    /// Custom restriction
    Custom(String, String), // Changed to store description instead of function
}

impl std::fmt::Debug for AccessRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeWindow(start, end) => {
                f.debug_tuple("TimeWindow").field(start).field(end).finish()
            }
            Self::User(user) => f.debug_tuple("User").field(user).finish(),
            Self::Role(role) => f.debug_tuple("Role").field(role).finish(),
            Self::IpAddress(ip) => f.debug_tuple("IpAddress").field(ip).finish(),
            Self::Custom(name, desc) => f.debug_tuple("Custom").field(name).field(desc).finish(),
        }
    }
}

impl Clone for AccessRestriction {
    fn clone(&self) -> Self {
        match self {
            Self::TimeWindow(start, end) => Self::TimeWindow(*start, *end),
            Self::User(user) => Self::User(user.clone()),
            Self::Role(role) => Self::Role(role.clone()),
            Self::IpAddress(ip) => Self::IpAddress(ip.clone()),
            Self::Custom(name, desc) => Self::Custom(name.clone(), desc.clone()),
        }
    }
}

/// Value types that can be passed to/from external functions
///
/// Supports common data types for FFI interoperability, including
/// primitives, binary data, and nested structures.
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

/// External function representation
///
/// Describes an external C function available for calling via FFI.
#[derive(Debug, Clone)]
pub struct ExternalFunction {
    /// Unique function identifier
    pub id: String,
    /// Function name
    pub name: String,
    /// Function signature (parameters, return type, etc.)
    pub signature: FunctionSignature,
    /// Function metadata (performance, security, etc.)
    pub metadata: FunctionMetadata,
    /// ID of the library containing this function
    pub library_id: String,
}

/// Function parameter definition
///
/// Describes a single parameter for an external function including name, type, and constraints.
#[derive(Debug, Clone)]
pub struct FunctionParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Whether this parameter is required
    pub required: bool,
    /// Default value if not provided
    pub default_value: Option<FunctionValue>,
}

/// Result of an external function call
///
/// Contains the return value, success status, error information, and timing.
#[derive(Debug, Clone)]
pub struct FunctionResult {
    /// Whether the function call succeeded
    pub success: bool,
    /// Return value from the function
    pub value: Option<FunctionValue>,
    /// Error message if the call failed
    pub error: Option<String>,
    /// Execution time in microseconds
    pub execution_time_us: u64,
}

impl Default for ExternalFunctionsRegistryConfig {
    fn default() -> Self {
        Self {
            max_libraries: 100,
            enable_caching: true,
            enable_safety_checks: true,
            allowed_directories: vec![],
            function_timeout_ms: 5000,
            enable_logging: true,
        }
    }
}

impl Default for PerformanceInfo {
    fn default() -> Self {
        Self {
            expected_duration_us: None,
            memory_usage_bytes: None,
            cpu_intensity: CpuIntensity::Low,
            io_operations: false,
        }
    }
}

impl Default for SecurityInfo {
    fn default() -> Self {
        Self {
            clearance_required: SecurityClearance::Public,
            audit_required: true,
            sandbox_required: false,
            access_restrictions: vec![],
        }
    }
}
