#!/usr/bin/env python3
"""
Final Compilation Fixes

This script fixes the remaining compilation issues after migration.
"""

import os
import re
from pathlib import Path

def fix_workflow_builder_fields():
    """Fix workflow builder field references"""
    file_path = Path("crates/beardog-workflows/src/lib.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Fix the field references back to config (not _config)
    content = content.replace("self._config", "self.config")
    
    file_path.write_text(content)
    print(f"✅ Fixed workflow builder field references in {file_path}")

def fix_threat_delimiter_final():
    """Final fix for threat delimiter issue"""
    file_path = Path("crates/beardog-threat/src/threat/types/engine/conditions.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    # Read the file
    content = file_path.read_text()
    
    # Remove any excessive closing braces
    if "}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}" in content:
        content = content.replace("}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}", "")
        
        # Ensure the enum is properly closed
        lines = content.split('\n')
        
        # Find the enum start and ensure proper closure
        enum_started = False
        brace_count = 0
        
        for i, line in enumerate(lines):
            if 'pub enum RuleCondition {' in line:
                enum_started = True
                brace_count = 1
                continue
                
            if enum_started:
                brace_count += line.count('{')
                brace_count -= line.count('}')
                
                if brace_count == 0:
                    break
        
        # If enum is not properly closed, add closing brace
        if brace_count > 0:
            lines.append('}')
            content = '\n'.join(lines)
        
        file_path.write_text(content)
        print(f"✅ Fixed threat delimiter issue in {file_path}")

def fix_unused_variables():
    """Fix remaining unused variables"""
    fixes = [
        ("crates/beardog-workflows/src/workflows/zero_cost_engine.rs", [
            ("let context = WorkflowProcessingContext::default();", "let _context = WorkflowProcessingContext::default();"),
            ("workflow: CanonicalWorkflow", "_workflow: CanonicalWorkflow"),
        ]),
        ("crates/beardog-compliance/src/compliance/handlers.rs", [
            ("event: &ComplianceEvent) -> BearDogResult<Vec<ComplianceViolation>>", "_event: &ComplianceEvent) -> BearDogResult<Vec<ComplianceViolation>>"),
        ]),
        ("crates/beardog-workflows/src/workflows/handlers.rs", [
            ("let approval_store = Arc::new", "let _approval_store = Arc::new"),
        ]),
        ("crates/beardog-workflows/src/lib.rs", [
            ("enabled: bool) -> Self", "_enabled: bool) -> Self"),
            ("config: WorkflowEngineConfig", "_config: WorkflowEngineConfig"),
        ]),
    ]
    
    for file_path_str, replacements in fixes:
        file_path = Path(file_path_str)
        
        if not file_path.exists():
            print(f"⚠️  File not found: {file_path}")
            continue
        
        content = file_path.read_text()
        original_content = content
        
        for old, new in replacements:
            content = content.replace(old, new)
        
        if content != original_content:
            file_path.write_text(content)
            print(f"✅ Fixed unused variables in {file_path}")

def main():
    """Main function to apply final compilation fixes"""
    print("🔧 Applying final compilation fixes...")
    
    fix_workflow_builder_fields()
    fix_threat_delimiter_final()
    fix_unused_variables()
    
    print("✅ Final compilation fixes completed!")
    print("🔍 Running comprehensive cargo check...")
    
    result = os.system("cargo check --workspace --quiet")
    if result == 0:
        print("✅ All crates compile successfully!")
    else:
        print("❌ Some crates still have issues - checking individual crates...")
        
        # Check key crates individually
        crates = ["beardog-types", "beardog-tunnel", "beardog-adapters", "beardog-core"]
        for crate in crates:
            result = os.system(f"cargo check -p {crate} --quiet")
            if result == 0:
                print(f"✅ {crate} compiles successfully")
            else:
                print(f"❌ {crate} has compilation issues")

if __name__ == "__main__":
    main() 