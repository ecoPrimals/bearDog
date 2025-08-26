

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

    pub key_generation: bool,

    pub signing: bool,

    pub encryption: bool,

    pub key_derivation: bool,

    pub hardware_backed: bool,

    pub attestation: bool,
}
impl Default for CoreCapabilities {}

    fn default() -> Self {
        Self {
            key_generation: true,
            signing: true,
            encryption: true,
            key_derivation: false,
            hardware_backed: false,
            attestation: false,
        }
    }

#[derive(Debug, Clone)]
pub struct FoundationHealth {

    pub is_healthy: bool,

    pub provider_count: usize,

    pub last_check: chrono::DateTime<chrono::Utc>,

    pub issues: Vec<String>,}

impl FoundationHealth {}

    #[must_use] pub fn healthy() -> Self {
            is_healthy: true,
            provider_count: 0,
            last_check: chrono::Utc::now(),
            issues: Vec::new(),
    #[must_use] pub fn unhealthy(reason: &str) -> Self {
            is_healthy: false,
            issues: vec![reason],
