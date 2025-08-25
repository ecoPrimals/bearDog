#!/usr/bin/env python3
"""
Manual Tunnel Crate Configuration Migration

This script manually performs the configuration consolidation for the tunnel crate
without relying on sed commands that may have syntax issues.
"""

import os
import re
import sys
from pathlib import Path

def migrate_config_file(file_path: Path) -> bool:
    """Migrate a single configuration file"""
    try:
        content = file_path.read_text()
        original_content = content
        
        # Configuration struct replacements
        replacements = [
            # Security configurations
            (r'pub struct SecurityLevelConfig\s*{[^}]*}', 
             '// MIGRATED: SecurityLevelConfig -> use beardog_types::config::UnifiedSecurityConfig;'),
            (r'pub struct SecurityConfig\s*{[^}]*}', 
             '// MIGRATED: SecurityConfig -> use beardog_types::config::UnifiedSecurityConfig;'),
            (r'pub struct AuthConfig\s*{[^}]*}', 
             '// MIGRATED: AuthConfig -> use beardog_types::config::UnifiedSecurityConfig;'),
            
            # Discovery configurations
            (r'pub struct DiscoveryConfig\s*{[^}]*}', 
             '// MIGRATED: DiscoveryConfig -> use beardog_types::config::UnifiedDiscoveryConfig;'),
            
            # Monitoring configurations
            (r'pub struct MonitoringConfig\s*{[^}]*}', 
             '// MIGRATED: MonitoringConfig -> use beardog_types::config::UnifiedMonitoringConfig;'),
            (r'pub struct HealthCheckConfig\s*{[^}]*}', 
             '// MIGRATED: HealthCheckConfig -> use beardog_types::config::UnifiedMonitoringConfig;'),
            
            # Performance configurations
            (r'pub struct PerformanceConfig\s*{[^}]*}', 
             '// MIGRATED: PerformanceConfig -> use beardog_types::config::UnifiedPerformanceConfig;'),
            
            # Database configurations
            (r'pub struct DatabaseConfig\s*{[^}]*}', 
             '// MIGRATED: DatabaseConfig -> use beardog_types::config::UnifiedDatabaseConfig;'),
            
            # Network configurations
            (r'pub struct NetworkScanConfig\s*{[^}]*}', 
             '// MIGRATED: NetworkScanConfig -> use beardog_types::config::UnifiedNetworkConfig;'),
        ]
        
        # Apply replacements
        for pattern, replacement in replacements:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
        
        # Add canonical imports at the top if we made changes
        if content != original_content:
            # Check if imports section exists
            import_section = []
            if '// MIGRATED:' in content:
                if 'UnifiedSecurityConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedSecurityConfig;')
                if 'UnifiedDiscoveryConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedDiscoveryConfig;')
                if 'UnifiedMonitoringConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedMonitoringConfig;')
                if 'UnifiedPerformanceConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedPerformanceConfig;')
                if 'UnifiedDatabaseConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedDatabaseConfig;')
                if 'UnifiedNetworkConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedNetworkConfig;')
            
            # Insert imports after existing use statements
            if import_section:
                lines = content.split('\n')
                insert_pos = 0
                
                # Find the last use statement
                for i, line in enumerate(lines):
                    if line.strip().startswith('use ') and not line.strip().startswith('use beardog_types::config::'):
                        insert_pos = i + 1
                
                # Insert new imports
                for imp in reversed(import_section):
                    lines.insert(insert_pos, f"// CANONICAL IMPORT: {imp}")
                
                content = '\n'.join(lines)
            
            # Write back the modified content
            file_path.write_text(content)
            return True
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Main migration function"""
    beardog_root = Path.cwd()
    tunnel_crate = beardog_root / "crates" / "beardog-tunnel"
    
    if not tunnel_crate.exists():
        print(f"Error: Tunnel crate not found at {tunnel_crate}")
        sys.exit(1)
    
    print("🚀 Starting manual tunnel crate configuration migration...")
    
    # Files to migrate based on our analysis
    files_to_migrate = [
        "src/universal_hsm/providers/software/config.rs",
        "src/universal_hsm_discovery/mod.rs",
        "src/universal_hsm_discovery/universal_adapter/external_primal_client.rs",
        "src/universal_hsm_discovery/universal_adapter/operation_routing.rs",
        "src/universal_hsm_discovery/universal_adapter/service_discovery.rs",
        "src/tunnel/config.rs",
        "src/tunnel/hsm/config.rs",
        "src/tunnel/hsm/software_hsm/mod.rs",
        "src/tunnel/hsm/manager/config.rs",
        "src/tunnel/hsm/types/config.rs",
        "src/tunnel/hsm/types/canonical.rs",
        "src/tunnel/hsm/universal_discovery/mod.rs",
        "src/tunnel/hsm/universal_discovery/discovery_engine.rs",
    ]
    
    migrated_count = 0
    total_files = 0
    
    for file_rel_path in files_to_migrate:
        file_path = tunnel_crate / file_rel_path
        
        if file_path.exists():
            total_files += 1
            print(f"📝 Migrating {file_rel_path}...")
            
            if migrate_config_file(file_path):
                migrated_count += 1
                print(f"✅ Successfully migrated {file_rel_path}")
            else:
                print(f"⏭️  No changes needed for {file_rel_path}")
        else:
            print(f"⚠️  File not found: {file_rel_path}")
    
    print(f"\n🎉 Migration completed!")
    print(f"📊 Files processed: {total_files}")
    print(f"📊 Files migrated: {migrated_count}")
    
    # Check compilation
    print(f"\n🔍 Checking compilation...")
    os.system("cargo check -p beardog-tunnel --quiet")
    
    print(f"\n✅ Manual tunnel crate migration completed!")
    print(f"📋 Next steps:")
    print(f"   1. Review the migrated files")
    print(f"   2. Update remaining import statements")
    print(f"   3. Run 'cargo check' to verify compilation")

if __name__ == "__main__":
    main() 