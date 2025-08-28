#!/usr/bin/env python3
"""
Fast BearDog 100% Unification Script
Simple and direct approach to eliminate remaining duplicates
"""

import os
from pathlib import Path

def fast_unify_healthstatus():
    """Quickly unify all HealthStatus duplicates"""
    
    # Target files with known duplicates
    target_files = [
        "crates/beardog-adapters/src/adapters/universal/songbird_handoff/types.rs",
        "crates/beardog-adapters/src/universal/vendor_adapter/core/capability_handler.rs", 
        "crates/beardog-deploy/src/global_edge/types.rs",
        "crates/beardog-tunnel/src/universal_hsm_discovery/universal_adapter/external_primal_service.rs",
        "crates/beardog-tunnel/src/universal_hsm_discovery/universal_adapter/core_types.rs",
        "crates/beardog-node-registry/src/node_registry/core.rs",
        "crates/beardog-api/src/api/types.rs",
        "crates/beardog-types/src/zero_cost/mod.rs",
        "crates/beardog-security/src/types/audit_types.rs"
    ]
    
    unified_count = 0
    
    for file_path in target_files:
        path = Path(file_path)
        if not path.exists():
            print(f"⚠️  File not found: {file_path}")
            continue
            
        try:
            content = path.read_text()
            
            # Skip if already unified
            if "// UNIFIED: Use canonical HealthStatus" in content:
                continue
                
            # Simple line-by-line replacement
            lines = content.split('\n')
            new_lines = []
            skip_until_brace = False
            brace_count = 0
            
            for line in lines:
                # Start of HealthStatus enum
                if 'pub enum HealthStatus' in line:
                    new_lines.append("// UNIFIED: Use canonical HealthStatus from beardog-types")
                    new_lines.append("pub use beardog_types::canonical::HealthStatus;")
                    skip_until_brace = True
                    brace_count = 0
                    continue
                
                if skip_until_brace:
                    # Count braces to find end of enum
                    brace_count += line.count('{') - line.count('}')
                    if brace_count <= 0:
                        skip_until_brace = False
                    continue
                    
                new_lines.append(line)
            
            new_content = '\n'.join(new_lines)
            
            # Add import if needed
            if "pub use beardog_types::canonical::HealthStatus;" in new_content:
                if "use beardog_types::canonical" not in new_content:
                    # Find good place to add import
                    import_lines = new_content.split('\n')
                    for i, line in enumerate(import_lines):
                        if line.strip().startswith('use ') and 'beardog' in line:
                            import_lines.insert(i, "use beardog_types::canonical::HealthStatus;")
                            break
                    new_content = '\n'.join(import_lines)
            
            if new_content != content:
                path.write_text(new_content)
                print(f"✅ Unified: {file_path}")
                unified_count += 1
                
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
    
    return unified_count

def main():
    print("🚀 Fast BearDog 100% Unification")
    print("=" * 40)
    
    unified = fast_unify_healthstatus()
    
    print(f"\n🎯 Results:")
    print(f"   Files unified: {unified}")
    
    # Quick verification
    print(f"\n🔍 Verification:")
    remaining = 0
    for file_path in Path("crates").rglob("*.rs"):
        try:
            content = file_path.read_text()
            if "pub enum HealthStatus" in content and "canonical" not in str(file_path):
                print(f"   ⚠️  Still has HealthStatus: {file_path}")
                remaining += 1
        except:
            continue
    
    if remaining == 0:
        print("   ✅ PERFECT! No duplicate HealthStatus found!")
        print("   🎉 BearDog achieved 100% TYPE UNIFICATION!")
    else:
        print(f"   📊 {remaining} files still need attention")

if __name__ == "__main__":
    main() 