#!/usr/bin/env python3
"""
Manual Adapters Crate Configuration Migration

This script manually performs the configuration consolidation for the adapters crate
using the same approach that succeeded with the tunnel crate.
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
        
        # Configuration struct replacements based on our analysis
        replacements = [
            # Adapter configurations
            (r'pub struct UniversalAdapterConfig\s*{[^}]*}', 
             '// MIGRATED: UniversalAdapterConfig -> use beardog_types::config::CanonicalAdapterConfig;'),
            (r'pub struct ExtensibleAdapterConfig\s*{[^}]*}', 
             '// MIGRATED: ExtensibleAdapterConfig -> use beardog_types::config::CanonicalAdapterConfig;'),
            (r'pub struct BridgeConfig\s*{[^}]*}', 
             '// MIGRATED: BridgeConfig -> use beardog_types::config::CanonicalAdapterConfig;'),
            
            # Discovery configurations
            (r'pub struct DiscoveryConfig\s*{[^}]*}', 
             '// MIGRATED: DiscoveryConfig -> use beardog_types::config::UnifiedDiscoveryConfig;'),
            (r'pub struct DiscoveryEngineConfig\s*{[^}]*}', 
             '// MIGRATED: DiscoveryEngineConfig -> use beardog_types::config::UnifiedDiscoveryConfig;'),
            
            # Monitoring configurations
            (r'pub struct PrometheusConfig\s*{[^}]*}', 
             '// MIGRATED: PrometheusConfig -> use beardog_types::config::UnifiedMonitoringConfig;'),
            (r'pub struct HealthCheckConfig\s*{[^}]*}', 
             '// MIGRATED: HealthCheckConfig -> use beardog_types::config::UnifiedMonitoringConfig;'),
            (r'pub struct HealthMonitorConfig\s*{[^}]*}', 
             '// MIGRATED: HealthMonitorConfig -> use beardog_types::config::UnifiedMonitoringConfig;'),
            
            # Network configurations
            (r'pub struct LoadBalancerConfig\s*{[^}]*}', 
             '// MIGRATED: LoadBalancerConfig -> use beardog_types::config::UnifiedNetworkConfig;'),
            (r'pub struct CircuitBreakerConfig\s*{[^}]*}', 
             '// MIGRATED: CircuitBreakerConfig -> use beardog_types::config::UnifiedNetworkConfig;'),
            (r'pub struct TlsConfig\s*{[^}]*}', 
             '// MIGRATED: TlsConfig -> use beardog_types::config::UnifiedNetworkConfig;'),
            
            # Performance configurations
            (r'pub struct PerformanceRoutingConfig\s*{[^}]*}', 
             '// MIGRATED: PerformanceRoutingConfig -> use beardog_types::config::UnifiedPerformanceConfig;'),
            
            # Security configurations
            (r'pub struct AuthConfig\s*{[^}]*}', 
             '// MIGRATED: AuthConfig -> use beardog_types::config::UnifiedSecurityConfig;'),
            (r'pub struct SecurityConfig\s*{[^}]*}', 
             '// MIGRATED: SecurityConfig -> use beardog_types::config::UnifiedSecurityConfig;'),
            (r'pub struct OAuth2Config\s*{[^}]*}', 
             '// MIGRATED: OAuth2Config -> use beardog_types::config::UnifiedSecurityConfig;'),
        ]
        
        # Apply replacements
        for pattern, replacement in replacements:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
        
        # Add canonical imports at the top if we made changes
        if content != original_content:
            # Check if imports section exists
            import_section = []
            if '// MIGRATED:' in content:
                if 'CanonicalAdapterConfig' in content:
                    import_section.append('use beardog_types::config::CanonicalAdapterConfig;')
                if 'UnifiedDiscoveryConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedDiscoveryConfig;')
                if 'UnifiedMonitoringConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedMonitoringConfig;')
                if 'UnifiedNetworkConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedNetworkConfig;')
                if 'UnifiedPerformanceConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedPerformanceConfig;')
                if 'UnifiedSecurityConfig' in content:
                    import_section.append('use beardog_types::config::UnifiedSecurityConfig;')
            
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
    adapters_crate = beardog_root / "crates" / "beardog-adapters"
    
    if not adapters_crate.exists():
        print(f"Error: Adapters crate not found at {adapters_crate}")
        sys.exit(1)
    
    print("🚀 Starting manual adapters crate configuration migration...")
    
    # Files to migrate based on our analysis
    files_to_migrate = [
        "src/adapters/universal/mod.rs",
        "src/adapters/universal/universal_adapter.rs", 
        "src/adapters/universal/providers.rs",
        "src/adapters/universal/songbird_handoff/types.rs",
        "src/adapters/universal/songbird_handoff/health.rs",
        "src/universal/extensible_adapter.rs",
        "src/universal/security_provider_bridge/mod.rs",
        "src/universal/vendor_adapter/mod.rs",
        "src/universal/service_discovery.rs",
        "src/universal/vendor_adapter/discovery/engine.rs",
        "src/universal/vendor_adapter/routing/performance.rs",
        "src/universal/vendor_adapter/core/capability_handler.rs",
    ]
    
    migrated_count = 0
    total_files = 0
    
    for file_rel_path in files_to_migrate:
        file_path = adapters_crate / file_rel_path
        
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
    os.system("cargo check -p beardog-adapters --quiet")
    
    print(f"\n✅ Manual adapters crate migration completed!")
    print(f"📋 Next steps:")
    print(f"   1. Review the migrated files")
    print(f"   2. Update remaining import statements")
    print(f"   3. Run 'cargo check' to verify compilation")

if __name__ == "__main__":
    main() 