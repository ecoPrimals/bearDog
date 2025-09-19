use beardog_errors::BearDogError;

use clap::{Arg, Command};
use std::path::Path;
use tracing::info;
use tracing_subscriber;

mod systematic_migrator;
mod enhanced_migrator;
mod refined_migrator;

use systematic_migrator::{SystematicUnwrapMigrator, MigratorResult};
use refined_migrator::{RefinedBearDogMigrator, MigratorConfig, SafetyLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let matches = Command::new("beardog-unwrap-migrator")
        .version("3.0.0")
        .about("🔄 BearDog Enhanced Unwrap/Expect Migrator - Context-aware panic elimination")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Root path to scan for Rust files (defaults to ./crates)")
                .default_value("./crates")
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be changed without applying changes")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("apply")
                .long("apply")
                .help("Apply the migration changes to files")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("stats-only")
                .long("stats-only")
                .help("Show statistics without performing migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("refined")
                .long("refined")
                .help("Use the refined migrator with enhanced context analysis (recommended)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("confidence")
                .long("confidence")
                .value_name("THRESHOLD")
                .help("Minimum confidence threshold for automatic migration (0.0-1.0)")
                .default_value("0.8")
        )
        .arg(
            Arg::new(safe, safe-with-review, requires-analysis")
                .default_value("safe-with-review")
        )
        .arg(
            Arg::new("migrate-tests")
                .long("migrate-tests")
                .help("Include test files in migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("migrate-examples")
                .long("migrate-examples")
                .help("Include example files in migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("migrate-benchmarks")
                .long("migrate-benchmarks")
                .help("Include benchmark files in migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("require-beardog-result")
                .long("require-beardog-result")
                .help("Only migrate functions that return BearDogResult")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("beardog-errors-only")
                .long("beardog-errors-only")
                .help("Only migrate patterns that can use BearDogError/BearDogResult")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("exclude-tests")
                .long("exclude-tests")
                .help("Exclude test files from migration (tests may legitimately use unwrap)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new('safe', 'expect', 'skip' (default: expect)")
                .default_value("expect")
        )
        .arg(
            Arg::new('safe', 'expect', 'skip' (default: expect)")
                .default_value("expect")
        )
        .arg(
            Arg::new("context-aware")
                .long("context-aware")
                .help("Use enhanced context-aware migration (recommended)")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let default_path = "./crates".to_string();
    let root_path = matches.get_one::<String>("path").unwrap_or(&default_path);
    let dry_run = matches.get_flag("dry-run");
    let apply_changes = matches.get_flag("apply");
    let stats_only = matches.get_flag("stats-only");
    let use_refined = matches.get_flag("refined");
    let beardog_errors_only = matches.get_flag("beardog-errors-only");
    let exclude_tests = matches.get_flag("exclude-tests");
    let context_aware = matches.get_flag("context-aware");
    let examples_strategy = matches.get_one::<String>("examples-strategy");
    let benchmarks_strategy = matches.get_one::<String>("benchmarks-strategy");

    let confidence: f32 = matches.get_one::<String>("confidence")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.8);
    
    let safety_level = match matches.get_one::<String>("safety-level").map(|s| s.as_str()) {
        Some("safe") => SafetyLevel::Safe,
        Some("safe-with-review") => SafetyLevel::SafeWithReview,
        Some("requires-analysis") => SafetyLevel::RequiresAnalysis,
        Some("production") => SafetyLevel::Production,
        _ => SafetyLevel::Safe,
    };

    let migrate_tests = matches.get_flag({}", root_path);
    info!("🔍 Mode: {}", if stats_only { "Statistics Only" } 
                              else if dry_run { "Dry Run" } 
                              else if apply_changes { "Apply Changes" } 
                              else { "Preview" });
    
    if use_refined {
        info!("🧠 Using refined migrator with enhanced context analysis");
        info!("📊 Confidence threshold: {:.1}%", confidence * 100.0);
        info!("🛡️ Safety level: {:?}", safety_level);

        let config = MigratorConfig {
            min_confidence: confidence,
            migrate_tests,
            migrate_examples,
            migrate_benchmarks,
            max_auto_safety_level: safety_level,
            require_beardog_result,
        };
        
        let mut migrator = RefinedBearDogMigrator::new({}", strategy);
            }
            if let Some({}", strategy);
            }
        }

        let migrator = SystematicUnwrapMigrator::new_beardog_optimized(beardog_errors_only);

        if stats_only {
            let stats = migrator.analyze_codebase(Path::new(root_path), exclude_tests).await?;
            
            println!("\n📊 BearDog Codebase Analysis:");
            println!("   📁 Files scanned: {}", stats.files_scanned);
            println!("   ⚠️  Total unwrap/expect calls: {}", stats.total_unwrap_calls);
            println!("   🔧 Migrable patterns: {}", stats.migrable_patterns);
            println!("   🧪 Test file patterns: {}", stats.test_file_patterns);
            println!("   🎯 BearDogError compatible: {}", stats.beardog_error_compatible);
            
            println!("\n📋 Pattern Breakdown:");
            for (category, count) in &stats.pattern_categories {
                println!("   {} {}: {}", 
                    match category.as_str() {
                        "Configuration" => "⚙️",
                        "Network" => "🌐",
                        "Storage" => "💾",
                        "Security" => "🛡️",
                        "Validation" => "✅",
                        _ => "📦"
                    },
                    category, count);
            }

            if context_aware {
                println!("\n🧠 Context Analysis:");
                println!("   📚 Example files detected: {}", count_files_by_pattern({}", count_files_by_pattern({}", count_files_by_pattern(root_path, "test")?);
            }
            
        } else if dry_run || apply_changes {
            let result = migrator.migrate_codebase(
                Path::new(root_path), 
                !apply_changes, // dry_run = !apply_changes
                exclude_tests
            ).await?;
            
            println!("\n🎉 BearDog Migration Complete:");
            println!("   📁 Files processed: {}", result.files_processed);
            println!("   🔧 Migrations applied: {}", result.migrations_applied);
            println!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
            
            if !result.failed_files.is_empty() {
                println!("\n⚠️  Files with issues:");
                for (file, error) in &result.failed_files {
                    println!("   ❌ {}: {}", file.display(), error);
                }
            }
            
            if !apply_changes {
                println!("\n💡 Run with --apply to make the changes permanent");
                if context_aware {
                    println!("   🧠 Add --context-aware for intelligent context-based migrations");
                }
                println!("   🚀 Try --refined for the new enhanced migrator");
            } else {
                println!("\n✅ Changes have been applied to your codebase");
                println!("   🧪 Run tests to verify everything works correctly");
                println!("   📊 Run with --stats-only to see remaining patterns");
            }
        } else {
            println!("Please specify --dry-run, --apply, or --stats-only");
            println!("\n💡 Helpful commands:");
            println!("   📊 Analysis: --stats-only --refined");
            println!("   🧪 Test run: --dry-run --refined --confidence 0.9");
            println!("   ⚡ Apply: --apply --refined --safety-level safe");
        }
    }

    Ok(&mut RefinedBearDogMigrator,
    root_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    let mut total_candidates = 0;
    let mut files_processed = 0;

    for entry in walkdir::WalkDir::new({} -> {}",
                                candidate.line_number,
                                candidate.original_code,
                                candidate.suggested_replacement);
                            println!("      Confidence: {:.1}%, Safety: {:?}",
                                candidate.confidence * 100.0,
                                candidate.safety_level);
                        }
                    }
                }
                Err({}", path.display(), e);
                }
            }
        }
    }
    
    let stats = migrator.get_stats();
    println!("\n📊 Refined Analysis Summary:");
    println!("   📁 Files analyzed: {}", stats.files_analyzed);
    println!("   🔧 Migration candidates: {}", total_candidates);
    println!("   ✅ Safe migrations: {}", stats.safe_migrations);
    println!("   ⚠️ Review required: {}", stats.review_migrations);
    println!("   ❌ Skipped: {}", stats.skipped_migrations);
    
    Ok(&mut RefinedBearDogMigrator,
    root_path: &str,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    let mut total_applied = 0;
    let mut files_modified = 0;

    for entry in walkdir::WalkDir::new({}", path.display(), e);
                }
            }
        }
    }
    
    println!("\n🎉 Refined Migration Complete:");
    println!("   📁 Files modified: {}", files_modified);
    println!("   🔧 Migrations applied: {}", total_applied);
    
    if dry_run {
        println!("\n💡 This was a dry run. Use --apply to make changes permanent.");
    } else {
        println!("\n✅ Changes have been applied to your codebase");
        println!("   🧪 Run tests to verify everything works correctly");
    }
    
    Ok(&str, pattern: &str) -> Result<usize, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    let mut count = 0;
    let path = Path::new(root_path);
    
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().unwrap_or_default().to_string_lossy().contains(pattern) {
                    count += count_rust_files(&path)?;
                }
            }
        }
    }
    
    Ok(count)
}

fn count_rust_files(dir: &std::path::Path) -> Result<usize, Box<dyn std::error::Error>> {
    use std::fs;
    
    let mut count = 0;
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += count_rust_files(&path)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            count += 1;
        }
    }
    
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let cmd = Command::new("test-unwrap-migrator")
            .version("3.0.0")
            .about("Test CLI");
        
        assert_eq!(cmd.get_name(), "test-unwrap-migrator");
    }
}
