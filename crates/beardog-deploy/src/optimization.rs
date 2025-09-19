// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildFeatures {
    /// Enable parallel compilation
    /// Whether parallel_builds is enabled
    pub parallel_builds: bool,
    /// Enable incremental compilation
    /// Whether incremental_builds is enabled
    pub incremental_builds: bool,
    /// Optimization settings
    /// The optimization value
    pub optimization: OptimizationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    /// Enable Link Time Optimization
    /// Whether lto is enabled
    pub lto: bool,
    /// Strip debug symbols from binaries
    /// Whether strip_symbols is enabled
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
    #[must_use]
    /// Gets rustc_flags
    /// Gets rustc_flags
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
    #[must_use]
    /// Gets cargo_flags
    /// Gets cargo_flags
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
