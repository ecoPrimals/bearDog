#!/usr/bin/env python3
"""
BearDog Genetics Module Pilot Migration
Specialized migration for beardog-genetics from BearDogResult<T> to GeneticsResult<T>
"""

import re
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple, Optional

class GeneticsPilotMigrator:
    def __init__(self):
        self.module_path = Path("crates/beardog-genetics")
        self.genetics_patterns = self._initialize_genetics_patterns()
        self.cross_domain_patterns = self._initialize_cross_domain_patterns()
        self.migration_stats = {
            'files_processed': 0,
            'genetics_conversions': 0,
            'security_conversions': 0,
            'cross_domain_conversions': 0,
            'manual_review_needed': 0
        }

    def _initialize_genetics_patterns(self) -> List[Dict]:
        """Initialize genetics-specific migration patterns"""
        return [
            {
                'pattern': r'fn\s+(\w*spawn\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(...) -> GeneticsResult<\2>',
                'confidence': 'high',
                'context': 'spawning operations'
            },
            {
                'pattern': r'fn\s+(\w*lineage\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'lineage operations'
            },
            {
                'pattern': r'fn\s+(\w*genetic\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'genetic operations'
            },
            {
                'pattern': r'fn\s+(\w*diversity\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'diversity operations'
            },
            {
                'pattern': r'fn\s+(\w*evolution\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'evolution operations'
            },
            {
                'pattern': r'fn\s+(\w*breed\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'breeding operations'
            },
            {
                'pattern': r'fn\s+(\w*mutation\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'high',
                'context': 'mutation operations'
            },
        ]

    def _initialize_cross_domain_patterns(self) -> List[Dict]:
        """Initialize cross-domain migration patterns"""
        return [
            {
                'pattern': r'fn\s+(\w*auth\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> SecurityResult<\3>',
                'confidence': 'medium',
                'context': 'authentication in genetics'
            },
            {
                'pattern': r'fn\s+(\w*encrypt\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> SecurityResult<\3>',
                'confidence': 'medium',
                'context': 'encryption in genetics'
            },
            {
                'pattern': r'fn\s+(\w*validate\w*)\s*\([^)]*\)\s*->\s*BearDogResult<([^>]+)>',
                'replacement': r'fn \1(\2) -> GeneticsResult<\3>',
                'confidence': 'medium',
                'context': 'validation operations'
            },
        ]

    def execute_migration(self, dry_run: bool = False) -> Dict:
        """Execute the genetics module migration"""
        print(f"🧬 Starting genetics module migration (dry_run={dry_run})")
        print("=" * 60)
        
        if not self.module_path.exists():
            raise FileNotFoundError(f"Module path {self.module_path} not found")
        
        # Find all Rust files in the genetics module
        rust_files = list(self.module_path.rglob("*.rs"))
        print(f"📁 Found {len(rust_files)} Rust files in genetics module")
        
        results = {
            'files_processed': 0,
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
                elif file_result['status'] == 'manual_review':
                    results['manual_review_files'].append(str(file_path))
                else:
                    results['failed_migrations'] += 1
                    
            except Exception as e:
                print(f"❌ Error processing {file_path}: {e}")
                results['failed_migrations'] += 1
        
        # Generate summary report
        self._generate_summary_report(results)
        return results

    def _migrate_file(self, file_path: Path, dry_run: bool) -> Dict:
        """Migrate a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return {
                'file': str(file_path),
                'status': 'error',
                'error': str(e),
                'changes': 0
            }
        
        original_content = content
        changes_made = 0
        
        # Add genetics imports if BearDogResult is used
        if 'BearDogResult' in content and 'use beardog_errors::' not in content:
            import_line = "use beardog_errors::{GeneticsResult, GeneticsError, migrate_genetics_result};\n"
            content = import_line + content
            changes_made += 1
        
        # Apply genetics-specific patterns
        for pattern_info in self.genetics_patterns:
            pattern = pattern_info['pattern']
            replacement = pattern_info['replacement']
            
            matches = re.findall(pattern, content)
            if matches:
                content = re.sub(pattern, replacement, content)
                changes_made += len(matches)
                print(f"🧬 Applied genetics pattern in {file_path.name}: {pattern_info['context']}")
        
        # Apply cross-domain patterns
        for pattern_info in self.cross_domain_patterns:
            pattern = pattern_info['pattern']
            replacement = pattern_info['replacement']
            
            matches = re.findall(pattern, content)
            if matches:
                content = re.sub(pattern, replacement, content)
                changes_made += len(matches)
                print(f"🔄 Applied cross-domain pattern in {file_path.name}: {pattern_info['context']}")
        
        # Convert error constructions
        error_changes = self._convert_error_constructions(content)
        content = error_changes['content']
        changes_made += error_changes['changes']
        
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
                    'changes': changes_made
                }
        
        return {
            'file': str(file_path),
            'status': status,
            'changes': changes_made,
            'original_size': len(original_content),
            'new_size': len(content)
        }

    def _convert_error_constructions(self, content: str) -> Dict:
        """Convert error construction sites to genetics-specific errors"""
        changes = 0
        
        # Convert generic genetics errors to specific GeneticsError variants
        error_patterns = [
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
        ]
        
        for pattern, replacement in error_patterns:
            matches = re.findall(pattern, content)
            if matches:
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
        ]
        
        for indicator in manual_review_indicators:
            if re.search(indicator, content):
                return True
        
        return False

    def _generate_summary_report(self, results: Dict):
        """Generate migration summary report"""
        print("\n🧬 **GENETICS MODULE MIGRATION SUMMARY**")
        print("=" * 50)
        print(f"Files Processed: {results['files_processed']}")
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
        
        print("\n✅ **GENETICS MIGRATION COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='BearDog Genetics Module Migration')
    parser.add_argument('--dry-run', action='store_true', help='Perform dry run without making changes')
    parser.add_argument('--output', help='Output file for migration report')
    
    args = parser.parse_args()
    
    migrator = GeneticsPilotMigrator()
    
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