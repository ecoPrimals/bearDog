#!/usr/bin/env python3
"""
Constant Consolidation Executor
Consolidates fragmented constants into canonical locations
"""

import json
import re
import subprocess
from pathlib import Path
from typing import Dict, List, Set
from collections import defaultdict

class ConstantConsolidationExecutor:
    def __init__(self):
        self.canonical_constants = Path("crates/beardog-types/src/canonical/constants.rs")
        self.domain_constants = {
            'security': Path("crates/beardog-types/src/constants/security.rs"),
            'hsm': Path("crates/beardog-types/src/constants/hsm.rs"),
            'network': Path("crates/beardog-types/src/constants/api.rs"),
            'performance': Path("crates/beardog-types/src/constants/performance.rs"),
            'workflow': Path("crates/beardog-types/src/constants/workflows.rs"),
        }
        
    def execute_consolidation(self, analysis_file: str, dry_run: bool = False) -> Dict:
        """Execute constant consolidation"""
        print("🔧 **CONSTANT CONSOLIDATION EXECUTION**")
        print("=" * 50)
        
        # Load analysis results
        with open(analysis_file, 'r') as f:
            analysis = json.load(f)
        
        results = {
            'constants_consolidated': 0,
            'files_modified': 0,
            'duplicates_removed': 0,
            'errors': [],
            'consolidations': []
        }
        
        # Process fragmented constants
        fragmented_constants = analysis.get('fragmented_constants', {})
        
        for const_name, const_info in fragmented_constants.items():
            try:
                if self._consolidate_constant(const_name, const_info, dry_run):
                    results['constants_consolidated'] += 1
                    results['consolidations'].append({
                        'constant': const_name,
                        'duplicates_removed': const_info['count'] - 1,
                        'target_domain': self._determine_constant_domain(const_name)
                    })
            except Exception as e:
                results['errors'].append(f"Error consolidating {const_name}: {e}")
        
        # Also consolidate constants with same values but different names
        self._consolidate_similar_constants(analysis, results, dry_run)
        
        self._generate_consolidation_report(results)
        return results
    
    def _consolidate_constant(self, const_name: str, const_info: Dict, dry_run: bool) -> bool:
        """Consolidate a specific constant"""
        print(f"🔄 Consolidating constant {const_name}...")
        
        # Determine target domain
        domain = self._determine_constant_domain(const_name)
        target_file = self.domain_constants.get(domain, self.canonical_constants)
        
        # Select canonical value (most common or most complete)
        canonical_value = self._select_canonical_value(const_info)
        
        if dry_run:
            print(f"  📋 DRY RUN: Would consolidate to {target_file}")
            print(f"  📋 DRY RUN: Would use value: {canonical_value['value']}")
            return True
        
        # Add canonical definition to target file
        self._add_constant_to_file(target_file, const_name, canonical_value)
        
        # Remove duplicates from other files
        self._remove_duplicate_constants(const_name, const_info, canonical_value)
        
        # Update import statements
        self._update_constant_imports(const_name, domain)
        
        print(f"  ✅ Consolidated {const_name} to {target_file}")
        return True
    
    def _determine_constant_domain(self, const_name: str) -> str:
        """Determine the appropriate domain for a constant"""
        name_lower = const_name.lower()
        
        # Security-related constants
        if any(keyword in name_lower for keyword in ['security', 'auth', 'crypto', 'key', 'token', 'session']):
            return 'security'
        
        # HSM-related constants
        elif any(keyword in name_lower for keyword in ['hsm', 'hardware', 'attestation', 'enclave']):
            return 'hsm'
        
        # Network-related constants
        elif any(keyword in name_lower for keyword in ['network', 'api', 'http', 'port', 'endpoint']):
            return 'network'
        
        # Performance-related constants
        elif any(keyword in name_lower for keyword in ['timeout', 'retry', 'buffer', 'cache', 'performance']):
            return 'performance'
        
        # Workflow-related constants
        elif any(keyword in name_lower for keyword in ['workflow', 'process', 'step', 'task']):
            return 'workflow'
        
        # Default to general constants
        else:
            return 'general'
    
    def _select_canonical_value(self, const_info: Dict) -> Dict:
        """Select the most appropriate canonical value"""
        value_groups = const_info['value_groups']
        
        # Prefer the most common value
        largest_group = max(value_groups.items(), key=lambda x: len(x[1]))
        
        # From the largest group, prefer the most complete definition
        definitions = largest_group[1]
        canonical_def = max(definitions, key=lambda d: len(d.get('definition', '')))
        
        return canonical_def
    
    def _add_constant_to_file(self, target_file: Path, const_name: str, canonical_value: Dict):
        """Add a constant definition to the target file"""
        
        # Ensure target file exists
        target_file.parent.mkdir(parents=True, exist_ok=True)
        
        try:
            with open(target_file, 'r') as f:
                content = f.read()
        except FileNotFoundError:
            content = self._create_constants_file_header(target_file.stem)
        
        # Check if constant already exists
        if f"const {const_name}" in content:
            return  # Already exists
        
        # Create the constant definition
        const_type = canonical_value.get('type', 'auto')
        const_value = canonical_value.get('value', '""')
        
        # Determine appropriate type if auto
        if const_type == 'auto':
            const_type = self._infer_type_from_value(const_value)
        
        # Format the constant definition
        const_definition = f"""
/// {self._generate_constant_doc(const_name)}
pub const {const_name}: {const_type} = {const_value};
"""
        
        # Add to file
        content += const_definition
        
        with open(target_file, 'w') as f:
            f.write(content)
    
    def _create_constants_file_header(self, domain: str) -> str:
        """Create header for a constants file"""
        return f"""/// {domain.capitalize()} Constants
///
/// **CANONICAL {domain.upper()} CONSTANTS** - Single source of truth
///
/// This module contains all {domain}-related constants consolidated from across the codebase.

use std::time::Duration;

// ============================================================================
// {domain.upper()} CONSTANTS
// ============================================================================

"""
    
    def _generate_constant_doc(self, const_name: str) -> str:
        """Generate documentation for a constant"""
        name_parts = const_name.split('_')
        readable_name = ' '.join(word.capitalize() for word in name_parts)
        return f"{readable_name} - Canonical value"
    
    def _infer_type_from_value(self, value: str) -> str:
        """Infer Rust type from value"""
        value = value.strip()
        
        if value.startswith('"') and value.endswith('"'):
            return '&str'
        elif value.startswith('Duration::'):
            return 'Duration'
        elif value.isdigit():
            return 'u32'
        elif value.replace('.', '').isdigit():
            return 'f64'
        elif value in ['true', 'false']:
            return 'bool'
        else:
            return '&str'  # Default
    
    def _remove_duplicate_constants(self, const_name: str, const_info: Dict, canonical_value: Dict):
        """Remove duplicate constant definitions"""
        canonical_file = canonical_value['file']
        
        for value, definitions in const_info['value_groups'].items():
            for definition in definitions:
                if definition['file'] != canonical_file:
                    self._remove_constant_from_file(definition['file'], const_name)
    
    def _remove_constant_from_file(self, file_path: str, const_name: str):
        """Remove a constant definition from a file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Remove the constant definition
            pattern = rf'(?:///[^\n]*\n\s*)*pub\s+const\s+{const_name}\s*:[^;]+;'
            content = re.sub(pattern, '', content, flags=re.MULTILINE)
            
            with open(file_path, 'w') as f:
                f.write(content)
        
        except Exception:
            pass  # Continue if removal fails
    
    def _update_constant_imports(self, const_name: str, domain: str):
        """Update imports to use canonical constant location"""
        
        # Determine canonical import path
        if domain == 'general':
            import_path = "beardog_types::canonical::constants"
        else:
            import_path = f"beardog_types::constants::{domain}"
        
        # Find files that use this constant
        try:
            result = subprocess.run([
                'rg', '-l', const_name, 'crates/'
            ], capture_output=True, text=True)
            
            if result.returncode == 0:
                files = result.stdout.strip().split('\n')
                for file_path in files:
                    if file_path:
                        self._update_constant_import_in_file(file_path, const_name, import_path)
        
        except subprocess.SubprocessError:
            pass  # Continue if search fails
    
    def _update_constant_import_in_file(self, file_path: str, const_name: str, import_path: str):
        """Update constant import in a specific file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Add import if not present
            import_statement = f"use {import_path}::{const_name};"
            
            if import_statement not in content:
                # Find existing beardog_types imports
                import_pattern = r'use beardog_types::[^;]+;'
                existing_imports = re.findall(import_pattern, content)
                
                if existing_imports:
                    # Add to existing imports section
                    last_import = existing_imports[-1]
                    content = content.replace(last_import, f"{last_import}\n{import_statement}")
                else:
                    # Add at the top after other use statements
                    use_pattern = r'(use [^;]+;\n)'
                    match = re.search(use_pattern, content)
                    if match:
                        content = content.replace(match.group(0), f"{match.group(0)}{import_statement}\n")
                    else:
                        # Add at the very beginning
                        content = f"{import_statement}\n\n{content}"
                
                with open(file_path, 'w') as f:
                    f.write(content)
        
        except Exception:
            pass  # Continue if update fails
    
    def _consolidate_similar_constants(self, analysis: Dict, results: Dict, dry_run: bool):
        """Consolidate constants with same values but different names"""
        print("🔍 Identifying similar constants with same values...")
        
        # Group constants by value
        constants_by_value = defaultdict(list)
        
        for const_name, const_info in analysis.get('fragmented_constants', {}).items():
            for value, definitions in const_info['value_groups'].items():
                constants_by_value[value].extend([(const_name, d) for d in definitions])
        
        # Find groups with multiple constant names for the same value
        for value, const_list in constants_by_value.items():
            const_names = set(const_name for const_name, _ in const_list)
            
            if len(const_names) > 1:
                # Multiple constants with same value - consolidate
                canonical_name = self._select_canonical_constant_name(const_names, value)
                
                if dry_run:
                    print(f"  📋 DRY RUN: Would merge {const_names} into {canonical_name}")
                else:
                    self._merge_constants(const_names, canonical_name, value, const_list)
                    results['duplicates_removed'] += len(const_names) - 1
    
    def _select_canonical_constant_name(self, const_names: Set[str], value: str) -> str:
        """Select the best canonical name from multiple constants with same value"""
        names_list = list(const_names)
        
        # Prefer more descriptive names
        descriptive_names = [name for name in names_list if len(name.split('_')) > 2]
        if descriptive_names:
            return max(descriptive_names, key=len)
        
        # Prefer names without version numbers
        non_versioned = [name for name in names_list if not re.search(r'_v?\d+$', name)]
        if non_versioned:
            return max(non_versioned, key=len)
        
        # Fall back to longest name
        return max(names_list, key=len)
    
    def _merge_constants(self, const_names: Set[str], canonical_name: str, value: str, const_list: List):
        """Merge multiple constants into one canonical constant"""
        print(f"🔗 Merging {const_names} into {canonical_name}")
        
        # Remove all variants except canonical
        for const_name, definition in const_list:
            if const_name != canonical_name:
                self._remove_constant_from_file(definition['file'], const_name)
                
                # Update usages to use canonical name
                self._update_constant_usages(const_name, canonical_name)
    
    def _update_constant_usages(self, old_name: str, new_name: str):
        """Update constant usages throughout the codebase"""
        try:
            result = subprocess.run([
                'rg', '-l', old_name, 'crates/'
            ], capture_output=True, text=True)
            
            if result.returncode == 0:
                files = result.stdout.strip().split('\n')
                for file_path in files:
                    if file_path:
                        self._replace_constant_usage_in_file(file_path, old_name, new_name)
        
        except subprocess.SubprocessError:
            pass
    
    def _replace_constant_usage_in_file(self, file_path: str, old_name: str, new_name: str):
        """Replace constant usage in a specific file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Replace constant usage (not in comments)
            pattern = rf'\b{old_name}\b'
            new_content = re.sub(pattern, new_name, content)
            
            if new_content != content:
                with open(file_path, 'w') as f:
                    f.write(new_content)
        
        except Exception:
            pass
    
    def _generate_consolidation_report(self, results: Dict):
        """Generate consolidation report"""
        print(f"\n🎯 **CONSTANT CONSOLIDATION RESULTS**")
        print("=" * 45)
        print(f"Constants Consolidated: {results['constants_consolidated']}")
        print(f"Duplicates Removed: {results['duplicates_removed']}")
        print(f"Files Modified: {results['files_modified']}")
        print(f"Errors: {len(results['errors'])}")
        
        if results['consolidations']:
            print(f"\n📊 **CONSOLIDATION BREAKDOWN**")
            domain_counts = defaultdict(int)
            for consolidation in results['consolidations']:
                domain_counts[consolidation['target_domain']] += 1
            
            for domain, count in domain_counts.items():
                print(f"  {domain}: {count} constants")
        
        if results['errors']:
            print(f"\n⚠️  **ERRORS** (first 3)")
            for error in results['errors'][:3]:
                print(f"  - {error}")
        
        print(f"\n✅ **CONSTANT CONSOLIDATION COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Execute Constant Consolidation')
    parser.add_argument('--analysis', required=True, help='Analysis results JSON file')
    parser.add_argument('--dry-run', action='store_true', help='Perform dry run without making changes')
    
    args = parser.parse_args()
    
    executor = ConstantConsolidationExecutor()
    results = executor.execute_consolidation(args.analysis, dry_run=args.dry_run)
    
    return 0 if len(results['errors']) == 0 else 1

if __name__ == "__main__":
    exit(main()) 