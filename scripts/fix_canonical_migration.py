#!/usr/bin/env python3
"""
Fix Canonical Migration Issues

This script fixes the issues created by the initial canonicalization migration,
specifically handling self-referencing imports and malformed derive attributes.
"""

import os
import re
import sys
from pathlib import Path
from typing import List

def fix_self_references(file_path: Path) -> bool:
    """Fix self-referencing imports in beardog-types crate."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        original_content = content
        changes_made = False
        
        # Fix self-references to beardog_types::canonical
        if 'beardog_types::canonical' in content:
            content = content.replace('beardog_types::canonical', 'crate::canonical')
            changes_made = True
        
        # Remove derive attributes from pub use statements
        derive_use_pattern = r'#\[derive\([^\]]+\)\]\s*\n\s*pub use [^;]+;'
        matches = re.findall(derive_use_pattern, content)
        if matches:
            for match in matches:
                # Extract just the pub use part
                pub_use_part = re.search(r'pub use [^;]+;', match)
                if pub_use_part:
                    content = content.replace(match, pub_use_part.group(0))
                    changes_made = True
        
        # Fix enum/struct definitions that got converted to imports incorrectly
        # Look for patterns like: pub use crate::canonical::Type; followed by enum variants
        broken_enum_pattern = r'pub use crate::canonical::(\w+);\s*,?\s*((?:\s*///[^\n]*\n)?\s*\w+[^,\n]*,?\s*)*'
        if re.search(broken_enum_pattern, content):
            # This is more complex - for now, just remove the trailing content after pub use
            content = re.sub(r'(pub use crate::canonical::\w+;)\s*,.*?(?=\n\n|\n///|\n#\[|\nuse|\npub|\Z)', r'\1', content, flags=re.DOTALL)
            changes_made = True
            
        if changes_made:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
                
        return changes_made
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def create_missing_modules(root_path: Path):
    """Create missing module files."""
    types_path = root_path / "crates" / "beardog-types" / "src"
    
    # Create network module
    network_path = types_path / "network.rs"
    if not network_path.exists():
        network_content = '''//! Network types and configurations
//!
//! Re-exports canonical network types and provides legacy compatibility.

pub use crate::canonical::{
    HealthStatus,
    OperationStatus,
};

// Legacy network types - these should be migrated to canonical types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub address: NetworkAddress,
    pub connection: ConnectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub protocol: NetworkProtocol,
    pub config: EndpointConfig,
}
'''
        with open(network_path, 'w') as f:
            f.write(network_content)
        print(f"Created {network_path}")
    
    # Create workflow module
    workflow_path = types_path / "workflow.rs"
    if not workflow_path.exists():
        workflow_content = '''//! Workflow types and definitions
//!
//! Re-exports canonical workflow types and provides legacy compatibility.

pub use crate::canonical::{
    WorkflowStatus,
    WorkflowType,
    WorkflowExecutionState,
};

// Legacy workflow types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    pub workflow_type: WorkflowType,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub step_type: String,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub state: WorkflowExecutionState,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub id: String,
    pub workflow_id: String,
    pub trigger_type: String,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub workflow_execution_id: String,
    pub step_id: String,
    pub requested_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
}
'''
        with open(workflow_path, 'w') as f:
            f.write(workflow_content)
        print(f"Created {workflow_path}")

def main():
    # Find project root
    script_path = Path(__file__).parent
    root_path = script_path.parent
    
    print("🔧 Fixing canonical migration issues...")
    
    # Create missing modules first
    create_missing_modules(root_path)
    
    # Fix beardog-types crate specifically
    types_src = root_path / "crates" / "beardog-types" / "src"
    
    files_fixed = 0
    for rust_file in types_src.rglob("*.rs"):
        if fix_self_references(rust_file):
            files_fixed += 1
            print(f"Fixed: {rust_file.relative_to(root_path)}")
    
    print(f"\n✅ Fixed {files_fixed} files")
    
    # Try to compile to check for remaining issues
    print("\n🔍 Testing compilation...")
    os.system("cd /home/eastgate/Development/ecoPrimals/beardog && cargo check --lib -p beardog-types --quiet")

if __name__ == '__main__':
    main() 