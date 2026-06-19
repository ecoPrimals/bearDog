use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CORE CONSTRAINT TYPES
// ============================================================================

/// Complete set of constraints for a key
///
/// These constraints are cryptographically signed and embedded in the key.
/// They cannot be removed or modified without invalidating the key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyConstraints {
    /// Scope restrictions (what the key can access)
    pub scope: ScopeConstraint,

    /// Lifetime management (when the key expires/evolves)
    pub lifetime: LifetimeConstraint,

    /// Data access rules (what data operations are allowed)
    pub data_access: DataAccessConstraint,

    /// Required co-signers for operations
    pub co_signers: Vec<String>,

    /// Behavioral requirements (biometric, patterns, etc.)
    pub behavior: BehavioralConstraint,

    /// Compute resource quotas (optional)
    pub compute_quota: Option<ComputeQuota>,
}

impl Default for KeyConstraints {
    fn default() -> Self {
        Self {
            scope: ScopeConstraint::Unrestricted,
            lifetime: LifetimeConstraint::default(),
            data_access: DataAccessConstraint::default(),
            co_signers: Vec::new(),
            behavior: BehavioralConstraint::default(),
            compute_quota: None,
        }
    }
}

// ============================================================================
// SCOPE CONSTRAINTS
// ============================================================================

/// Defines what scope the key can operate in
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScopeConstraint {
    /// No scope restrictions
    Unrestricted,

    /// Key can only be used for specific project
    Project {
        /// Project name
        name: String,
        /// Cryptographic hash of project definition (immutable)
        project_hash: [u8; 32],
    },

    /// Key can only access specific resources
    Resources {
        /// Glob patterns for allowed reads
        allow_read: Vec<String>,
        /// Glob patterns for allowed writes
        allow_write: Vec<String>,
        /// Paths that cannot be deleted (cryptographically enforced)
        deny_delete: Vec<String>,
    },

    /// Key scoped to specific operations
    Operations {
        /// List of allowed operation names
        allowed_operations: Vec<String>,
    },
}

// ============================================================================
// LIFETIME CONSTRAINTS
// ============================================================================

/// Defines when a key expires and how it can be renewed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifetimeConstraint {
    /// Hard expiration - key becomes invalid (self-destructs)
    pub expires_at: DateTime<Utc>,

    /// Soft expiration - triggers evolution to new key
    pub evolution_trigger: Option<DateTime<Utc>>,

    /// Can this key be renewed?
    pub renewable: bool,

    /// Who can approve renewal? (key IDs)
    pub renewal_approvers: Vec<String>,

    /// Maximum number of renewals allowed
    pub max_renewals: Option<u32>,

    /// Current renewal count
    pub renewal_count: u32,
}

impl Default for LifetimeConstraint {
    fn default() -> Self {
        Self {
            expires_at: Utc::now() + chrono::Duration::days(365), // 1 year default
            evolution_trigger: None,
            renewable: true,
            renewal_approvers: Vec::new(),
            max_renewals: None,
            renewal_count: 0,
        }
    }
}

// ============================================================================
// DATA ACCESS CONSTRAINTS
// ============================================================================

/// Defines what data operations are allowed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DataAccessConstraint {
    /// Paths that cannot be deleted (cryptographically enforced)
    pub immutable_paths: Vec<String>,

    /// Data must be encrypted to these keys
    pub mandatory_encryption: Vec<String>,

    /// All operations must be audited
    pub audit_required: bool,

    /// Additional metadata for data access rules
    pub metadata: HashMap<String, String>,
}

// ============================================================================
// BEHAVIORAL CONSTRAINTS
// ============================================================================

/// Defines behavioral requirements for key usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BehavioralConstraint {
    /// Biometric verification required?
    pub biometric_required: bool,

    /// Expected usage patterns (detects hijacking)
    pub expected_patterns: Vec<UsagePattern>,

    /// Challenge-response on anomaly
    pub challenge_on_anomaly: bool,

    /// Require specific network conditions
    pub network_constraints: Option<NetworkConstraint>,
}

/// Expected usage pattern for anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsagePattern {
    /// Time pattern (e.g., "business hours")
    pub time_pattern: Option<String>,

    /// Location pattern (e.g., "home or office")
    pub location_pattern: Option<String>,

    /// Operation frequency (ops per hour)
    pub frequency_threshold: Option<u32>,
}

/// Network-based constraints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConstraint {
    /// Allowed SSIDs
    pub allowed_ssids: Vec<String>,

    /// Required VPN
    pub require_vpn: Option<String>,

    /// Geo-fence (latitude, longitude, radius in meters)
    pub geo_fence: Option<(f64, f64, f64)>,
}

// ============================================================================
// COMPUTE QUOTA
// ============================================================================

/// Compute resource quotas
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputeQuota {
    /// Maximum compute hours
    pub max_hours: f64,

    /// Maximum memory (bytes)
    pub max_memory_bytes: u64,

    /// Maximum CPU percent
    pub max_cpu_percent: u8,

    /// Current usage
    pub current_usage: ComputeUsage,
}

/// Current compute usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComputeUsage {
    /// Hours used
    pub hours_used: f64,

    /// Memory used (bytes)
    pub memory_used: u64,

    /// Last updated
    pub last_updated: Option<DateTime<Utc>>,
}

// ============================================================================
// KEY OPERATIONS (for verification)
// ============================================================================

/// Represents an operation to be verified against constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Read operation
    Read {
        /// Path being read
        path: String,
        /// Project context
        project: Option<String>,
    },

    /// Write operation
    Write {
        /// Path being written
        path: String,
        /// Data size
        size_bytes: u64,
        /// Project context
        project: Option<String>,
    },

    /// Delete operation
    Delete {
        /// Path being deleted
        path: String,
    },

    /// RPC call
    RpcCall {
        /// Target service
        target_service: String,
        /// Method name
        method: String,
        /// Project context
        project: Option<String>,
    },

    /// Compute operation
    ComputeAllocation {
        /// Requested compute hours
        hours: f64,
        /// Requested memory
        memory_bytes: u64,
    },
}

// ============================================================================
// CONSTRAINT SIGNATURE (for cryptographic binding)
// ============================================================================

/// Cryptographic signature of constraints
///
/// This ensures constraints cannot be modified after key generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSignature {
    /// The constraints being signed
    pub constraints_hash: [u8; 32],

    /// Ed25519 signature
    pub signature: Vec<u8>,

    /// When the signature was created
    pub signed_at: DateTime<Utc>,

    /// Public key that can verify this signature
    pub public_key: Vec<u8>,
}
