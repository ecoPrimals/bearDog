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


/// # AI CLI Configuration Operations
///
/// **EXTRACTED FROM LARGE FILE** - Configuration operations and handlers (~100 lines)
/// This module contains configuration management CLI operations including
/// getting, setting, and validating configuration parameters.
use super::types::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

/// Configuration operations
#[derive(Debug, Subcommand)]
pub enum ConfigOperation {
    /// Get configuration value
    Get {
        /// Configuration key
        #[arg(long)]
        key: String,
        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
        /// Show configuration path
        show_path: bool,
    },
    /// Set configuration value
    Set {
        /// Configuration value
        value: String,
        /// Configuration type
        value_type: Option<String>,
        /// Validate before setting
        validate: bool,
    /// List all configuration
    List {
        /// Filter by key prefix
        prefix: Option<String>,
        /// Show only modified values
        modified_only: bool,
        /// Include descriptions
        include_descriptions: bool,
    /// Reset configuration to defaults
    Reset {
        /// Configuration key to reset (empty for all)
        key: Option<String>,
        /// Force reset without confirmation
        force: bool,
    /// Export configuration
    Export {
        /// Output file
        output: PathBuf,
        /// Export format
        #[arg(long, default_value = "toml")]
        format: String,
        /// Include default values
        include_defaults: bool,
        /// Include sensitive values (encrypted)
        include_sensitive: bool,
    /// Import configuration
    Import {
        /// Input file
        input: PathBuf,
        /// Merge with existing configuration
        merge: bool,
        /// Validate before importing
        /// Dry run (validate only)
        dry_run: bool,
    /// Validate configuration
    Validate {
        /// Configuration file to validate
        config: Option<PathBuf>,
        /// Show warnings
        show_warnings: bool,
    /// Show configuration schema
    Schema {
        /// Schema section to show
        section: Option<String>,
        /// Include examples
        include_examples: bool,
    /// Configuration diff
    Diff {
        /// First configuration file
        config1: PathBuf,
        /// Second configuration file
        config2: PathBuf,
        /// Show only differences
        differences_only: bool,
}
