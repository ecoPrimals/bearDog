#!/usr/bin/env python3
"""
Phase 4 Workflows Crate Modernization

This script systematically modernizes the beardog-workflows crate by:
1. Converting 20 async_trait usages to native async fn
2. Eliminating 3 Arc<dyn> patterns with zero-cost abstractions
3. Optimizing workflow execution patterns
"""

import os
import re
import sys
from pathlib import Path

def modernize_workflows_async_trait(file_path: Path) -> bool:
    """Convert async_trait usage to native async fn in workflows crate"""
    try:
        content = file_path.read_text()
        original_content = content
        
        # Skip files already modernized
        if '// MODERNIZED' in content or '// Removed async_trait' in content:
            return False
        
        # Remove async_trait imports and attributes
        content = re.sub(r'use async_trait::async_trait;\s*', '', content)
        content = re.sub(r'#\[async_trait\]\s*', '', content)
        
        # Convert common workflow trait patterns
        workflow_patterns = [
            # Workflow processor patterns
            (r'trait\s+WorkflowProcessor\s*(?:<[^>]*>)?\s*{[^}]*async\s+fn\s+process\([^}]*}', 
             lambda m: m.group(0).replace('#[async_trait]', '')),
            
            # Execution engine patterns
            (r'trait\s+ExecutionEngine\s*(?:<[^>]*>)?\s*{[^}]*async\s+fn\s+execute\([^}]*}',
             lambda m: m.group(0).replace('#[async_trait]', '')),
             
            # Notification patterns
            (r'trait\s+NotificationHandler\s*(?:<[^>]*>)?\s*{[^}]*async\s+fn\s+notify\([^}]*}',
             lambda m: m.group(0).replace('#[async_trait]', '')),
        ]
        
        for pattern, replacement in workflow_patterns:
            if callable(replacement):
                content = re.sub(pattern, replacement, content, flags=re.DOTALL)
            else:
                content = re.sub(pattern, replacement, content)
        
        # Add modernization marker if changes were made
        if content != original_content:
            content = f"// MODERNIZED: Converted async_trait to native async fn\n{content}"
            file_path.write_text(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"⚠️  Error processing {file_path}: {e}")
        return False

def modernize_workflows_arc_dyn(file_path: Path) -> bool:
    """Convert Arc<dyn> patterns to zero-cost abstractions in workflows"""
    try:
        content = file_path.read_text()
        original_content = content
        
        # Zero-cost replacements for workflow Arc<dyn> patterns
        replacements = [
            # Workflow processor patterns
            (r'Arc<dyn WorkflowProcessor(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 
             'impl WorkflowProcessor + Send + Sync'),
            (r'Arc<dyn ExecutionEngine(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 
             'impl ExecutionEngine + Send + Sync'),
            (r'Arc<dyn NotificationHandler(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 
             'impl NotificationHandler + Send + Sync'),
            
            # HSM workflow patterns
            (r'Arc<dyn HsmWorkflowProvider(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 
             'impl HsmWorkflowProvider + Send + Sync'),
            (r'Arc<dyn ZeroCostProcessor(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 
             'impl ZeroCostProcessor + Send + Sync'),
        ]
        
        for pattern, replacement in replacements:
            content = re.sub(pattern, replacement, content)
        
        # Add modernization marker if changes were made
        if content != original_content:
            content = f"// MODERNIZED: Converted Arc<dyn> to zero-cost abstractions\n{content}"
            file_path.write_text(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"⚠️  Error processing Arc<dyn> patterns in {file_path}: {e}")
        return False

def modernize_workflows_crate():
    """Modernize the entire beardog-workflows crate"""
    print("🚀 Phase 4: Modernizing beardog-workflows crate...")
    
    workflows_path = Path("crates/beardog-workflows")
    if not workflows_path.exists():
        print("❌ beardog-workflows crate not found!")
        return
    
    # Find all Rust files
    rust_files = list(workflows_path.rglob("*.rs"))
    
    async_trait_modernized = 0
    arc_dyn_modernized = 0
    
    print(f"📁 Found {len(rust_files)} Rust files in beardog-workflows")
    
    for rust_file in rust_files:
        print(f"🔧 Processing: {rust_file.relative_to(workflows_path)}")
        
        # Check for async_trait usage first
        try:
            content = rust_file.read_text()
            has_async_trait = 'async_trait' in content
            has_arc_dyn = 'Arc<dyn' in content
            
            if has_async_trait:
                print(f"   📝 Found async_trait usage")
            if has_arc_dyn:
                print(f"   📝 Found Arc<dyn> usage")
        except:
            continue
        
        # Try async_trait modernization
        if modernize_workflows_async_trait(rust_file):
            async_trait_modernized += 1
            print(f"   ✅ Modernized async_trait patterns")
        
        # Try Arc<dyn> modernization
        if modernize_workflows_arc_dyn(rust_file):
            arc_dyn_modernized += 1
            print(f"   ✅ Modernized Arc<dyn> patterns")
    
    print(f"\n🎯 BEARDOG-WORKFLOWS MODERNIZATION COMPLETE:")
    print(f"   • async_trait files modernized: {async_trait_modernized}")
    print(f"   • Arc<dyn> files modernized: {arc_dyn_modernized}")
    print(f"   • Total files processed: {len(rust_files)}")
    print(f"   • Modernization success rate: {((async_trait_modernized + arc_dyn_modernized) / len(rust_files)) * 100:.1f}%")
    
    return async_trait_modernized, arc_dyn_modernized

if __name__ == "__main__":
    modernize_workflows_crate() 