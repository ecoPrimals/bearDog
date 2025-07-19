//! HSM operations for AI interface

use serde::{Deserialize, Serialize};

/// AI HSM Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHsmStatusResponse {
    pub status: String,
    pub available_tiers: Vec<String>,
    pub active_tier: String,
}

/// AI HSM Tiers Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHsmTiersResponse {
    pub tiers: Vec<String>,
}

/// AI Select HSM Tier Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISelectHsmTierRequest {
    pub tier: String,
}

/// AI Select HSM Tier Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISelectHsmTierResponse {
    pub success: bool,
    pub active_tier: String,
}

impl Default for AIHsmStatusResponse {
    fn default() -> Self {
        Self {
            status: "active".to_string(),
            available_tiers: vec!["software".to_string()],
            active_tier: "software".to_string(),
        }
    }
}

impl Default for AIHsmTiersResponse {
    fn default() -> Self {
        Self {
            tiers: vec!["software".to_string(), "hardware".to_string()],
        }
    }
}

impl Default for AISelectHsmTierResponse {
    fn default() -> Self {
        Self {
            success: false,
            active_tier: "software".to_string(),
        }
    }
}
