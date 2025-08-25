// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// HSM operations for AI interface

use serde::{Deserialize, Serialize};
/// AI HSM Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHsmStatusResponse {
    pub status: String,
    pub available_tiers: Vec<String>,
    pub active_tier: String,
}
/// AI HSM Tiers Response
pub struct AIHsmTiersResponse {
    pub tiers: Vec<String>,
/// AI Select HSM Tier Request
pub struct AISelectHsmTierRequest {
    pub tier: String,
/// AI Select HSM Tier Response
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
