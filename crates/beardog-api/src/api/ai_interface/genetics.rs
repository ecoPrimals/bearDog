

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISpawnNodeRequest {
    pub node_config: String,
    pub genetics_config: String,
}

pub struct AISpawnNodeResponse {
    pub node_id: String,
    pub success: bool,

pub struct AISpawnStatusResponse {
    pub status: String,
    pub progress: f64,
    pub estimated_completion: String,

pub struct AISpawnRequest {
    pub node_count: u32,
    pub spawn_strategy: String,
    pub parent_nodes: Vec<String>,
    pub evolution_parameters: String,

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
