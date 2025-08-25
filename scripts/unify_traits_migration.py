#!/usr/bin/env python3
"""
BearDog Trait Unification Migration Script

This script automatically updates trait imports across the codebase to use
the new canonical traits from beardog-traits::canonical instead of fragmented
trait definitions across different crates.

Usage:
    python3 scripts/unify_traits_migration.py [--dry-run] [--verbose]
"""

import os
import re
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Set

# Mapping from legacy trait paths to canonical equivalents
TRAIT_MAPPINGS = {
    # Security Provider traits
    "beardog_security::types::SecurityProvider": "beardog_traits::canonical::SecurityProvider",
    "beardog_types::providers::SecurityProvider": "beardog_traits::canonical::SecurityProvider",
    "beardog_tunnel::security_provider::BStpSecurityProvider": "beardog_traits::canonical::SecurityProvider",
    
    # HSM Provider traits
    "beardog_tunnel::hsm::HsmProvider": "beardog_traits::canonical::HsmProvider",
    "beardog_types::providers::HsmProvider": "beardog_traits::canonical::HsmProvider",
    "beardog_types::canonical::hsm::traits::HsmProvider": "beardog_traits::canonical::HsmProvider",
    "beardog_types::canonical::providers::HsmProvider": "beardog_traits::canonical::HsmProvider",
    "beardog_tunnel::universal_hsm::traits::UniversalHsmProvider": "beardog_traits::canonical::HsmProvider",
    "beardog_workflows::zero_cost_hsm::ZeroCostHsmProvider": "beardog_traits::canonical::HsmProvider",
    
    # Cache Provider traits
    "beardog_api::cache::CacheProvider": "beardog_traits::canonical::CacheProvider",
    "beardog_types::providers::CacheProvider": "beardog_traits::canonical::CacheProvider",
    "beardog_core::zero_cost_architecture::ZeroCostCache": "beardog_traits::canonical::CacheProvider",
    
    # Crypto Provider traits
    "beardog_traits::security::CryptoProvider": "beardog_traits::canonical::CryptoProvider",
    "beardog_types::providers::CryptoProvider": "beardog_traits::canonical::CryptoProvider",
    
    # Authentication/Authorization traits (consolidated into SecurityProvider)
    "beardog_traits::security::AuthenticationProvider": "beardog_traits::canonical::SecurityProvider",
    "beardog_traits::security::AuthorizationProvider": "beardog_traits::canonical::SecurityProvider",
    
    # Monitoring traits
    "beardog_traits::monitoring::HealthMonitor": "beardog_traits::canonical::MonitoringProvider",
    "beardog_traits::monitoring::MetricsCollector": "beardog_traits::canonical::MonitoringProvider",
    "beardog_traits::monitoring::PerformanceMonitor": "beardog_traits::canonical::MonitoringProvider",
    
    # Genetics traits
    "beardog_traits::genetics::GeneticsEngine": "beardog_traits::canonical::GeneticsProvider",
    
    # Workflow traits
    "beardog_workflows::workflows::processors::WorkflowProcessor": "beardog_traits::canonical::WorkflowProvider",
}

# Import patterns to update
IMPORT_PATTERNS = [
    # Standard use statements
    (r'use\s+([\w_:]+)::([\w_]+);', r'use \1::\2;'),
    # Use statements with braces
    (r'use\s+([\w_:]+)::\{([^}]+)\};', r'use \1::{\2};'),
    # Trait bounds
    (r':\s*([\w_:]+)', r': \1'),
    # Type annotations
    (r'([\w_]+):\s*Box<dyn\s+([\w_:]+)', r'\1: Box<dyn \2'),
    (r'([\w_]+):\s*Arc<dyn\s+([\w_:]+)', r'\1: Arc<dyn \2'),
]

def find_rust_files(directory: Path) -> List[Path]:
    """Find all Rust source files in the directory tree."""
    rust_files = []
    for root, dirs, files in os.walk(directory):
        # Skip target directories and other build artifacts
        dirs[:] = [d for d in dirs if d not in ['target', 'node_modules', '.git']]
        
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(Path(root) / file)
    
    return rust_files

def update_trait_imports(content: str, file_path: Path) -> Tuple[str, List[str]]:
    """Update trait imports in the file content."""
    updated_content = content
    changes = []
    
    for legacy_path, canonical_path in TRAIT_MAPPINGS.items():
        # Update direct imports
        pattern = re.compile(rf'\buse\s+{re.escape(legacy_path)}\b', re.MULTILINE)
        if pattern.search(updated_content):
            updated_content = pattern.sub(f'use {canonical_path}', updated_content)
            changes.append(f"Updated import: {legacy_path} -> {canonical_path}")
        
        # Update trait bounds and type annotations
        trait_name = legacy_path.split('::')[-1]
        canonical_trait_name = canonical_path.split('::')[-1]
        
        # Update trait bounds like `: SecurityProvider`
        bound_pattern = re.compile(rf'\b:\s*{re.escape(trait_name)}\b')
        if bound_pattern.search(updated_content):
            updated_content = bound_pattern.sub(f': {canonical_trait_name}', updated_content)
            changes.append(f"Updated trait bound: {trait_name} -> {canonical_trait_name}")
        
        # Update dyn trait objects
        dyn_pattern = re.compile(rf'\bdyn\s+{re.escape(trait_name)}\b')
        if dyn_pattern.search(updated_content):
            updated_content = dyn_pattern.sub(f'dyn {canonical_trait_name}', updated_content)
            changes.append(f"Updated dyn trait: dyn {trait_name} -> dyn {canonical_trait_name}")
    
    return updated_content, changes

def add_canonical_import(content: str, traits_used: Set[str]) -> str:
    """Add the canonical traits import if canonical traits are used."""
    if not traits_used:
        return content
    
    # Check if beardog_traits::canonical is already imported
    if 'use beardog_traits::canonical' in content or 'use beardog_traits::prelude' in content:
        return content
    
    # Find the best place to add the import (after existing beardog imports)
    lines = content.split('\n')
    import_line_idx = 0
    
    # Find the last beardog import or the last use statement
    for i, line in enumerate(lines):
        if line.strip().startswith('use beardog_') or line.strip().startswith('use crate::'):
            import_line_idx = i + 1
        elif line.strip().startswith('use ') and '::' in line:
            import_line_idx = i + 1
    
    # Add the canonical import
    canonical_import = f"use beardog_traits::canonical::{{{', '.join(sorted(traits_used))}}};"
    lines.insert(import_line_idx, canonical_import)
    
    return '\n'.join(lines)

def process_file(file_path: Path, dry_run: bool = False, verbose: bool = False) -> bool:
    """Process a single Rust file for trait unification."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    updated_content, changes = update_trait_imports(original_content, file_path)
    
    if not changes:
        if verbose:
            print(f"No changes needed in {file_path}")
        return True
    
    # Determine which canonical traits are now being used
    canonical_traits_used = set()
    for change in changes:
        if "beardog_traits::canonical::" in change:
            trait_name = change.split("beardog_traits::canonical::")[-1].split()[0]
            canonical_traits_used.add(trait_name)
    
    # Add canonical import if needed
    if canonical_traits_used:
        updated_content = add_canonical_import(updated_content, canonical_traits_used)
    
    if dry_run:
        print(f"Would update {file_path}:")
        for change in changes:
            print(f"  - {change}")
        if canonical_traits_used:
            print(f"  - Would add canonical import for: {', '.join(canonical_traits_used)}")
        return True
    
    # Write the updated content
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(updated_content)
        
        print(f"Updated {file_path}:")
        for change in changes:
            print(f"  ✓ {change}")
        if canonical_traits_used:
            print(f"  ✓ Added canonical import for: {', '.join(canonical_traits_used)}")
        
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}")
        return False

def generate_migration_report(processed_files: int, updated_files: int, total_changes: int):
    """Generate a summary report of the migration."""
    print("\n" + "="*60)
    print("TRAIT UNIFICATION MIGRATION REPORT")
    print("="*60)
    print(f"Files processed: {processed_files}")
    print(f"Files updated: {updated_files}")
    print(f"Total changes: {total_changes}")
    print("\nMigration mappings applied:")
    for legacy, canonical in TRAIT_MAPPINGS.items():
        print(f"  {legacy} -> {canonical}")
    print("\nNext steps:")
    print("1. Run 'cargo check --workspace' to verify compilation")
    print("2. Update any remaining manual trait implementations")
    print("3. Run tests to ensure functionality is preserved")
    print("4. Consider removing deprecated trait definitions")
    print("="*60)

def main():
    parser = argparse.ArgumentParser(description="Migrate BearDog traits to canonical system")
    parser.add_argument("--dry-run", action="store_true", help="Show changes without applying them")
    parser.add_argument("--verbose", action="store_true", help="Show verbose output")
    parser.add_argument("--directory", type=str, default=".", help="Directory to process (default: current)")
    
    args = parser.parse_args()
    
    # Find the project root (look for Cargo.toml)
    project_root = Path(args.directory).resolve()
    if not (project_root / "Cargo.toml").exists():
        print(f"Error: No Cargo.toml found in {project_root}")
        print("Please run this script from the project root or specify --directory")
        sys.exit(1)
    
    print(f"Processing BearDog trait unification in: {project_root}")
    if args.dry_run:
        print("DRY RUN - No files will be modified")
    print()
    
    # Find all Rust files
    rust_files = find_rust_files(project_root)
    print(f"Found {len(rust_files)} Rust files")
    
    # Process each file
    processed_files = 0
    updated_files = 0
    total_changes = 0
    
    for file_path in rust_files:
        # Skip generated files and certain directories
        if any(part in str(file_path) for part in ['target/', 'build.rs', '/build/', '.cargo/']):
            continue
        
        processed_files += 1
        original_content = ""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                original_content = f.read()
        except:
            continue
        
        success = process_file(file_path, args.dry_run, args.verbose)
        if success:
            # Check if file was actually changed
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    new_content = f.read()
                if new_content != original_content:
                    updated_files += 1
                    # Count changes (rough estimate)
                    total_changes += len([line for line in new_content.split('\n') 
                                        if 'beardog_traits::canonical' in line])
            except:
                pass
    
    generate_migration_report(processed_files, updated_files, total_changes)

if __name__ == "__main__":
    main() 