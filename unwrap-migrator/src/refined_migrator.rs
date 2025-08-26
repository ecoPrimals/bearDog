

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use regex::Regex;
use thiserror::Error;
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum RefinedMigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
    #[error("Context analysis error: {message}")]
    ContextAnalysis { message: String },
}

pub use beardog_types::aliases::MigrationResult as RefinedResult;

#[derive(Debug, Clone)]
pub struct BearDogMigrationPattern {

    pub name: String,

    pub pattern: Regex,

    pub replacement: String,

    pub error_category: BearDogErrorCategory,

    pub context_requirements: Vec<ContextRequirement>,

    pub safety_level: SafetyLevel,

    pub priority: u32,

    pub requires_beardog_result: bool,
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

    Genetics,

    Workflow,
}

#[derive(Debug, Clone)]
pub enum ContextRequirement {

    InBearDogResultFunction,

    InTestFunction,

    InBenchmarkFunction,

    InExampleCode,

    HasErrorHandling,

    IsOptionType,

    IsResultType,

    InProductionCode,

    HasLoggingContext,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {

    Safe,

    SafeWithReview,

    RequiresAnalysis,

    ManualOnly,

    Production,

    TestOnly,
}

#[derive(Debug)]
pub struct BearDogContextAnalyzer {

    function_patterns: HashMap<String, Regex>,

    import_patterns: HashMap<String, Regex>,

    type_patterns: HashMap<String, Regex>,

    error_patterns: HashMap<String, Regex>,
}

#[derive(Debug, Clone)]
pub struct MigrationCandidate {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub pattern_name: String,
    pub original_code: String,
    pub suggested_replacement: String,
    pub safety_level: SafetyLevel,
    pub context_analysis: ContextAnalysis,
    pub confidence: f32,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub struct ContextAnalysis {
    pub function_name: Option<String>,
    pub function_return_type: Option<String>,
    pub has_beardog_imports: bool,
    pub has_error_handling: bool,
    pub has_logging: bool,
    pub is_test_code: bool,
    pub is_example_code: bool,
    pub is_benchmark_code: bool,
    pub surrounding_context: String,
}

pub struct RefinedBearDogMigrator {

    patterns: Vec<BearDogMigrationPattern>,

    context_analyzer: BearDogContextAnalyzer,

    stats: MigrationStats,

    config: MigratorConfig,
}

#[derive(Debug, Default)]
pub struct MigrationStats {
    pub files_analyzed: usize,
    pub patterns_found: usize,
    pub safe_migrations: usize,
    pub review_migrations: usize,
    pub skipped_migrations: usize,
    pub test_patterns: usize,
    pub confidence_distribution: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct MigratorConfig {

    pub min_confidence: f32,

    pub migrate_tests: bool,

    pub migrate_examples: bool,

    pub migrate_benchmarks: bool,

    pub max_auto_safety_level: SafetyLevel,

    pub require_beardog_result: bool,
}

impl Default for MigratorConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.8,
            migrate_tests: false,
            migrate_examples: true,
            migrate_benchmarks: true,
            max_auto_safety_level: SafetyLevel::SafeWithReview,
            require_beardog_result: true,
        }
    }
}

impl BearDogContextAnalyzer {
    pub fn new() -> RefinedResult<Self> {
        let mut function_patterns = HashMap::with_capacity(16);
        let mut import_patterns = HashMap::with_capacity(16);
        let mut type_patterns = HashMap::with_capacity(16);
        let mut error_patterns = HashMap::with_capacity(16);

        function_patterns.insert(
            "beardog_result_function".to_string(),
            Regex::new(r"fn\s+\w+\s*\([^)]*\)\s*->\s*(?:async\s+)?BearDogResult<")?
        );
        function_patterns.insert(
            "test_function".to_string(),
            Regex::new(r"#\[test\]|#\[tokio::test\]")?
        );
        function_patterns.insert(
            "benchmark_function".to_string(),
            Regex::new(r"#\[bench\]|fn\s+bench_")?
        );

        import_patterns.insert(
            "beardog_errors".to_string(),
            Regex::new(r"use\s+beardog_errors::")?
        );
        import_patterns.insert(
            "beardog_traits".to_string(),
            Regex::new(r"use\s+beardog_traits::")?
        );
        import_patterns.insert(
            "tracing".to_string(),
            Regex::new(r"use\s+tracing::")?
        );

        type_patterns.insert(
            "option_type".to_string(),
            Regex::new(r"Option<[^>]+>")?
        );
        type_patterns.insert(
            "result_type".to_string(),
            Regex::new(r"Result<[^,]+,\s*[^>]+>")?
        );
        type_patterns.insert(
            "beardog_result".to_string(),
            Regex::new(r"BearDogResult<[^>]+>")?
        );

        error_patterns.insert(
            "match_error".to_string(),
            Regex::new(r"match\s+.+\{\s*Ok\(.+\)\s*=>\s*.+,\s*Err\(.+\)\s*=>")?
        );
        error_patterns.insert(
            "map_err".to_string(),
            Regex::new(r"\.map_err\(")?
        );
        error_patterns.insert(
            "question_mark".to_string(),
            Regex::new(r"\?\s*;")?
        );

        Ok(Self {
            function_patterns,
            import_patterns,
            type_patterns,
            error_patterns,
        })
    }

    pub fn analyze_context(&self, content: &str, position: usize) -> RefinedResult<ContextAnalysis> {
        let lines: Vec<&str> = content.lines().collect();
        let line_number = content[..position].matches('\n').count();

        let start_line = line_number.saturating_sub(10);
        let end_line = (line_number + 10).min(lines.len());
        let surrounding_context = lines[start_line..end_line].join("\n");

        let function_name = self.find_containing_function(&surrounding_context);
        let function_return_type = self.analyze_function_return_type(&surrounding_context);

        let has_beardog_imports = self.import_patterns.get("beardog_errors")
            .map_or(false, |_| true);

        if !has_beardog_imports {
            tracing::debug!("No BearDog imports found, adding error handling import");
        }

        let has_logging = self.import_patterns.get("tracing")
            .map_or(false, |_| true);

        let is_test_code = self.function_patterns.get("test_function")
            .map_or(false, |_| true);

        if is_test_code {
            tracing::debug!("Test code detected, using test-appropriate replacement");
        }

        let is_benchmark_code = self.function_patterns.get("benchmark_function")
            .map_or(false, |_| true);

        Ok(ContextAnalysis {
            function_name,
            function_return_type,
            has_beardog_imports,
            has_error_handling: self.error_patterns.values()
                .any(|pattern| pattern.is_match(&surrounding_context)),
            has_logging,
            is_test_code,
            is_example_code: content.contains("examples/") || 
                surrounding_context.contains("// Example") ||
                surrounding_context.contains("/// Example"),
            is_benchmark_code,
            surrounding_context,
        })
    }

    fn find_containing_function(&self, context: &str) -> Option<String> {
        let fn_regex = Regex::new(r"fn\s+(\w+)\s*\(").ok()?;
        fn_regex.captures(context)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn analyze_function_return_type(&self, context: &str) -> Option<String> {
        let return_regex = Regex::new(r"fn\s+\w+\s*\([^)]*\)\s*->\s*([^{]+)").ok()?;
        return_regex.captures(context)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    }
}

impl RefinedBearDogMigrator {
    pub fn new() -> RefinedResult<Self> {
        let context_analyzer = BearDogContextAnalyzer::new()?;
        let patterns = Self::create_beardog_patterns()?;
        let config = MigratorConfig::default();
        
        Ok(Self {
            patterns,
            context_analyzer,
            stats: MigrationStats::default(),
            config,
        })
    }

    pub fn with_config(mut self, config: MigratorConfig) -> Self {
        self.config = config;
        self
    }

    fn create_beardog_patterns() -> RefinedResult<Vec<BearDogMigrationPattern>> {
        let mut patterns = Vec::new();

        patterns.push(BearDogMigrationPattern {
            name: "safe_ops_unwrap".to_string(),
            pattern: Regex::new(r"SafeOps::safe_(\w+)\([^)]+\)\.unwrap\(\)")?,
            replacement: "SafeOps::safe_$1($1)?".to_string(),
            error_category: BearDogErrorCategory::Validation,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
                ContextRequirement::InProductionCode,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 100,
            requires_beardog_result: true,
        });

        patterns.push(BearDogMigrationPattern {
            name: "config_load_unwrap".to_string(),
            pattern: Regex::new(r"BearDogConfig::load[^(]*\([^)]+\)\.unwrap\(\)")?,
            replacement: "BearDogConfig::load($1).map_err(|e| BearDogError::Configuration { message: format!(\"Failed to load configuration: {}\", e) })?".to_string(),
            error_category: BearDogErrorCategory::Configuration,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
                ContextRequirement::HasLoggingContext,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 90,
            requires_beardog_result: true,
        });

        patterns.push(BearDogMigrationPattern {
            name: "json_parse_unwrap".to_string(),
            pattern: Regex::new(r"serde_json::(from_str|to_string)\([^)]+\)\.unwrap\(\)")?,
            replacement: "serde_json::$1($1).map_err(|e| BearDogError::Validation { message: format!(\"JSON operation failed: {}\", e) })?".to_string(),
            error_category: BearDogErrorCategory::Validation,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 80,
            requires_beardog_result: true,
        });

        patterns.push(BearDogMigrationPattern {
            name: "network_unwrap".to_string(),
            pattern: Regex::new(r"(TcpStream::connect|HttpClient::get|reqwest::get)\([^)]+\)\.await\.unwrap\(\)")?,
            replacement: "$1($1).await.map_err(|e| BearDogError::Network { message: format!(\"Network operation failed: {}\", e) })?".to_string(),
            error_category: BearDogErrorCategory::Network,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
                ContextRequirement::HasErrorHandling,
            ],
            safety_level: SafetyLevel::SafeWithReview,
            priority: 70,
            requires_beardog_result: true,
        });

        patterns.push(BearDogMigrationPattern {
            name: "collection_unwrap".to_string(),
            pattern: Regex::new(r"\.get\([^)]+\)\.unwrap\(\)")?,
            replacement: ".get($1).ok_or_else(|| BearDogError::Validation { message: \"Collection access failed: index out of bounds\".to_string() })?".to_string(),
            error_category: BearDogErrorCategory::Validation,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
            ],
            safety_level: SafetyLevel::SafeWithReview,
            priority: 60,
            requires_beardog_result: true,
        });

        patterns.push(BearDogMigrationPattern {
            name: "test_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)")?,
            replacement: ".expect(\"Test assertion failed\")".to_string(),
            error_category: BearDogErrorCategory::System,
            context_requirements: vec![
                ContextRequirement::InTestFunction,
            ],
            safety_level: SafetyLevel::TestOnly,
            priority: 50,
            requires_beardog_result: false,
        });

        patterns.push(BearDogMigrationPattern {
            name: "example_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)")?,
            replacement: ".expect(\"Example operation failed\")".to_string(),
            error_category: BearDogErrorCategory::System,
            context_requirements: vec![
                ContextRequirement::InExampleCode,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 40,
            requires_beardog_result: false,
        });

        patterns.push(BearDogMigrationPattern {
            name: "generic_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)")?,
            replacement: ".map_err(|e| BearDogError::Internal { message: format!(\"Operation failed: {:?}\", e) })?".to_string(),
            error_category: BearDogErrorCategory::System,
            context_requirements: vec![
                ContextRequirement::InBearDogResultFunction,
                ContextRequirement::InProductionCode,
            ],
            safety_level: SafetyLevel::RequiresAnalysis,
            priority: 10,
            requires_beardog_result: true,
        });

        patterns.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(patterns)
    }

    pub async fn analyze_file(&mut self, file_path: &Path) -> RefinedResult<Vec<MigrationCandidate>> {
        let content = fs::read_to_string(file_path).await?;
        let mut candidates = Vec::new();

        debug!("Analyzing file: {}", file_path.display());

        let unwrap_regex = Regex::new(r"\.(?:unwrap|expect)\([^)]*\)")?;
        
        for mat in unwrap_regex.find_iter(&content) {
            let position = mat.start();
            let matched_text = mat.as_str();

            let context = self.context_analyzer.analyze_context(&content, position)?;

            if let Some(candidate) = self.create_migration_candidate(
                file_path,
                position,
                matched_text,
                &context,
                &content,
            )? {
                candidates.push(candidate);
            }
        }

        self.stats.files_analyzed += 1;
        self.stats.patterns_found += candidates.len();

        Ok(candidates)
    }

    fn create_migration_candidate(
        &self,
        file_path: &Path,
        position: usize,
        matched_text: &str,
        context: &ContextAnalysis,
        full_content: &str,
    ) -> RefinedResult<Option<MigrationCandidate>> {

        let best_pattern = self.patterns.iter()
            .find(|pattern| {
                pattern.pattern.is_match(matched_text) &&
                self.check_context_requirements(&pattern.context_requirements, context)
            });

        if let Some(pattern) = best_pattern {
            let line_number = full_content[..position].matches('\n').count() + 1;

            let confidence = self.calculate_confidence(pattern, context);

            if confidence >= self.config.min_confidence &&
               pattern.safety_level <= self.config.max_auto_safety_level {
                
                let replacement = self.generate_replacement(pattern, matched_text, context)?;
                
                let candidate = MigrationCandidate {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    column_start: position,
                    column_end: position + matched_text.len(),
                    pattern_name: pattern.name.clone(),
                    original_code: matched_text.to_string(),
                    suggested_replacement: replacement,
                    safety_level: pattern.safety_level.clone(),
                    context_analysis: context.clone(),
                    confidence,
                    reasoning: format!(
                        "Pattern '{}' matched with {:.1}% confidence. Context: {}",
                        pattern.name,
                        confidence * 100.0,
                        if context.is_test_code { "test code" }
                        else if context.is_example_code { "example code" }
                        else { "production code" }
                    ),
                };

                return Ok(Some(candidate));
            }
        }

        Ok(None)
    }

    fn check_context_requirements(
        &self,
        requirements: &[ContextRequirement],
        context: &ContextAnalysis,
    ) -> bool {
        requirements.iter().all(|req| {
            match req {
                ContextRequirement::InBearDogResultFunction => {
                    context.function_return_type
                        .as_ref()
                        .map(|t| t.contains("BearDogResult"))
                        .unwrap_or(false)
                }
                ContextRequirement::InTestFunction => context.is_test_code,
                ContextRequirement::InExampleCode => context.is_example_code,
                ContextRequirement::InBenchmarkFunction => context.is_benchmark_code,
                ContextRequirement::HasErrorHandling => context.has_error_handling,
                ContextRequirement::InProductionCode => {
                    !context.is_test_code && !context.is_example_code && !context.is_benchmark_code
                }
                ContextRequirement::HasLoggingContext => context.has_logging,
                _ => true, // Other requirements not implemented yet
            }
        })
    }

    fn calculate_confidence(&self, pattern: &BearDogMigrationPattern, context: &ContextAnalysis) -> f32 {
        let mut confidence: f32 = 0.5; // Base confidence

        if context.has_beardog_imports {
            confidence += 0.2;
        }
        if context.has_error_handling {
            confidence += 0.15;
        }
        if context.has_logging {
            confidence += 0.1;
        }

        match pattern.name.as_str() {
            name if name.contains("safe_ops") => confidence += 0.3,
            name if name.contains("config") => confidence += 0.2,
            name if name.contains("json") => confidence += 0.15,
            _ => {}
        }

        if context.is_test_code && pattern.safety_level == SafetyLevel::TestOnly {
            confidence += 0.2;
        }
        if context.is_example_code && !pattern.requires_beardog_result {
            confidence += 0.15;
        }

        confidence.min(1.0)
    }

    fn generate_replacement(
        &self,
        pattern: &BearDogMigrationPattern,
        matched_text: &str,
        _context: &ContextAnalysis,
    ) -> RefinedResult<String> {

        let mut replacement = pattern.replacement.clone();

        if let Some(caps) = pattern.pattern.captures(matched_text) {
            for (i, cap) in caps.iter().enumerate() {
                if let Some(cap_match) = cap {
                    let placeholder = format_args!("${}", i).to_string();
                    replacement = replacement.replace(&placeholder, cap_match.as_str());
                }
            }
        }

        Ok(replacement)
    }

    pub async fn apply_migrations(
        &mut self,
        file_path: &Path,
        candidates: &[MigrationCandidate],
        dry_run: bool,
    ) -> RefinedResult<usize> {
        if candidates.is_empty() {
            return Ok(0);
        }

        let content = fs::read_to_string(file_path).await?;
        let mut modified_content = content.clone();
        let mut applied_count = 0;

        let mut sorted_candidates = candidates.to_vec();
        sorted_candidates.sort_by(|a, b| b.column_start.cmp(&a.column_start));

        for candidate in &sorted_candidates {

            let start = candidate.column_start;
            let end = candidate.column_end;
            
            if start < modified_content.len() && end <= modified_content.len() {
                let before = &modified_content[..start];
                let after = &modified_content[end..];
                modified_content = format_args!("{}{}{}", before, candidate.suggested_replacement, after).to_string();
                applied_count += 1;

                info!(
                    "Applied migration in {}: {} -> {}",
                    file_path.display(),
                    candidate.original_code,
                    candidate.suggested_replacement
                );
            }
        }

        if !dry_run && applied_count > 0 {
            fs::write(file_path, modified_content).await?;
            info!("Updated file: {} ({} migrations)", file_path.display(), applied_count);
        }

        self.stats.safe_migrations += applied_count;
        Ok(applied_count)
    }

    pub fn get_stats(&self) -> &MigrationStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_context_analysis() {
        let analyzer = BearDogContextAnalyzer::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let content = r#"
use beardog_errors::BearDogResult;

fn test_function() -> BearDogResult<String> {
    let value = some_operation().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    Ok(value)
}
"#;
        
        let unwrap_pos = content.find("unwrap").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let context = analyzer.analyze_context(content, unwrap_pos).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        
        assert!(context.has_beardog_imports);
        assert!(context.function_return_type.as_ref().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?.contains("BearDogResult"));
    }

    #[tokio::test]
    async fn test_pattern_matching() {
        let migrator = RefinedBearDogMigrator::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        assert!(!migrator.patterns.is_empty());

        let priorities: Vec<u32> = migrator.patterns.iter().map(|p| p.priority).collect();
        for window in priorities.windows(2) {
            assert!(window[0] >= window[1], "Patterns should be sorted by priority");
        }
    }
} 