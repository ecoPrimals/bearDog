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


use crate::config::{
    database::UnifiedDatabaseConfig as DatabaseConfig, 
    monitoring::BasicMonitoringConfig as MonitoringConfig,
    performance::GeneralPerformanceConfig as PerformanceConfig,
    security_unified::UnifiedSecurityConfig as SecurityConfig,
};
/// # Application Configuration
///
/// This module provides the main application configuration structure that
/// consolidates all domain-specific configurations into a unified structure.
use serde::{Deserialize, Serialize};

/// **CANONICAL** Application Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub monitoring: MonitoringConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
}

