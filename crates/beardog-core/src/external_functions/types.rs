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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    /// Safe to call without restrictions
    Safe,
    /// Potentially unsafe but verified
    Verified,
    /// Unsafe but sandboxed
    Sandboxed,
    /// Unsafe and unsandboxed
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

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Parameter types
    /// Collection of parameters
    pub parameters: Vec<ParameterType>,
    /// Return type
    /// The return type value
    pub return_type: ReturnType,
    /// Calling convention
    /// The calling convention value
    pub calling_convention: CallingConvention,
    /// Function attributes
    /// Collection of attributes
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

/// Function metadata
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    /// Function description
    /// The description value
    pub description: String,
    /// Safety level
    /// The safety level value
    pub safety_level: SafetyLevel,
    pub performance: PerformanceInfo,
    /// Security considerations
    /// The security value
    pub security: SecurityInfo,
    /// Custom metadata
    /// Mapping of custom
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PerformanceInfo {
    /// Expected execution time in microseconds
    /// Optional expected duration us
    pub expected_duration_us: Option<u64>,
    /// Memory usage estimate in bytes
    /// Optional memory usage bytes
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

#[derive(Debug, Clone)]
pub struct SecurityInfo {
    /// Security clearance required
    /// The clearance required value
    pub clearance_required: SecurityClearance,
    /// Audit logging required
    /// Whether `audit_required` is enabled
    pub audit_required: bool,
    /// Sandbox required
    /// Whether `sandbox_required` is enabled
    pub sandbox_required: bool,
    /// Access restrictions
    /// Collection of access restrictions
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
#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The signature value
    pub signature: FunctionSignature,
    /// The metadata value
    pub metadata: FunctionMetadata,
    pub library_id: String,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct FunctionParameter {
    /// Name of the item
    pub name: String,
    /// The param type value
    pub param_type: ParameterType,
    /// Whether required is enabled
    pub required: bool,
    /// Optional default value
    pub default_value: Option<FunctionValue>,
}

/// Function result
#[derive(Debug, Clone)]
pub struct FunctionResult {
    /// Whether success is enabled
    pub success: bool,
    /// Optional value
    pub value: Option<FunctionValue>,
    /// Optional error
    pub error: Option<String>,
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
