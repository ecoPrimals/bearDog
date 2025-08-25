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


/// Performance recommendations and optimization suggestions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Performance optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Unique recommendation identifier
    pub recommendation_id: String,
    /// Type of recommendation
    pub recommendation_type: String,
    /// Priority level of this recommendation
    pub priority: RecommendationPriority,
    /// Description of the issue
    pub description: String,
    /// Suggested action to take
    pub suggested_action: String,
    /// Expected impact of implementing the recommendation
    pub expected_impact: String,
    /// Estimated effort required to implement
    pub effort_estimate: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
/// Priority levels for performance recommendations
pub enum RecommendationPriority {
    /// Low priority, implement when convenient
    Low,
    /// Medium priority, implement in next maintenance window
    Medium,
    /// High priority, implement soon
    High,
    /// Critical priority, implement immediately
    Critical,
