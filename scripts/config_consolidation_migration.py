#!/usr/bin/env python3
"""
BearDog Configuration Consolidation Migration Script

This script helps migrate from fragmented configuration structs (511 total)
to canonical unified configurations (<50 total).

Usage:
    python3 config_consolidation_migration.py --crate beardog-tunnel --dry-run
    python3 config_consolidation_migration.py --crate beardog-adapters --apply
"""

import os
import re
import sys
import argparse
import json
from pathlib import Path
from typing import Dict, List, Tuple, Set
from dataclasses import dataclass

@dataclass
class ConfigStruct:
    name: str
    file_path: str
    line_number: int
    content: str
    canonical_replacement: str

@dataclass 
class MigrationReport:
    crate_name: str
    total_configs_found: int
    configs_to_migrate: int
    canonical_configs_used: int
    files_affected: int
    estimated_loc_reduction: int

class ConfigConsolidationMigrator:
    """Migrates fragmented configurations to canonical types"""
    
    def __init__(self, beardog_root: Path):
        self.beardog_root = beardog_root
        self.crates_dir = beardog_root / "crates"
        
        # Configuration migration mappings
        self.config_mappings = {
            # HSM configurations -> CanonicalHsmConfig
            "HsmConfig": "CanonicalHsmConfig",
            "HsmCoreConfig": "CanonicalHsmConfig",
            "HsmSecurityConfig": "CanonicalHsmConfig", 
            "HsmPerformanceConfig": "CanonicalHsmConfig",
            "HsmProviderConfig": "CanonicalHsmConfig",
            "HsmFailoverConfig": "CanonicalHsmConfig",
            "HsmMonitoringConfig": "CanonicalHsmConfig",
            "HsmAuditConfig": "CanonicalHsmConfig",
            
            # Network configurations -> UnifiedNetworkConfig
            "NetworkConfig": "UnifiedNetworkConfig",
            "NetworkPortsConfig": "UnifiedNetworkConfig",
            "NetworkScanConfig": "UnifiedNetworkConfig", 
            "NetworkResourceConfig": "UnifiedNetworkConfig",
            "TlsConfig": "UnifiedNetworkConfig",
            "LoadBalancerConfig": "UnifiedNetworkConfig",
            "CircuitBreakerConfig": "UnifiedNetworkConfig",
            
            # Security configurations -> UnifiedSecurityConfig
            "SecurityConfig": "UnifiedSecurityConfig",
            "SecurityLevelConfig": "UnifiedSecurityConfig",
            "SecuritySentinelConfig": "UnifiedSecurityConfig",
            "SecurityProcessorConfig": "UnifiedSecurityConfig",
            "AuthConfig": "UnifiedSecurityConfig",
            "OAuth2Config": "UnifiedSecurityConfig",
            
            # Monitoring configurations -> UnifiedMonitoringConfig
            "MonitoringConfig": "UnifiedMonitoringConfig",
            "MetricCollectionConfig": "UnifiedMonitoringConfig",
            "AlertProcessingConfig": "UnifiedMonitoringConfig",
            "PrometheusConfig": "UnifiedMonitoringConfig",
            "HealthCheckConfig": "UnifiedMonitoringConfig",
            "HealthMonitorConfig": "UnifiedMonitoringConfig",
            
            # Performance configurations -> UnifiedPerformanceConfig
            "PerformanceConfig": "UnifiedPerformanceConfig",
            "PerformanceRoutingConfig": "UnifiedPerformanceConfig",
            "LoadTestConfiguration": "UnifiedPerformanceConfig",
            
            # Adapter configurations -> CanonicalAdapterConfig
            "UniversalAdapterConfig": "CanonicalAdapterConfig",
            "VendorAdapterConfig": "CanonicalAdapterConfig",
            "ExtensibleAdapterConfig": "CanonicalAdapterConfig",
            "BridgeConfig": "CanonicalAdapterConfig",
            
            # Database configurations -> UnifiedDatabaseConfig
            "DatabaseConfig": "UnifiedDatabaseConfig",
            
            # Discovery configurations -> UnifiedDiscoveryConfig
            "DiscoveryConfig": "UnifiedDiscoveryConfig",
            "DiscoveryEngineConfig": "UnifiedDiscoveryConfig",
            "ServiceDiscoveryConfig": "UnifiedDiscoveryConfig",
        }
        
        # Import mappings for canonical types
        self.canonical_imports = {
            "CanonicalHsmConfig": "use beardog_types::config::CanonicalHsmConfig;",
            "CanonicalTunnelConfig": "use beardog_types::config::CanonicalTunnelConfig;",
            "CanonicalAdapterConfig": "use beardog_types::config::CanonicalAdapterConfig;",
            "UnifiedNetworkConfig": "use beardog_types::config::UnifiedNetworkConfig;",
            "UnifiedSecurityConfig": "use beardog_types::config::UnifiedSecurityConfig;",
            "UnifiedMonitoringConfig": "use beardog_types::config::UnifiedMonitoringConfig;",
            "UnifiedPerformanceConfig": "use beardog_types::config::UnifiedPerformanceConfig;",
            "UnifiedDatabaseConfig": "use beardog_types::config::UnifiedDatabaseConfig;",
            "UnifiedDiscoveryConfig": "use beardog_types::config::UnifiedDiscoveryConfig;",
        }

    def find_config_structs(self, crate_path: Path) -> List[ConfigStruct]:
        """Find all configuration structs in a crate"""
        configs = []
        
        for rs_file in crate_path.rglob("*.rs"):
            try:
                content = rs_file.read_text()
                lines = content.split('\n')
                
                for i, line in enumerate(lines):
                    # Match pub struct *Config patterns
                    match = re.search(r'pub struct\s+(\w*Config\w*)', line)
                    if match:
                        config_name = match.group(1)
                        canonical = self.config_mappings.get(config_name, "Unknown")
                        
                        configs.append(ConfigStruct(
                            name=config_name,
                            file_path=str(rs_file),
                            line_number=i + 1,
                            content=line.strip(),
                            canonical_replacement=canonical
                        ))
            except Exception as e:
                print(f"Error reading {rs_file}: {e}")
                
        return configs

    def generate_migration_plan(self, crate_name: str) -> Tuple[List[ConfigStruct], MigrationReport]:
        """Generate migration plan for a crate"""
        crate_path = self.crates_dir / crate_name
        
        if not crate_path.exists():
            raise ValueError(f"Crate {crate_name} not found at {crate_path}")
            
        configs = self.find_config_structs(crate_path)
        
        # Filter configs that can be migrated
        migratable_configs = [c for c in configs if c.canonical_replacement != "Unknown"]
        
        # Count unique files affected
        affected_files = len(set(c.file_path for c in migratable_configs))
        
        # Count unique canonical configs used
        canonical_configs_used = len(set(c.canonical_replacement for c in migratable_configs))
        
        # Estimate lines of code reduction (assume 10-50 lines per config struct)
        estimated_loc_reduction = len(migratable_configs) * 25  # Average estimate
        
        report = MigrationReport(
            crate_name=crate_name,
            total_configs_found=len(configs),
            configs_to_migrate=len(migratable_configs),
            canonical_configs_used=canonical_configs_used,
            files_affected=affected_files,
            estimated_loc_reduction=estimated_loc_reduction
        )
        
        return migratable_configs, report

    def generate_migration_script(self, configs: List[ConfigStruct], crate_name: str) -> str:
        """Generate a migration script for the configs"""
        script_lines = [
            f"#!/bin/bash",
            f"# Migration script for {crate_name} configuration consolidation",
            f"# Generated by BearDog Configuration Consolidation Migrator",
            f"",
            f"set -e",
            f"",
            f"CRATE_DIR=\"crates/{crate_name}\"",
            f"",
            f"echo \"Starting configuration migration for {crate_name}...\"",
            f"",
        ]
        
        # Group configs by file for efficient processing
        configs_by_file = {}
        for config in configs:
            file_path = config.file_path
            if file_path not in configs_by_file:
                configs_by_file[file_path] = []
            configs_by_file[file_path].append(config)
        
        # Generate file-specific migration commands
        for file_path, file_configs in configs_by_file.items():
            relative_path = Path(file_path).relative_to(self.beardog_root)
            script_lines.extend([
                f"echo \"Migrating configurations in {relative_path}...\"",
                f"",
            ])
            
            # Add import statements for canonical configs
            canonical_types_used = set(c.canonical_replacement for c in file_configs)
            for canonical_type in canonical_types_used:
                if canonical_type in self.canonical_imports:
                    import_line = self.canonical_imports[canonical_type]
                    script_lines.append(f"# Add import: {import_line}")
            
            script_lines.append("")
            
            # Generate sed commands to replace struct definitions
            for config in file_configs:
                old_pattern = f"pub struct {config.name}"
                new_comment = f"// MIGRATED: {config.name} -> {config.canonical_replacement}"
                script_lines.extend([
                    f"# Replace {config.name} with canonical type reference",
                    f"sed -i 's/{old_pattern}/{new_comment}/g' \"{relative_path}\"",
                ])
            
            script_lines.append("")
        
        script_lines.extend([
            f"echo \"Configuration migration completed for {crate_name}\"",
            f"echo \"Please review changes and update imports manually\"",
            f"echo \"Run 'cargo check' to verify compilation\"",
        ])
        
        return "\n".join(script_lines)

    def print_migration_report(self, report: MigrationReport, configs: List[ConfigStruct]):
        """Print a detailed migration report"""
        print(f"\n🔧 Configuration Consolidation Report: {report.crate_name}")
        print("=" * 60)
        
        print(f"📊 Summary:")
        print(f"  • Total config structs found: {report.total_configs_found}")
        print(f"  • Configs ready for migration: {report.configs_to_migrate}")
        print(f"  • Canonical configs to use: {report.canonical_configs_used}")
        print(f"  • Files affected: {report.files_affected}")
        print(f"  • Estimated LOC reduction: {report.estimated_loc_reduction}")
        
        if report.configs_to_migrate > 0:
            consolidation_ratio = report.total_configs_found / report.canonical_configs_used
            print(f"  • Consolidation ratio: {consolidation_ratio:.1f}:1")
        
        print(f"\n📋 Configuration Migration Plan:")
        
        # Group by canonical type
        by_canonical = {}
        for config in configs:
            canonical = config.canonical_replacement
            if canonical not in by_canonical:
                by_canonical[canonical] = []
            by_canonical[canonical].append(config)
        
        for canonical_type, config_list in sorted(by_canonical.items()):
            print(f"\n  🎯 {canonical_type} ({len(config_list)} configs):")
            for config in config_list[:5]:  # Show first 5
                rel_path = Path(config.file_path).relative_to(self.beardog_root)
                print(f"    • {config.name} ({rel_path}:{config.line_number})")
            if len(config_list) > 5:
                print(f"    • ... and {len(config_list) - 5} more")
        
        print(f"\n🚀 Next Steps:")
        print(f"  1. Review the migration plan above")
        print(f"  2. Run with --apply to generate migration script")
        print(f"  3. Execute the migration script")
        print(f"  4. Update imports to use canonical types")
        print(f"  5. Run 'cargo check' to verify compilation")

def main():
    parser = argparse.ArgumentParser(
        description="BearDog Configuration Consolidation Migration Tool"
    )
    parser.add_argument(
        "--crate", 
        required=True, 
        help="Crate name to migrate (e.g., beardog-tunnel, beardog-adapters)"
    )
    parser.add_argument(
        "--dry-run", 
        action="store_true", 
        help="Show migration plan without generating scripts"
    )
    parser.add_argument(
        "--apply", 
        action="store_true", 
        help="Generate migration script"
    )
    
    args = parser.parse_args()
    
    if not (args.dry_run or args.apply):
        print("Error: Must specify either --dry-run or --apply")
        sys.exit(1)
    
    # Find BearDog root directory
    current_dir = Path.cwd()
    beardog_root = None
    
    # Look for Cargo.toml with BearDog workspace
    for path in [current_dir] + list(current_dir.parents):
        cargo_toml = path / "Cargo.toml"
        if cargo_toml.exists():
            content = cargo_toml.read_text()
            if "beardog" in content.lower() and "workspace" in content:
                beardog_root = path
                break
    
    if not beardog_root:
        print("Error: Could not find BearDog root directory")
        sys.exit(1)
    
    print(f"🔍 Found BearDog root: {beardog_root}")
    
    migrator = ConfigConsolidationMigrator(beardog_root)
    
    try:
        configs, report = migrator.generate_migration_plan(args.crate)
        
        migrator.print_migration_report(report, configs)
        
        if args.apply and report.configs_to_migrate > 0:
            script = migrator.generate_migration_script(configs, args.crate)
            script_path = beardog_root / f"scripts/migrate_{args.crate}_configs.sh"
            
            # Create scripts directory if it doesn't exist
            script_path.parent.mkdir(exist_ok=True)
            
            script_path.write_text(script)
            script_path.chmod(0o755)  # Make executable
            
            print(f"\n✅ Migration script generated: {script_path}")
            print(f"   Run: ./{script_path.relative_to(beardog_root)}")
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main() 