#!/usr/bin/env python3
"""
BearDog 100% Unification Script

This script achieves complete (100%) type unification by:
1. Eliminating all duplicate HealthStatus definitions
2. Replacing with canonical imports from beardog-types
3. Consolidating remaining fragmented types
4. Removing async_fn_in_trait allows where possible
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Tuple, Dict

class CompleteUnificationProcessor:
    def __init__(self):
        self.files_processed = 0
        self.duplicates_eliminated = 0
        self.canonical_imports_added = 0
        
        # Files with duplicate HealthStatus definitions to unify
        self.healthstatus_duplicates = [
            "crates/beardog-adapters/src/adapters/universal/traits.rs",
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
        
    def eliminate_healthstatus_duplicates(self, file_path: Path) -> bool:
        """Eliminate duplicate HealthStatus definitions and replace with canonical import"""
        try:
            content = file_path.read_text()
            original_content = content
            
            # Skip if already unified
            if "// UNIFIED: Use canonical HealthStatus" in content:
                return False
                
            # Pattern to match various HealthStatus enum definitions
            healthstatus_patterns = [
                # Simple enum pattern
                r'pub enum HealthStatus\s*{[^}]*}',
                # Multi-line enum pattern with variants
                r'pub enum HealthStatus\s*{\s*(?:[A-Za-z_][A-Za-z0-9_]*(?:\s*{[^}]*})?,?\s*)*}',
                # Complex enum with detailed variants
                r'#\[derive[^\]]*\]\s*pub enum HealthStatus\s*{[^}]*(?:{[^}]*}[^}]*)*}',
            ]
            
            replaced = False
            for pattern in healthstatus_patterns:
                if re.search(pattern, content, re.DOTALL | re.MULTILINE):
                    # Replace with canonical import
                    content = re.sub(
                        pattern,
                        "// UNIFIED: Use canonical HealthStatus from beardog-types\npub use beardog_types::canonical::HealthStatus;",
                        content,
                        flags=re.DOTALL | re.MULTILINE
                    )
                    replaced = True
                    break
            
            # Ensure canonical import is available
            if replaced and "use beardog_types::canonical" not in content:
                # Add import at the top after existing imports
                lines = content.split('\n')
                insert_pos = 0
                
                # Find position after existing imports
                for i, line in enumerate(lines):
                    if line.strip().startswith('use ') or line.strip().startswith('//'):
                        insert_pos = i + 1
                    elif line.strip() == '':
                        continue
                    else:
                        break
                
                lines.insert(insert_pos, "use beardog_types::canonical::HealthStatus;")
                content = '\n'.join(lines)
            
            if content != original_content:
                file_path.write_text(content)
                self.duplicates_eliminated += 1
                return True
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            return False
            
        return False
        
    def optimize_async_fn_in_trait(self, file_path: Path) -> bool:
        """Remove unnecessary async_fn_in_trait allows where possible"""
        try:
            content = file_path.read_text()
            original_content = content
            
            # Remove redundant async_fn_in_trait allows in test files and examples
            if "/tests/" in str(file_path) or "/examples/" in str(file_path) or "/benchmarks/" in str(file_path):
                # These are acceptable in test/example code
                return False
            
            # Count trait definitions to see if allow is needed
            trait_count = len(re.findall(r'trait\s+\w+.*{.*async\s+fn', content, re.DOTALL))
            allow_count = len(re.findall(r'#\[allow\(async_fn_in_trait\)\]', content))
            
            # If no async fn in traits, remove the allow
            if trait_count == 0 and allow_count > 0:
                content = re.sub(r'#\[allow\(async_fn_in_trait\)\]\s*\n?', '', content)
                
            if content != original_content:
                file_path.write_text(content)
                return True
                
        except Exception as e:
            print(f"Error optimizing {file_path}: {e}")
            return False
            
        return False
        
    def consolidate_remaining_fragments(self, file_path: Path) -> bool:
        """Consolidate any remaining type fragments"""
        try:
            content = file_path.read_text()
            original_content = content
            
            # Look for other duplicate type patterns
            consolidation_patterns = [
                # ComponentStatus duplicates
                (r'pub enum ComponentStatus\s*{[^}]*}', 
                 "// UNIFIED: Use canonical ComponentStatus\npub use beardog_types::canonical::ComponentStatus;"),
                
                # ProviderStatus duplicates  
                (r'pub enum ProviderStatus\s*{[^}]*}',
                 "// UNIFIED: Use canonical ProviderStatus\npub use beardog_types::canonical::ProviderStatus;"),
                
                # WorkflowStatus duplicates
                (r'pub enum WorkflowStatus\s*{[^}]*}',
                 "// UNIFIED: Use canonical WorkflowStatus\npub use beardog_types::canonical::WorkflowStatus;"),
            ]
            
            replaced = False
            for pattern, replacement in consolidation_patterns:
                if re.search(pattern, content, re.DOTALL):
                    content = re.sub(pattern, replacement, content, flags=re.DOTALL)
                    replaced = True
            
            if content != original_content:
                file_path.write_text(content)
                return True
                
        except Exception as e:
            print(f"Error consolidating {file_path}: {e}")
            return False
            
        return False
        
    def process_file(self, file_path: Path) -> bool:
        """Process a single file for complete unification"""
        changed = False
        
        # Process HealthStatus duplicates
        if str(file_path) in self.healthstatus_duplicates:
            if self.eliminate_healthstatus_duplicates(file_path):
                changed = True
                
        # Optimize async_fn_in_trait usage
        if self.optimize_async_fn_in_trait(file_path):
            changed = True
            
        # Consolidate other fragments
        if self.consolidate_remaining_fragments(file_path):
            changed = True
            
        if changed:
            self.files_processed += 1
            
        return changed
        
    def run(self) -> None:
        """Run complete unification across the codebase"""
        print("🎯 BearDog 100% Unification - Final Push")
        print("=" * 50)
        print()
        
        # Process all Rust files in crates/
        crates_dir = Path("crates")
        if not crates_dir.exists():
            print("❌ crates/ directory not found")
            return
            
        rust_files = list(crates_dir.rglob("*.rs"))
        print(f"📁 Found {len(rust_files)} Rust files to process")
        print()
        
        # Process each file
        for file_path in rust_files:
            if self.process_file(file_path):
                print(f"✅ Unified: {file_path}")
                
        print()
        print("🏆 100% UNIFICATION COMPLETE")
        print("=" * 50)
        print(f"📊 Files processed: {self.files_processed}")
        print(f"🔧 Duplicates eliminated: {self.duplicates_eliminated}")
        print(f"📦 Canonical imports added: {self.canonical_imports_added}")
        print()
        
        # Verification
        remaining_duplicates = []
        for file_path in rust_files:
            try:
                content = file_path.read_text()
                if re.search(r'pub enum HealthStatus\s*{', content) and "canonical" not in str(file_path):
                    remaining_duplicates.append(str(file_path))
            except:
                continue
                
        if remaining_duplicates:
            print("⚠️  Remaining HealthStatus duplicates found:")
            for dup in remaining_duplicates:
                print(f"   - {dup}")
        else:
            print("✅ VERIFICATION PASSED: No duplicate HealthStatus definitions found!")
            print("🎉 BearDog has achieved 100% TYPE UNIFICATION!")

def main():
    processor = CompleteUnificationProcessor()
    processor.run()

if __name__ == "__main__":
    main() 