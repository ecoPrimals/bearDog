#!/usr/bin/env python3
"""
BearDog Type Canonicalization Script

This script automatically migrates duplicate type definitions to use canonical types
from beardog-types::canonical, eliminating type fragmentation across the codebase.

Usage:
    python3 scripts/canonicalize_types.py --dry-run
    python3 scripts/canonicalize_types.py --apply
"""

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
import argparse

# Canonical type mappings
CANONICAL_MAPPINGS = {
    # Health Status Types
    'HealthStatus': 'beardog_types::canonical::HealthStatus',
    'ComponentStatus': 'beardog_types::canonical::ComponentStatus', 
    'OperationStatus': 'beardog_types::canonical::OperationStatus',
    'WorkflowStatus': 'beardog_types::canonical::WorkflowStatus',
    'SystemHealthStatus': 'beardog_types::canonical::HealthStatus',
    'ComponentHealthStatus': 'beardog_types::canonical::HealthStatus',
    'HsmHealthStatus': 'beardog_types::canonical::HealthStatus',
    'ServiceHealthStatus': 'beardog_types::canonical::HealthStatus',
    
    # Configuration Types
    'SessionConfig': 'beardog_types::canonical::SessionConfig',
    'RateLimitConfig': 'beardog_types::canonical::RateLimitConfig',
    'AuthenticationConfig': 'beardog_types::canonical::AuthenticationConfig',
    'EncryptionConfig': 'beardog_types::canonical::EncryptionConfig',
    'ProviderConfig': 'beardog_types::canonical::ProviderConfig',
    'PasswordPolicyConfig': 'beardog_types::canonical::PasswordPolicyConfig',
    'MfaConfig': 'beardog_types::canonical::MfaConfig',
    
    # Security Types
    'SecurityContext': 'beardog_types::canonical::SecurityContext',
    'SecurityAuditEvent': 'beardog_types::canonical::SecurityAuditEvent',
    'PolicyDecision': 'beardog_types::canonical::PolicyDecision',
    'AuthorizationLevel': 'beardog_types::canonical::AuthorizationLevel',
    'SecurityFlags': 'beardog_types::canonical::SecurityFlags',
    'RiskLevel': 'beardog_types::canonical::RiskLevel',
    
    # Key & Crypto Types
    'KeyStatus': 'beardog_types::canonical::KeyStatus',
    'KeyType': 'beardog_types::canonical::KeyType',
    
    # Audit Types
    'AuditEventType': 'beardog_types::canonical::AuditEventType',
    
    # Provider Types
    'ProviderHealthStatus': 'beardog_types::canonical::ProviderHealthStatus',
    
    # Workflow Types
    'WorkflowType': 'beardog_types::canonical::WorkflowType',
    'WorkflowExecutionState': 'beardog_types::canonical::WorkflowExecutionState',
}

# Files to exclude from canonicalization
EXCLUDE_FILES = {
    'beardog-types/src/canonical.rs',
    'beardog-types/src/lib.rs',
    'scripts/canonicalize_types.py',
}

class TypeCanonicalizer:
    def __init__(self, root_path: Path, dry_run: bool = True):
        self.root_path = root_path
        self.dry_run = dry_run
        self.changes = []
        self.errors = []
        
    def find_rust_files(self) -> List[Path]:
        """Find all Rust source files in the project."""
        rust_files = []
        for path in self.root_path.rglob("*.rs"):
            # Skip excluded files
            relative_path = path.relative_to(self.root_path)
            if any(exclude in str(relative_path) for exclude in EXCLUDE_FILES):
                continue
            rust_files.append(path)
        return rust_files
    
    def analyze_file(self, file_path: Path) -> List[Tuple[str, str, int]]:
        """Analyze a file for duplicate type usage."""
        duplicates = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                
            for line_num, line in enumerate(lines, 1):
                for duplicate_type, canonical_type in CANONICAL_MAPPINGS.items():
                    # Look for type usage patterns
                    patterns = [
                        rf'\b{duplicate_type}\b',  # Direct type usage
                        rf'pub\s+enum\s+{duplicate_type}\b',  # Enum definition
                        rf'pub\s+struct\s+{duplicate_type}\b',  # Struct definition
                        rf'use\s+.*::{duplicate_type}\b',  # Import
                        rf'{duplicate_type}::\w+',  # Associated items
                    ]
                    
                    for pattern in patterns:
                        if re.search(pattern, line):
                            duplicates.append((duplicate_type, canonical_type, line_num))
                            
        except Exception as e:
            self.errors.append(f"Error analyzing {file_path}: {e}")
            
        return duplicates
    
    def migrate_file(self, file_path: Path) -> bool:
        """Migrate a file to use canonical types."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            original_content = content
            changes_made = False
            
            for duplicate_type, canonical_type in CANONICAL_MAPPINGS.items():
                # Replace type definitions with re-exports
                enum_pattern = rf'pub\s+enum\s+{duplicate_type}\s*\{{[^}}]*\}}'
                struct_pattern = rf'pub\s+struct\s+{duplicate_type}\s*\{{[^}}]*\}}'
                
                if re.search(enum_pattern, content, re.DOTALL):
                    content = re.sub(
                        enum_pattern,
                        f'pub use {canonical_type};',
                        content,
                        flags=re.DOTALL
                    )
                    changes_made = True
                    
                if re.search(struct_pattern, content, re.DOTALL):
                    content = re.sub(
                        struct_pattern, 
                        f'pub use {canonical_type};',
                        content,
                        flags=re.DOTALL
                    )
                    changes_made = True
                
                # Replace direct type usage (be careful not to replace inside strings)
                usage_pattern = rf'\b{duplicate_type}\b(?!["\'])'
                if re.search(usage_pattern, content):
                    # Only replace if not already using canonical import
                    if f'use {canonical_type}' not in content:
                        # Add canonical import at the top
                        import_line = f'use {canonical_type};\n'
                        if 'use ' in content:
                            content = re.sub(
                                r'(use [^;]+;)', 
                                rf'\1\n{import_line}',
                                content,
                                count=1
                            )
                        else:
                            content = import_line + content
                        changes_made = True
            
            if changes_made:
                if not self.dry_run:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                        
                self.changes.append({
                    'file': str(file_path.relative_to(self.root_path)),
                    'changes': self._count_changes(original_content, content)
                })
                
            return changes_made
            
        except Exception as e:
            self.errors.append(f"Error migrating {file_path}: {e}")
            return False
    
    def _count_changes(self, original: str, modified: str) -> int:
        """Count the number of changes made."""
        original_lines = original.split('\n')
        modified_lines = modified.split('\n')
        
        changes = 0
        for orig, mod in zip(original_lines, modified_lines):
            if orig != mod:
                changes += 1
                
        return changes
    
    def run_analysis(self) -> Dict:
        """Run complete analysis of type duplications."""
        print("🔍 Analyzing BearDog codebase for type duplications...")
        
        rust_files = self.find_rust_files()
        print(f"Found {len(rust_files)} Rust files to analyze")
        
        all_duplicates = {}
        total_duplicates = 0
        
        for file_path in rust_files:
            duplicates = self.analyze_file(file_path)
            if duplicates:
                relative_path = str(file_path.relative_to(self.root_path))
                all_duplicates[relative_path] = duplicates
                total_duplicates += len(duplicates)
        
        return {
            'files_analyzed': len(rust_files),
            'files_with_duplicates': len(all_duplicates),
            'total_duplicates': total_duplicates,
            'duplicates_by_file': all_duplicates,
            'errors': self.errors
        }
    
    def run_migration(self) -> Dict:
        """Run complete migration to canonical types."""
        action = "🧪 DRY RUN" if self.dry_run else "🔧 APPLYING"
        print(f"{action}: Migrating to canonical types...")
        
        rust_files = self.find_rust_files()
        print(f"Processing {len(rust_files)} Rust files")
        
        migrated_files = 0
        for file_path in rust_files:
            if self.migrate_file(file_path):
                migrated_files += 1
                
        return {
            'files_processed': len(rust_files),
            'files_migrated': migrated_files,
            'changes': self.changes,
            'errors': self.errors
        }

def print_analysis_report(results: Dict):
    """Print analysis report."""
    print("\n" + "="*60)
    print("📊 TYPE DUPLICATION ANALYSIS REPORT")
    print("="*60)
    
    print(f"Files analyzed: {results['files_analyzed']}")
    print(f"Files with duplications: {results['files_with_duplicates']}")
    print(f"Total duplications found: {results['total_duplicates']}")
    
    if results['duplicates_by_file']:
        print("\n🔍 DUPLICATIONS BY FILE:")
        for file_path, duplicates in results['duplicates_by_file'].items():
            print(f"\n📁 {file_path}")
            for duplicate_type, canonical_type, line_num in duplicates:
                print(f"   Line {line_num}: {duplicate_type} → {canonical_type}")
    
    if results['errors']:
        print("\n❌ ERRORS:")
        for error in results['errors']:
            print(f"   {error}")

def print_migration_report(results: Dict):
    """Print migration report."""
    print("\n" + "="*60)
    print("🔧 TYPE MIGRATION REPORT")
    print("="*60)
    
    print(f"Files processed: {results['files_processed']}")
    print(f"Files migrated: {results['files_migrated']}")
    
    if results['changes']:
        print(f"\n✅ CHANGES MADE ({len(results['changes'])} files):")
        for change in results['changes']:
            print(f"   📁 {change['file']}: {change['changes']} changes")
    
    if results['errors']:
        print("\n❌ ERRORS:")
        for error in results['errors']:
            print(f"   {error}")

def main():
    parser = argparse.ArgumentParser(description='Canonicalize BearDog types')
    parser.add_argument('--dry-run', action='store_true', default=True,
                       help='Analyze without making changes (default)')
    parser.add_argument('--apply', action='store_true',
                       help='Apply changes to files')
    parser.add_argument('--analyze-only', action='store_true',
                       help='Only run analysis, no migration')
    
    args = parser.parse_args()
    
    # Determine mode
    dry_run = not args.apply
    
    # Find project root
    script_path = Path(__file__).parent
    root_path = script_path.parent
    
    print(f"🚀 BearDog Type Canonicalization")
    print(f"Root path: {root_path}")
    print(f"Mode: {'DRY RUN' if dry_run else 'APPLY CHANGES'}")
    
    canonicalizer = TypeCanonicalizer(root_path, dry_run)
    
    if args.analyze_only:
        # Run analysis only
        results = canonicalizer.run_analysis()
        print_analysis_report(results)
    else:
        # Run migration (with or without dry run)
        results = canonicalizer.run_migration()
        print_migration_report(results)
    
    if results.get('errors'):
        sys.exit(1)
    
    print(f"\n✅ Canonicalization {'analysis' if args.analyze_only else 'migration'} completed successfully!")

if __name__ == '__main__':
    main() 