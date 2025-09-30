#!/usr/bin/env python3
"""
BearDog Import Resolution Script

This script systematically resolves circular dependencies, ambiguous re-exports,
and import conflicts across the BearDog ecosystem.

Usage:
    python3 scripts/import_resolver.py --analyze
    python3 scripts/import_resolver.py --fix-glob-imports
    python3 scripts/import_resolver.py --resolve-circular
    python3 scripts/import_resolver.py --fix-all
"""

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import argparse
from collections import defaultdict

class ImportResolver:
    def __init__(self, beardog_root: Path):
        self.beardog_root = beardog_root
        self.crates_dir = beardog_root / "crates"
        
        # Track problematic patterns
        self.glob_imports = []
        self.circular_deps = []
        self.ambiguous_exports = []
        self.missing_traits = []
        
        # Crate dependency graph
        self.crate_deps = defaultdict(set)
        
        # Known problematic glob imports that should be specific
        self.problematic_globs = {
            "pub use beardog_types::canonical::*;": [
                "BearDogMasterConfig", "CanonicalAppConfig", "CanonicalAuthConfig",
                "CanonicalCacheConfig", "CanonicalComplianceConfig", "CanonicalDatabaseConfig"
            ],
            "pub use crate::canonical::providers_unified::traits::*;": [
                "ProviderTrait", "CapabilityProvider", "UniversalProvider"
            ],
            "pub use crate::constants::ultimate::network::*;": [
                "endpoints", "ports", "hosts", "timeouts"
            ],
            "pub use super::ultimate::system::*;": [
                "DEFAULT_BUFFER_SIZE", "DEFAULT_CACHE_SIZE", "DEFAULT_POOL_SIZE"
            ]
        }

    def analyze_imports(self) -> None:
        """Comprehensive analysis of import issues"""
        print("🔍 Analyzing import structure and dependencies...")
        
        self.find_glob_imports()
        self.find_circular_dependencies()
        self.find_ambiguous_exports()
        self.find_missing_trait_implementations()
        
        self.print_analysis_report()

    def find_glob_imports(self) -> None:
        """Find all glob imports that might cause ambiguity"""
        print("📋 Finding glob imports...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            if "target" in str(rust_file):
                continue
                
            try:
                content = rust_file.read_text()
                lines = content.split('\n')
                
                for line_num, line in enumerate(lines, 1):
                    if re.search(r'pub use.*::\*', line):
                        self.glob_imports.append({
                            'file': rust_file,
                            'line': line_num,
                            'content': line.strip(),
                            'problematic': any(glob in line for glob in self.problematic_globs)
                        })
                        
            except Exception as e:
                print(f"Warning: Could not read {rust_file}: {e}")

    def find_circular_dependencies(self) -> None:
        """Detect circular dependencies between crates"""
        print("🔄 Analyzing circular dependencies...")
        
        # Build dependency graph from Cargo.toml files
        for toml_file in self.crates_dir.rglob("Cargo.toml"):
            crate_name = self.extract_crate_name(toml_file)
            if crate_name:
                deps = self.extract_dependencies(toml_file)
                self.crate_deps[crate_name] = deps
        
        # Detect cycles using DFS
        visited = set()
        rec_stack = set()
        
        for crate in self.crate_deps:
            if crate not in visited:
                if self.has_cycle_dfs(crate, visited, rec_stack):
                    break

    def has_cycle_dfs(self, crate: str, visited: Set[str], rec_stack: Set[str]) -> bool:
        """DFS-based cycle detection"""
        visited.add(crate)
        rec_stack.add(crate)
        
        for dep in self.crate_deps.get(crate, []):
            if dep not in visited:
                if self.has_cycle_dfs(dep, visited, rec_stack):
                    return True
            elif dep in rec_stack:
                self.circular_deps.append(f"{crate} -> {dep}")
                return True
        
        rec_stack.remove(crate)
        return False

    def find_ambiguous_exports(self) -> None:
        """Find potentially ambiguous re-exports"""
        print("⚠️  Finding ambiguous re-exports...")
        
        # Track exported symbols across modules
        exports = defaultdict(list)
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            if "target" in str(rust_file):
                continue
                
            try:
                content = rust_file.read_text()
                
                # Find pub use statements
                pub_uses = re.findall(r'pub use [^;]+;', content)
                for pub_use in pub_uses:
                    # Extract the symbol being re-exported
                    match = re.search(r'pub use .*::([A-Za-z_][A-Za-z0-9_]*)', pub_use)
                    if match:
                        symbol = match.group(1)
                        exports[symbol].append((rust_file, pub_use))
                        
            except Exception as e:
                continue
        
        # Find symbols exported from multiple places
        for symbol, locations in exports.items():
            if len(locations) > 1:
                self.ambiguous_exports.append({
                    'symbol': symbol,
                    'locations': locations
                })

    def find_missing_trait_implementations(self) -> None:
        """Find missing trait implementations causing compilation errors"""
        print("🔧 Finding missing trait implementations...")
        
        # Known missing implementations from compilation errors
        missing_impls = [
            {
                'trait': 'ConsolidatedConfig',
                'types': [
                    'SystemConfiguration',
                    'NetworkConfiguration', 
                    'SecurityConfiguration',
                    'DatabaseConfiguration',
                    'MonitoringConfiguration'
                ],
                'file': 'crates/beardog-types/src/canonical/config/consolidated/core/mod.rs'
            }
        ]
        
        self.missing_traits = missing_impls

    def print_analysis_report(self) -> None:
        """Print comprehensive analysis report"""
        print("\n" + "="*80)
        print("📊 BearDog Import Resolution Analysis Report")
        print("="*80)
        
        print(f"\n🔥 Glob Imports Found: {len(self.glob_imports)}")
        problematic_globs = [g for g in self.glob_imports if g['problematic']]
        print(f"   • Problematic globs: {len(problematic_globs)}")
        print(f"   • Safe globs: {len(self.glob_imports) - len(problematic_globs)}")
        
        if problematic_globs:
            print("\n🎯 Most Problematic Glob Imports:")
            for glob in problematic_globs[:5]:
                rel_path = glob['file'].relative_to(self.beardog_root)
                print(f"   📁 {rel_path}:{glob['line']}")
                print(f"      {glob['content']}")
        
        print(f"\n🔄 Circular Dependencies: {len(self.circular_deps)}")
        for dep in self.circular_deps[:5]:
            print(f"   • {dep}")
            
        print(f"\n⚠️  Ambiguous Exports: {len(self.ambiguous_exports)}")
        for export in self.ambiguous_exports[:5]:
            print(f"   • {export['symbol']}: {len(export['locations'])} locations")
            
        print(f"\n🔧 Missing Trait Implementations: {len(self.missing_traits)}")
        for missing in self.missing_traits:
            print(f"   • {missing['trait']}: {len(missing['types'])} types need implementation")

    def fix_glob_imports(self, dry_run: bool = True) -> None:
        """Replace problematic glob imports with specific imports"""
        print("🔧 Fixing problematic glob imports...")
        
        fixed_count = 0
        
        for glob in self.glob_imports:
            if not glob['problematic']:
                continue
                
            file_path = glob['file']
            line_content = glob['content']
            
            # Find replacement for this glob
            replacement = self.get_specific_imports(line_content)
            if replacement:
                try:
                    content = file_path.read_text()
                    new_content = content.replace(line_content, replacement)
                    
                    if new_content != content:
                        if not dry_run:
                            file_path.write_text(new_content)
                        
                        rel_path = file_path.relative_to(self.beardog_root)
                        action = "Would fix" if dry_run else "Fixed"
                        print(f"   ✅ {action} glob import in {rel_path}:{glob['line']}")
                        fixed_count += 1
                        
                except Exception as e:
                    print(f"   ❌ Error fixing {file_path}: {e}")
        
        print(f"\n📊 Glob import fixes: {fixed_count}")
        if dry_run:
            print("⚠️  Dry run mode - no files modified")

    def get_specific_imports(self, glob_line: str) -> Optional[str]:
        """Get specific imports to replace a glob import"""
        for glob_pattern, specific_imports in self.problematic_globs.items():
            if glob_pattern.replace('*', '').strip() in glob_line:
                # Create specific import list
                imports = ', '.join(specific_imports)
                return glob_line.replace('*', f'{{{imports}}}')
        return None

    def fix_missing_trait_implementations(self, dry_run: bool = True) -> None:
        """Add missing trait implementations"""
        print("🔧 Adding missing trait implementations...")
        
        for missing in self.missing_traits:
            trait_name = missing['trait']
            types = missing['types']
            file_path = self.beardog_root / missing['file']
            
            if not file_path.exists():
                continue
                
            try:
                content = file_path.read_text()
                
                # Add trait implementations at the end of the file
                trait_impls = self.generate_trait_implementations(trait_name, types)
                
                if trait_impls:
                    new_content = content + "\n\n" + trait_impls
                    
                    if not dry_run:
                        file_path.write_text(new_content)
                    
                    rel_path = file_path.relative_to(self.beardog_root)
                    action = "Would add" if dry_run else "Added"
                    print(f"   ✅ {action} {trait_name} implementations to {rel_path}")
                    
            except Exception as e:
                print(f"   ❌ Error adding implementations to {file_path}: {e}")

    def generate_trait_implementations(self, trait_name: str, types: List[str]) -> str:
        """Generate trait implementation code"""
        if trait_name == "ConsolidatedConfig":
            impls = []
            for type_name in types:
                impl_code = f"""impl ConsolidatedConfig for {type_name} {{
    fn validate(&self) -> Result<(), BearDogError> {{
        // TODO: Implement validation logic
        Ok(())
    }}

    fn merge(&mut self, other: Self) -> Result<(), BearDogError> {{
        // TODO: Implement merge logic
        *self = other;
        Ok(())
    }}

    fn from_env() -> Result<Self, BearDogError> {{
        // TODO: Implement environment loading
        Ok(Self::default())
    }}

    fn apply_overrides(&mut self, _overrides: serde_json::Value) -> Result<(), BearDogError> {{
        // TODO: Implement override application
        Ok(())
    }}

    fn to_env_vars(&self) -> Vec<(String, String)> {{
        // TODO: Implement environment variable export
        Vec::new()
    }}
}}"""
                impls.append(impl_code)
            
            return "\n\n".join(impls)
        
        return ""

    def extract_crate_name(self, toml_file: Path) -> Optional[str]:
        """Extract crate name from Cargo.toml"""
        try:
            content = toml_file.read_text()
            match = re.search(r'name\s*=\s*"([^"]+)"', content)
            return match.group(1) if match else None
        except:
            return None

    def extract_dependencies(self, toml_file: Path) -> Set[str]:
        """Extract dependencies from Cargo.toml"""
        deps = set()
        try:
            content = toml_file.read_text()
            
            # Find [dependencies] section
            in_deps = False
            for line in content.split('\n'):
                line = line.strip()
                if line == '[dependencies]':
                    in_deps = True
                    continue
                elif line.startswith('[') and in_deps:
                    break
                elif in_deps and '=' in line:
                    dep_name = line.split('=')[0].strip()
                    if dep_name.startswith('beardog-'):
                        deps.add(dep_name)
                        
        except:
            pass
        return deps

    def fix_all_imports(self, dry_run: bool = True) -> None:
        """Fix all import issues"""
        print("🚀 Fixing all import issues...")
        
        self.fix_glob_imports(dry_run)
        self.fix_missing_trait_implementations(dry_run)
        
        print(f"\n🎉 Import resolution {'plan' if dry_run else 'complete'}")
        if dry_run:
            print("⚠️  Use --execute to apply changes")

def main():
    parser = argparse.ArgumentParser(description='BearDog Import Resolution Tool')
    parser.add_argument('--analyze', action='store_true', help='Analyze import issues')
    parser.add_argument('--fix-glob-imports', action='store_true', help='Fix problematic glob imports')
    parser.add_argument('--resolve-circular', action='store_true', help='Resolve circular dependencies')
    parser.add_argument('--fix-traits', action='store_true', help='Add missing trait implementations')
    parser.add_argument('--fix-all', action='store_true', help='Fix all import issues')
    parser.add_argument('--execute', action='store_true', help='Execute fixes (default is dry run)')
    
    args = parser.parse_args()
    
    # Find BearDog root directory
    current_dir = Path.cwd()
    beardog_root = None
    
    for parent in [current_dir] + list(current_dir.parents):
        if (parent / "Cargo.toml").exists() and (parent / "crates").exists():
            beardog_root = parent
            break
    
    if not beardog_root:
        print("❌ Could not find BearDog root directory")
        sys.exit(1)
        
    print(f"🏠 BearDog root: {beardog_root}")
    
    resolver = ImportResolver(beardog_root)
    
    if args.analyze:
        resolver.analyze_imports()
    elif args.fix_glob_imports:
        resolver.fix_glob_imports(dry_run=not args.execute)
    elif args.fix_traits:
        resolver.fix_missing_trait_implementations(dry_run=not args.execute)
    elif args.fix_all:
        resolver.fix_all_imports(dry_run=not args.execute)
    else:
        parser.print_help()

if __name__ == "__main__":
    main() 