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


/// Ecosystem integration types
///
/// Types related to ecosystem capabilities and network effect analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::genetics::NodeCapability;
/// Network operation for ecosystem analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOperation {
    /// Type of operation being performed
    pub operation_type: String,
    /// Resources involved in the operation
    pub resources: Vec<String>,
    /// Expected duration in seconds
    pub expected_duration: u64,
    /// Priority level (1-10)
    pub priority: u8,
    /// Required capabilities for the operation
    pub required_capabilities: Vec<NodeCapability>,
}
/// Analysis of network effects for an operation
pub struct NetworkEffectAnalysis {
    /// Predicted impact score (0.0 to 1.0)
    pub predicted_impact: f64,
    /// List of nodes that will be affected
    pub affected_nodes: Vec<String>,
    /// Resource requirements by type
    pub resource_requirements: HashMap<String, u64>,
    /// Expected performance impact (0.0 to 1.0)
    pub performance_impact: f64,
    /// Security implications of the operation
    pub security_implications: Vec<String>,
