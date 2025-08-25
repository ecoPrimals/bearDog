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


/// Health Monitoring
///
/// Health monitoring for discovered services

use super::super::traits::*;
/// Ecosystem service health information
#[derive(Debug, Clone)]
pub struct EcosystemServiceHealth {
    /// Unique identifier for the service
    pub service_id: String,
    /// Current health status of the service
    pub health_status: HealthStatus,
    /// When the last health check was performed
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Availability percentage (0-100)
    pub availability_percentage: f64,
    /// Error rate percentage (0-100)
    pub error_rate_percentage: f64,
} 
