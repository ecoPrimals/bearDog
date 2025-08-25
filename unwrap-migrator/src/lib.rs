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


//! BearDog Unwrap Migrator Library
//!
//! This library provides tools for systematically migrating unwrap/expect/panic
//! patterns to use BearDog's graceful error handling with BearDogError/BearDogResult.

pub mod systematic_migrator;
pub mod enhanced_migrator;
pub mod refined_migrator;

// Re-export the main components
pub use systematic_migrator::{SystematicUnwrapMigrator, MigratorResult};
pub use enhanced_migrator::{EnhancedUnwrapMigrator, EnhancedMigratorResult};
pub use refined_migrator::{RefinedBearDogMigrator, RefinedResult};

/// Common error handling patterns for migration
pub mod patterns {
    /// Standard BearDog error mapping pattern
    pub const BEARDOG_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Internal { 
        message: format!("Operation failed: {:?}", e) 
    })?"#;
    
    /// Configuration error pattern
    pub const CONFIG_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Configuration { 
        message: format!("Configuration error: {}", e) 
    })?"#;
    
    /// Network error pattern
    pub const NETWORK_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Network { 
        message: format!("Network operation failed: {}", e) 
    })?"#;
    
    /// Validation error pattern
    pub const VALIDATION_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Validation { 
        message: format!("Validation failed: {}", e) 
    })?"#;
}

/// Utility functions for migration analysis
pub mod utils {
    use std::path::Path;
    
    /// Check if a file is a test file
    pub fn is_test_file(path: &Path) -> bool {
        path.to_string_lossy().contains("test") ||
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("test_") || n.ends_with("_test.rs"))
            .unwrap_or(false)
    }
    
    /// Check if a file is an example file
    pub fn is_example_file(path: &Path) -> bool {
        path.to_string_lossy().contains("example") ||
        path.ancestors().any(|p| p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "examples")
            .unwrap_or(false))
    }
    
    /// Check if a file is a benchmark file
    pub fn is_benchmark_file(path: &Path) -> bool {
        path.to_string_lossy().contains("bench") ||
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("bench_") || n.contains("benchmark"))
            .unwrap_or(false)
    }
}
