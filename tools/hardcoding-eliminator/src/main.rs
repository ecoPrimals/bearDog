//! Hardcoding Elimination Tool
//!
//! This tool systematically finds and helps migrate hardcoded vendor and primal
//! references to capability-based discovery patterns.
//!
//! ## Usage
//! ```bash
//! cargo run --bin hardcoding-eliminator -- scan
//! cargo run --bin hardcoding-eliminator -- migrate --type vendor
//! cargo run --bin hardcoding-eliminator -- migrate --type primal
//! cargo run --bin hardcoding-eliminator -- validate
//! ```

use clap::{Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod vendor_migrator;
mod primal_migrator;
mod url_migrator;
mod validation;

use vendor_migrator::VendorMigrator;
use primal_migrator::PrimalMigrator;
use url_migrator::UrlMigrator;
use validation::HardcodingValidator;

#[derive(Parser)]
#[command(name = "hardcoding-eliminator")]
#[command(about = "Eliminate vendor and primal hardcoding for true sovereignty")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan codebase for hardcoding patterns
    Scan {
        /// Directory to scan
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Migrate hardcoded references
    Migrate {
        /// Migration type (vendor, primal, url)
        #[arg(short, long)]
        migration_type: String,
        /// Directory to migrate
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Dry run (don't make changes)
        #[arg(long)]
        dry_run: bool,
    },
    /// Validate zero-hardcoding compliance
    Validate {
        /// Directory to validate
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
    /// Generate migration report
    Report {
        /// Directory to analyze
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Output file
        #[arg(short, long, default_value = "hardcoding_report.md")]
        output: PathBuf,
    },
}

/// Hardcoding scan results
#[derive(Debug, Serialize, Deserialize)]
pub struct HardcodingScanResults {
    pub vendor_hardcoding: Vec<HardcodingInstance>,
    pub primal_hardcoding: Vec<HardcodingInstance>,
    pub url_hardcoding: Vec<HardcodingInstance>,
    pub summary: HardcodingSummary,
}

/// Individual hardcoding instance
#[derive(Debug, Serialize, Deserialize)]
pub struct HardcodingInstance {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub hardcoding_type: HardcodingType,
    pub pattern_matched: String,
    pub suggested_replacement: String,
    pub priority: Priority,
}

/// Type of hardcoding found
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum HardcodingType {
    Vendor,
    Primal,
    Url,
    ServiceName,
    Endpoint,
}

/// Priority level for fixing
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,  // Blocks primal sovereignty
    High,      // Important for ecosystem scalability
    Medium,    // Good to fix for cleanliness
    Low,       // Cosmetic or test-only
}

/// Summary of hardcoding scan
#[derive(Debug, Serialize, Deserialize)]
pub struct HardcodingSummary {
    pub total_files_scanned: usize,
    pub files_with_hardcoding: usize,
    pub total_instances: usize,
    pub critical_instances: usize,
    pub high_priority_instances: usize,
    pub compliance_score: f64, // 0-100, where 100 = zero hardcoding
}

/// Hardcoding patterns to detect
pub struct HardcodingPatterns {
    /// Vendor-specific patterns
    pub vendor_patterns: Vec<HardcodingPattern>,
    /// Primal name patterns
    pub primal_patterns: Vec<HardcodingPattern>,
    /// URL/endpoint patterns
    pub url_patterns: Vec<HardcodingPattern>,
}

/// Individual hardcoding pattern
pub struct HardcodingPattern {
    pub name: String,
    pub regex: Regex,
    pub hardcoding_type: HardcodingType,
    pub priority: Priority,
    pub replacement_template: String,
    pub migration_guidance: String,
}

impl Default for HardcodingPatterns {
    fn default() -> Self {
        Self {
            vendor_patterns: vec![
                HardcodingPattern {
                    name: "AWS References".to_string(),
                    regex: Regex::new(r"(?i)\b(aws|amazon)\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::High,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::KeyManagement)".to_string(),
                    migration_guidance: "Replace AWS-specific code with capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "GCP References".to_string(),
                    regex: Regex::new(r"(?i)\b(gcp|google)\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::High,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::CloudProvider)".to_string(),
                    migration_guidance: "Replace GCP-specific code with capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "Azure References".to_string(),
                    regex: Regex::new(r"(?i)\bazure\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::High,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::CloudProvider)".to_string(),
                    migration_guidance: "Replace Azure-specific code with capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "Kubernetes References".to_string(),
                    regex: Regex::new(r"(?i)\b(kubernetes|k8s)\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::High,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::ContainerOrchestration)".to_string(),
                    migration_guidance: "Replace K8s-specific code with container orchestration capability".to_string(),
                },
                HardcodingPattern {
                    name: "Vault References".to_string(),
                    regex: Regex::new(r"(?i)\bvault\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::High,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::SecretsManagement)".to_string(),
                    migration_guidance: "Replace Vault-specific code with secrets management capability".to_string(),
                },
                HardcodingPattern {
                    name: "Consul References".to_string(),
                    regex: Regex::new(r"(?i)\bconsul\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::Medium,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::ServiceDiscovery)".to_string(),
                    migration_guidance: "Replace Consul-specific code with service discovery capability".to_string(),
                },
                HardcodingPattern {
                    name: "Etcd References".to_string(),
                    regex: Regex::new(r"(?i)\betcd\b").unwrap(),
                    hardcoding_type: HardcodingType::Vendor,
                    priority: Priority::Medium,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::DistributedStorage)".to_string(),
                    migration_guidance: "Replace etcd-specific code with distributed storage capability".to_string(),
                },
            ],
            primal_patterns: vec![
                HardcodingPattern {
                    name: "ToadStool References".to_string(),
                    regex: Regex::new(r"(?i)\btoadstool\b").unwrap(),
                    hardcoding_type: HardcodingType::Primal,
                    priority: Priority::Critical,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::ComputeIntelligence)".to_string(),
                    migration_guidance: "Replace ToadStool hardcoding with compute capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "SongBird References".to_string(),
                    regex: Regex::new(r"(?i)\bsongbird\b").unwrap(),
                    hardcoding_type: HardcodingType::Primal,
                    priority: Priority::Critical,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::ServiceMesh)".to_string(),
                    migration_guidance: "Replace SongBird hardcoding with service mesh capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "Squirrel References".to_string(),
                    regex: Regex::new(r"(?i)\bsquirrel\b").unwrap(),
                    hardcoding_type: HardcodingType::Primal,
                    priority: Priority::Critical,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::DistributedIntelligence)".to_string(),
                    migration_guidance: "Replace Squirrel hardcoding with AI capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "NestGate References".to_string(),
                    regex: Regex::new(r"(?i)\bnestgate\b").unwrap(),
                    hardcoding_type: HardcodingType::Primal,
                    priority: Priority::Critical,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::DataStorage)".to_string(),
                    migration_guidance: "Replace NestGate hardcoding with storage capability discovery".to_string(),
                },
                HardcodingPattern {
                    name: "BiomeOS References".to_string(),
                    regex: Regex::new(r"(?i)\bbiome[oO][sS]\b").unwrap(),
                    hardcoding_type: HardcodingType::Primal,
                    priority: Priority::Critical,
                    replacement_template: "universal_adapter.discover_capability(ServiceCapabilityType::ContainerOrchestration)".to_string(),
                    migration_guidance: "Replace BiomeOS hardcoding with orchestration capability discovery".to_string(),
                },
            ],
            url_patterns: vec![
                HardcodingPattern {
                    name: "Hardcoded HTTP URLs".to_string(),
                    regex: Regex::new(r#""https?://[^"]+""#).unwrap(),
                    hardcoding_type: HardcodingType::Url,
                    priority: Priority::Medium,
                    replacement_template: "discovered_endpoint.url".to_string(),
                    migration_guidance: "Replace hardcoded URLs with discovered endpoints".to_string(),
                },
                HardcodingPattern {
                    name: "Localhost References".to_string(),
                    regex: Regex::new(r"localhost|127\.0\.0\.1").unwrap(),
                    hardcoding_type: HardcodingType::Endpoint,
                    priority: Priority::Low,
                    replacement_template: "universal_adapter.discover_local_endpoint()".to_string(),
                    migration_guidance: "Replace localhost with dynamic local endpoint discovery".to_string(),
                },
                HardcodingPattern {
                    name: "Internal Domain References".to_string(),
                    regex: Regex::new(r"\.internal|\.local").unwrap(),
                    hardcoding_type: HardcodingType::Endpoint,
                    priority: Priority::Medium,
                    replacement_template: "universal_adapter.discover_internal_endpoint()".to_string(),
                    migration_guidance: "Replace internal domains with dynamic discovery".to_string(),
                },
            ],
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scan { dir, format } => {
            println!("🔍 Scanning for hardcoding patterns in: {}", dir.display());
            let results = scan_hardcoding(&dir).await?;
            
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&results)?);
                }
                _ => {
                    print_scan_results(&results);
                }
            }
        }
        Commands::Migrate { migration_type, dir, dry_run } => {
            println!("🔄 Migrating {} hardcoding in: {}", migration_type, dir.display());
            
            match migration_type.as_str() {
                "vendor" => {
                    let migrator = VendorMigrator::new();
                    migrator.migrate(&dir, dry_run).await?;
                }
                "primal" => {
                    let migrator = PrimalMigrator::new();
                    migrator.migrate(&dir, dry_run).await?;
                }
                "url" => {
                    let migrator = UrlMigrator::new();
                    migrator.migrate(&dir, dry_run).await?;
                }
                _ => {
                    eprintln!("❌ Unknown migration type: {}", migration_type);
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate { dir } => {
            println!("✅ Validating zero-hardcoding compliance in: {}", dir.display());
            let validator = HardcodingValidator::new();
            let compliance = validator.validate(&dir).await?;
            
            if compliance.is_compliant {
                println!("🎉 COMPLIANCE ACHIEVED: Zero hardcoding detected!");
                println!("📊 Compliance Score: {:.1}%", compliance.score);
            } else {
                println!("⚠️ COMPLIANCE ISSUES FOUND:");
                for violation in &compliance.violations {
                    println!("   {} ({}:{})", violation.description, violation.file, violation.line);
                }
                println!("📊 Compliance Score: {:.1}%", compliance.score);
                std::process::exit(1);
            }
        }
        Commands::Report { dir, output } => {
            println!("📋 Generating hardcoding elimination report...");
            generate_report(&dir, &output).await?;
            println!("✅ Report generated: {}", output.display());
        }
    }
    
    Ok(())
}

/// Scan directory for hardcoding patterns
async fn scan_hardcoding(dir: &Path) -> Result<HardcodingScanResults, Box<dyn std::error::Error>> {
    let patterns = HardcodingPatterns::default();
    let mut results = HardcodingScanResults {
        vendor_hardcoding: Vec::new(),
        primal_hardcoding: Vec::new(),
        url_hardcoding: Vec::new(),
        summary: HardcodingSummary {
            total_files_scanned: 0,
            files_with_hardcoding: 0,
            total_instances: 0,
            critical_instances: 0,
            high_priority_instances: 0,
            compliance_score: 0.0,
        },
    };
    
    let mut files_with_hardcoding = std::collections::HashSet::new();
    
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        
        // Only scan Rust files
        if !path.extension().map_or(false, |ext| ext == "rs") {
            continue;
        }
        
        // Skip target directories and archived files
        if path.to_string_lossy().contains("/target/") || 
           path.to_string_lossy().contains("/archive/") ||
           path.file_name().map_or(false, |name| name.to_string_lossy().ends_with(".disabled")) {
            continue;
        }
        
        results.summary.total_files_scanned += 1;
        
        let content = fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // Scan for vendor hardcoding
        for pattern in &patterns.vendor_patterns {
            for (line_num, line) in lines.iter().enumerate() {
                if pattern.regex.is_match(line) {
                    files_with_hardcoding.insert(path.to_path_buf());
                    results.vendor_hardcoding.push(HardcodingInstance {
                        file_path: path.to_path_buf(),
                        line_number: line_num + 1,
                        line_content: line.to_string(),
                        hardcoding_type: pattern.hardcoding_type.clone(),
                        pattern_matched: pattern.name.clone(),
                        suggested_replacement: pattern.replacement_template.clone(),
                        priority: pattern.priority.clone(),
                    });
                    
                    if pattern.priority == Priority::Critical {
                        results.summary.critical_instances += 1;
                    } else if pattern.priority == Priority::High {
                        results.summary.high_priority_instances += 1;
                    }
                }
            }
        }
        
        // Scan for primal hardcoding
        for pattern in &patterns.primal_patterns {
            for (line_num, line) in lines.iter().enumerate() {
                if pattern.regex.is_match(line) {
                    files_with_hardcoding.insert(path.to_path_buf());
                    results.primal_hardcoding.push(HardcodingInstance {
                        file_path: path.to_path_buf(),
                        line_number: line_num + 1,
                        line_content: line.to_string(),
                        hardcoding_type: pattern.hardcoding_type.clone(),
                        pattern_matched: pattern.name.clone(),
                        suggested_replacement: pattern.replacement_template.clone(),
                        priority: pattern.priority.clone(),
                    });
                    
                    if pattern.priority == Priority::Critical {
                        results.summary.critical_instances += 1;
                    } else if pattern.priority == Priority::High {
                        results.summary.high_priority_instances += 1;
                    }
                }
            }
        }
        
        // Scan for URL hardcoding
        for pattern in &patterns.url_patterns {
            for (line_num, line) in lines.iter().enumerate() {
                if pattern.regex.is_match(line) {
                    files_with_hardcoding.insert(path.to_path_buf());
                    results.url_hardcoding.push(HardcodingInstance {
                        file_path: path.to_path_buf(),
                        line_number: line_num + 1,
                        line_content: line.to_string(),
                        hardcoding_type: pattern.hardcoding_type.clone(),
                        pattern_matched: pattern.name.clone(),
                        suggested_replacement: pattern.replacement_template.clone(),
                        priority: pattern.priority.clone(),
                    });
                    
                    if pattern.priority == Priority::Critical {
                        results.summary.critical_instances += 1;
                    } else if pattern.priority == Priority::High {
                        results.summary.high_priority_instances += 1;
                    }
                }
            }
        }
    }
    
    results.summary.files_with_hardcoding = files_with_hardcoding.len();
    results.summary.total_instances = results.vendor_hardcoding.len() + 
                                     results.primal_hardcoding.len() + 
                                     results.url_hardcoding.len();
    
    // Calculate compliance score (100% = zero hardcoding)
    results.summary.compliance_score = if results.summary.total_files_scanned > 0 {
        let clean_files = results.summary.total_files_scanned - results.summary.files_with_hardcoding;
        (clean_files as f64 / results.summary.total_files_scanned as f64) * 100.0
    } else {
        100.0
    };
    
    Ok(results)
}

/// Print scan results in human-readable format
fn print_scan_results(results: &HardcodingScanResults) {
    println!("\n🔍 HARDCODING SCAN RESULTS");
    println!("==========================");
    
    println!("\n📊 SUMMARY:");
    println!("   Files Scanned: {}", results.summary.total_files_scanned);
    println!("   Files with Hardcoding: {}", results.summary.files_with_hardcoding);
    println!("   Total Instances: {}", results.summary.total_instances);
    println!("   Critical Issues: {}", results.summary.critical_instances);
    println!("   High Priority Issues: {}", results.summary.high_priority_instances);
    println!("   Compliance Score: {:.1}%", results.summary.compliance_score);
    
    if results.summary.compliance_score < 100.0 {
        println!("\n🚨 CRITICAL PRIMAL HARDCODING (violates sovereignty):");
        for instance in &results.primal_hardcoding {
            if instance.priority == Priority::Critical {
                println!("   {}:{} - {}", 
                        instance.file_path.display(), 
                        instance.line_number, 
                        instance.pattern_matched);
                println!("     Line: {}", instance.line_content.trim());
                println!("     Fix:  {}", instance.suggested_replacement);
            }
        }
        
        println!("\n🏢 VENDOR HARDCODING:");
        for instance in &results.vendor_hardcoding {
            if instance.priority == Priority::High {
                println!("   {}:{} - {}", 
                        instance.file_path.display(), 
                        instance.line_number, 
                        instance.pattern_matched);
            }
        }
        
        println!("\n🌐 URL HARDCODING:");
        println!("   Found {} hardcoded URLs/endpoints", results.url_hardcoding.len());
    } else {
        println!("\n🎉 ZERO HARDCODING ACHIEVED! Perfect sovereignty compliance!");
    }
}

/// Generate detailed hardcoding elimination report
async fn generate_report(dir: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let results = scan_hardcoding(dir).await?;
    
    let report = format!(r#"# 🔥 Hardcoding Elimination Report

**Generated**: {}
**Directory**: {}
**Compliance Score**: {:.1}%

## 📊 Summary

- **Files Scanned**: {}
- **Files with Hardcoding**: {}
- **Total Instances**: {}
- **Critical Issues**: {}
- **High Priority Issues**: {}

## 🚨 Critical Primal Hardcoding

These violations prevent true primal sovereignty:

{}

## 🏢 Vendor Hardcoding

These create vendor lock-in:

{}

## 🌐 URL/Endpoint Hardcoding

These prevent dynamic discovery:

{}

## 🛠️ Migration Recommendations

### Priority 1: Critical Primal Sovereignty
1. Replace all primal name references with capability discovery
2. Remove PrimalType enum usage
3. Implement zero-knowledge bootstrap

### Priority 2: Vendor Independence  
1. Replace cloud provider hardcoding with capability discovery
2. Remove service-specific integrations (K8s, Vault, etc.)
3. Implement universal adapter patterns

### Priority 3: Dynamic Discovery
1. Replace hardcoded URLs with endpoint discovery
2. Implement environment-agnostic configuration
3. Enable true zero-knowledge bootstrap

## 🎯 Next Steps

Run the migration tool:
```bash
# Migrate primal hardcoding (CRITICAL)
cargo run --bin hardcoding-eliminator migrate --type primal

# Migrate vendor hardcoding (HIGH)  
cargo run --bin hardcoding-eliminator migrate --type vendor

# Migrate URL hardcoding (MEDIUM)
cargo run --bin hardcoding-eliminator migrate --type url

# Validate compliance
cargo run --bin hardcoding-eliminator validate
```

**Target**: 100% compliance (zero hardcoding)
**Vision**: True primal sovereignty with infant-like learning
"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        dir.display(),
        results.summary.compliance_score,
        results.summary.total_files_scanned,
        results.summary.files_with_hardcoding,
        results.summary.total_instances,
        results.summary.critical_instances,
        results.summary.high_priority_instances,
        format_instances(&results.primal_hardcoding),
        format_instances(&results.vendor_hardcoding),
        format_instances(&results.url_hardcoding),
    );
    
    fs::write(output, report)?;
    Ok(())
}

/// Format hardcoding instances for report
fn format_instances(instances: &[HardcodingInstance]) -> String {
    if instances.is_empty() {
        return "✅ None found!".to_string();
    }
    
    instances.iter()
        .map(|instance| format!(
            "- `{}:{}` - **{}**\n  ```\n  {}\n  ```\n  **Suggested Fix**: `{}`\n",
            instance.file_path.display(),
            instance.line_number,
            instance.pattern_matched,
            instance.line_content.trim(),
            instance.suggested_replacement
        ))
        .collect::<Vec<_>>()
        .join("\n")
} 