#!/usr/bin/env python3
"""
BearDog Idiomatic Migration Executor

**WEEK 2 IMPLEMENTATION** - Automated migration execution

This script performs the actual migration from BearDogResult<T> to Result<T, E> patterns
based on the analysis reports and migration confidence levels.
"""

import os
import re
import json
import shutil
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass

@dataclass
class MigrationTransformation:
    """Single migration transformation"""
    file_path: str
    line_number: int
    old_pattern: str
    new_pattern: str
    domain_type: str
    confidence: str

class IdiomaticMigrationExecutor:
    """Executes idiomatic migrations based on analysis reports"""
    
    def __init__(self):
        # Migration patterns for different domains
        self.migration_patterns = {
            'security': {
                'import_additions': [
                    'use beardog_errors::{SecurityError, SecurityResult};',
                ],
                'function_signatures': [
                    (r'fn\s+(\w*auth\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> SecurityResult<\3>'),
                    (r'fn\s+(\w*encrypt\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> SecurityResult<\3>'),
                    (r'fn\s+(\w*hsm\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> SecurityResult<\3>'),
                ],
                'error_construction': [
                    (r'BearDogError::authentication\(([^)]+)\)', 
                     r'SecurityError::AuthenticationFailed { reason: \1, user_id: "unknown".to_string(), context: create_default_context(), metadata: create_default_security_metadata(), remediation: vec![], threat_assessment: create_default_threat_assessment(), metrics: create_default_auth_metrics() }'),
                ],
            },
            'genetics': {
                'import_additions': [
                    'use beardog_errors::{GeneticsError, GeneticsResult};',
                ],
                'function_signatures': [
                    (r'fn\s+(\w*spawn\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> GeneticsResult<\3>'),
                    (r'fn\s+(\w*genetic\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> GeneticsResult<\3>'),
                ],
            },
            'network': {
                'import_additions': [
                    'use beardog_errors::{NetworkError, NetworkResult};',
                ],
                'function_signatures': [
                    (r'fn\s+(\w*connect\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> NetworkResult<\3>'),
                ],
            },
            'workflow': {
                'import_additions': [
                    'use beardog_errors::{WorkflowError, WorkflowResult};',
                ],
                'function_signatures': [
                    (r'fn\s+(\w*workflow\w*[^(]*)\([^)]*\)\s*->\s*BearDogResult<([^>]+)>', 
                     r'fn \1(\2) -> WorkflowResult<\3>'),
                ],
            },
        }
    
    def execute_migration(self, report_path: Path, dry_run: bool = True) -> bool:
        """Execute migration based on analysis report"""
        try:
            with open(report_path, 'r') as f:
                report = json.load(f)
        except Exception as e:
            print(f"❌ Error loading report: {e}")
            return False
        
        print(f"🚀 Starting migration execution for: {report['module']}")
        print(f"Dry run: {'Yes' if dry_run else 'No (LIVE MIGRATION)'}")
        print("=" * 60)
        
        # Process high confidence files first
        high_conf_files = report['migration_plan']['high_confidence']
        medium_conf_files = report['migration_plan']['medium_confidence']
        
        success_count = 0
        total_files = len(high_conf_files) + len(medium_conf_files)
        
        # Migrate high confidence files
        print(f"\n🎯 **HIGH CONFIDENCE MIGRATIONS** ({len(high_conf_files)} files)")
        for file_path in high_conf_files:
            if self._migrate_file(file_path, report, dry_run, confidence='high'):
                success_count += 1
                print(f"  ✅ {Path(file_path).name}")
            else:
                print(f"  ❌ {Path(file_path).name}")
        
        # Migrate medium confidence files (with review prompts)
        print(f"\n🔍 **MEDIUM CONFIDENCE MIGRATIONS** ({len(medium_conf_files)} files)")
        for file_path in medium_conf_files:
            if self._migrate_file(file_path, report, dry_run, confidence='medium'):
                success_count += 1
                print(f"  ✅ {Path(file_path).name} (review recommended)")
            else:
                print(f"  ❌ {Path(file_path).name}")
        
        print(f"\n📊 **MIGRATION SUMMARY**")
        print(f"Successfully migrated: {success_count}/{total_files} files")
        print(f"Success rate: {(success_count/total_files)*100:.1f}%" if total_files > 0 else "N/A")
        
        if not dry_run:
            print(f"\n⚠️  **POST-MIGRATION ACTIONS REQUIRED**")
            print(f"1. Run `cargo check` to verify compilation")
            print(f"2. Run `cargo test` to verify functionality")
            print(f"3. Review medium confidence changes manually")
            print(f"4. Update imports in dependent modules")
        
        return success_count == total_files
    
    def _migrate_file(self, file_path: str, report: Dict, dry_run: bool, confidence: str) -> bool:
        """Migrate a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"    Error reading {file_path}: {e}")
            return False
        
        # Find file analysis in report
        file_analysis = None
        for f in report['detailed_analysis']:
            if f['file_path'] == file_path:
                file_analysis = f
                break
        
        if not file_analysis:
            print(f"    No analysis found for {file_path}")
            return False
        
        # Determine primary domain for this file
        primary_domain = file_analysis['suggested_domain']
        if primary_domain == 'unknown':
            print(f"    Unknown domain for {file_path}, skipping")
            return False
        
        # Apply transformations
        original_content = content
        transformations = []
        
        # Add imports if needed
        if primary_domain in self.migration_patterns:
            patterns = self.migration_patterns[primary_domain]
            
            # Add import statements
            for import_line in patterns['import_additions']:
                if import_line not in content and 'use beardog_errors::' not in content:
                    # Find good place to add import (after existing beardog_errors imports)
                    lines = content.split('\n')
                    insert_index = 0
                    
                    for i, line in enumerate(lines):
                        if 'use beardog_errors::' in line:
                            insert_index = i + 1
                            break
                        elif line.strip().startswith('use ') and '::' in line:
                            insert_index = i + 1
                    
                    lines.insert(insert_index, import_line)
                    content = '\n'.join(lines)
                    transformations.append(f"Added import: {import_line}")
            
            # Transform function signatures
            for old_pattern, new_pattern in patterns['function_signatures']:
                if re.search(old_pattern, content):
                    content = re.sub(old_pattern, new_pattern, content)
                    transformations.append(f"Updated function signature: {old_pattern[:50]}...")
        
        # Apply migration helper for error returns
        content = self._apply_error_migration_helpers(content, primary_domain)
        
        if transformations:
            transformations.append("Applied migration helpers for error returns")
        
        # Save changes (if not dry run)
        if not dry_run and content != original_content:
            # Create backup
            backup_path = f"{file_path}.backup"
            shutil.copy2(file_path, backup_path)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
        
        return len(transformations) > 0
    
    def _apply_error_migration_helpers(self, content: str, domain: str) -> str:
        """Apply migration helpers for error returns"""
        if domain == 'security':
            # Add migration helper usage for security functions
            content = re.sub(
                r'(\s+)(Err\(BearDogError::authentication\([^)]+\)\))',
                r'\1migrate_security_result(\2)',
                content
            )
        elif domain == 'genetics':
            # Add migration helper usage for genetics functions  
            content = re.sub(
                r'(\s+)(Err\(BearDogError::[^)]+genetics[^)]+\)\))',
                r'\1migrate_genetics_result(\2)',
                content
            )
        
        return content

def main():
    parser = argparse.ArgumentParser(description='Execute BearDog idiomatic migrations')
    parser.add_argument('report_path', help='Path to the migration analysis report')
    parser.add_argument('--execute', action='store_true', help='Execute migration (default is dry run)')
    parser.add_argument('--confidence', choices=['high', 'medium', 'all'], default='high',
                       help='Migration confidence level to execute')
    
    args = parser.parse_args()
    
    executor = IdiomaticMigrationExecutor()
    report_path = Path(args.report_path)
    
    if not report_path.exists():
        print(f"❌ Error: Report file {report_path} does not exist")
        return 1
    
    dry_run = not args.execute
    success = executor.execute_migration(report_path, dry_run)
    
    return 0 if success else 1

if __name__ == "__main__":
    exit(main()) 