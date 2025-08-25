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


//! BearDog Systematic Unwrap & Panic Migrator - Production-Grade Migration Tool
//!
//! This module provides automated migration of unwrap/expect/panic calls to use
//! BearDog's graceful error handling patterns with BearDogError/BearDogResult.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use regex::Regex;
use thiserror::Error;
use tracing::{info, error};

/// Migration tool error type
#[derive(Error, Debug)]
pub enum MigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
}

// Use centralized migration result type from beardog-types
pub use beardog_types::aliases::MigrationResult as MigratorResult;

/// Systematic migrator for unwrap/expect/panic patterns optimized for BearDog
pub struct SystematicUnwrapMigrator {
    /// Unified error patterns for migration
    error_patterns: HashMap<String, MigrationPattern>,
    /// Files processed counter
    files_processed: std::sync::atomic::AtomicU64,
    /// Migrations applied counter  
    migrations_applied: std::sync::atomic::AtomicU64,
    /// BearDog-specific optimization
    beardog_errors_only: bool,
}

#[derive(Debug, Clone)]
pub struct MigrationPattern {
    /// Pattern to match
    pub pattern: String,
    /// Replacement template
    pub replacement: String,
    /// Error category for unified error system
    pub error_category: BearDogErrorCategory,
    /// Context description
    pub context: String,
    /// Whether this pattern is BearDogError compatible
    pub beardog_compatible: bool,
}

#[derive(Debug, Clone)]
pub enum BearDogErrorCategory {
    Configuration,
    Network,
    Storage,
    Authentication,
    Validation,
    Security,
    Hardware,
    Protocol,
    System,
    Plugin,
}

#[derive(Debug, Clone)]
pub struct UnwrapCall {
    pub pattern: String,
    pub position: usize,
    pub context: String,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct CodebaseStats {
    pub files_scanned: usize,
    pub total_unwrap_calls: usize,
    pub migrable_patterns: usize,
    pub test_file_patterns: usize,
    pub beardog_error_compatible: usize,
    pub pattern_categories: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub files_processed: usize,
    pub migrations_applied: usize,
    pub failed_files: Vec<(PathBuf, String)>,
    pub execution_time_ms: u64,
}

impl SystematicUnwrapMigrator {
    /// Create new systematic migrator with BearDog-optimized patterns
    pub fn new_beardog_optimized(beardog_errors_only: bool) -> Self {
        let mut error_patterns = HashMap::new();
        
        // ===============================================================
        // BEARDOG-SPECIFIC CONFIGURATION PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "env_var_beardog".to_string(),
            MigrationPattern {
                pattern: r#"std::env::var\("([^"]+)"\)\.unwrap\(\)"#.to_string(),
                replacement: r#"std::env::var("$1").map_err(|e| {
    tracing::error!("Environment variable '{}' not found: {}", "$1", e);
    beardog_errors::BearDogError::ConfigurationError(format!("Missing environment variable: {}", "$1"))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Configuration,
                context: "Environment variable access".to_string(),
                beardog_compatible: true,
            }
        );
        
        error_patterns.insert(
            "env_var_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"std::env::var\("([^"]+)"\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#"std::env::var("$1").map_err(|e| {
    tracing::error!("Environment variable '{}' not found ({}): {}", "$1", "$2", e);
    beardog_errors::BearDogError::ConfigurationError(format!("Missing environment variable '{}': {}", "$1", "$2"))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Configuration,
                context: "Environment variable access with expect message".to_string(),
                beardog_compatible: true,
            }
        );
        
        // ===============================================================
        // BEARDOG LOCK PATTERNS WITH RECOVERY
        // ===============================================================
        
        error_patterns.insert(
            "lock_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.lock\(\)\.unwrap\(\)"#.to_string(),
                replacement: ".lock().unwrap_or_else(|poisoned| {\n        tracing::warn!(\"Mutex poisoned, recovering\");\n        poisoned.into_inner()\n    })".to_string(),
                error_category: BearDogErrorCategory::System,
                context: "Mutex lock acquisition".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "lock_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.lock\(\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".lock().unwrap_or_else(|poisoned| {
        tracing::warn!("Mutex poisoned ({}), recovering", "$1");
        poisoned.into_inner()
    })"#.to_string(),
                error_category: BearDogErrorCategory::System,
                context: "Mutex lock acquisition with expect message".to_string(),
                beardog_compatible: true,
            }
        );
        
        // ===============================================================
        // BEARDOG JSON & SERIALIZATION PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "json_parse_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"serde_json::from_str\(([^)]+)\)\.unwrap\(\)"#.to_string(),
                replacement: r#"serde_json::from_str($1).map_err(|e| {
    tracing::error!("JSON parsing failed: {}", e);
    beardog_errors::BearDogError::ValidationError(format!("JSON parsing error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "JSON deserialization".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "json_parse_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"serde_json::from_str\(([^)]+)\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#"serde_json::from_str($1).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "$2", e);
    beardog_errors::BearDogError::ValidationError(format!("JSON parsing error ({}): {}", "$2", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "JSON deserialization with expect message".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "json_to_string_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"serde_json::to_string\(([^)]+)\)\.unwrap\(\)"#.to_string(),
                replacement: r#"serde_json::to_string($1).map_err(|e| {
    tracing::error!("JSON serialization failed: {}", e);
    beardog_errors::BearDogError::ValidationError(format!("JSON serialization error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "JSON serialization".to_string(),
                beardog_compatible: true,
            }
        );
        
        // ===============================================================
        // BEARDOG HTTP & NETWORK PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "http_send_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.send\(\)\.await\.unwrap\(\)"#.to_string(),
                replacement: r#".send().await.map_err(|e| {
    tracing::error!("HTTP request failed: {}", e);
    beardog_errors::BearDogError::NetworkError(format!("HTTP error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Network,
                context: "HTTP request execution".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "http_send_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.send\(\)\.await\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".send().await.map_err(|e| {
    tracing::error!("HTTP request failed ({}): {}", "$1", e);
    beardog_errors::BearDogError::NetworkError(format!("HTTP error ({}): {}", "$1", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Network,
                context: "HTTP request execution with expect message".to_string(),
                beardog_compatible: true,
            }
        );
        
        // ===============================================================
        // BEARDOG FILE I/O PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "file_read_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"fs::read_to_string\(([^)]+)\)\.unwrap\(\)"#.to_string(),
                replacement: r#"fs::read_to_string($1).map_err(|e| {
    tracing::error!("File read failed: {}", e);
    beardog_errors::BearDogError::StorageError(format!("File read error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Storage,
                context: "File read operation".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "file_write_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"fs::write\(([^,]+),\s*([^)]+)\)\.unwrap\(\)"#.to_string(),
                replacement: r#"fs::write($1, $2).map_err(|e| {
    tracing::error!("File write failed: {}", e);
    beardog_errors::BearDogError::StorageError(format!("File write error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Storage,
                context: "File write operation".to_string(),
                beardog_compatible: true,
            }
        );
        
        // ===============================================================
        // BEARDOG HSM & SECURITY PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "hsm_operation_unwrap".to_string(),
            MigrationPattern {
                pattern: r#"\.hsm_([a-zA-Z_]+)\([^)]*\)\.unwrap\(\)"#.to_string(),
                replacement: r#".hsm_$1().map_err(|e| {
    tracing::error!("HSM operation failed: {}", e);
    beardog_errors::BearDogError::SecurityError(format!("HSM error: {}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Security,
                context: "HSM operation".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // BEARDOG LOCK PATTERNS - COMPREHENSIVE
        // ===============================================================
        
        error_patterns.insert(
            "read_lock_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.read\(\)\.unwrap\(\)"#.to_string(),
                replacement: r#".read().unwrap_or_else(|poisoned| {
        tracing::warn!("RwLock poisoned for read, recovering");
        poisoned.into_inner()
    })"#.to_string(),
                error_category: BearDogErrorCategory::System,
                context: "RwLock read operation".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "write_lock_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.write\(\)\.unwrap\(\)"#.to_string(),
                replacement: r#".write().unwrap_or_else(|poisoned| {
        tracing::warn!("RwLock poisoned for write, recovering");
        poisoned.into_inner()
    })"#.to_string(),
                error_category: BearDogErrorCategory::System,
                context: "RwLock write operation".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // BEARDOG ENVIRONMENT VARIABLE PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "env_var_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"env::var\("([^"]+)"\)\.unwrap\(\)"#.to_string(),
                replacement: r#"env::var("$1").map_err(|e| {
    tracing::error!("Environment variable '{}' not found: {}", "$1", e);
    beardog_errors::BearDogError::config(format!("Missing environment variable: {}", "$1"))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Configuration,
                context: "Environment variable access".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // BEARDOG PARSING PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "parse_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.parse\(\)\.unwrap\(\)"#.to_string(),
                replacement: r#".parse().map_err(|e| {
    tracing::error!("Parsing failed: {:?}", e);
    beardog_errors::BearDogError::validation(format!("Parse error: {:?}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "String parsing operation".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "parse_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.parse\(\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".parse().map_err(|e| {
    tracing::error!("Parsing failed ({}): {:?}", "$1", e);
    beardog_errors::BearDogError::validation(format!("Parse error ({}): {:?}", "$1", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "String parsing operation with expect message".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // BEARDOG COLLECTION ACCESS PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "first_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.first\(\)\.unwrap\(\)"#.to_string(),
                replacement: r#".first().ok_or_else(|| {
    tracing::error!("Collection is empty when accessing first element");
    beardog_errors::BearDogError::validation("Collection is empty")
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "Collection first element access".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "last_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.last\(\)\.unwrap\(\)"#.to_string(),
                replacement: r#".last().ok_or_else(|| {
    tracing::error!("Collection is empty when accessing last element");
    beardog_errors::BearDogError::validation("Collection is empty")
})?"#.to_string(),
                error_category: BearDogErrorCategory::Validation,
                context: "Collection last element access".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // BEARDOG GENERAL UNWRAP PATTERNS
        // ===============================================================
        
        error_patterns.insert(
            "general_unwrap_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.unwrap\(\)"#.to_string(),
                replacement: r#".map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::System,
                context: "General operation".to_string(),
                beardog_compatible: true,
            }
        );

        error_patterns.insert(
            "general_expect_beardog".to_string(),
            MigrationPattern {
                pattern: r#"\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "$1", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "$1", e))
})?"#.to_string(),
                error_category: BearDogErrorCategory::System,
                context: "General operation with expect message".to_string(),
                beardog_compatible: true,
            }
        );

        // ===============================================================
        // EXAMPLE CODE PATTERNS - Use expect with clear messages
        // ===============================================================
        
        error_patterns.insert("example_runtime_unwrap".to_string(), MigrationPattern {
            pattern: r"Runtime::new\(\)\.unwrap\(\)".to_string(),
            replacement: r#"Runtime::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create async runtime for example", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create async runtime for example", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Example async runtime creation".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("example_json_unwrap".to_string(), MigrationPattern {
            pattern: r"serde_json::to_string\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#"serde_json::to_string($1).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "JSON serialization failed in example", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "JSON serialization failed in example", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Example JSON serialization".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("example_json_from_str_unwrap".to_string(), MigrationPattern {
            pattern: r"serde_json::from_str\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#"serde_json::from_str($1).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "JSON deserialization failed in example", e);
    beardog_errors::BearDogError::ValidationError(format!("JSON parsing error ({}): {}", "JSON deserialization failed in example", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Example JSON deserialization".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("example_sort_unwrap".to_string(), MigrationPattern {
            pattern: r"\.sort_by\([^)]*\.partial_cmp\([^)]+\)\.unwrap\(\)[^)]*\)".to_string(),
            replacement: r".sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))".to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Example sorting with float comparison".to_string(),
            beardog_compatible: false,
        });
        
        // ===============================================================
        // BENCHMARK PATTERNS - Performance-focused expect messages
        // ===============================================================
        
        error_patterns.insert("benchmark_runtime_unwrap".to_string(), MigrationPattern {
            pattern: r"Runtime::new\(\)\.unwrap\(\)".to_string(),
            replacement: r#"Runtime::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Benchmark async runtime creation".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("benchmark_buffer_unwrap".to_string(), MigrationPattern {
            pattern: r"ZeroCopyBuffer::from_vec\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#"ZeroCopyBuffer::from_vec($1).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark buffer creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark buffer creation failed", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Benchmark buffer creation".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("benchmark_key_generation_unwrap".to_string(), MigrationPattern {
            pattern: r"\.generate_key\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: r#".generate_key($1).await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark key generation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark key generation failed", e))
})?"#.to_string(),
            error_category: BearDogErrorCategory::Security,
            context: "Benchmark key generation".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("benchmark_result_unwrap".to_string(), MigrationPattern {
            pattern: r"results\.into_iter\(\)\.map\([^)]*\.unwrap\(\)[^)]*\)\.sum".to_string(),
            replacement: r"results.into_iter().map(|r| r.unwrap_or_default()).sum".to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Benchmark result aggregation".to_string(),
            beardog_compatible: false,
        });
        
        // ===============================================================
        // PRODUCTION PANIC PATTERNS - Convert to proper errors
        // ===============================================================
        
        error_patterns.insert("production_panic_auth".to_string(), MigrationPattern {
            pattern: r#"panic!\("Expected Authentication event"\)"#.to_string(),
            replacement: r#"return Err(BearDogError::internal("Expected Authentication event"))"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Authentication event validation".to_string(),
            beardog_compatible: true,
        });
        
        error_patterns.insert("production_panic_serialization".to_string(), MigrationPattern {
            pattern: r#"panic!\("Serialization failed: \{e:\?\}"\)"#.to_string(),
            replacement: r#"return Err(BearDogError::serialization(format!("Serialization failed: {e:?}")))"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Serialization error handling".to_string(),
            beardog_compatible: true,
        });
        
        // ===============================================================
        // TEST PANIC PATTERNS - Convert to proper test failures
        // ===============================================================
        
        error_patterns.insert("test_panic_setup".to_string(), MigrationPattern {
            pattern: r#"panic!\("Test failed to (\w+): \{e:\?\}"\)"#.to_string(),
            replacement: r#"panic!("Test setup failed during {}: {:?}", "$1", e)"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Test setup failure".to_string(),
            beardog_compatible: false,
        });
        
        error_patterns.insert("test_panic_assertion".to_string(), MigrationPattern {
            pattern: r#"panic!\("Expected (\w+) error, got \{error:\?\}"\)"#.to_string(),
            replacement: r#"panic!("Expected {} error, got {:?}", "$1", error)"#.to_string(),
            error_category: BearDogErrorCategory::System,
            context: "Test assertion failure".to_string(),
            beardog_compatible: false,
        });

        Self {
            error_patterns,
            files_processed: std::sync::atomic::AtomicU64::new(0),
            migrations_applied: std::sync::atomic::AtomicU64::new(0),
            beardog_errors_only,
        }
    }

    /// Analyze codebase for unwrap/expect patterns with enhanced categorization
    pub async fn analyze_codebase(&self, root_path: &Path, exclude_tests: bool) -> MigratorResult<CodebaseStats> {
        let mut stats = CodebaseStats {
            files_scanned: 0,
            total_unwrap_calls: 0,
            migrable_patterns: 0,
            test_file_patterns: 0,
            beardog_error_compatible: 0,
            pattern_categories: HashMap::new(),
        };

        let mut files_to_process = Vec::new();
        self.collect_rust_files(root_path, &mut files_to_process).await?;

        for file_path in &files_to_process {
            let is_test_file = self.is_test_file(file_path);
            
            if exclude_tests && is_test_file {
                continue;
            }

            stats.files_scanned += 1;
            
            let content = fs::read_to_string(file_path).await?;
            let unwrap_calls = self.find_unwrap_patterns(&content);
            
            for call in unwrap_calls {
                stats.total_unwrap_calls += 1;
                
                if is_test_file || self.is_in_test_function(&content, call.position) {
                    stats.test_file_patterns += 1;
                    continue;
                }
                
                // Only count non-test unwraps as migrable
                if self.is_migrable_pattern(&call.pattern) {
                    stats.migrable_patterns += 1;
                    
                    if self.beardog_errors_only {
                        if self.is_beardog_compatible(&call.context) {
                            stats.beardog_error_compatible += 1;
                        }
                    } else {
                        stats.beardog_error_compatible += 1;
                    }
                    
                    let category = self.categorize_pattern(&call.context);
                    *stats.pattern_categories.entry(category).or_insert(0) += 1;
                }
            }
        }

        Ok(stats)
    }

    /// Migrate the entire codebase
    pub async fn migrate_codebase(&self, root_path: &Path, dry_run: bool, exclude_tests: bool) -> Result<MigrationResult, MigratorError> {
        let start_time = std::time::Instant::now();
        let mut result = MigrationResult {
            files_processed: 0,
            migrations_applied: 0,
            failed_files: Vec::new(),
            execution_time_ms: 0,
        };

        let rust_files = self.find_rust_files(root_path, exclude_tests).await?;
        
        for file_path in rust_files {
            match self.migrate_file(&file_path, dry_run).await {
                Ok(migrations) => {
                    result.files_processed += 1;
                    result.migrations_applied += migrations;
                }
                Err(e) => {
                    result.failed_files.push((file_path, e.to_string()));
                }
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    }

    async fn find_rust_files(&self, root_path: &Path, exclude_tests: bool) -> Result<Vec<PathBuf>, MigratorError> {
        let mut rust_files = Vec::new();
        self.collect_rust_files_recursive(root_path, &mut rust_files, exclude_tests).await?;
        Ok(rust_files)
    }

    async fn collect_rust_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>, exclude_tests: bool) -> Result<(), MigratorError> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                // Skip target directories and hidden directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "target" || dir_name.starts_with('.') {
                        continue;
                    }
                }
                // Use Box::pin for recursive async call
                Box::pin(self.collect_rust_files_recursive(&path, files, exclude_tests)).await?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                // Skip test files if requested
                if exclude_tests && path.to_string_lossy().contains("test") {
                    continue;
                }
                files.push(path);
            }
        }
        
        Ok(())
    }

    async fn analyze_file_content(&self, content: &str, _file_path: &Path) -> CodebaseStats {
        let mut stats = CodebaseStats {
            files_scanned: 1,
            total_unwrap_calls: 0,
            migrable_patterns: 0,
            test_file_patterns: 0,
            beardog_error_compatible: 0,
            pattern_categories: HashMap::new(),
        };

        // Count basic unwrap/expect patterns
        let unwrap_regex = match Regex::new(r"\.unwrap\(\)") {
            Ok(regex) => regex,
            Err(_) => return stats,
        };
        let expect_regex = match Regex::new(r"\.expect\(") {
            Ok(regex) => regex,
            Err(_) => return stats,
        };
        
        stats.total_unwrap_calls += unwrap_regex.find_iter(content).count();
        stats.total_unwrap_calls += expect_regex.find_iter(content).count();

        // Analyze specific patterns
        for (_name, pattern) in &self.error_patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                let matches = regex.find_iter(content).count();
                if matches > 0 {
                    stats.migrable_patterns += matches;
                    let category = format!("{:?}", pattern.error_category);
                    *stats.pattern_categories.entry(category).or_insert(0) += matches;
                    
                    if pattern.beardog_compatible {
                        stats.beardog_error_compatible += matches;
                    }
                }
            }
        }

        stats
    }

    async fn migrate_file(&self, file_path: &Path, dry_run: bool) -> Result<usize, MigratorError> {
        let content = fs::read_to_string(file_path).await?;
        let mut modified_content = content.clone();
        let mut migrations_applied = 0;

        // Apply error patterns
        for (_name, pattern) in &self.error_patterns {
            // Skip non-BearDogError patterns if beardog_errors_only is true
            if self.beardog_errors_only && !pattern.beardog_compatible {
                continue;
            }

            if let Ok(regex) = Regex::new(&pattern.pattern) {
                let matches = regex.find_iter(&modified_content).count();
                if matches > 0 {
                    modified_content = regex.replace_all(&modified_content, pattern.replacement.as_str()).to_string();
                    migrations_applied += matches;
                    info!("Applied {} '{}' pattern {} times in {}", 
                          pattern.pattern, pattern.context, matches, file_path.display());
                }
            }
        }

        // Write the file if not dry run and changes were made
        if !dry_run && migrations_applied > 0 {
            fs::write(file_path, modified_content).await?;
        }

        Ok(migrations_applied)
    }

    /// Check if content is within a test function
    fn is_in_test_function(&self, content: &str, position: usize) -> bool {
        // Find the function containing this position
        let before_position = &content[..position];
        
        // Look for test function markers before this position
        let test_markers = [
            "#[test]",
            "#[tokio::test]", 
            "#[cfg(test)]",
            "mod tests {",
            "assert!",
            "assert_eq!",
            "assert_ne!",
        ];
        
        // Check if we're in a test context
        for marker in &test_markers {
            if before_position.rfind(marker).is_some() {
                // Check if this is closer than the last function boundary
                if let Some(last_fn) = before_position.rfind("fn ") {
                    if let Some(marker_pos) = before_position.rfind(marker) {
                        if marker_pos > last_fn {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    /// Enhanced pattern detection
    fn find_unwrap_patterns(&self, content: &str) -> Vec<UnwrapCall> {
        let mut calls = Vec::new();
        
        // More precise regex patterns
        let patterns = [
            (r"\.unwrap\(\)", "unwrap"),
            (r"\.expect\([^)]+\)", "expect"),
            (r"panic!\([^)]*\)", "panic"),
        ];
        
        for (pattern, call_type) in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for mat in regex.find_iter(content) {
                    let start = mat.start();
                    let end = mat.end();
                    
                    // Get surrounding context (50 chars before and after)
                    // Safe Unicode boundary handling
                    let context_start = content.char_indices()
                        .map(|(i, _)| i)
                        .find(|&i| i >= start.saturating_sub(50))
                        .unwrap_or(0);
                    let context_end = content.char_indices()
                        .map(|(i, _)| i)
                        .find(|&i| i >= end + 50)
                        .unwrap_or(content.len());
                    let context = &content[context_start..context_end];
                    
                    calls.push(UnwrapCall {
                        pattern: call_type.to_string(),
                        position: start,
                        context: context.to_string(),
                        line_number: content[..start].matches('\n').count() + 1,
                    });
                }
            }
        }
        
        calls
    }

    /// Enhanced test file detection
    fn is_test_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("/tests/") || 
        path_str.contains("\\tests\\") || 
        path_str.ends_with("_test.rs") ||
        path_str.ends_with("_tests.rs") ||
        path_str.contains("test_") ||
        path_str.contains("/test/") ||
        path_str.contains("\\test\\")
    }

    /// Collect all Rust files in the given directory
    fn collect_rust_files<'a>(&'a self, root_path: &'a Path, files: &'a mut Vec<PathBuf>) -> std::pin::Pin<Box<dyn std::future::Future<Output = MigratorResult<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(root_path).await?;
            
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                
                if path.is_dir() {
                    // Skip target directories and hidden directories
                    if let Some(dir_name) = path.file_name() {
                        let dir_str = dir_name.to_string_lossy();
                        if dir_str.starts_with('.') || dir_str == "target" {
                            continue;
                        }
                    }
                    
                    self.collect_rust_files(&path, files).await?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
            
            Ok(())
        })
    }

    /// Check if a pattern is migrable
    fn is_migrable_pattern(&self, pattern: &str) -> bool {
        matches!(pattern, "unwrap" | "expect")
    }

    /// Check if context is BearDog compatible
    fn is_beardog_compatible(&self, context: &str) -> bool {
        // Check for BearDogResult, BearDogError, or other BearDog patterns
        context.contains("BearDog") || 
        context.contains("beardog") ||
        !context.contains("test") // Assume non-test code is compatible
    }

    /// Categorize pattern based on context
    fn categorize_pattern(&self, context: &str) -> String {
        if context.contains("env::var") || context.contains("config") {
            "Configuration".to_string()
        } else if context.contains("network") || context.contains("http") || context.contains("reqwest") {
            "Network".to_string()
        } else if context.contains("fs::") || context.contains("file") || context.contains("storage") {
            "Storage".to_string()
        } else if context.contains("auth") || context.contains("token") {
            "Authentication".to_string()
        } else if context.contains("security") || context.contains("crypto") {
            "Security".to_string()
        } else {
            "System".to_string()
        }
    }
} 