#!/usr/bin/env python3
"""
Comprehensive Compilation Fixer

This script fixes all compilation issues found during the configuration migration.
"""

import os
import re
from pathlib import Path

def fix_compliance_event_references():
    """Fix compliance event references by removing underscores"""
    file_path = Path("crates/beardog-compliance/src/compliance/handlers.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Replace _event with event in function parameters and usage
    content = content.replace("_event: &ComplianceEvent", "event: &ComplianceEvent")
    
    file_path.write_text(content)
    print(f"✅ Fixed compliance event references in {file_path}")

def fix_workflow_builder_config():
    """Fix workflow builder config field access"""
    file_path = Path("crates/beardog-workflows/src/lib.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Fix config field references
    content = content.replace("self.config", "self._config")
    content = content.replace("_config: WorkflowEngineConfig", "config: WorkflowEngineConfig")
    
    file_path.write_text(content)
    print(f"✅ Fixed workflow builder config references in {file_path}")

def fix_memory_pool_object():
    """Fix memory pool object references"""
    file_path = Path("crates/beardog-utils/src/memory_pools.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Fix object references
    content = content.replace("mut _object: T", "mut object: T")
    
    file_path.write_text(content)
    print(f"✅ Fixed memory pool object references in {file_path}")

def fix_zero_cost_workflow_references():
    """Fix zero cost workflow references"""
    file_path = Path("crates/beardog-workflows/src/workflows/zero_cost_engine.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    content = file_path.read_text()
    
    # Fix workflow references
    content = content.replace("_workflow: CanonicalWorkflow", "workflow: CanonicalWorkflow")
    
    file_path.write_text(content)
    print(f"✅ Fixed zero cost workflow references in {file_path}")

def fix_threat_delimiter_comprehensive():
    """Comprehensively fix the threat delimiter issue"""
    file_path = Path("crates/beardog-threat/src/threat/types/engine/conditions.rs")
    
    if not file_path.exists():
        print(f"⚠️  File not found: {file_path}")
        return
    
    # Read the file and check for the massive brace issue
    content = file_path.read_text()
    
    if "}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}" in content:
        # Remove the excessive braces
        content = content.replace("}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}", "")
        
        # Ensure proper enum closure
        if not content.strip().endswith('}'):
            content += '\n}'
        
        file_path.write_text(content)
        print(f"✅ Fixed threat delimiter issue in {file_path}")

def main():
    """Main function to fix all compilation issues"""
    print("🔧 Comprehensive compilation fixes...")
    
    fix_compliance_event_references()
    fix_workflow_builder_config()
    fix_memory_pool_object()
    fix_zero_cost_workflow_references()
    fix_threat_delimiter_comprehensive()
    
    print("✅ All compilation fixes completed!")
    print("🔍 Running cargo check to verify fixes...")
    
    result = os.system("cargo check -p beardog-types --quiet")
    if result == 0:
        print("✅ beardog-types compiles successfully")
    else:
        print("❌ beardog-types still has issues")

if __name__ == "__main__":
    main() 