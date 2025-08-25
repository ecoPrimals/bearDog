#!/usr/bin/env python3
"""
BearDog Security Module Pilot Migration

**PILOT IMPLEMENTATION** - First production migration to Result<T, SecurityError>

This script performs the pilot migration of the beardog-security module from 
BearDogResult<T> to SecurityResult<T> patterns, serving as the template for 
all future domain migrations.
"""

import os
import re
import shutil
import argparse
from pathlib import Path
from typing import List, Tuple, Dict

class SecurityPilotMigrator:
    """Specialized migrator for beardog-security pilot"""
    
    def __init__(self):
        # Security-specific function patterns for high-confidence migration
        self.security_function_patterns = [
            # Authentication functions
            (r'(fn\s+\w*auth\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            
            # Encryption functions
            (r'(fn\s+\w*encrypt\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            (r'(fn\s+\w*decrypt\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            
            # HSM functions
            (r'(fn\s+\w*hsm\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            
            # Key management functions
            (r'(fn\s+\w*key\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            
            # Security operations
            (r'(fn\s+\w*security\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
            
            # Crypto operations
            (r'(fn\s+\w*crypto\w*[^(]*\([^)]*\))\s*->\s*BearDogResult<([^>]+)>', 
             r'\1 -> SecurityResult<\2>'),
        ]
        
        # Import patterns to add
        self.required_imports = [
            'use beardog_errors::{SecurityError, SecurityResult, migrate_security_result};',
        ]
        
        # Error construction migrations
        self.error_migrations = [
            # Simple authentication errors
            (r'BearDogError::authentication\(([^)]+)\)', 
             r'SecurityError::AuthenticationFailed { reason: \1, user_id: "migrated".to_string(), context: create_migration_context(), metadata: create_default_security_metadata(), remediation: vec![], threat_assessment: create_default_threat_assessment(), metrics: create_default_auth_metrics() }'),
            
            # Simple encryption errors  
            (r'BearDogError::encryption\(([^,]+),\s*([^)]+)\)', 
             r'SecurityError::EncryptionFailed { operation: \1, reason: \2, context: create_migration_context(), crypto_metadata: create_default_crypto_metadata(), performance_impact: create_default_encryption_metrics() }'),
        ]
    
    def migrate_file(self, file_path: Path, dry_run: bool = True) -> Tuple[bool, List[str]]:
        """Migrate a single security file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return False, [f"Error reading file: {e}"]
        
        original_content = content
        transformations = []
        
        # 1. Add required imports
        content, import_changes = self._add_security_imports(content)
        transformations.extend(import_changes)
        
        # 2. Migrate function signatures
        content, signature_changes = self._migrate_function_signatures(content)
        transformations.extend(signature_changes)
        
        # 3. Migrate error constructions
        content, error_changes = self._migrate_error_constructions(content)
        transformations.extend(error_changes)
        
        # 4. Add migration helpers where needed
        content, helper_changes = self._add_migration_helpers(content)
        transformations.extend(helper_changes)
        
        # Save changes if not dry run
        if not dry_run and content != original_content:
            # Create backup
            backup_path = f"{file_path}.pre_idiomatic_migration"
            shutil.copy2(file_path, backup_path)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            transformations.append(f"Backup created: {backup_path}")
        
        return len(transformations) > 0, transformations
    
    def _add_security_imports(self, content: str) -> Tuple[str, List[str]]:
        """Add required imports for security migration"""
        changes = []
        lines = content.split('\n')
        
        # Check if we already have the imports
        has_security_imports = any('SecurityError' in line or 'SecurityResult' in line for line in lines)
        
        if not has_security_imports:
            # Find insertion point (after existing beardog_errors imports)
            insert_index = 0
            for i, line in enumerate(lines):
                if 'use beardog_errors::' in line:
                    insert_index = i + 1
                elif line.strip().startswith('use ') and '::' in line:
                    insert_index = i + 1
            
            # Insert the import
            for import_line in self.required_imports:
                lines.insert(insert_index, import_line)
                changes.append(f"Added import: {import_line}")
                insert_index += 1
            
            content = '\n'.join(lines)
        
        return content, changes
    
    def _migrate_function_signatures(self, content: str) -> Tuple[str, List[str]]:
        """Migrate function signatures from BearDogResult to SecurityResult"""
        changes = []
        
        for old_pattern, new_pattern in self.security_function_patterns:
            matches = re.findall(old_pattern, content)
            if matches:
                content = re.sub(old_pattern, new_pattern, content)
                changes.append(f"Migrated {len(matches)} function signature(s): {old_pattern[:30]}...")
        
        return content, changes
    
    def _migrate_error_constructions(self, content: str) -> Tuple[str, List[str]]:
        """Migrate error construction patterns"""
        changes = []
        
        for old_pattern, new_pattern in self.error_migrations:
            matches = re.findall(old_pattern, content)
            if matches:
                content = re.sub(old_pattern, new_pattern, content)
                changes.append(f"Migrated {len(matches)} error construction(s)")
        
        return content, changes
    
    def _add_migration_helpers(self, content: str) -> Tuple[str, List[str]]:
        """Add migration helper functions where needed"""
        changes = []
        
        # Add helper functions if we have complex error handling
        if 'BearDogResult' in content and 'SecurityResult' in content:
            helper_functions = '''
// Migration helper functions (temporary)
fn create_migration_context() -> beardog_errors::OperationContext {
    beardog_errors::OperationContext {
        operation_id: format!("migration-{}", chrono::Utc::now().timestamp()),
        started_at: chrono::Utc::now(),
        completed_at: chrono::Utc::now(),
        component: "beardog-security".to_string(),
        initiator: "migration".to_string(),
        request_id: None,
        metadata: std::collections::HashMap::new(),
    }
}

fn create_default_security_metadata() -> beardog_errors::SecurityMetadata {
    beardog_errors::SecurityMetadata {
        security_level: beardog_errors::SecurityLevel::Standard,
        compliance_context: beardog_errors::ComplianceContext {
            standards: vec!["basic".to_string()],
            compliance_level: "standard".to_string(),
        },
        audit_trail: vec![],
        failed_attempt_count: 0,
        lockout_remaining: None,
        additional_context: std::collections::HashMap::new(),
    }
}

fn create_default_threat_assessment() -> beardog_errors::ThreatAssessment {
    beardog_errors::ThreatAssessment {
        threat_level: beardog_errors::ThreatLevel::Low,
        suspicious_patterns: vec![],
        recommended_actions: vec![],
    }
}

fn create_default_auth_metrics() -> beardog_errors::AuthenticationMetrics {
    beardog_errors::AuthenticationMetrics {
        duration: std::time::Duration::from_millis(100),
        attempts_analyzed: 1,
        security_checks_performed: 1,
        hsm_operations: 0,
    }
}

fn create_default_crypto_metadata() -> beardog_errors::CryptographicMetadata {
    beardog_errors::CryptographicMetadata {
        algorithm: "AES-256-GCM".to_string(),
        key_size: 256,
    }
}

fn create_default_encryption_metrics() -> beardog_errors::EncryptionMetrics {
    beardog_errors::EncryptionMetrics {
        encryption_time: std::time::Duration::from_millis(10),
        data_size: 0,
    }
}
'''
            
            # Add at the end of the file
            if helper_functions.strip() not in content:
                content += helper_functions
                changes.append("Added migration helper functions")
        
        return content, changes
    
    def migrate_module(self, module_path: Path, dry_run: bool = True) -> Dict:
        """Migrate entire beardog-security module"""
        rust_files = [f for f in module_path.rglob("*.rs") if f.is_file()]
        
        print(f"🚀 Starting pilot migration: {module_path.name}")
        print(f"Files to process: {len(rust_files)}")
        print(f"Dry run: {'Yes' if dry_run else 'No (LIVE MIGRATION)'}")
        print("=" * 60)
        
        results = {
            'total_files': len(rust_files),
            'migrated_files': 0,
            'failed_files': 0,
            'transformations': [],
            'files_with_changes': [],
        }
        
        for file_path in rust_files:
            print(f"Processing: {file_path.name}...", end=" ")
            
            success, transformations = self.migrate_file(file_path, dry_run)
            
            if success:
                results['migrated_files'] += 1
                results['files_with_changes'].append(str(file_path))
                results['transformations'].extend([
                    f"{file_path.name}: {t}" for t in transformations
                ])
                print("✅")
            else:
                results['failed_files'] += 1
                print("⏭️ (no changes)")
        
        return results

def main():
    parser = argparse.ArgumentParser(description='Execute beardog-security pilot migration')
    parser.add_argument('--execute', action='store_true', help='Execute migration (default is dry run)')
    parser.add_argument('--module-path', default='crates/beardog-security', help='Path to beardog-security module')
    
    args = parser.parse_args()
    
    migrator = SecurityPilotMigrator()
    module_path = Path(args.module_path)
    
    if not module_path.exists():
        print(f"❌ Error: Module path {module_path} does not exist")
        return 1
    
    dry_run = not args.execute
    results = migrator.migrate_module(module_path, dry_run)
    
    print(f"\n📊 **PILOT MIGRATION SUMMARY**")
    print(f"Total files: {results['total_files']}")
    print(f"Files with changes: {results['migrated_files']}")
    print(f"Files unchanged: {results['failed_files']}")
    print(f"Success rate: {(results['migrated_files']/results['total_files'])*100:.1f}%")
    
    if results['transformations']:
        print(f"\n🔧 **TRANSFORMATIONS APPLIED**")
        for transformation in results['transformations'][:10]:  # Show first 10
            print(f"  • {transformation}")
        if len(results['transformations']) > 10:
            print(f"  ... and {len(results['transformations']) - 10} more")
    
    if not dry_run:
        print(f"\n⚠️  **POST-MIGRATION CHECKLIST**")
        print(f"1. ✅ Run: cargo check --manifest-path crates/beardog-security/Cargo.toml")
        print(f"2. ✅ Run: cargo test --manifest-path crates/beardog-security/Cargo.toml")
        print(f"3. ✅ Review changes in files with transformations")
        print(f"4. ✅ Update dependent modules if needed")
        print(f"5. ✅ Commit changes with descriptive message")
    else:
        print(f"\n🚀 **READY FOR EXECUTION**")
        print(f"Run with --execute flag to perform actual migration")
        print(f"Example: python3 scripts/security_pilot_migration.py --execute")
    
    return 0

if __name__ == "__main__":
    exit(main()) 