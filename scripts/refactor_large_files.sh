#!/bin/bash
# BearDog File Refactoring Script
# Splits large files (>1000 lines) into focused modules

set -e

echo "🔧 BearDog File Size Refactoring"
echo "==============================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check current file sizes
echo -e "${BLUE}📊 Current large files (>1000 lines):${NC}"
find src/ -name "*.rs" -exec wc -l {} + | sort -n | awk '$1 > 1000 {print}' | head -20

echo ""
echo -e "${YELLOW}🎯 Files requiring refactoring:${NC}"

# Define refactoring targets
declare -A REFACTOR_TARGETS=(
    ["src/workflows.rs"]="src/workflows/"
    ["src/security_provider.rs"]="src/security/"
    ["src/cross_node_auth.rs"]="src/auth/"
    ["src/threat_detection.rs"]="src/threat/"
    ["src/proof_verifier.rs"]="src/verification/"
    ["src/genetics_engine.rs"]="src/genetics/"
    ["src/compliance.rs"]="src/compliance/"
)

# Check which files need refactoring
for file in "${!REFACTOR_TARGETS[@]}"; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        target="${REFACTOR_TARGETS[$file]}"
        
        if [ "$lines" -gt 1000 ]; then
            echo -e "${RED}❌ $file${NC} (${lines} lines) → ${GREEN}${target}${NC}"
        else
            echo -e "${GREEN}✅ $file${NC} (${lines} lines) - OK"
        fi
    else
        echo -e "${YELLOW}⚠️  $file${NC} - Not found"
    fi
done

echo ""
echo -e "${BLUE}🚀 Refactoring Strategy:${NC}"
echo "1. Each large file becomes a focused module directory"
echo "2. Split into: mod.rs, types.rs, handlers.rs, tests.rs"
echo "3. Maximum ~300 lines per file"
echo "4. Maintain all existing functionality"
echo ""

# Function to create module structure
create_module_structure() {
    local source_file=$1
    local target_dir=$2
    
    echo -e "${YELLOW}🔧 Refactoring $source_file → $target_dir${NC}"
    
    # Create target directory
    mkdir -p "$target_dir"
    
    # Create basic module structure
    cat > "${target_dir}/mod.rs" << 'EOF'
//! Main module file - exports and core functionality
//! 
//! This module was refactored from a large file to improve maintainability.

// Re-export public types and functions
pub use types::*;
pub use handlers::*;

// Module declarations
mod types;
mod handlers;

#[cfg(test)]
mod tests;

// Core module functionality will be implemented here
EOF

    cat > "${target_dir}/types.rs" << 'EOF'
//! Type definitions and data structures
//! 
//! Contains all structs, enums, and type aliases for this module.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type definitions will be moved here from the original file
EOF

    cat > "${target_dir}/handlers.rs" << 'EOF'
//! Implementation logic and handlers
//! 
//! Contains the main business logic and implementation details.

use super::types::*;
use crate::*;

// Implementation logic will be moved here from the original file
EOF

    cat > "${target_dir}/tests.rs" << 'EOF'
//! Unit tests for this module
//! 
//! Contains all test functions for this module's functionality.

use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test functions will be moved here
}
EOF

    echo -e "${GREEN}✅ Created module structure in $target_dir${NC}"
}

# Interactive refactoring
echo -e "${BLUE}🤖 Automated Refactoring Available:${NC}"
echo "This script can create the basic module structure."
echo "Manual code splitting will be required for each module."
echo ""

read -p "Create module structures now? (y/n): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}🚀 Creating module structures...${NC}"
    
    for file in "${!REFACTOR_TARGETS[@]}"; do
        if [ -f "$file" ]; then
            lines=$(wc -l < "$file")
            target="${REFACTOR_TARGETS[$file]}"
            
            if [ "$lines" -gt 1000 ]; then
                # Create backup
                cp "$file" "${file}.backup"
                echo -e "${BLUE}📋 Backed up $file to ${file}.backup${NC}"
                
                # Create module structure
                create_module_structure "$file" "$target"
            fi
        fi
    done
    
    echo ""
    echo -e "${GREEN}✅ Module structures created!${NC}"
    echo ""
    echo -e "${YELLOW}📝 Next Steps:${NC}"
    echo "1. Review generated module structures"
    echo "2. Move code from .backup files to appropriate modules"
    echo "3. Update imports and exports"
    echo "4. Run tests to verify functionality"
    echo "5. Remove .backup files when complete"
    
else
    echo -e "${BLUE}ℹ️  Module creation skipped${NC}"
fi

echo ""
echo -e "${BLUE}📋 Manual Refactoring Checklist:${NC}"
echo "□ Split workflows.rs (1,866 lines) → src/workflows/"
echo "□ Split security_provider.rs (1,645 lines) → src/security/"  
echo "□ Split cross_node_auth.rs (1,621 lines) → src/auth/"
echo "□ Split threat_detection.rs (1,295 lines) → src/threat/"
echo "□ Split proof_verifier.rs (1,261 lines) → src/verification/"
echo "□ Update src/lib.rs with new module declarations"
echo "□ Fix import statements throughout codebase"
echo "□ Run cargo test to verify refactoring"
echo "□ Update documentation"

echo ""
echo -e "${GREEN}🎯 Target: All files under 1000 lines${NC}"
echo -e "${BLUE}📊 Run this script again to check progress${NC}"
echo "" 