#!/usr/bin/env python3
"""
BearDog Configuration Deduplication Script

This script systematically identifies and migrates duplicate configuration structs
to canonical versions in beardog-types/src/canonical/.

Usage:
    python3 scripts/config_deduplication.py --analyze
    python3 scripts/config_deduplication.py --migrate MonitoringConfig
    python3 scripts/config_deduplication.py --migrate-all
"""

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
import argparse

class ConfigDeduplicator:
    def __init__(self, beardog_root: Path):
        self.beardog_root = beardog_root
        self.crates_dir = beardog_root / "crates"
        
        # Canonical location for unified configs
        self.canonical_dir = self.crates_dir / "beardog-types" / "src" / "canonical"
        
        # Top duplicate configs identified from analysis
        self.target_configs = {
            "MonitoringConfig": {
                "canonical_path": "beardog_types::canonical::monitoring::MonitoringConfig",
                "canonical_file": "crates/beardog-types/src/canonical/monitoring.rs",
                "instances": 13
            },
            "SecurityConfig": {
                "canonical_path": "beardog_types::canonical::security_unified::SecurityConfig", 
                "canonical_file": "crates/beardog-types/src/canonical/security_unified/mod.rs",
                "instances": 12
            },
            "HealthCheckConfig": {
                "canonical_path": "beardog_types::canonical::monitoring::HealthCheckConfig",
                "canonical_file": "crates/beardog-types/src/canonical/monitoring.rs", 
                "instances": 12
            },
            "AuthenticationConfig": {
                "canonical_path": "beardog_types::canonical::config::consolidated_simple::AuthenticationConfig",
                "canonical_file": "crates/beardog-types/src/canonical/config/consolidated_simple/core_systems.rs",
                "instances": 10
            },
            "PerformanceConfig": {
                "canonical_path": "beardog_types::canonical::config::consolidated_simple::PerformanceConfig", 
                "canonical_file": "crates/beardog-types/src/canonical/config/consolidated_simple/specialized_systems.rs",
                "instances": 9
            },
            "NetworkConfig": {
                "canonical_path": "beardog_types::canonical::config::consolidated_simple::NetworkConfig",
                "canonical_file": "crates/beardog-types/src/canonical/config/consolidated_simple/core_systems.rs",
                "instances": 9
            },
            "LoggingConfig": {
                "canonical_path": "beardog_types::canonical::config::consolidated_simple::LoggingConfig",
                "canonical_file": "crates/beardog-types/src/canonical/config/consolidated_simple/core_systems.rs", 
                "instances": 9
            },
            "EncryptionConfig": {
                "canonical_path": "beardog_types::canonical::config::consolidated_simple::EncryptionConfig",
                "canonical_file": "crates/beardog-types/src/canonical/config/consolidated_simple/core_systems.rs",
                "instances": 9
            }
        }

    def find_all_config_structs(self) -> Dict[str, List[Path]]:
        """Find all Config struct definitions across the codebase"""
        config_locations = {}
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            if "target" in str(rust_file) or "_original.rs" in str(rust_file):
                continue
                
            try:
                content = rust_file.read_text()
                
                # Find all pub struct *Config definitions
                config_matches = re.findall(r'pub struct ([A-Za-z0-9_]*Config[A-Za-z0-9_]*)', content)
                
                for config_name in config_matches:
                    if config_name not in config_locations:
                        config_locations[config_name] = []
                    config_locations[config_name].append(rust_file)
                    
            except Exception as e:
                print(f"Warning: Could not read {rust_file}: {e}")
                
        return config_locations

    def analyze_duplicates(self) -> None:
        """Analyze and report duplicate configuration structs"""
        print("🔍 Analyzing configuration struct duplicates...")
        
        config_locations = self.find_all_config_structs()
        
        # Sort by number of duplicates (descending)
        sorted_configs = sorted(config_locations.items(), key=lambda x: len(x[1]), reverse=True)
        
        print(f"\n📊 Found {len(config_locations)} unique config struct names")
        print(f"📊 Total config struct definitions: {sum(len(locations) for locations in config_locations.values())}")
        
        print("\n🔥 Top 20 Most Duplicated Configs:")
        print("=" * 60)
        for config_name, locations in sorted_configs[:20]:
            count = len(locations)
            if count > 1:
                print(f"{count:3d}x {config_name}")
                
                # Show locations for highly duplicated configs
                if count >= 5:
                    for location in locations[:5]:  # Show first 5 locations
                        rel_path = location.relative_to(self.beardog_root)
                        print(f"     📁 {rel_path}")
                    if len(locations) > 5:
                        print(f"     ... and {len(locations) - 5} more")
                print()

    def migrate_config(self, config_name: str) -> bool:
        """Migrate a specific config to its canonical version"""
        if config_name not in self.target_configs:
            print(f"❌ {config_name} is not in the target migration list")
            return False
            
        config_info = self.target_configs[config_name]
        canonical_path = config_info["canonical_path"]
        
        print(f"🔄 Migrating {config_name} to {canonical_path}")
        
        # Find all instances of this config
        config_locations = self.find_all_config_structs()
        if config_name not in config_locations:
            print(f"✅ No instances of {config_name} found - already migrated?")
            return True
            
        locations = config_locations[config_name]
        canonical_file = self.beardog_root / config_info["canonical_file"]
        
        # Filter out the canonical file itself
        non_canonical_locations = [loc for loc in locations if loc != canonical_file]
        
        if not non_canonical_locations:
            print(f"✅ All instances of {config_name} are already in canonical location")
            return True
            
        print(f"📍 Found {len(non_canonical_locations)} duplicate instances to migrate:")
        
        migration_plan = []
        
        for location in non_canonical_locations:
            rel_path = location.relative_to(self.beardog_root)
            print(f"   📁 {rel_path}")
            
            # Analyze the file to create migration plan
            try:
                content = location.read_text()
                
                # Check if this file defines the struct (vs just imports it)
                struct_pattern = rf'pub struct {re.escape(config_name)}\s*{{[^}}]*}}'
                if re.search(struct_pattern, content, re.DOTALL):
                    migration_plan.append({
                        'file': location,
                        'action': 'remove_definition',
                        'canonical_import': canonical_path
                    })
                
                # Check for imports/uses of the config
                import_pattern = rf'use.*{re.escape(config_name)}'
                if re.search(import_pattern, content):
                    migration_plan.append({
                        'file': location, 
                        'action': 'update_import',
                        'canonical_import': canonical_path
                    })
                    
            except Exception as e:
                print(f"⚠️  Could not analyze {location}: {e}")
        
        print(f"\n📋 Migration plan for {config_name}:")
        for plan in migration_plan:
            action = plan['action']
            file_path = plan['file'].relative_to(self.beardog_root)
            if action == 'remove_definition':
                print(f"   🗑️  Remove struct definition in {file_path}")
            elif action == 'update_import':
                print(f"   🔄 Update import in {file_path}")
                
        # For now, just show the plan. Actual migration would require careful implementation
        print(f"\n⚠️  Dry run mode - no files modified")
        print(f"✅ Migration plan created for {config_name}")
        
        return True

    def create_migration_summary(self) -> None:
        """Create a summary of the migration plan"""
        print("📋 BearDog Configuration Deduplication Summary")
        print("=" * 60)
        
        config_locations = self.find_all_config_structs()
        total_configs = sum(len(locations) for locations in config_locations.values())
        
        target_instances = sum(info['instances'] for info in self.target_configs.values())
        
        print(f"📊 Current State:")
        print(f"   • Total config structs: {total_configs}")
        print(f"   • Unique config names: {len(config_locations)}")
        print(f"   • Target configs for migration: {len(self.target_configs)}")
        print(f"   • Instances to be migrated: {target_instances}")
        
        print(f"\n🎯 Migration Targets:")
        for config_name, info in self.target_configs.items():
            print(f"   • {config_name}: {info['instances']} instances → {info['canonical_path']}")
        
        potential_reduction = target_instances - len(self.target_configs)
        print(f"\n💡 Potential Reduction: {potential_reduction} duplicate definitions eliminated")

def main():
    parser = argparse.ArgumentParser(description='BearDog Configuration Deduplication Tool')
    parser.add_argument('--analyze', action='store_true', help='Analyze duplicate configurations')
    parser.add_argument('--migrate', type=str, help='Migrate specific config (e.g., MonitoringConfig)')
    parser.add_argument('--migrate-all', action='store_true', help='Migrate all target configurations')
    parser.add_argument('--summary', action='store_true', help='Show migration summary')
    
    args = parser.parse_args()
    
    # Find BearDog root directory
    current_dir = Path.cwd()
    beardog_root = None
    
    # Look for beardog root (contains Cargo.toml and crates/ directory)
    for parent in [current_dir] + list(current_dir.parents):
        if (parent / "Cargo.toml").exists() and (parent / "crates").exists():
            beardog_root = parent
            break
    
    if not beardog_root:
        print("❌ Could not find BearDog root directory")
        sys.exit(1)
        
    print(f"🏠 BearDog root: {beardog_root}")
    
    deduplicator = ConfigDeduplicator(beardog_root)
    
    if args.analyze:
        deduplicator.analyze_duplicates()
    elif args.migrate:
        deduplicator.migrate_config(args.migrate)
    elif args.migrate_all:
        for config_name in deduplicator.target_configs:
            print(f"\n{'='*60}")
            deduplicator.migrate_config(config_name)
    elif args.summary:
        deduplicator.create_migration_summary()
    else:
        parser.print_help()

if __name__ == "__main__":
    main() 