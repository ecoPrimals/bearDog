// BearDog Unwrap Migrator - Pure Rust unwrap/expect to Result migration tool
//
// This tool analyzes and migrates unwrap()/expect() calls to proper Result handling

use clap::{Arg, Command};
use std::path::PathBuf;
use tracing::{info, warn};

mod refined_migrator;
use refined_migrator::{RefinedBearDogMigrator, MigratorConfig, SafetyLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let matches = Command::new("beardog-unwrap-migrator")
        .version("3.0.0")
        .about("🔄 BearDog Unwrap/Expect Migrator - Convert unwrap/expect to Result handling")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Root path to scan for Rust files")
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
            Arg::new("confidence")
                .long("confidence")
                .value_name("THRESHOLD")
                .help("Minimum confidence threshold (0.0-1.0)")
                .default_value("0.8")
        )
        .arg(
            Arg::new("safety-level")
                .long("safety-level")
                .value_name("LEVEL")
                .help("Maximum safety level: safe, safe-with-review, requires-analysis")
                .default_value("safe-with-review")
        )
        .arg(
            Arg::new("exclude-tests")
                .long("exclude-tests")
                .help("Exclude test files (tests may legitimately use unwrap)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("migrate-tests")
                .long("migrate-tests")
                .help("Include test files in migration")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let root_path: PathBuf = matches.get_one::<String>("path")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./crates"));
    
    let dry_run = matches.get_flag("dry-run");
    let apply_changes = matches.get_flag("apply");
    let stats_only = matches.get_flag("stats-only");
    let exclude_tests = matches.get_flag("exclude-tests");
    let migrate_tests = matches.get_flag("migrate-tests");

    let confidence: f32 = matches.get_one::<String>("confidence")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.8);
    
    let safety_level = match matches.get_one::<String>("safety-level").map(|s| s.as_str()) {
        Some("safe") => SafetyLevel::Safe,
        Some("safe-with-review") => SafetyLevel::SafeWithReview,
        Some("requires-analysis") => SafetyLevel::RequiresAnalysis,
        _ => SafetyLevel::SafeWithReview,
    };

    info!("🐻 BearDog Unwrap Migrator v3.0.0");
    info!("📁 Scanning path: {}", root_path.display());
    info!("🔍 Mode: {}", if stats_only { "Statistics Only" } 
                          else if dry_run { "Dry Run" } 
                          else if apply_changes { "Apply Changes" } 
                          else { "Preview" });
    info!("🎯 Confidence threshold: {:.1}%", confidence * 100.0);
    info!("🛡️ Safety level: {:?}", safety_level);

    let config = MigratorConfig {
        min_confidence: confidence,
        migrate_tests,
        max_auto_safety_level: safety_level,
    };
    
    let mut migrator = RefinedBearDogMigrator::new(config);

    if stats_only {
        info!("📊 Analyzing codebase...");
        let stats = migrator.analyze_directory(&root_path, exclude_tests).await?;
        
        println!("\n📊 BearDog Codebase Analysis:");
        println!("   📁 Files scanned: {}", stats.files_scanned);
        println!("   ⚠️  Total unwrap calls: {}", stats.unwrap_count);
        println!("   ⚠️  Total expect calls: {}", stats.expect_count);
        println!("   🔧 Migrable patterns: {}", stats.migrable_count);
        if stats.test_file_count > 0 {
            println!("   🧪 Test files: {} (excluded: {})", stats.test_file_count, exclude_tests);
        }
        
        if stats.by_category.len() > 0 {
            println!("\n📋 Patterns by Context:");
            for (category, count) in &stats.by_category {
                println!("   {}: {}", category, count);
            }
        }
    } else if dry_run {
        info!("🧪 Running dry-run migration...");
        let result = migrator.migrate_directory(&root_path, true, exclude_tests).await?;
        
        println!("\n🎉 Dry Run Complete:");
        println!("   📁 Files processed: {}", result.files_processed);
        println!("   🔧 Would migrate: {}", result.patterns_migrated);
        println!("   ⚠️  Skipped (low confidence): {}", result.skipped_count);
        
        if !result.failed_files.is_empty() {
            warn!("\n⚠️  Files with issues:");
            for (file, error) in &result.failed_files {
                warn!("   ❌ {}: {}", file.display(), error);
            }
        }
        
        println!("\n💡 Run with --apply to make the changes permanent");
    } else if apply_changes {
        info!("⚡ Applying migrations...");
        let result = migrator.migrate_directory(&root_path, false, exclude_tests).await?;
        
        println!("\n🎉 Migration Complete:");
        println!("   📁 Files processed: {}", result.files_processed);
        println!("   🔧 Patterns migrated: {}", result.patterns_migrated);
        println!("   ⚠️  Skipped: {}", result.skipped_count);
        
        if !result.failed_files.is_empty() {
            warn!("\n⚠️  Files with issues:");
            for (file, error) in &result.failed_files {
                warn!("   ❌ {}: {}", file.display(), error);
            }
        }
        
        println!("\n✅ Changes have been applied to your codebase");
        println!("   🧪 Run tests to verify everything works: cargo test");
        println!("   📊 Check remaining patterns: --stats-only");
    } else {
        println!("Please specify --dry-run, --apply, or --stats-only");
        println!("\n💡 Helpful commands:");
        println!("   📊 Analysis:  cargo run --release -- --stats-only");
        println!("   🧪 Test run:  cargo run --release -- --dry-run");
        println!("   ⚡ Apply:     cargo run --release -- --apply");
        println!("   🎯 Advanced:  cargo run --release -- --dry-run --confidence 0.9");
    }

    Ok(())
}
