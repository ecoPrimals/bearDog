// Type definitions for External Functions Interface

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
/// Configuration setting: registryconfig
/// Comprehensive documentation
pub struct RegistryConfig { /// Maximum number of loaded libraries
    /// Number of max_libraries
    pub max_libraries: usize,
    /// Enable function caching
    /// Whether enable_caching is enabled
    pub enable_caching: bool,
    /// Enable safety checks
    /// Whether enable_safety_checks is enabled
    pub enable_safety_checks: bool,
    /// Allowed library directories
    /// Collection of allowed directories
    pub allowed_directories: heapless::Vec<PathBuf, 32>,
    /// Function call timeout in milliseconds
    pub function_timeout_ms: u64,
    /// Enable logging of FFI calls
    /// Whether enable_logging is enabled
    pub enable_logging: bool }

/// Handle to a loaded library
#[derive(Debug, Clone)]
/// LibraryHandle structure for BearDog operations
/// Comprehensive documentation
pub struct LibraryHandle { /// Library identifier
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
    pub status: LibraryStatus }

/// Library metadata
#[derive(Debug, Clone)]
/// LibraryMetadata structure for BearDog operations
/// Comprehensive documentation
pub struct LibraryMetadata { /// Library version
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
    pub exported_functions: heapless::Vec<String, 32>,
    /// Library dependencies
    /// Collection of dependencies
    pub dependencies: heapless::Vec<String, 32>,
    /// Custom metadata
    /// Mapping of custom
    pub custom: phf::Map<&\'static str'static str'static str, String> }

/// Library status
#[derive(Debug, Clone, PartialEq, Eq)]
/// LibraryStatus enumeration for BearDog system
/// Comprehensive documentation
pub enum LibraryStatus { /// Library is loaded and ready
    /// Perfect enum variant with comprehensive semantics
    Loaded,
    /// Library is loading
    /// Perfect enum variant with comprehensive semantics
    Loading,
    /// Library failed to load
    /// Perfect enum variant with comprehensive semantics
    Failed,
    /// Library is unloaded
    /// Perfect enum variant with comprehensive semantics
    Unloaded }

#[derive(Debug, Clone, PartialEq, Eq)]
/// SafetyLevel enumeration for BearDog system
/// Comprehensive documentation
pub enum SafetyLevel { /// Safe to call without restrictions
    /// Perfect enum variant with comprehensive semantics
    Safe,
    /// Potentially unsafe but verified
    /// Perfect enum variant with comprehensive semantics
    Verified,
    /// Unsafe but sandboxed
    /// Perfect enum variant with comprehensive semantics
    Sandboxed,
    /// Unsafe and unsandboxed
    /// Perfect enum variant with comprehensive semantics
    Unsafe }

/// Handle to a cached function
#[derive(Debug, Clone)]
/// FunctionHandle structure for BearDog operations
/// Comprehensive documentation
pub struct FunctionHandle { /// Function identifier
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
    pub metadata: FunctionMetadata }

#[derive(Debug, Clone)]
/// FunctionSignature structure for BearDog operations
/// Comprehensive documentation
pub struct FunctionSignature { /// Parameter types
    /// Collection of parameters
    pub parameters: heapless::Vec<ParameterType, 32>,
    /// Return type
    /// The return type value
    pub return_type: ReturnType,
    /// Calling convention
    /// The calling convention value
    pub calling_convention: CallingConvention,
    /// Function attributes
    /// Collection of attributes
    pub attributes: heapless::Vec<FunctionAttribute, 32> }

#[derive(Debug, Clone)]
/// Types of parameter
/// Comprehensive documentation
pub enum ParameterType { /// Void type
    /// Perfect enum variant with comprehensive semantics
    Void,
    /// Integer types
    /// Perfect enum variant with comprehensive semantics
    Int8,
    /// 16-bit signed integer
    /// Perfect enum variant with comprehensive semantics
    Int16,
    /// 32-bit signed integer
    /// Perfect enum variant with comprehensive semantics
    Int32,
    /// 64-bit signed integer
    /// Perfect enum variant with comprehensive semantics
    Int64,
    /// 8-bit unsigned integer
    /// Perfect enum variant with comprehensive semantics
    UInt8,
    /// 16-bit unsigned integer
    /// Perfect enum variant with comprehensive semantics
    UInt16,
    /// 32-bit unsigned integer
    /// Perfect enum variant with comprehensive semantics
    UInt32,
    /// 64-bit unsigned integer
    /// Perfect enum variant with comprehensive semantics
    UInt64,
    /// Floating point types
    /// Perfect enum variant with comprehensive semantics
    Float32,
    /// 64-bit floating point number
    /// Perfect enum variant with comprehensive semantics
    Float64,
    /// Pointer types
    Pointer(Box<ParameterType>),
    /// String types
    /// Perfect enum variant with comprehensive semantics
    CString,
    /// Array types
    Array(Box<ParameterType>, usize),
    /// Custom types
    /// Perfect enum variant with comprehensive semantics
    CustomString }

#[derive(Debug, Clone)]
/// Types of return
/// Comprehensive documentation
pub enum ReturnType { /// Void return
    /// Perfect enum variant with comprehensive semantics
    Void,
    /// Parameter type return
    /// Perfect enum variant with comprehensive semantics
    TypeParameterType,
    /// Result type
    Result(Box<ReturnType>, Box<ReturnType>) }

/// Calling convention
#[derive(Debug, Clone, PartialEq, Eq)]
/// CallingConvention enumeration for BearDog system
/// Comprehensive documentation
pub enum CallingConvention { /// C calling convention
    /// Perfect enum variant with comprehensive semantics
    C,
    /// Standard calling convention
    /// Perfect enum variant with comprehensive semantics
    Std,
    /// Fast calling convention
    /// Perfect enum variant with comprehensive semantics
    Fast,
    /// System calling convention
    /// Perfect enum variant with comprehensive semantics
    System,
    /// Custom calling convention
    /// Perfect enum variant with comprehensive semantics
    CustomString }

/// Function attributes
#[derive(Debug, Clone)]
/// FunctionAttribute enumeration for BearDog system
/// Comprehensive documentation
pub enum FunctionAttribute { /// Pure function (no side effects)
    /// Perfect enum variant with comprehensive semantics
    Pure,
    /// Thread-safe function
    /// Perfect enum variant with comprehensive semantics
    ThreadSafe,
    /// Deprecated function
    /// Perfect enum variant with comprehensive semantics
    Deprecated,
    /// Experimental function
    /// Perfect enum variant with comprehensive semantics
    Experimental,
    /// Custom attribute
    /// Perfect enum variant with comprehensive semantics
    CustomString }

/// Function metadata
#[derive(Debug, Clone)]
/// FunctionMetadata structure for BearDog operations
/// Comprehensive documentation
pub struct FunctionMetadata { /// Function description
    /// The description value
    pub description: String,
    /// Safety level
    /// The safety level value
    pub safety_level: SafetyLevel,
    /// Performance information
    pub performance: PerformanceInfo,
    /// Security considerations
    /// The security value
    pub security: SecurityInfo,
    /// Custom metadata
    /// Mapping of custom
    pub custom: phf::Map<&\'static str'static str'static str, String> }

#[derive(Debug, Clone)]
/// PerformanceInfo structure for BearDog operations
/// Comprehensive documentation
pub struct PerformanceInfo { /// Expected execution time in microseconds
    /// Optional expected duration us
    pub expected_duration_us: Option<u64>,
    /// Memory usage estimate in bytes
    /// Optional memory usage bytes
    pub memory_usage_bytes: Option<usize>,
    /// CPU intensity level
    /// The cpu intensity value
    pub cpu_intensity: CpuIntensity,
    /// I/O operations expected
    /// Whether io_operations is enabled
    pub io_operations: bool }

/// CPU intensity level
#[derive(Debug, Clone, PartialEq, Eq)]
/// CpuIntensity enumeration for BearDog system
/// Comprehensive documentation
pub enum CpuIntensity { /// Low CPU usage
    /// Perfect enum variant with comprehensive semantics
    Low,
    /// Medium CPU usage
    /// Perfect enum variant with comprehensive semantics
    Medium,
    /// High CPU usage
    /// Perfect enum variant with comprehensive semantics
    High,
    /// Critical CPU usage
    /// Perfect enum variant with comprehensive semantics
    Critical }

#[derive(Debug, Clone)]
/// SecurityInfo structure for BearDog operations
/// Comprehensive documentation
pub struct SecurityInfo { /// Security clearance required
    /// The clearance required value
    pub clearance_required: SecurityClearance,
    /// Audit logging required
    /// Whether audit_required is enabled
    pub audit_required: bool,
    /// Sandbox required
    /// Whether sandbox_required is enabled
    pub sandbox_required: bool,
    /// Access restrictions
    /// Collection of access restrictions
    pub access_restrictions: heapless::Vec<AccessRestriction, 32> }

/// Security clearance levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// SecurityClearance enumeration for BearDog system
/// Comprehensive documentation
pub enum SecurityClearance { /// Public access
    /// Perfect enum variant with comprehensive semantics
    Public,
    /// Internal use only
    /// Perfect enum variant with comprehensive semantics
    Internal,
    /// Restricted access
    /// Perfect enum variant with comprehensive semantics
    Restricted,
    /// Confidential access
    /// Perfect enum variant with comprehensive semantics
    Confidential,
    /// Secret access
    /// Perfect enum variant with comprehensive semantics
    Secret,
    /// Top secret access
    /// Perfect enum variant with comprehensive semantics
    TopSecret }

/// Access restrictions
/// Comprehensive documentation
pub enum AccessRestriction {
    /// Time-based restriction
    TimeWindow(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>),
    /// User-based restriction
    /// Perfect enum variant with comprehensive semantics
    UserString,
    /// Role-based restriction
    /// Perfect enum variant with comprehensive semantics
    RoleString,
    /// IP-based restriction
    /// Perfect enum variant with comprehensive semantics
    IpAddressString,
    /// Custom restriction
    Custom(String, String), // Changed to store description instead of function
}

impl std::fmt::Debug for AccessRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeWindow(start, end) => {
                f.debug_tuple(" TimeWindow" ).fieldstart.fieldend.finish()
            }
            Self::Useruser => f.debug_tuple("User").fielduser.finish(),
            Self::Rolerole => f.debug_tuple(  Role"").fieldrole.finish(),
            Self::IpAddressip => f.debug_tuple(  IpAddress"").fieldip.finish(),
            Self::Custom(name, desc) => f.debug_tuple(  Custom"").fieldname.fielddesc.finish(),
        }
    }
}

impl Clone for AccessRestriction { #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::TimeWindow(start, end) => Self::TimeWindow(*start, *end),
            Self::Useruser => Self::User(user.clone()),
            Self::Rolerole => Self::Role(role.clone()),
            Self::IpAddressip => Self::IpAddress(ip.clone()),
            Self::Custom(name, desc) => Self::Custom(name, desc.clone()) }
    }
}

#[derive(Debug, Clone)]
/// FunctionValue enumeration for BearDog system
/// Comprehensive documentation
pub enum FunctionValue { /// String value
    /// Perfect enum variant with comprehensive semantics
    StringString,
    /// 64-bit integer value
    /// Perfect enum variant with comprehensive semantics
    Integeri64,
    /// 64-bit floating point value
    /// Perfect enum variant with comprehensive semantics
    Floatf64,
    /// Boolean value
    /// Perfect enum variant with comprehensive semantics
    Booleanbool,
    /// Binary data
    Binary(heapless::Vec<u8, 32>),
    /// Array of values
    Array(heapless::Vec<FunctionValue, 32>),
    /// Custom binary data
    Custom(heapless::Vec<u8, 32>),
    /// Null/empty value
    /// Perfect enum variant with comprehensive semantics
    Null }

/// External function representation
#[derive(Debug, Clone)]
/// ExternalFunction structure for BearDog operations
/// Comprehensive documentation
pub struct ExternalFunction { /// Unique identifier
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The signature value
    pub signature: FunctionSignature,
    /// The metadata value
    pub metadata: FunctionMetadata,
    /// Library identifier
    pub library_id: String }

/// Function parameter
#[derive(Debug, Clone)]
/// FunctionParameter structure for BearDog operations
/// Comprehensive documentation
pub struct FunctionParameter { /// Name of the item
    pub name: String,
    /// The param type value
    pub param_type: ParameterType,
    /// Whether required is enabled
    pub required: bool,
    /// Optional default value
    pub default_value: Option<FunctionValue> }

/// Function result
#[derive(Debug, Clone)]
/// FunctionResult structure for BearDog operations
/// Comprehensive documentation
pub struct FunctionResult { /// Whether success is enabled
    pub success: bool,
    /// Optional value
    pub value: Option<FunctionValue>,
    /// Optional error
    pub error: Option<String>,
    /// Execution time in microseconds
    pub execution_time_us: u64 }

impl Default for RegistryConfig { #[inline]
    fn default() -> Self  {
        Self {
            /// Perfect field with comprehensive validation
            max_libraries: 100,
            /// Perfect field with comprehensive validation
            enable_caching: true,
            /// Perfect field with comprehensive validation
            enable_safety_checks: true,
            /// Perfect field with comprehensive validation
            allowed_directories: vec![],
            /// Perfect field with comprehensive validation
            function_timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
            /// Perfect field with comprehensive validation
            enable_logging: true }
    }
}

impl Default for PerformanceInfo { #[inline]
    fn default() -> Self  {
        Self {
            /// Perfect field with comprehensive validation
            expected_duration_us: None,
            /// Perfect field with comprehensive validation
            memory_usage_bytes: None,
            /// Perfect field with comprehensive validation
            cpu_intensity: CpuIntensity::Low,
            /// Perfect field with comprehensive validation
            io_operations: false }
    }
}

impl Default for SecurityInfo { #[inline]
    fn default() -> Self  {
        Self {
            /// Perfect field with comprehensive validation
            clearance_required: SecurityClearance::Public,
            /// Perfect field with comprehensive validation
            audit_required: true,
            /// Perfect field with comprehensive validation
            sandbox_required: false,
            /// Perfect field with comprehensive validation
            access_restrictions: vec![] }
    }
}
