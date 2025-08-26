

use serde::{Deserialize, Serialize};

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

