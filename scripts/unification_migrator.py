#!/usr/bin/env python3
"""
BearDog Unification Migration Script
Migrates fragmented provider traits and configuration structs to unified system
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import json

class UnificationMigrator:
    def __init__(self, codebase_path: str = "."):
        self.codebase_path = Path(codebase_path)
        self.migration_report = {
            "provider_traits": {"migrated": 0, "deprecated": 0, "files_updated": []},
            "config_structs": {"migrated": 0, "deprecated": 0, "files_updated": []},
            "imports": {"updated": 0, "files_updated": []},
            "errors": []
        }
        
        # Trait migration mappings
        self.trait_migrations = {
            "UniversalPrimalProvider": "UniversalProvider",
            "ExternalSystemProvider": "UniversalProvider", 
            "UniversalServiceProvider": "UniversalProvider",
            "SafeHardwareProvider": "PlatformProvider",
            "SafeIOSProvider": "PlatformProvider",
            "GamingSecurityProvider": "PlatformProvider",
            "SimpleCacheProvider": "EnhancedCacheProvider",
        }
        
        # Config migration mappings
        self.config_migrations = {
            "SecurityProcessorConfig": "UnifiedProcessorConfig",
            "PolicyProcessorConfig": "UnifiedProcessorConfig",
            "SystemProcessorConfig": "UnifiedProcessorConfig",
            "KeyManagementConfig": "UnifiedProcessorConfig",
            "RegistryProcessorConfig": "UnifiedProcessorConfig",
            "RecoveryConfig": "UnifiedRecoveryConfig",
            "SocialRecoveryConfig": "UnifiedRecoveryConfig",
            "FederationRecoveryConfig": "UnifiedRecoveryConfig",
            "LoadTestConfiguration": "UnifiedTestingConfig",
            "StressTestConfiguration": "UnifiedTestingConfig",
            "PenetrationTestConfiguration": "UnifiedTestingConfig",
            "AuthConfig": "UnifiedAuthConfig",
            "VerificationConfig": "UnifiedAuthConfig",
            "SessionConfig": "UnifiedAuthConfig",
        }

    def migrate_provider_traits(self) -> None:
        """Migrate deprecated provider traits to unified traits"""
        print("🔄 Migrating Provider Traits...")
        
        rust_files = list(self.codebase_path.rglob("*.rs"))
        rust_files = [f for f in rust_files if "/target/" not in str(f)]
        
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                original_content = content
                updated = False
                
                # Migrate trait definitions
                for old_trait, new_trait in self.trait_migrations.items():
                    # Match trait definitions
                    trait_def_pattern = rf"pub trait {re.escape(old_trait)}\b"
                    if re.search(trait_def_pattern, content):
                        # Add deprecation notice
                        deprecation_notice = f'#[deprecated(since = "3.1.0", note = "Use {new_trait} instead")]\n'
                        content = re.sub(
                            trait_def_pattern,
                            f"{deprecation_notice}pub trait {old_trait}",
                            content
                        )
                        updated = True
                        self.migration_report["provider_traits"]["deprecated"] += 1
                
                # Migrate trait usage in implementations and bounds
                for old_trait, new_trait in self.trait_migrations.items():
                    # Match trait bounds and implementations
                    patterns = [
                        rf"\b{re.escape(old_trait)}\b(?=\s*[+>])",  # Trait bounds
                        rf"impl\s+{re.escape(old_trait)}\s+for",   # Implementations
                        rf":\s*{re.escape(old_trait)}\b",          # Inheritance
                    ]
                    
                    for pattern in patterns:
                        if re.search(pattern, content):
                            content = re.sub(pattern, lambda m: m.group(0).replace(old_trait, new_trait), content)
                            updated = True
                
                if updated and content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    
                    self.migration_report["provider_traits"]["files_updated"].append(str(file_path))
                    self.migration_report["provider_traits"]["migrated"] += 1
                    print(f"  ✅ Updated {file_path.relative_to(self.codebase_path)}")
                    
            except Exception as e:
                error_msg = f"Error processing {file_path}: {e}"
                self.migration_report["errors"].append(error_msg)
                print(f"  ❌ {error_msg}")

    def migrate_config_structs(self) -> None:
        """Migrate deprecated config structs to unified configs"""
        print("🔄 Migrating Configuration Structs...")
        
        rust_files = list(self.codebase_path.rglob("*.rs"))
        rust_files = [f for f in rust_files if "/target/" not in str(f)]
        
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                original_content = content
                updated = False
                
                # Migrate struct definitions
                for old_config, new_config in self.config_migrations.items():
                    # Match struct definitions
                    struct_def_pattern = rf"pub struct {re.escape(old_config)}\b"
                    if re.search(struct_def_pattern, content):
                        # Add deprecation notice
                        deprecation_notice = f'#[deprecated(since = "3.1.0", note = "Use {new_config} instead")]\n'
                        content = re.sub(
                            struct_def_pattern,
                            f"{deprecation_notice}pub struct {old_config}",
                            content
                        )
                        updated = True
                        self.migration_report["config_structs"]["deprecated"] += 1
                
                # Migrate struct usage
                for old_config, new_config in self.config_migrations.items():
                    # Match type annotations, field types, etc.
                    usage_patterns = [
                        rf"\b{re.escape(old_config)}\b(?=\s*[,;>)])",  # Type usage
                        rf":\s*{re.escape(old_config)}\b",             # Field types
                    ]
                    
                    for pattern in usage_patterns:
                        if re.search(pattern, content):
                            content = re.sub(pattern, lambda m: m.group(0).replace(old_config, new_config), content)
                            updated = True
                
                if updated and content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    
                    self.migration_report["config_structs"]["files_updated"].append(str(file_path))
                    self.migration_report["config_structs"]["migrated"] += 1
                    print(f"  ✅ Updated {file_path.relative_to(self.codebase_path)}")
                    
            except Exception as e:
                error_msg = f"Error processing {file_path}: {e}"
                self.migration_report["errors"].append(error_msg)
                print(f"  ❌ {error_msg}")

    def update_imports(self) -> None:
        """Update import statements to use unified modules"""
        print("🔄 Updating Import Statements...")
        
        rust_files = list(self.codebase_path.rglob("*.rs"))
        rust_files = [f for f in rust_files if "/target/" not in str(f)]
        
        import_updates = {
            # Provider trait imports
            "beardog_traits::canonical::{UniversalPrimalProvider}": "beardog_traits::canonical::{UniversalProvider}",
            "beardog_traits::canonical::{ExternalSystemProvider}": "beardog_traits::canonical::{UniversalProvider}",
            "beardog_traits::canonical::{SimpleCacheProvider}": "beardog_traits::canonical::{EnhancedCacheProvider}",
            
            # Config imports
            "beardog_types::config::{SecurityProcessorConfig}": "beardog_types::config::{UnifiedProcessorConfig}",
            "beardog_types::config::{RecoveryConfig}": "beardog_types::config::{UnifiedRecoveryConfig}",
            "beardog_types::config::{AuthConfig}": "beardog_types::config::{UnifiedAuthConfig}",
        }
        
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                original_content = content
                updated = False
                
                for old_import, new_import in import_updates.items():
                    if old_import in content:
                        content = content.replace(old_import, new_import)
                        updated = True
                
                if updated and content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    
                    self.migration_report["imports"]["files_updated"].append(str(file_path))
                    self.migration_report["imports"]["updated"] += 1
                    print(f"  ✅ Updated imports in {file_path.relative_to(self.codebase_path)}")
                    
            except Exception as e:
                error_msg = f"Error updating imports in {file_path}: {e}"
                self.migration_report["errors"].append(error_msg)
                print(f"  ❌ {error_msg}")

    def generate_migration_guide(self) -> None:
        """Generate a migration guide for manual updates"""
        guide_content = """# BearDog Unification Migration Guide

## Provider Trait Migrations

### Deprecated → Unified Mappings

"""
        
        for old_trait, new_trait in self.trait_migrations.items():
            guide_content += f"- `{old_trait}` → `{new_trait}`\n"
        
        guide_content += """
## Configuration Struct Migrations

### Deprecated → Unified Mappings

"""
        
        for old_config, new_config in self.config_migrations.items():
            guide_content += f"- `{old_config}` → `{new_config}`\n"
        
        guide_content += """
## Migration Steps

1. **Update Imports**: Change import statements to use new unified types
2. **Update Type Annotations**: Replace deprecated types with unified equivalents  
3. **Update Implementations**: Migrate trait implementations to new unified traits
4. **Test Thoroughly**: Ensure all functionality works with new unified system
5. **Remove Deprecated**: After migration period, remove deprecated types

## Example Migration

```rust
// Before
use beardog_traits::canonical::UniversalPrimalProvider;
use beardog_types::config::SecurityProcessorConfig;

impl UniversalPrimalProvider for MyProvider {
    // implementation
}

// After  
use beardog_traits::canonical::UniversalProvider;
use beardog_types::config::UnifiedProcessorConfig;

impl UniversalProvider for MyProvider {
    // updated implementation
}
```
"""
        
        with open("UNIFICATION_MIGRATION_GUIDE.md", 'w') as f:
            f.write(guide_content)
        
        print("📋 Generated UNIFICATION_MIGRATION_GUIDE.md")

    def run_migration(self) -> None:
        """Run the complete migration process"""
        print("🚀 BearDog Unification Migration")
        print("=" * 50)
        
        # Run migration steps
        self.migrate_provider_traits()
        self.migrate_config_structs() 
        self.update_imports()
        self.generate_migration_guide()
        
        # Generate report
        self.generate_report()

    def generate_report(self) -> None:
        """Generate migration report"""
        print("\n📊 Migration Report")
        print("=" * 30)
        
        print(f"Provider Traits:")
        print(f"  - Migrated: {self.migration_report['provider_traits']['migrated']}")
        print(f"  - Deprecated: {self.migration_report['provider_traits']['deprecated']}")
        print(f"  - Files Updated: {len(self.migration_report['provider_traits']['files_updated'])}")
        
        print(f"Config Structs:")
        print(f"  - Migrated: {self.migration_report['config_structs']['migrated']}")
        print(f"  - Deprecated: {self.migration_report['config_structs']['deprecated']}")
        print(f"  - Files Updated: {len(self.migration_report['config_structs']['files_updated'])}")
        
        print(f"Imports:")
        print(f"  - Updated: {self.migration_report['imports']['updated']}")
        print(f"  - Files Updated: {len(self.migration_report['imports']['files_updated'])}")
        
        if self.migration_report["errors"]:
            print(f"Errors: {len(self.migration_report['errors'])}")
            for error in self.migration_report["errors"][:5]:  # Show first 5 errors
                print(f"  ❌ {error}")
        
        # Save detailed report
        with open("unification_migration_report.json", 'w') as f:
            json.dump(self.migration_report, f, indent=2)
        
        print("\n✅ Migration Complete!")
        print("📄 Detailed report saved to: unification_migration_report.json")

if __name__ == "__main__":
    migrator = UnificationMigrator()
    migrator.run_migration() 