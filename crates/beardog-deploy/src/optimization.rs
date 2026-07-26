// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

/// Feature flags and nested optimization settings applied when building artifacts for deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildFeatures {
    /// Enable parallel compilation
    /// Whether `parallel_builds` is enabled
    pub parallel_builds: bool,
    /// Enable incremental compilation
    /// Whether `incremental_builds` is enabled
    pub incremental_builds: bool,
    /// Optimization settings
    /// The optimization value
    pub optimization: OptimizationSettings,
}

/// Fine-grained rustc-oriented options (LTO and symbol stripping) grouped under [`BuildFeatures`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    /// Enable Link Time Optimization
    /// Whether lto is enabled
    pub lto: bool,
    /// Strip debug symbols from binaries
    /// Whether `strip_symbols` is enabled
    pub strip_symbols: bool,
}

impl Default for BuildFeatures {
    fn default() -> Self {
        Self {
            parallel_builds: true,
            incremental_builds: true,
            optimization: OptimizationSettings::default(),
        }
    }
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        Self {
            lto: false,
            strip_symbols: true,
        }
    }
}

/// Deployment optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentOptimizationConfig {
    /// Build features to enable during deployment
    /// The features value
    pub features: BuildFeatures,
    /// Optional target cpu optimization
    pub target_cpu_optimization: Option<String>,
    /// The optimization level value
    pub optimization_level: OptimizationLevel,
}

/// Preset optimization tier used to derive `rustc` and `cargo` flag lines for a deployment build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// Debug build with no optimizations
    Debug,
    /// Standard release build
    Release,
    /// Release build with link-time optimization
    ReleaseLto,
    /// Release build with maximum optimizations
    ReleaseMaxOpt,
}

impl Default for DeploymentOptimizationConfig {
    fn default() -> Self {
        Self {
            features: BuildFeatures::default(),
            target_cpu_optimization: None,
            optimization_level: OptimizationLevel::Release,
        }
    }
}

impl DeploymentOptimizationConfig {
    /// Creates development optimization configuration
    ///
    /// # Returns
    #[must_use]
    pub const fn development() -> Self {
        Self {
            features: BuildFeatures {
                parallel_builds: true,
                incremental_builds: true,
                optimization: OptimizationSettings {
                    lto: false,
                    strip_symbols: false,
                },
            },
            target_cpu_optimization: None,
            optimization_level: OptimizationLevel::Debug,
        }
    }

    /// Creates production optimization configuration
    ///
    /// # Returns
    #[must_use]
    pub fn production() -> Self {
        Self {
            features: BuildFeatures {
                parallel_builds: true,
                incremental_builds: false,
                optimization: OptimizationSettings {
                    lto: true,
                    strip_symbols: true,
                },
            },
            target_cpu_optimization: Some("native".to_owned()),
            optimization_level: OptimizationLevel::ReleaseMaxOpt,
        }
    }

    /// Gets the rustc optimization flags based on configuration
    ///
    /// # Returns
    /// Gets `rustc_flags`
    #[must_use]
    pub fn get_rustc_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        match self.optimization_level {
            OptimizationLevel::Debug => flags.push("-C opt-level=0".to_string()),
            OptimizationLevel::Release | OptimizationLevel::ReleaseMaxOpt => {
                flags.push("-C opt-level=3".to_string());
            }
            OptimizationLevel::ReleaseLto => {
                flags.push("-C opt-level=3".to_string());
                flags.push("-C lto=thin".to_string());
            }
        }

        if self.features.optimization.lto {
            flags.push("-C lto=fat".to_string());
        }

        if self.features.optimization.strip_symbols {
            flags.push("-C strip=symbols".to_string());
        }

        if let Some(ref target_cpu) = self.target_cpu_optimization {
            flags.push(format!("-C target-cpu={target_cpu}"));
        }

        flags
    }

    /// Gets the cargo build flags based on configuration
    ///
    /// # Returns
    /// Gets `cargo_flags`
    #[must_use]
    pub fn get_cargo_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        if self.features.parallel_builds {
            // Cargo enables parallel builds by default
            // We could add specific job count configuration here
        }

        if self.features.incremental_builds {
            flags.push("--incremental".to_string());
        }

        match self.optimization_level {
            OptimizationLevel::Debug => {
                // Debug is default, no extra flags needed
            }
            OptimizationLevel::Release
            | OptimizationLevel::ReleaseLto
            | OptimizationLevel::ReleaseMaxOpt => {
                flags.push("--release".to_string());
            }
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === OptimizationSettings Tests ===

    #[test]
    fn test_optimization_settings_default() {
        let settings = OptimizationSettings::default();
        assert!(!settings.lto);
        assert!(settings.strip_symbols);
    }

    #[test]
    fn test_optimization_settings_clone() {
        let settings = OptimizationSettings {
            lto: true,
            strip_symbols: false,
        };
        let cloned = settings.clone();
        assert_eq!(settings.lto, cloned.lto);
    }

    #[test]
    fn test_optimization_settings_serialization() {
        let settings = OptimizationSettings {
            lto: true,
            strip_symbols: true,
        };
        let serialized = serde_json::to_string(&settings).expect("serialize");
        let deserialized: OptimizationSettings =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(settings.lto, deserialized.lto);
    }

    // === BuildFeatures Tests ===

    #[test]
    fn test_build_features_default() {
        let features = BuildFeatures::default();
        assert!(features.parallel_builds);
        assert!(features.incremental_builds);
    }

    #[test]
    fn test_build_features_clone() {
        let features = BuildFeatures::default();
        let cloned = features.clone();
        assert_eq!(features.parallel_builds, cloned.parallel_builds);
    }

    #[test]
    fn test_build_features_serialization() {
        let features = BuildFeatures::default();
        let serialized = serde_json::to_string(&features).expect("serialize");
        let deserialized: BuildFeatures = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(features.parallel_builds, deserialized.parallel_builds);
    }

    // === OptimizationLevel Tests ===

    #[test]
    fn test_optimization_level_serialization() {
        let level = OptimizationLevel::ReleaseMaxOpt;
        let serialized = serde_json::to_string(&level).expect("serialize");
        let deserialized: OptimizationLevel =
            serde_json::from_str(&serialized).expect("deserialize");
        assert!(matches!(deserialized, OptimizationLevel::ReleaseMaxOpt));
    }

    #[test]
    fn test_optimization_level_variants() {
        let debug = OptimizationLevel::Debug;
        let release = OptimizationLevel::Release;
        let lto = OptimizationLevel::ReleaseLto;
        let max = OptimizationLevel::ReleaseMaxOpt;

        assert!(matches!(debug, OptimizationLevel::Debug));
        assert!(matches!(release, OptimizationLevel::Release));
        assert!(matches!(lto, OptimizationLevel::ReleaseLto));
        assert!(matches!(max, OptimizationLevel::ReleaseMaxOpt));
    }

    // === DeploymentOptimizationConfig Tests ===

    #[test]
    fn test_config_default() {
        let config = DeploymentOptimizationConfig::default();
        assert!(config.target_cpu_optimization.is_none());
        assert!(matches!(
            config.optimization_level,
            OptimizationLevel::Release
        ));
    }

    #[test]
    fn test_config_development() {
        let config = DeploymentOptimizationConfig::development();
        assert!(matches!(
            config.optimization_level,
            OptimizationLevel::Debug
        ));
        assert!(!config.features.optimization.lto);
        assert!(!config.features.optimization.strip_symbols);
    }

    #[test]
    fn test_config_production() {
        let config = DeploymentOptimizationConfig::production();
        assert!(matches!(
            config.optimization_level,
            OptimizationLevel::ReleaseMaxOpt
        ));
        assert!(config.features.optimization.lto);
        assert!(config.features.optimization.strip_symbols);
        assert_eq!(config.target_cpu_optimization, Some("native".to_owned()));
    }

    #[test]
    fn test_config_serialization() {
        let config = DeploymentOptimizationConfig::default();
        let serialized = serde_json::to_string(&config).expect("serialize");
        let deserialized: DeploymentOptimizationConfig =
            serde_json::from_str(&serialized).expect("deserialize");
        assert!(matches!(
            deserialized.optimization_level,
            OptimizationLevel::Release
        ));
    }

    #[test]
    fn test_get_rustc_flags_debug() {
        let config = DeploymentOptimizationConfig::development();
        let flags = config.get_rustc_flags();
        assert!(flags.contains(&"-C opt-level=0".to_string()));
    }

    #[test]
    fn test_get_rustc_flags_release() {
        let config = DeploymentOptimizationConfig::default();
        let flags = config.get_rustc_flags();
        assert!(flags.contains(&"-C opt-level=3".to_string()));
    }

    #[test]
    fn test_get_rustc_flags_release_lto() {
        let config = DeploymentOptimizationConfig {
            features: BuildFeatures::default(),
            target_cpu_optimization: None,
            optimization_level: OptimizationLevel::ReleaseLto,
        };
        let flags = config.get_rustc_flags();
        assert!(flags.contains(&"-C opt-level=3".to_string()));
        assert!(flags.contains(&"-C lto=thin".to_string()));
    }

    #[test]
    fn test_get_rustc_flags_with_lto() {
        let config = DeploymentOptimizationConfig::production();
        let flags = config.get_rustc_flags();
        assert!(flags.contains(&"-C lto=fat".to_string()));
    }

    #[test]
    fn test_get_rustc_flags_with_strip() {
        let config = DeploymentOptimizationConfig::production();
        let flags = config.get_rustc_flags();
        assert!(flags.contains(&"-C strip=symbols".to_string()));
    }

    #[test]
    fn test_get_rustc_flags_with_target_cpu() {
        let config = DeploymentOptimizationConfig::production();
        let flags = config.get_rustc_flags();
        assert!(flags.iter().any(|f| f.contains("target-cpu=native")));
    }

    #[test]
    fn test_get_cargo_flags_debug() {
        let config = DeploymentOptimizationConfig::development();
        let flags = config.get_cargo_flags();
        // Debug mode doesn't add --release
        assert!(!flags.contains(&"--release".to_string()));
    }

    #[test]
    fn test_get_cargo_flags_release() {
        let config = DeploymentOptimizationConfig::default();
        let flags = config.get_cargo_flags();
        assert!(flags.contains(&"--release".to_string()));
    }

    #[test]
    fn test_get_cargo_flags_incremental() {
        let config = DeploymentOptimizationConfig::development();
        let flags = config.get_cargo_flags();
        assert!(flags.contains(&"--incremental".to_string()));
    }
}
