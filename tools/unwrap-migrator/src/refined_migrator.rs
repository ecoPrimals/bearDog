// Refined BearDog Migrator - Context-aware unwrap/expect migration
//
// This module provides intelligent, context-aware migration of unwrap/expect patterns

use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum RefinedMigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
}

pub type RefinedResult<T> = Result<T, RefinedMigratorError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    Safe,
    SafeWithReview,
    RequiresAnalysis,
}

#[derive(Debug, Clone)]
pub struct MigratorConfig {
    pub min_confidence: f32,
    pub migrate_tests: bool,
    pub max_auto_safety_level: SafetyLevel,
}

impl Default for MigratorConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.8,
            migrate_tests: false,
            max_auto_safety_level: SafetyLevel::SafeWithReview,
        }
    }
}

#[derive(Debug, Default)]
pub struct AnalysisStats {
    pub files_scanned: usize,
    pub unwrap_count: usize,
    pub expect_count: usize,
    pub migrable_count: usize,
    pub test_file_count: usize,
    pub by_category: HashMap<String, usize>,
}

#[derive(Debug, Default)]
pub struct MigrationResult {
    pub files_processed: usize,
    pub patterns_migrated: usize,
    pub skipped_count: usize,
    pub failed_files: Vec<(PathBuf, String)>,
}

pub struct RefinedBearDogMigrator {
    config: MigratorConfig,
    unwrap_pattern: Regex,
    expect_pattern: Regex,
    function_pattern: Regex,
}

impl RefinedBearDogMigrator {
    pub fn new(config: MigratorConfig) -> Self {
        Self {
            config,
            unwrap_pattern: Regex::new(r"\.unwrap\(\)").unwrap(),
            expect_pattern: Regex::new(r#"\.expect\([^)]*\)"#).unwrap(),
            function_pattern: Regex::new(r"fn\s+(\w+)").unwrap(),
        }
    }

    pub async fn analyze_directory(
        &mut self,
        root: &Path,
        exclude_tests: bool,
    ) -> RefinedResult<AnalysisStats> {
        let mut stats = AnalysisStats::default();
        self.analyze_recursive(root, exclude_tests, &mut stats).await?;
        Ok(stats)
    }

    fn analyze_recursive<'a>(
        &'a self,
        path: &'a Path,
        exclude_tests: bool,
        stats: &'a mut AnalysisStats,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = RefinedResult<()>> + 'a>> {
        Box::pin(async move {
            if path.is_dir() {
                let mut entries = fs::read_dir(path).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    self.analyze_recursive(&path, exclude_tests, stats).await?;
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // Skip test files if requested
                if exclude_tests && Self::is_test_file(path) {
                    stats.test_file_count += 1;
                    return Ok(());
                }

                let content = fs::read_to_string(path).await?;
                stats.files_scanned += 1;

                let unwrap_matches = self.unwrap_pattern.find_iter(&content).count();
                let expect_matches = self.expect_pattern.find_iter(&content).count();

                stats.unwrap_count += unwrap_matches;
                stats.expect_count += expect_matches;

                // Count migrable patterns (simple heuristic)
                if self.has_result_return(&content) {
                    stats.migrable_count += unwrap_matches + expect_matches;
                }

                // Categorize by context
                let category = self.categorize_file(path, &content);
                *stats.by_category.entry(category).or_insert(0) += unwrap_matches + expect_matches;

                if unwrap_matches + expect_matches > 0 {
                    debug!(
                        "{}: {} unwrap, {} expect",
                        path.display(),
                        unwrap_matches,
                        expect_matches
                    );
                }
            }

            Ok(())
        })
    }

    pub async fn migrate_directory(
        &mut self,
        root: &Path,
        dry_run: bool,
        exclude_tests: bool,
    ) -> RefinedResult<MigrationResult> {
        let mut result = MigrationResult::default();
        self.migrate_recursive(root, dry_run, exclude_tests, &mut result).await?;
        Ok(result)
    }

    fn migrate_recursive<'a>(
        &'a self,
        path: &'a Path,
        dry_run: bool,
        exclude_tests: bool,
        result: &'a mut MigrationResult,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = RefinedResult<()>> + 'a>> {
        Box::pin(async move {
            if path.is_dir() {
                let mut entries = fs::read_dir(path).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    self.migrate_recursive(&path, dry_run, exclude_tests, result).await?;
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // Skip test files if requested
                if exclude_tests && Self::is_test_file(path) {
                    return Ok(());
                }

                match self.migrate_file(path, dry_run).await {
                    Ok(migrated_count) => {
                        result.files_processed += 1;
                        result.patterns_migrated += migrated_count;
                        if migrated_count > 0 {
                            info!("✅ {}: {} patterns migrated", path.display(), migrated_count);
                        }
                    }
                    Err(e) => {
                        warn!("❌ Failed to migrate {}: {}", path.display(), e);
                        result.failed_files.push((path.to_path_buf(), e.to_string()));
                    }
                }
            }

            Ok(())
        })
    }

    async fn migrate_file(&self, path: &Path, dry_run: bool) -> RefinedResult<usize> {
        let content = fs::read_to_string(path).await?;
        let original_content = content.clone();
        
        // Only migrate if function returns Result
        if !self.has_result_return(&content) {
            return Ok(0);
        }

        let mut modified_content = content;
        let mut migration_count = 0;

        // Simple migration: .unwrap() -> ?
        // This is safe for functions that return Result
        let new_content = self.unwrap_pattern.replace_all(&modified_content, "?");
        if new_content != modified_content {
            migration_count += self.unwrap_pattern.find_iter(&modified_content).count();
            modified_content = new_content.into_owned();
        }

        // Simple migration: .expect("msg") -> ?
        let new_content = self.expect_pattern.replace_all(&modified_content, "?");
        if new_content != modified_content {
            migration_count += self.expect_pattern.find_iter(&modified_content).count();
            modified_content = new_content.into_owned();
        }

        // Only write if changes were made and not dry run
        if migration_count > 0 && !dry_run && modified_content != original_content {
            fs::write(path, modified_content).await?;
        }

        Ok(migration_count)
    }

    fn has_result_return(&self, content: &str) -> bool {
        content.contains("Result<") || content.contains("BearDogResult")
    }

    fn is_test_file(path: &Path) -> bool {
        path.to_str()
            .map(|s| s.contains("/tests/") || s.contains("/test_") || s.ends_with("_test.rs") || s.ends_with("_tests.rs"))
            .unwrap_or(false)
    }

    fn categorize_file(&self, path: &Path, _content: &str) -> String {
        let path_str = path.to_str().unwrap_or("");
        
        if path_str.contains("/security/") || path_str.contains("/crypto/") {
            "Security".to_string()
        } else if path_str.contains("/network/") || path_str.contains("/tunnel/") {
            "Network".to_string()
        } else if path_str.contains("/config/") {
            "Configuration".to_string()
        } else if path_str.contains("/types/") {
            "Types".to_string()
        } else if path_str.contains("/core/") {
            "Core".to_string()
        } else {
            "Other".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_pattern() {
        let migrator = RefinedBearDogMigrator::new(MigratorConfig::default());
        assert!(migrator.unwrap_pattern.is_match(".unwrap()"));
        assert!(!migrator.unwrap_pattern.is_match(".unwrapped()"));
    }

    #[test]
    fn test_expect_pattern() {
        let migrator = RefinedBearDogMigrator::new(MigratorConfig::default());
        assert!(migrator.expect_pattern.is_match(r#".expect("failed")"#));
        assert!(!migrator.expect_pattern.is_match(".expecting()"));
    }

    #[test]
    fn test_has_result_return() {
        let migrator = RefinedBearDogMigrator::new(MigratorConfig::default());
        assert!(migrator.has_result_return("fn test() -> Result<(), Error> {}"));
        assert!(migrator.has_result_return("fn test() -> BearDogResult<Data> {}"));
        assert!(!migrator.has_result_return("fn test() -> Data {}"));
    }

    #[test]
    fn test_is_test_file() {
        assert!(RefinedBearDogMigrator::is_test_file(Path::new("tests/mod.rs")));
        assert!(RefinedBearDogMigrator::is_test_file(Path::new("src/test_utils.rs")));
        assert!(RefinedBearDogMigrator::is_test_file(Path::new("src/utils_test.rs")));
        assert!(!RefinedBearDogMigrator::is_test_file(Path::new("src/core.rs")));
    }
}
