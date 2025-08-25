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


/// # Platform Configuration - Canonical
///
/// **UNIFIED PLATFORM CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};

/// **CANONICAL** Platform Target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformTarget {
    Android,
    Ios,
    Linux,
    Windows,
    MacOs,
    Web,
}

impl Default for PlatformTarget {
    fn default() -> Self {
        Self::Linux
    }
}

/// **CANONICAL** Unified Platform Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPlatformConfig {
    pub target: PlatformTarget,
    pub optimization_level: String,
    pub features: Vec<String>,
    pub build_settings: BuildSettings,
}

impl Default for UnifiedPlatformConfig {
    fn default() -> Self {
        Self {
            target: PlatformTarget::default(),
            optimization_level: "release".to_string(),
            features: vec!["default".to_string()],
            build_settings: BuildSettings::default(),
        }
    }
}

/// **CANONICAL** Build Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSettings {
    pub debug_symbols: bool,
    pub link_time_optimization: bool,
    pub code_generation_units: u32,
    pub target_cpu: String,
}

impl Default for BuildSettings {
    fn default() -> Self {
        Self {
            debug_symbols: false,
            link_time_optimization: true,
            code_generation_units: 1,
            target_cpu: "native".to_string(),
        }
    }
}

