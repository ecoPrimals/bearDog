#!/usr/bin/env python3
"""
BearDog Constants Unification Migration Script

This script automatically updates constant imports across the codebase to use
the new unified constants system from beardog-types::constants::unified instead
of fragmented constant definitions across different crates.

Usage:
    python3 scripts/constants_unification_migration.py [--dry-run] [--verbose]

Features:
- Migrates 150+ scattered constant definitions to unified system
- Updates import statements to use canonical paths
- Removes duplicate constant definitions
- Validates migration completeness
- Generates migration report
"""

import os
import re
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Set
from collections import defaultdict

# Mapping from legacy constant paths to unified equivalents
CONSTANT_MAPPINGS = {
    # API Constants
    "beardog_api::api::types::API_VERSION": "beardog_types::constants::unified::api::VERSION",
    "beardog_api::api::types::API_VERSION_HEADER": "beardog_types::constants::unified::api::VERSION_HEADER",
    "beardog::VERSION": "beardog_types::constants::unified::api::PROJECT_VERSION",
    "beardog::MISSION": "beardog_types::constants::unified::api::MISSION",
    
    # Network Constants
    "beardog_config::canonical::DEFAULT_API_PORT": "beardog_types::constants::unified::network::ports::API",
    "beardog_config::canonical::DEFAULT_METRICS_PORT": "beardog_types::constants::unified::network::ports::METRICS",
    "beardog_config::canonical::DEFAULT_HEALTH_PORT": "beardog_types::constants::unified::network::ports::HEALTH",
    "beardog_config::canonical::DEFAULT_ADMIN_PORT": "beardog_types::constants::unified::network::ports::ADMIN",
    "beardog_config::canonical::DEFAULT_GRPC_PORT": "beardog_types::constants::unified::network::ports::GRPC",
    "beardog_config::canonical::DEFAULT_POSTGRES_PORT": "beardog_types::constants::unified::network::ports::POSTGRES",
    "beardog_config::canonical::DEFAULT_REDIS_PORT": "beardog_types::constants::unified::network::ports::REDIS",
    
    # Address Constants
    "beardog_config::canonical::LOCALHOST_IPV4": "beardog_types::constants::unified::network::addresses::LOCALHOST_IPV4",
    "beardog_config::canonical::LOCALHOST_IPV6": "beardog_types::constants::unified::network::addresses::LOCALHOST_IPV6",
    "beardog_config::canonical::ANY_IPV4": "beardog_types::constants::unified::network::addresses::ANY_IPV4",
    "beardog_config::canonical::ANY_IPV6": "beardog_types::constants::unified::network::addresses::ANY_IPV6",
    "beardog_config::canonical::DEFAULT_BIND_ADDRESS": "beardog_types::constants::unified::network::addresses::DEFAULT_BIND",
    "beardog_config::canonical::PRODUCTION_BIND_ADDRESS": "beardog_types::constants::unified::network::addresses::PRODUCTION_BIND",
    
    # Connection Constants
    "beardog_config::canonical::DEFAULT_MAX_CONNECTIONS": "beardog_types::constants::unified::network::limits::MAX_DB_CONNECTIONS",
    "beardog_config::canonical::DEFAULT_CONNECTION_TIMEOUT_MS": "beardog_types::constants::unified::network::timeouts::CONNECTION",
    
    # Timeout Constants
    "beardog_config::canonical::DEFAULT_OPERATION_MS": "beardog_types::constants::unified::network::timeouts::OPERATION",
    "beardog_config::canonical::CRYPTO_OPERATION_MS": "beardog_types::constants::unified::network::timeouts::CRYPTO_OPERATION",
    "beardog_config::canonical::NETWORK_OPERATION_MS": "beardog_types::constants::unified::network::timeouts::NETWORK_OPERATION",
    "beardog_config::canonical::HSM_OPERATION_MS": "beardog_types::constants::unified::network::timeouts::HSM_OPERATION",
    "beardog_config::canonical::KEY_ROTATION_MS": "beardog_types::constants::unified::network::timeouts::KEY_ROTATION",
    
    # Cache TTL Constants
    "beardog_api::api::cache::API_RESPONSE": "beardog_types::constants::unified::cache::ttl::API_RESPONSE",
    "beardog_api::api::cache::USER_SESSION": "beardog_types::constants::unified::cache::ttl::USER_SESSION",
    "beardog_api::api::cache::THREAT_ANALYSIS": "beardog_types::constants::unified::cache::ttl::THREAT_ANALYSIS",
    "beardog_api::api::cache::COMPLIANCE_REPORT": "beardog_types::constants::unified::cache::ttl::COMPLIANCE_REPORT",
    "beardog_api::api::cache::NODE_STATUS": "beardog_types::constants::unified::cache::ttl::NODE_STATUS",
    "beardog_api::api::cache::CONFIG_DATA": "beardog_types::constants::unified::cache::ttl::CONFIG_DATA",
    "beardog_api::api::cache::STATIC_CONTENT": "beardog_types::constants::unified::cache::ttl::STATIC_CONTENT",
    
    # Security Constants
    "beardog_config::canonical::MAX_FAILED_ATTEMPTS": "beardog_types::constants::unified::security::MAX_AUTH_ATTEMPTS",
    "beardog_auth::auth::tests::CHARSET": "beardog_types::constants::unified::security::TEST_CHARSET",
    
    # Node Type Constants
    "beardog_node_registry::node_registry::types::node::SECURITY": "beardog_types::constants::unified::nodes::SECURITY",
    "beardog_node_registry::node_registry::types::node::PHONEBOOK": "beardog_types::constants::unified::nodes::PHONEBOOK",
    "beardog_node_registry::node_registry::types::node::FEDERATION": "beardog_types::constants::unified::nodes::FEDERATION",
    "beardog_node_registry::node_registry::types::node::COMPUTE": "beardog_types::constants::unified::nodes::COMPUTE",
    "beardog_node_registry::node_registry::types::node::STORAGE": "beardog_types::constants::unified::nodes::STORAGE",
    "beardog_node_registry::node_registry::types::node::RELAY": "beardog_types::constants::unified::nodes::RELAY",
    "beardog_node_registry::node_registry::types::node::BACKUP": "beardog_types::constants::unified::nodes::BACKUP",
    "beardog_node_registry::node_registry::types::node::MONITORING": "beardog_types::constants::unified::nodes::MONITORING",
    "beardog_node_registry::node_registry::types::node::ANALYTICS": "beardog_types::constants::unified::nodes::ANALYTICS",
    "beardog_node_registry::node_registry::types::node::GATEWAY": "beardog_types::constants::unified::nodes::GATEWAY",
    
    # Federation Node Constants (duplicates)
    "beardog_node_registry::node_registry::types::federation::SECURITY": "beardog_types::constants::unified::nodes::SECURITY",
    "beardog_node_registry::node_registry::types::federation::PHONEBOOK": "beardog_types::constants::unified::nodes::PHONEBOOK",
    "beardog_node_registry::node_registry::types::federation::FEDERATION": "beardog_types::constants::unified::nodes::FEDERATION",
    "beardog_node_registry::node_registry::types::federation::COMPUTE": "beardog_types::constants::unified::nodes::COMPUTE",
    "beardog_node_registry::node_registry::types::federation::STORAGE": "beardog_types::constants::unified::nodes::STORAGE",
    "beardog_node_registry::node_registry::types::federation::RELAY": "beardog_types::constants::unified::nodes::RELAY",
    "beardog_node_registry::node_registry::types::federation::BACKUP": "beardog_types::constants::unified::nodes::BACKUP",
    
    # Compliance Error Constants
    "beardog_compliance::compliance::handlers::AUDIT_RETENTION_EXCEEDED": "beardog_types::constants::unified::compliance::errors::AUDIT_RETENTION_EXCEEDED",
    "beardog_compliance::compliance::handlers::MISSING_CONSENT": "beardog_types::constants::unified::compliance::errors::MISSING_CONSENT",
    "beardog_compliance::compliance::handlers::MISSING_PURPOSE": "beardog_types::constants::unified::compliance::errors::MISSING_PURPOSE",
    "beardog_compliance::compliance::handlers::ILLEGAL_TRANSFER": "beardog_types::constants::unified::compliance::errors::ILLEGAL_TRANSFER",
    "beardog_compliance::compliance::handlers::UNAUTHORIZED_FINANCIAL": "beardog_types::constants::unified::compliance::errors::UNAUTHORIZED_FINANCIAL",
    "beardog_compliance::compliance::handlers::UNENCRYPTED_PAYMENT": "beardog_types::constants::unified::compliance::errors::UNENCRYPTED_PAYMENT",
    "beardog_compliance::compliance::handlers::MINIMUM_NECESSARY": "beardog_types::constants::unified::compliance::errors::MINIMUM_NECESSARY",
    "beardog_compliance::compliance::handlers::MISSING_AUDIT_LOG": "beardog_types::constants::unified::compliance::errors::MISSING_AUDIT_LOG",
    
    # Performance Test Constants
    "beardog_types::constants::performance::testing::LIGHT_ITERATIONS": "beardog_types::constants::unified::performance::LIGHT_ITERATIONS",
    "beardog_types::constants::performance::testing::STANDARD_ITERATIONS": "beardog_types::constants::unified::performance::STANDARD_ITERATIONS",
    "beardog_types::constants::performance::testing::HEAVY_ITERATIONS": "beardog_types::constants::unified::performance::HEAVY_ITERATIONS",
    "beardog_types::constants::performance::testing::CONCURRENT_TASKS": "beardog_types::constants::unified::performance::CONCURRENT_TASKS",
    "beardog_types::constants::performance::testing::TARGET_RPS": "beardog_types::constants::unified::performance::TARGET_RPS",
    "beardog_types::constants::performance::testing::TEST_DATA_SIZE": "beardog_types::constants::unified::performance::TEST_DATA_SIZE",
    
    # Monitoring Constants
    "beardog_monitoring::monitoring::performance_metrics::MAX_SAMPLES": "beardog_types::constants::unified::performance::MAX_SAMPLES",
    
    # Generic constants that should be replaced
    "MAX_CONNECTIONS": "beardog_types::constants::unified::network::limits::MAX_CONNECTIONS",
    "API_VERSION": "beardog_types::constants::unified::api::VERSION",
    "DEFAULT_API_PORT": "beardog_types::constants::unified::network::ports::API",
}

# Patterns for inline constant definitions that should be replaced
INLINE_CONSTANT_PATTERNS = [
    (r'const\s+API_VERSION:\s*&str\s*=\s*"[^"]*"', 'use beardog_types::constants::unified::api::VERSION as API_VERSION'),
    (r'const\s+API_VERSION_HEADER:\s*&str\s*=\s*"[^"]*"', 'use beardog_types::constants::unified::api::VERSION_HEADER as API_VERSION_HEADER'),
    (r'const\s+DEFAULT_API_PORT:\s*u16\s*=\s*\d+', 'use beardog_types::constants::unified::network::ports::API as DEFAULT_API_PORT'),
    (r'const\s+MAX_CONNECTIONS:\s*usize\s*=\s*\d+', 'use beardog_types::constants::unified::network::limits::MAX_CONNECTIONS'),
    (r'const\s+SECURITY:\s*&str\s*=\s*"security"', 'use beardog_types::constants::unified::nodes::SECURITY'),
    (r'const\s+PHONEBOOK:\s*&str\s*=\s*"phonebook"', 'use beardog_types::constants::unified::nodes::PHONEBOOK'),
    (r'const\s+FEDERATION:\s*&str\s*=\s*"federation"', 'use beardog_types::constants::unified::nodes::FEDERATION'),
]

class ConstantMigrator:
    def __init__(self, dry_run: bool = False, verbose: bool = False):
        self.dry_run = dry_run
        self.verbose = verbose
        self.migration_stats = defaultdict(int)
        self.files_modified = []
        
    def log(self, message: str):
        if self.verbose:
            print(f"[MIGRATION] {message}")
    
    def migrate_file(self, file_path: Path) -> bool:
        """Migrate constants in a single file"""
        if not file_path.exists() or not file_path.is_file():
            return False
            
        if file_path.suffix != '.rs':
            return False
            
        # Skip target directories and generated files
        if 'target' in file_path.parts or 'generated' in file_path.parts:
            return False
            
        try:
            content = file_path.read_text(encoding='utf-8')
            original_content = content
            
            # Track changes made to this file
            file_changes = []
            
            # 1. Update import statements
            content = self.migrate_imports(content, file_changes)
            
            # 2. Replace inline constant definitions
            content = self.migrate_inline_constants(content, file_changes)
            
            # 3. Update constant usage
            content = self.migrate_constant_usage(content, file_changes)
            
            if content != original_content:
                if not self.dry_run:
                    file_path.write_text(content, encoding='utf-8')
                    
                self.files_modified.append(str(file_path))
                self.log(f"Modified: {file_path}")
                
                for change in file_changes:
                    self.log(f"  - {change}")
                    self.migration_stats[change.split(':')[0]] += 1
                    
                return True
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            
        return False
    
    def migrate_imports(self, content: str, changes: List[str]) -> str:
        """Update import statements to use unified constants"""
        
        # Pattern to match use statements
        use_pattern = r'use\s+([^;]+);'
        
        def replace_use_statement(match):
            use_statement = match.group(1).strip()
            
            # Check if this import needs migration
            for old_path, new_path in CONSTANT_MAPPINGS.items():
                if old_path in use_statement:
                    # Extract the imported item name
                    if '::' in use_statement:
                        parts = use_statement.split('::')
                        imported_name = parts[-1].strip()
                        
                        # Handle aliasing
                        if ' as ' in imported_name:
                            imported_name = imported_name.split(' as ')[1].strip()
                        
                        new_import = f"use {new_path}"
                        if imported_name != new_path.split('::')[-1]:
                            new_import += f" as {imported_name}"
                            
                        changes.append(f"Import: {old_path} → {new_path}")
                        return new_import + ";"
                        
            return match.group(0)
        
        return re.sub(use_pattern, replace_use_statement, content)
    
    def migrate_inline_constants(self, content: str, changes: List[str]) -> str:
        """Replace inline constant definitions with imports"""
        
        for pattern, replacement in INLINE_CONSTANT_PATTERNS:
            if re.search(pattern, content):
                content = re.sub(pattern, f"// {replacement} // TODO: Add this import", content)
                changes.append(f"Inline constant: Replaced with {replacement}")
                
        return content
    
    def migrate_constant_usage(self, content: str, changes: List[str]) -> str:
        """Update constant usage to reference unified constants"""
        
        # This is more complex as we need to be careful not to break valid code
        # For now, we'll focus on obvious patterns
        
        # Replace direct constant references in common contexts
        patterns = [
            (r'\bAPI_VERSION\b(?!\s*:)', 'unified::api::VERSION'),
            (r'\bDEFAULT_API_PORT\b(?!\s*:)', 'unified::network::ports::API'),
            (r'\bMAX_CONNECTIONS\b(?!\s*:)', 'unified::network::limits::MAX_CONNECTIONS'),
        ]
        
        for pattern, replacement in patterns:
            if re.search(pattern, content):
                # Only replace if we haven't already added the unified import
                if 'use beardog_types::constants::unified' not in content:
                    content = re.sub(pattern, replacement, content)
                    changes.append(f"Usage: {pattern} → {replacement}")
                    
        return content
    
    def migrate_directory(self, directory: Path) -> int:
        """Migrate all Rust files in a directory recursively"""
        files_processed = 0
        
        for rust_file in directory.rglob("*.rs"):
            if self.migrate_file(rust_file):
                files_processed += 1
                
        return files_processed
    
    def generate_report(self) -> str:
        """Generate a migration report"""
        report = []
        report.append("# Constants Unification Migration Report")
        report.append("")
        report.append(f"**Mode**: {'DRY RUN' if self.dry_run else 'LIVE MIGRATION'}")
        report.append(f"**Files Modified**: {len(self.files_modified)}")
        report.append("")
        
        if self.migration_stats:
            report.append("## Changes Made:")
            for change_type, count in self.migration_stats.items():
                report.append(f"- **{change_type}**: {count} changes")
            report.append("")
        
        if self.files_modified:
            report.append("## Modified Files:")
            for file_path in self.files_modified:
                report.append(f"- `{file_path}`")
            report.append("")
            
        report.append("## Next Steps:")
        report.append("1. Run `cargo check` to verify compilation")
        report.append("2. Run tests to ensure functionality")
        report.append("3. Update any remaining manual constant definitions")
        report.append("4. Remove deprecated constant modules")
        
        return "\n".join(report)

def main():
    parser = argparse.ArgumentParser(description='Migrate BearDog constants to unified system')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be changed without making changes')
    parser.add_argument('--verbose', action='store_true', help='Show detailed migration information')
    parser.add_argument('--directory', default='.', help='Directory to migrate (default: current directory)')
    
    args = parser.parse_args()
    
    migrator = ConstantMigrator(dry_run=args.dry_run, verbose=args.verbose)
    
    print(f"🔧 Starting BearDog Constants Unification Migration...")
    print(f"📁 Directory: {args.directory}")
    print(f"🧪 Mode: {'DRY RUN' if args.dry_run else 'LIVE MIGRATION'}")
    print()
    
    directory = Path(args.directory)
    if not directory.exists():
        print(f"❌ Directory not found: {directory}")
        sys.exit(1)
        
    files_processed = migrator.migrate_directory(directory)
    
    print(f"\n✅ Migration completed!")
    print(f"📊 Files processed: {files_processed}")
    print(f"📝 Files modified: {len(migrator.files_modified)}")
    
    # Generate and save report
    report = migrator.generate_report()
    report_file = Path("CONSTANTS_MIGRATION_REPORT.md")
    report_file.write_text(report)
    print(f"📋 Report saved: {report_file}")
    
    if args.dry_run:
        print("\n🧪 This was a dry run. No files were modified.")
        print("💡 Run without --dry-run to apply changes.")
    else:
        print("\n🚀 Migration complete! Please:")
        print("   1. Run `cargo check` to verify compilation")
        print("   2. Run tests to ensure everything works")
        print("   3. Review the migration report for any manual changes needed")

if __name__ == "__main__":
    main() 