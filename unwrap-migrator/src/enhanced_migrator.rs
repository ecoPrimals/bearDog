use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use regex::Regex;
use thiserror::Error;
use tracing::{info, warn, error};

#[derive(Error, Debug)]
pub enum EnhancedMigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
    #[error("Unicode error: {message}")]
    Unicode { message: String },
}

pub use beardog_types::aliases::MigrationResult as EnhancedMigratorResult;

#[derive(Debug, Clone)]
pub struct EnhancedPattern {
    pub pattern_type: PatternType,
    pub regex: Regex,
    pub replacement_strategy: ReplacementStrategy,
    pub context_requirements: Vec<ContextRequirement>,
    pub safety_level: SafetyLevel,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    OptionUnwrap,
    ResultUnwrap,
    OptionExpect,
    ResultExpect,
    TestPanic,
    ProductionPanic,
    BenchmarkUnwrap,
    ExampleUnwrap,
}

#[derive(Debug, Clone)]
pub enum ReplacementStrategy {
    SafeUnwrapWithContext,
    PropagateError,
    DefaultValue,
    TestAssertion,
    BenchmarkSafe,
    LogAndContinue,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ContextRequirement {
    InTestFunction,
    InBenchmarkFunction,
    InExampleCode,
    HasBearDogResult,
    HasErrorHandling,
    IsOptionType,
    IsResultType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,        // Can be migrated safely
    Caution,     // Needs review
    Unsafe,      // Should not be auto-migrated
    TestOnly,    // Only in test code
}

pub struct EnhancedUnwrapMigrator {
    patterns: Vec<EnhancedPattern>,
    context_analyzers: HashMap<String, ContextAnalyzer>,
    migration_stats: MigrationStats,
}

#[derive(Debug, Default)]
pub struct MigrationStats {
    pub files_analyzed: usize,
    pub patterns_found: usize,
    pub safe_migrations: usize,
    pub caution_migrations: usize,
    pub skipped_migrations: usize,
    pub test_patterns: usize,
}

#[derive(Debug, Clone)]
pub struct ContextAnalyzer {
    pub function_detector: Regex,
    pub import_detector: Regex,
    pub type_detector: Regex,
}

#[derive(Debug, Clone)]
pub struct MigrationCandidate {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub pattern_type: PatternType,
    pub original_code: String,
    pub suggested_replacement: String,
    pub safety_level: SafetyLevel,
    pub context: String,
    pub reasoning: String,
}

impl EnhancedUnwrapMigrator {

    pub fn new() -> EnhancedMigratorResult<Self> {
        let mut patterns = Vec::new();

        patterns.push(EnhancedPattern {
            pattern_type: PatternType::OptionUnwrap,
            regex: Regex::new(r"(\w+(?:\.\w+)*(?:\([^)]*\))?(?:\?)?)\s*\.\s*unwrap\(\)")?,
            replacement_strategy: ReplacementStrategy::SafeUnwrapWithContext,
            context_requirements: vec![ContextRequirement::IsOptionType],
            safety_level: SafetyLevel::Safe,
        });

        patterns.push(EnhancedPattern {
            pattern_type: PatternType::ResultUnwrap,
            regex: Regex::new(r"(\w+(?:\.\w+)*(?:\([^)]*\))?(?:\?)?)\s*\.\s*unwrap\(\)")?,
            replacement_strategy: ReplacementStrategy::PropagateError,
            context_requirements: vec![ContextRequirement::IsResultType, ContextRequirement::HasBearDogResult],
            safety_level: SafetyLevel::Safe,
        });

        patterns.push(EnhancedPattern {
            pattern_type: PatternType::OptionExpect,
            regex: Regex::new(r#"(\w+(?:\.\w+)*(?:\([^)]*\))?(?:\?)?)\s*\.\s*expect\s*\(\s*"([^"]+)"\s*\)"#)?,
            replacement_strategy: ReplacementStrategy::SafeUnwrapWithContext,
            context_requirements: vec![ContextRequirement::IsOptionType],
            safety_level: SafetyLevel::Safe,
        });

        patterns.push(EnhancedPattern {
            pattern_type: PatternType::TestPanic,
            regex: Regex::new(r#"panic!\s*\(\s*"([^"]+)"\s*(?:,\s*[^)]+)?\s*\)"#)?,
            replacement_strategy: ReplacementStrategy::TestAssertion,
            context_requirements: vec![ContextRequirement::InTestFunction],
            safety_level: SafetyLevel::TestOnly,
        });

        patterns.push(EnhancedPattern {
            pattern_type: PatternType::BenchmarkUnwrap,
            regex: Regex::new(r"(\w+(?:\.\w+)*(?:\([^)]*\))?)\s*\.\s*unwrap\(\)")?,
            replacement_strategy: ReplacementStrategy::BenchmarkSafe,
            context_requirements: vec![ContextRequirement::InBenchmarkFunction],
            safety_level: SafetyLevel::Caution,
        });
        
        let mut context_analyzers = HashMap::with_capacity(16);

        context_analyzers.insert("test".to_string(), ContextAnalyzer {
            function_detector: Regex::new(r"#\[tokio::test\]|#\[test\]|fn test_")?,
            import_detector: Regex::new(r"use.*test")?,
            type_detector: Regex::new(r"TestResult|TestCase")?,
        });

        context_analyzers.insert("benchmark".to_string(), ContextAnalyzer {
            function_detector: Regex::new(r"#\[bench\]|fn\s+bench_(\w+)")?,
            import_detector: Regex::new(r"use.*bench")?,
            type_detector: Regex::new(r"Bencher|BenchmarkId")?,
        });
        
        Ok(Self {
            patterns,
            context_analyzers,
            migration_stats: MigrationStats::default(),
        })
    }

    pub async fn analyze_file(&mut self, file_path: &Path) -> EnhancedMigratorResult<Vec<MigrationCandidate>> {
        let content = fs::read_to_string(file_path).await?;
        let mut candidates = Vec::new();
        
        self.migration_stats.files_analyzed += 1;

        let file_context = self.analyze_file_context(&content, file_path)?;
        
        for pattern in &self.patterns {
            for mat in pattern.regex.find_iter(&content) {
                let line_number = content[..mat.start()].matches('\n').count() + 1;

                let context = self.extract_safe_context(&content, mat.start(), mat.end())?;

                if self.meets_context_requirements(&pattern.context_requirements, &content, &file_context, mat.start())? {
                    let candidate = self.create_migration_candidate(
                        file_path.to_path_buf(),
                        line_number,
                        pattern,
                        mat.as_str(),
                        &context,
                        &file_context,
                    )?;
                    
                    candidates.push(candidate);
                    self.migration_stats.patterns_found += 1;
                }
            }
        }
        
        Ok(candidates)
    }

    fn extract_safe_context(&self, content: &str, start: usize, end: usize) -> EnhancedMigratorResult<String> {
        let context_size = 100;

        let context_start = content.char_indices()
            .map(|(i, _)| i)
            .find(|&i| i >= start.saturating_sub(context_size))
            .unwrap_or(0);

        let context_end = content.char_indices()
            .map(|(i, _)| i)
            .find(|&i| i >= end + context_size)
            .unwrap_or(content.len());
            
        Ok(content[context_start..context_end].to_string())
    }

    fn analyze_file_context(&self, content: &str, file_path: &Path) -> EnhancedMigratorResult<FileContext> {
        let mut context = FileContext::default();

        if file_path.to_string_lossy().contains("/tests/") || 
           file_path.to_string_lossy().contains("test_") ||
           content.contains("#[test]") || content.contains("#[tokio::test]") {
            context.is_test_file = true;
        }
        
        if file_path.to_string_lossy().contains("/benches/") || 
           file_path.to_string_lossy().contains("bench") ||
           content.contains("#[bench]") {
            context.is_benchmark_file = true;
        }
        
        if file_path.to_string_lossy().contains("/examples/") {
            context.is_example_file = true;
        }

        if content.contains("Result<T, BearDogError>") || content.contains("beardog_errors::BearDogResult") {
            context.has_beardog_result = true;
        }

        if content.contains("map_err") || content.contains("?") || content.contains("match") {
            context.has_error_handling = true;
        }
        
        Ok(context)
    }

    fn meets_context_requirements(
        &self, 
        requirements: &[ContextRequirement],
        content: &str,
        file_context: &FileContext,
        position: usize
    ) -> EnhancedMigratorResult<bool> {
        for requirement in requirements {
            match requirement {
                ContextRequirement::InTestFunction => {
                    if !file_context.is_test_file && !self.is_in_test_function(content, position)? {
                        return Ok(false);
                    }
                }
                ContextRequirement::InBenchmarkFunction => {
                    if !file_context.is_benchmark_file && !self.is_in_benchmark_function(content, position)? {
                        return Ok(false);
                    }
                }
                ContextRequirement::InExampleCode => {
                    if !file_context.is_example_file {
                        return Ok(false);
                    }
                }
                ContextRequirement::HasBearDogResult => {
                    if !file_context.has_beardog_result {
                        return Ok(false);
                    }
                }
                ContextRequirement::HasErrorHandling => {
                    if !file_context.has_error_handling {
                        return Ok(false);
                    }
                }
                _ => {} // Other requirements can be checked later
            }
        }
        Ok(true)
    }

    fn is_in_test_function(&self, content: &str, position: usize) -> EnhancedMigratorResult<bool> {
        let before_position = &content[..position];
        let test_regex = Regex::new(r"#\[(tokio::)?test\][\s\n]*(?:async\s+)?fn\s+(\w+)")?;
        
        if let Some(last_match) = test_regex.find_iter(before_position).last() {

            let function_start = last_match.start();
            if let Some(function_end) = self.find_function_end(content, function_start)? {
                return Ok(position >= function_start && position <= function_end);
            }
        }
        
        Ok(false)
    }

    fn is_in_benchmark_function(&self, content: &str, position: usize) -> EnhancedMigratorResult<bool> {
        let before_position = &content[..position];
        let bench_regex = Regex::new(r"#\[bench\][\s\n]*fn\s+(\w+)|fn\s+bench_(\w+)")?;
        
        if let Some(last_match) = bench_regex.find_iter(before_position).last() {
            let function_start = last_match.start();
            if let Some(function_end) = self.find_function_end(content, function_start)? {
                return Ok(position >= function_start && position <= function_end);
            }
        }
        
        Ok(false)
    }

    fn find_function_end(&self, content: &str, function_start: usize) -> EnhancedMigratorResult<Option<usize>> {
        let after_function = &content[function_start..];
        let mut brace_count = 0;
        let mut found_opening = false;
        
        for (i, ch) in after_function.char_indices() {
            match ch {
                '{' => {
                    brace_count += 1;
                    found_opening = true;
                }
                '}' => {
                    brace_count -= 1;
                    if found_opening && brace_count == 0 {
                        return Ok(Some(function_start + i));
                    }
                }
                _ => {}
            }
        }
        
        Ok(None)
    }

    fn create_migration_candidate(
        &self,
        file_path: PathBuf,
        line_number: usize,
        pattern: &EnhancedPattern,
        original_code: &str,
        context: &str,
        file_context: &FileContext,
    ) -> EnhancedMigratorResult<MigrationCandidate> {
        let suggested_replacement = self.generate_replacement(
            &pattern.replacement_strategy,
            original_code,
            context,
            file_context,
        )?;
        
        let reasoning = self.generate_reasoning(&pattern.pattern_type, &pattern.safety_level, file_context);
        
        Ok(MigrationCandidate {
            file_path,
            line_number,
            pattern_type: pattern.pattern_type.clone(),
            original_code: original_code.to_string(),
            suggested_replacement,
            safety_level: pattern.safety_level.clone(),
            context: context.to_string(),
            reasoning,
        })
    }

    fn generate_replacement(
        &self,
        strategy: &ReplacementStrategy,
        original_code: &str,
        context: &str,
        file_context: &FileContext,
    ) -> EnhancedMigratorResult<String> {
        match strategy {
            ReplacementStrategy::SafeUnwrapWithContext => {
                if file_context.has_beardog_result {
                    Ok(format!("{}.ok_or_else(|| BearDogError::internal(\"Expected value not found\"))?", 
                               self.extract_expression(original_code)?))
                } else {
                    Ok(format!("{}.expect(\"Expected value not found\")", 
                               self.extract_expression(original_code)?))
                }
            }
            ReplacementStrategy::PropagateError => {
                Ok(format_args!("{}?", self.extract_expression(original_code).to_string()?))
            }
            ReplacementStrategy::TestAssertion => {
                Ok(format!("assert!(false, \"Test assertion failed: {}\");", 
                           self.extract_panic_message(original_code).unwrap_or("test failed".to_string())))
            }
            ReplacementStrategy::BenchmarkSafe => {
                Ok(format!("{}.expect(\"Benchmark setup failed\")", 
                           self.extract_expression(original_code)?))
            }
            ReplacementStrategy::LogAndContinue => {
                Ok(format!("{{ tracing::warn!(\"Operation failed, continuing\"); Default::default() }}"))
            }
            ReplacementStrategy::Custom(replacement) => {
                Ok(replacement.clone())
            }
            _ => Ok(original_code.to_string()), // No change for other strategies
        }
    }

    fn extract_expression(&self, code: &str) -> EnhancedMigratorResult<String> {
        let unwrap_regex = Regex::new(r"(.+)\s*\.\s*(?:unwrap|expect)\s*\([^)]*\)")?;
        if let Some(caps) = unwrap_regex.captures(code) {
            Ok(caps[1].trim().to_string())
        } else {
            Ok(code.to_string())
        }
    }

    fn extract_panic_message(&self, code: &str) -> Option<String> {
        let panic_regex = Regex::new(r#"panic!\s*\(\s*"([^"]+)""#).ok()?;
        panic_regex.captures(code)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn generate_reasoning(&self, pattern_type: &PatternType, safety_level: &SafetyLevel, file_context: &FileContext) -> String {
        match (pattern_type, safety_level) {
            (PatternType::OptionUnwrap, SafetyLevel::Safe) => {
                "Option unwrap can be safely replaced with proper error handling".to_string()
            }
            (PatternType::ResultUnwrap, SafetyLevel::Safe) => {
                "Result unwrap can be replaced with error propagation using ?".to_string()
            }
            (PatternType::TestPanic, SafetyLevel::TestOnly) => {
                "Test panic can be replaced with assertion for better test reporting".to_string()
            }
            (PatternType::BenchmarkUnwrap, SafetyLevel::Caution) => {
                "Benchmark unwrap should be replaced with expect for clearer error messages".to_string()
            }
            _ => "Pattern can be improved for better error handling".to_string()
        }
    }

    pub async fn apply_migration(&mut self, candidate: &MigrationCandidate) -> EnhancedMigratorResult<bool> {
        let content = fs::read_to_string(&candidate.file_path).await?;
        let new_content = content.replace(&candidate.original_code, &candidate.suggested_replacement);
        
        if content != new_content {
            fs::write(&candidate.file_path, new_content).await?;
            
            match candidate.safety_level {
                SafetyLevel::Safe => self.migration_stats.safe_migrations += 1,
                SafetyLevel::Caution => self.migration_stats.caution_migrations += 1,
                SafetyLevel::TestOnly => self.migration_stats.test_patterns += 1,
                SafetyLevel::Unsafe => self.migration_stats.skipped_migrations += 1,
            }
            
            info!("Applied migration in {}: {} -> {}", 
                  candidate.file_path.display(),
                  candidate.original_code,
                  candidate.suggested_replacement);
            
            return Ok(true);
        }
        
        Ok(false)
    }

    pub fn get_stats(&self) -> &MigrationStats {
        &self.migration_stats
    }
}

#[derive(Debug, Default)]
struct FileContext {
    is_test_file: bool,
    is_benchmark_file: bool,
    is_example_file: bool,
    has_beardog_result: bool,
    has_error_handling: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_migrator_creation() {
        let migrator = EnhancedUnwrapMigrator::new();
        assert!(migrator.is_ok());
    }
    
    #[test]
    fn test_safe_context_extraction() {
        let migrator = EnhancedUnwrapMigrator::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let content = "Hello 🦀 world with unicode";
        let context = migrator.extract_safe_context(content, 6, 8);
        assert!(context.is_ok());
    }
} 