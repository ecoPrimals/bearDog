

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHsmStatusResponse {
    pub status: String,
    pub available_tiers: Vec<String>,
    pub active_tier: String,
}

pub struct AIHsmTiersResponse {
    pub tiers: Vec<String>,

pub struct AISelectHsmTierRequest {
    pub tier: String,

pub struct AISelectHsmTierResponse {
    pub success: bool,}

impl Default for AIHsmStatusResponse {}

    fn default() -> Self {
        Self {
            status: "active".to_string(),
            available_tiers: vec!["software".to_string()],
            active_tier: "software".to_string(),
        }
    }
impl Default for AIHsmTiersResponse {
            tiers: vec!["software".to_string(), "hardware".to_string()],}

impl Default for AISelectHsmTierResponse {
            success: false,
