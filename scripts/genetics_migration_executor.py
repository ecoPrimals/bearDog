#!/usr/bin/env python3
"""
Enhanced Genetics Module Migration Executor
Executes genetics module migration with improved patterns and validation
"""

import re
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Set

class GeneticsMigrationExecutor:
    def __init__(self):
        self.module_path = Path("crates/beardog-genetics")
        self.genetics_patterns = self._initialize_genetics_patterns()
        self.migration_stats = {
            'files_processed': 0,
            'functions_migrated': 0,
            'error_constructions_updated': 0,
            'imports_added': 0
        }

    def _initialize_genetics_patterns(self) -> List[Dict]:
        """Initialize enhanced genetics migration patterns"""
        return [
            {
                'pattern': r'fn\s+(\w*spawn\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'spawning operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*lineage\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'lineage operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*genetic\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'genetic operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*breed\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'breeding operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*diversity\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'diversity operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*evolution\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'evolution operations',
                'domain': 'genetics'
            },
            {
                'pattern': r'fn\s+(\w*mutation\w*)[^{]*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'mutation operations',
                'domain': 'genetics'
            },
        ]

    def execute_migration(self, dry_run: bool = False) -> Dict:
        """Execute the enhanced genetics migration"""
        print("🧬 **ENHANCED GENETICS MODULE MIGRATION**")
        print("=" * 60)
        
        if not self.module_path.exists():
            raise FileNotFoundError(f"Module path {self.module_path} not found")
        
        # Find all Rust files in the genetics module
        rust_files = list(self.module_path.rglob("*.rs"))
        print(f"📁 Found {len(rust_files)} Rust files in genetics module")
        
        results = {
            'files_processed': 0,
            'functions_migrated': 0,
            'successful_migrations': 0,
            'failed_migrations': 0,
            'manual_review_files': [],
            'migration_details': []
        }
        
        for file_path in rust_files:
            try:
                file_result = self._migrate_file(file_path, dry_run)
                results['files_processed'] += 1
                results['migration_details'].append(file_result)
                
                if file_result['status'] == 'success':
                    results['successful_migrations'] += 1
                    results['functions_migrated'] += file_result['functions_migrated']
                elif file_result['status'] == 'manual_review':
                    results['manual_review_files'].append(str(file_path))
                else:
                    results['failed_migrations'] += 1
                    
            except Exception as e:
                print(f"❌ Error processing {file_path}: {e}")
                results['failed_migrations'] += 1
        
        self._generate_summary_report(results)
        return results

    def _migrate_file(self, file_path: Path, dry_run: bool) -> Dict:
        """Migrate a single file with enhanced patterns"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return {
                'file': str(file_path),
                'status': 'error',
                'error': str(e),
                'functions_migrated': 0
            }
        
        original_content = content
        changes_made = 0
        functions_migrated = 0
        
        # Add genetics imports if BearDogResult is used
        if 'BearDogResult' in content and self._needs_genetics_imports(content):
            import_additions = self._add_genetics_imports(content)
            content = import_additions['content']
            changes_made += import_additions['changes']
        
        # Apply genetics-specific function signature migrations
        for pattern_info in self.genetics_patterns:
            matches = list(re.finditer(pattern_info['pattern'], content, re.MULTILINE))
            if matches:
                for match in matches:
                    # Extract function signature details
                    function_name = match.group(1) if match.groups() else "unknown"
                    return_type = match.group(2) if len(match.groups()) > 1 else "T"
                    
                    # Create the replacement
                    old_signature = match.group(0)
                    new_signature = self._create_genetics_signature(old_signature, return_type)
                    
                    if dry_run:
                        print(f"  📋 DRY RUN: Would migrate {function_name} in {file_path.name}")
                    else:
                        content = content.replace(old_signature, new_signature)
                        changes_made += 1
                        functions_migrated += 1
                        print(f"  🧬 Migrated {function_name} to GeneticsResult<{return_type}>")
        
        # Update error constructions
        error_updates = self._update_error_constructions(content, dry_run)
        content = error_updates['content']
        changes_made += error_updates['changes']
        
        # Determine migration status
        status = 'success' if changes_made > 0 else 'no_changes'
        
        # Check for manual review requirements
        if self._requires_manual_review(content):
            status = 'manual_review'
        
        # Save changes if not dry run
        if not dry_run and changes_made > 0:
            try:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
            except Exception as e:
                return {
                    'file': str(file_path),
                    'status': 'error',
                    'error': f"Failed to write file: {e}",
                    'functions_migrated': functions_migrated
                }
        
        return {
            'file': str(file_path),
            'status': status,
            'functions_migrated': functions_migrated,
            'changes': changes_made,
            'original_size': len(original_content),
            'new_size': len(content)
        }

    def _needs_genetics_imports(self, content: str) -> bool:
        """Check if file needs genetics imports"""
        return ('BearDogResult' in content and 
                'use beardog_errors::idiomatic' not in content and
                any(keyword in content.lower() for keyword in ['spawn', 'genetic', 'lineage', 'breed']))

    def _add_genetics_imports(self, content: str) -> Dict:
        """Add necessary genetics imports"""
        import_lines = [
            "use beardog_errors::idiomatic::{GeneticsResult, GeneticsError, migrate_genetics_result};",
            "use beardog_errors::idiomatic::enhanced_context::{LineageMetadata, GeneticsMetadata};"
        ]
        
        # Find the best place to insert imports
        lines = content.split('\n')
        insert_index = 0
        
        # Find existing use statements
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_index = i + 1
        
        # Insert imports
        for import_line in reversed(import_lines):
            if import_line not in content:
                lines.insert(insert_index, import_line)
        
        return {
            'content': '\n'.join(lines),
            'changes': len(import_lines)
        }

    def _create_genetics_signature(self, old_signature: str, return_type: str) -> str:
        """Create new genetics function signature"""
        # Extract function name and parameters
        fn_match = re.search(r'fn\s+(\w+)\s*\(([^)]*)\)', old_signature)
        if not fn_match:
            return old_signature
        
        fn_name = fn_match.group(1)
        params = fn_match.group(2)
        
        # Create new signature with GeneticsResult
        new_signature = f"fn {fn_name}({params}) -> GeneticsResult<{return_type}>"
        
        return new_signature

    def _update_error_constructions(self, content: str, dry_run: bool) -> Dict:
        """Update error construction sites to genetics-specific errors"""
        changes = 0
        
        # Enhanced error construction patterns
        error_patterns = [
            # Basic genetics errors
            (
                r'BearDogError::Genetics\(GeneticsError::SpawningError\s*\{\s*message:\s*([^}]+)\s*\}\)',
                r'GeneticsError::SpawningFailed { reason: \1, context: create_genetics_context(), metadata: LineageMetadata::default(), improvement: None }'
            ),
            (
                r'BearDogError::Genetics\(GeneticsError::InvalidLineage\s*\{\s*message:\s*([^}]+)\s*\}\)',
                r'GeneticsError::InvalidLineage { lineage_id: "unknown".to_string(), reason: \1, context: create_genetics_context(), metadata: LineageMetadata::default(), improvement: None }'
            ),
            (
                r'BearDogError::Genetics\(GeneticsError::InsufficientDiversity\s*\{\s*message:\s*([^}]+)\s*\}\)',
                r'GeneticsError::InsufficientDiversity { required_diversity: 0.6, actual_diversity: 0.0, context: create_genetics_context(), metadata: LineageMetadata::default(), improvement: None }'
            ),
            # Generic BearDogError patterns in genetics context
            (
                r'BearDogError::internal\(([^)]+)\)',
                r'GeneticsError::InternalError { reason: \1, context: create_genetics_context(), metadata: GeneticsMetadata::default(), improvement: None }'
            ),
        ]
        
        for pattern, replacement in error_patterns:
            matches = re.findall(pattern, content)
            if matches:
                if not dry_run:
                    content = re.sub(pattern, replacement, content)
                changes += len(matches)
        
        return {'content': content, 'changes': changes}

    def _requires_manual_review(self, content: str) -> bool:
        """Check if file requires manual review"""
        manual_review_indicators = [
            r'impl.*From.*BearDogError',
            r'match.*BearDogError::',
            r'\.map_err\(.*BearDogError',
            r'unsafe\s+',
            r'BearDogResult.*BearDogResult',  # Complex nested results
            r'async.*BearDogResult',  # Async functions need special handling
        ]
        
        for indicator in manual_review_indicators:
            if re.search(indicator, content):
                return True
        
        return False

    def _generate_summary_report(self, results: Dict):
        """Generate enhanced migration summary report"""
        print("\n🧬 **GENETICS MODULE MIGRATION SUMMARY**")
        print("=" * 50)
        print(f"Files Processed: {results['files_processed']}")
        print(f"Functions Migrated: {results['functions_migrated']}")
        print(f"Successful Migrations: {results['successful_migrations']}")
        print(f"Failed Migrations: {results['failed_migrations']}")
        print(f"Manual Review Required: {len(results['manual_review_files'])}")
        
        if results['manual_review_files']:
            print("\n📋 **MANUAL REVIEW REQUIRED**")
            for file_path in results['manual_review_files']:
                print(f"  - {file_path}")
        
        # Calculate success rate
        total_attempts = results['successful_migrations'] + results['failed_migrations']
        if total_attempts > 0:
            success_rate = (results['successful_migrations'] / total_attempts) * 100
            print(f"\n🎯 **SUCCESS RATE**: {success_rate:.1f}%")
        
        # Calculate function migration rate
        if results['functions_migrated'] > 0:
            print(f"🔬 **FUNCTIONS MIGRATED**: {results['functions_migrated']} genetics functions")
        
        print("\n✅ **GENETICS MIGRATION COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Enhanced BearDog Genetics Module Migration')
    parser.add_argument('--dry-run', action='store_true', help='Perform dry run without making changes')
    parser.add_argument('--output', help='Output file for migration report')
    
    args = parser.parse_args()
    
    migrator = GeneticsMigrationExecutor()
    
    try:
        results = migrator.execute_migration(dry_run=args.dry_run)
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump(results, f, indent=2)
            print(f"📄 Migration report saved to: {args.output}")
            
    except Exception as e:
        print(f"❌ Migration failed: {e}")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main()) 