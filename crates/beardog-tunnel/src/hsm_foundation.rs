
 /// Error types and handling
 /// Error types and handling

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod error;
pub mod providers;
pub mod traits;
pub mod types;

pub use error::*;
pub use providers::*;
pub use traits::*;
pub use types::*;

pub use beardog_types::constants::api::versions::HSM_FOUNDATION_VERSION;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CoreCapabilities {

    /// Whether key_generation is enabled
    pub key_generation: bool,

    /// Whether signing is enabled
    pub signing: bool,

    /// Whether encryption is enabled
    pub encryption: bool,

    /// Whether key_derivation is enabled
    pub key_derivation: bool,

    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,

    /// Whether attestation is enabled
    pub attestation: bool,
}
impl Default for CoreCapabilities {}

    fn default(true,
            signing: true,
            encryption: true,
            key_derivation: false,
            hardware_backed: false,
            attestation: false,
        }
    }

#[derive(Debug, Clone)]
    pub provider_count: usize,

    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,

    /// Collection of issues
    pub issues: Vec<String>,}

impl FoundationHealth {}

/// Healthy operation.
    #[must_use] pub fn healthy(true,
            provider_count: 0,
            last_check: chrono::Utc::now(),
            issues: Vec::new(),
/// Unhealthy operation.
    #[must_use] pub fn unhealthy(reason: &str) -> Self {
            is_healthy: false,
            issues: vec![reason],
