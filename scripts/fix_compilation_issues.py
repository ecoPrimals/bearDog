#!/usr/bin/env python3
"""
Fix Compilation Issues

This script fixes the compilation issues found during the configuration migration.
"""

import os
import re
from pathlib import Path

def fix_threat_delimiter_issue():
    """Fix the unclosed delimiter in threat crate"""
    file_path = Path("crates/beardog-threat/src/threat/types/engine/conditions.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Look for the RuleCondition enum and ensure it's properly closed
    # This is a complex fix, so let's add a simple closing brace if missing
    if "RuleCondition::Not { condition } => condition.is_valid()," in content:
        # Check if the enum is properly closed
        lines = content.split('\n')
        brace_count = 0
        enum_started = False
        
        for i, line in enumerate(lines):
            if 'pub enum RuleCondition {' in line:
                enum_started = True
                brace_count = 1
                continue
            
            if enum_started:
                brace_count += line.count('{')
                brace_count -= line.count('}')
                
                if brace_count == 0:
                    # Enum properly closed
                    break
        
        if brace_count > 0:
            # Add missing closing braces
            content += '\n' + '}' * brace_count
            file_path.write_text(content)
            print(f"✅ Fixed delimiter issue in {file_path}")

def fix_workflow_duplicate_default():
    """Fix duplicate Default implementation in workflow crate"""
    file_path = Path("crates/beardog-workflows/src/workflows/canonical/approval.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Find and remove duplicate Default implementations
    lines = content.split('\n')
    new_lines = []
    in_default_impl = False
    default_impl_count = 0
    skip_lines = 0
    
    for i, line in enumerate(lines):
        if skip_lines > 0:
            skip_lines -= 1
            continue
            
        if 'impl Default for WorkflowApprovalEngine {' in line:
            default_impl_count += 1
            if default_impl_count > 1:
                # Skip this duplicate implementation
                in_default_impl = True
                brace_count = 1
                skip_lines = 0
                # Find the end of this impl block
                for j in range(i + 1, len(lines)):
                    brace_count += lines[j].count('{')
                    brace_count -= lines[j].count('}')
                    skip_lines += 1
                    if brace_count == 0:
                        break
                continue
        
        new_lines.append(line)
    
    if default_impl_count > 1:
        file_path.write_text('\n'.join(new_lines))
        print(f"✅ Fixed duplicate Default implementation in {file_path}")

def fix_unused_warnings():
    """Fix unused variable warnings by prefixing with underscore"""
    warning_fixes = [
        ("crates/beardog-compliance/src/compliance/handlers.rs", [
            (r'event: &ComplianceEvent', r'_event: &ComplianceEvent'),
        ]),
        ("crates/beardog-workflows/src/lib.rs", [
            (r'config: WorkflowEngineConfig', r'_config: WorkflowEngineConfig'),
        ]),
        ("crates/beardog-workflows/src/workflows/canonical/execution/engine.rs", [
            (r'let processor = processors', r'let _processor = processors'),
            (r'let workflow = self', r'let _workflow = self'),
        ]),
        ("crates/beardog-workflows/src/workflows/zero_cost_engine.rs", [
            (r'workflow_id: &WorkflowId', r'_workflow_id: &WorkflowId'),
            (r'workflow: CanonicalWorkflow', r'_workflow: CanonicalWorkflow'),
        ]),
        ("crates/beardog-workflows/src/workflows/handlers.rs", [
            (r'let processors = self', r'let _processors = self'),
            (r'execution_rx', r'_execution_rx'),
            (r'let shutdown_rx', r'let _shutdown_rx'),
        ]),
        ("crates/beardog-utils/src/memory_pools.rs", [
            (r'mut object: T', r'mut _object: T'),
        ]),
    ]
    
    for file_path_str, replacements in warning_fixes:
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
            print(f"✅ Fixed unused warnings in {file_path}")

def main():
    """Main function to fix compilation issues"""
    print("🔧 Fixing compilation issues...")
    
    fix_threat_delimiter_issue()
    fix_workflow_duplicate_default()
    fix_unused_warnings()
    
    print("✅ Compilation fixes completed!")
    print("🔍 Running cargo check to verify fixes...")
    
    os.system("cargo check --quiet")

if __name__ == "__main__":
    main() 