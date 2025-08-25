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


/// Anomaly detection for performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Performance anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnomaly {
    /// Unique anomaly identifier
    pub anomaly_id: String,
    /// Component where anomaly was detected
    pub component: String,
    /// Type of metric that showed anomaly
    pub metric_type: String,
    /// Current value that triggered the anomaly
    pub current_value: f64,
    /// Expected value based on historical data
    pub expected_value: f64,
    /// Severity of the anomaly
    pub severity: AnomalySeverity,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Additional context about the anomaly
    pub context: HashMap<String, String>,
}
/// Severity levels for performance anomalies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    /// Minor deviation from normal
    Low,
    /// Moderate deviation requiring attention
    Medium,
    /// Significant deviation requiring immediate action
    High,
    /// Critical anomaly requiring emergency response
    Critical,
