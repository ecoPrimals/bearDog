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


/// Genetics and spawning operations for AI interface

use serde::{Deserialize, Serialize};
/// AI Spawn Node Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISpawnNodeRequest {
    pub node_config: String,
    pub genetics_config: String,
}
/// AI Spawn Node Response
pub struct AISpawnNodeResponse {
    pub node_id: String,
    pub success: bool,
/// AI Spawn Status Response
pub struct AISpawnStatusResponse {
    pub status: String,
    pub progress: f64,
    pub estimated_completion: String,
/// AI-optimized genetic spawning request
pub struct AISpawnRequest {
    pub node_count: u32,
    pub spawn_strategy: String,
    pub parent_nodes: Vec<String>,
    pub evolution_parameters: String,
/// AI-optimized spawn response
pub struct AISpawnResponse {
    pub spawn_id: String,
    pub node_ids: Vec<String>,
    pub estimated_time: u64,
    pub metadata: String,}


impl Default for AISpawnNodeRequest {}


    fn default() -> Self {
        Self {
            node_config: "{}".to_string(),
            genetics_config: "{}".to_string(),
        }
    }
impl Default for AISpawnNodeResponse {
            node_id: uuid::Uuid::new_v4().to_string(),
            success: false,}


impl Default for AISpawnStatusResponse {
            node_id: "unknown".to_string(),
            status: "unknown".to_string(),
            progress: 0.0,
            estimated_completion: chrono::Utc::now().to_rfc3339(),
