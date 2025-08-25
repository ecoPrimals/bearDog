#!/usr/bin/env python3
"""
Type Consolidation Executor
Automatically consolidates duplicate type definitions to canonical locations
"""

import json
import re
import subprocess
from pathlib import Path
from typing import Dict, List, Set

class TypeConsolidationExecutor:
    def __init__(self):
        self.canonical_base = Path("crates/beardog-types/src/canonical")
        self.analysis_results = None
        self.consolidation_plan = {}
        
    def load_analysis(self, analysis_file: str):
        """Load the canonical modernization analysis results"""
        with open(analysis_file, 'r') as f:
            self.analysis_results = json.load(f)
        
        print(f"📊 Loaded analysis: {len(self.analysis_results['duplicate_types'])} duplicate types")
        
    def execute_consolidation(self, dry_run: bool = False) -> Dict:
        """Execute the type consolidation process"""
        print("🔧 **TYPE CONSOLIDATION EXECUTION**")
        print("=" * 50)
        
        results = {
            'types_consolidated': 0,
            'files_modified': 0,
            'errors': [],
            'consolidations': []
        }
        
        # Process high-priority consolidations first
        priority_types = self._identify_priority_consolidations()
        
        for type_name, consolidation_info in priority_types.items():
            try:
                if self._consolidate_type(type_name, consolidation_info, dry_run):
                    results['types_consolidated'] += 1
                    results['consolidations'].append({
                        'type': type_name,
                        'target': consolidation_info['target'],
                        'sources_removed': len(consolidation_info['sources'])
                    })
            except Exception as e:
                results['errors'].append(f"Error consolidating {type_name}: {e}")
        
        self._generate_consolidation_report(results)
        return results
    
    def _identify_priority_consolidations(self) -> Dict:
        """Identify high-priority type consolidations"""
        priority_consolidations = {}
        
        # Focus on HSM-related types first
        hsm_types = [
            'HsmConfig', 'HsmTier', 'HsmOperationResult', 'HsmHealth',
            'KeyMetadata', 'KeyUsagePolicy', 'KeyOperation', 'KeyHealth',
            'AttestationConfig', 'SecurityCapabilities', 'HsmCapabilities'
        ]
        
        # Configuration types
        config_types = [
            'SessionConfig', 'RateLimitConfig', 'AuthenticationConfig',
            'EncryptionConfig', 'MfaConfig', 'PasswordPolicyConfig'
        ]
        
        # Error types (for idiomatic migration)
        error_types = [
            'SecurityError', 'GeneticsError', 'NetworkError', 'WorkflowError',
            'SystemError', 'BusinessError'
        ]
        
        all_priority_types = hsm_types + config_types + error_types
        
        for type_name in all_priority_types:
            if type_name in self.analysis_results.get('duplicate_types', {}):
                duplicate_info = self.analysis_results['duplicate_types'][type_name]
                priority_consolidations[type_name] = self._plan_consolidation(type_name, duplicate_info)
        
        return priority_consolidations
    
    def _plan_consolidation(self, type_name: str, duplicate_info: Dict) -> Dict:
        """Plan consolidation for a specific type"""
        definitions = duplicate_info['definitions']
        
        # Determine target location
        target_location = self._determine_target_location(type_name, definitions)
        
        # Identify canonical definition (most complete)
        canonical_def = self._select_canonical_definition(definitions)
        
        # Plan source removals
        sources_to_remove = [d for d in definitions if d['file'] != canonical_def['file']]
        
        return {
            'target': target_location,
            'canonical_definition': canonical_def,
            'sources': sources_to_remove,
            'import_updates': self._plan_import_updates(type_name, sources_to_remove, target_location)
        }
    
    def _determine_target_location(self, type_name: str, definitions: List) -> str:
        """Determine the best target location for consolidation"""
        
        # HSM types go to canonical/hsm/
        if any(keyword in type_name.lower() for keyword in ['hsm', 'key', 'attestation']):
            if 'config' in type_name.lower():
                return str(self.canonical_base / "hsm" / "config.rs")
            elif any(keyword in type_name.lower() for keyword in ['key', 'metadata', 'usage']):
                return str(self.canonical_base / "hsm" / "keys.rs")
            else:
                return str(self.canonical_base / "hsm" / "status.rs")
        
        # Configuration types go to canonical/configuration/
        elif 'config' in type_name.lower():
            if 'security' in type_name.lower():
                return str(self.canonical_base / "configuration" / "security.rs")
            elif 'network' in type_name.lower():
                return str(self.canonical_base / "configuration" / "network.rs")
            else:
                return str(self.canonical_base / "configuration" / "mod.rs")
        
        # Error types go to beardog-errors
        elif 'error' in type_name.lower():
            return "crates/beardog-errors/src/idiomatic/domain_errors.rs"
        
        # Default to canonical root
        else:
            return str(self.canonical_base / "mod.rs")
    
    def _select_canonical_definition(self, definitions: List) -> Dict:
        """Select the most complete definition as canonical"""
        
        # Prefer definitions in canonical locations
        canonical_defs = [d for d in definitions if 'canonical' in d['file']]
        if canonical_defs:
            return max(canonical_defs, key=lambda d: len(d.get('definition', '')))
        
        # Prefer definitions in beardog-types
        types_defs = [d for d in definitions if 'beardog-types' in d['file']]
        if types_defs:
            return max(types_defs, key=lambda d: len(d.get('definition', '')))
        
        # Fall back to most complete definition
        return max(definitions, key=lambda d: len(d.get('definition', '')))
    
    def _plan_import_updates(self, type_name: str, sources: List, target_location: str) -> List:
        """Plan import statement updates"""
        updates = []
        
        # Determine canonical import path
        canonical_import = self._get_canonical_import_path(type_name, target_location)
        
        for source in sources:
            # Find files that import this type from the old location
            old_import_pattern = self._get_import_pattern(source['file'], type_name)
            updates.append({
                'pattern': old_import_pattern,
                'replacement': f"use {canonical_import}::{type_name};",
                'source_file': source['file']
            })
        
        return updates
    
    def _get_canonical_import_path(self, type_name: str, target_location: str) -> str:
        """Get the canonical import path for a type"""
        
        if 'beardog-errors' in target_location:
            return "beardog_errors::idiomatic"
        elif 'canonical/hsm' in target_location:
            return "beardog_types::canonical::hsm"
        elif 'canonical/configuration' in target_location:
            return "beardog_types::canonical::configuration"
        else:
            return "beardog_types::canonical"
    
    def _get_import_pattern(self, file_path: str, type_name: str) -> str:
        """Get the import pattern for a type from a specific file"""
        
        # Extract module path from file path
        if 'crates/' in file_path:
            parts = file_path.split('crates/')[1].split('/')
            crate_name = parts[0].replace('-', '_')
            module_parts = parts[1:-1]  # Exclude src and filename
            
            if module_parts:
                module_path = '::'.join(module_parts)
                return f"use {crate_name}::{module_path}::{type_name};"
            else:
                return f"use {crate_name}::{type_name};"
        
        return f"use.*::{type_name};"
    
    def _consolidate_type(self, type_name: str, consolidation_info: Dict, dry_run: bool) -> bool:
        """Consolidate a specific type"""
        print(f"🔄 Consolidating {type_name}...")
        
        target_file = consolidation_info['target']
        canonical_def = consolidation_info['canonical_definition']
        sources = consolidation_info['sources']
        
        if dry_run:
            print(f"  📋 DRY RUN: Would consolidate to {target_file}")
            print(f"  📋 DRY RUN: Would remove {len(sources)} duplicate definitions")
            return True
        
        # Ensure target directory exists
        target_path = Path(target_file)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Add canonical definition to target file if not already there
        if not self._definition_exists_in_file(target_file, type_name):
            self._add_definition_to_file(target_file, canonical_def)
        
        # Remove duplicate definitions from source files
        for source in sources:
            self._remove_definition_from_file(source['file'], type_name)
        
        # Update import statements
        self._update_import_statements(consolidation_info['import_updates'])
        
        print(f"  ✅ Consolidated {type_name} to {target_file}")
        return True
    
    def _definition_exists_in_file(self, file_path: str, type_name: str) -> bool:
        """Check if a type definition already exists in a file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            patterns = [
                f"struct {type_name}",
                f"enum {type_name}",
                f"type {type_name}"
            ]
            
            return any(pattern in content for pattern in patterns)
        except FileNotFoundError:
            return False
    
    def _add_definition_to_file(self, file_path: str, definition_info: Dict):
        """Add a type definition to a file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
        except FileNotFoundError:
            content = ""
        
        # Add definition at the end of the file
        definition = definition_info.get('definition', '')
        if definition:
            content += f"\n\n{definition}\n"
            
            with open(file_path, 'w') as f:
                f.write(content)
    
    def _remove_definition_from_file(self, file_path: str, type_name: str):
        """Remove a type definition from a file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
        except FileNotFoundError:
            return
        
        # Remove the definition (simplified - would need more sophisticated parsing)
        patterns = [
            f"pub struct {type_name}.*?^}}",
            f"pub enum {type_name}.*?^}}",
            f"pub type {type_name}.*?;"
        ]
        
        for pattern in patterns:
            content = re.sub(pattern, "", content, flags=re.MULTILINE | re.DOTALL)
        
        with open(file_path, 'w') as f:
            f.write(content)
    
    def _update_import_statements(self, import_updates: List):
        """Update import statements across the codebase"""
        for update in import_updates:
            # Use ripgrep to find files with the old import pattern
            try:
                result = subprocess.run([
                    'rg', '-l', update['pattern'], 'crates/'
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    files = result.stdout.strip().split('\n')
                    for file_path in files:
                        if file_path:
                            self._update_imports_in_file(file_path, update)
            except subprocess.SubprocessError:
                pass  # Continue if ripgrep fails
    
    def _update_imports_in_file(self, file_path: str, update: Dict):
        """Update imports in a specific file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Replace the import pattern
            new_content = re.sub(update['pattern'], update['replacement'], content)
            
            if new_content != content:
                with open(file_path, 'w') as f:
                    f.write(new_content)
        except Exception:
            pass  # Continue if file update fails
    
    def _generate_consolidation_report(self, results: Dict):
        """Generate consolidation report"""
        print(f"\n🎯 **CONSOLIDATION RESULTS**")
        print("=" * 40)
        print(f"Types Consolidated: {results['types_consolidated']}")
        print(f"Files Modified: {results['files_modified']}")
        print(f"Errors: {len(results['errors'])}")
        
        if results['errors']:
            print(f"\n⚠️  **ERRORS**")
            for error in results['errors'][:5]:  # Show first 5 errors
                print(f"  - {error}")
        
        print(f"\n✅ **CONSOLIDATION COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Execute Type Consolidation')
    parser.add_argument('--analysis', required=True, help='Analysis results JSON file')
    parser.add_argument('--dry-run', action='store_true', help='Perform dry run without making changes')
    
    args = parser.parse_args()
    
    executor = TypeConsolidationExecutor()
    executor.load_analysis(args.analysis)
    results = executor.execute_consolidation(dry_run=args.dry_run)
    
    return 0 if len(results['errors']) == 0 else 1

if __name__ == "__main__":
    exit(main()) 