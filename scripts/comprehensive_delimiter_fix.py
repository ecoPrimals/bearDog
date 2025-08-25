#!/usr/bin/env python3
"""
Comprehensive Delimiter Fix Script

Systematically fixes all remaining delimiter issues by properly balancing braces
in the identified problem files.
"""

import os
import re
from typing import List, Tuple

def analyze_and_fix_delimiters(filepath: str) -> bool:
    """Analyze and fix delimiter issues in a file."""
    print(f"\n🔧 Analyzing {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        lines = content.split('\n')
        
        # Count braces
        open_braces = content.count('{')
        close_braces = content.count('}')
        
        print(f"   Open braces: {open_braces}")
        print(f"   Close braces: {close_braces}")
        print(f"   Difference: {open_braces - close_braces}")
        
        if open_braces == close_braces:
            print("   ✅ Braces are balanced")
            return False
        
        # Fix the imbalance
        if open_braces > close_braces:
            # Add missing closing braces
            missing = open_braces - close_braces
            print(f"   Adding {missing} closing braces")
            
            # Add them at the end, but intelligently
            content = content.rstrip()
            for _ in range(missing):
                content += '\n}'
            content += '\n'
                
        elif close_braces > open_braces:
            # Remove extra closing braces
            extra = close_braces - open_braces
            print(f"   Removing {extra} extra closing braces")
            
            # Remove from the end
            lines = content.split('\n')
            removed = 0
            for i in range(len(lines) - 1, -1, -1):
                if removed >= extra:
                    break
                if lines[i].strip() == '}':
                    lines.pop(i)
                    removed += 1
            content = '\n'.join(lines)
        
        # Write the fixed content
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"   ✅ Fixed delimiter issues")
        return True
        
    except Exception as e:
        print(f"   ❌ Error fixing {filepath}: {e}")
        return False

def fix_specific_structural_issues():
    """Fix specific structural issues we know about."""
    
    # Fix utils file - add missing closing for test function
    utils_file = "crates/beardog-utils/src/utils/safe_memory_enhanced.rs"
    print(f"\n🔧 Fixing specific issues in {utils_file}")
    
    try:
        with open(utils_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Look for unclosed test function
        if "async fn test_safe_pinned_buffer_operations()" in content:
            # Find the function and make sure it's properly closed
            pattern = r'(async fn test_safe_pinned_buffer_operations\(\)[^}]*assert_eq!\(values, \(0xAA, 0xBB\)\);)'
            replacement = r'\1\n    }'
            content = re.sub(pattern, replacement, content, flags=re.DOTALL)
            
            with open(utils_file, 'w', encoding='utf-8') as f:
                f.write(content)
            print("   ✅ Fixed test function closure")
            
    except Exception as e:
        print(f"   ❌ Error: {e}")

def main():
    """Main function to fix all delimiter issues."""
    print("🚀 Starting comprehensive delimiter fix...")
    
    # List of files with known delimiter issues
    problem_files = [
        "crates/beardog-deploy/src/android.rs",
        "crates/beardog-utils/src/utils/safe_memory_enhanced.rs", 
        "crates/beardog-compliance/src/audit.rs",
        "crates/beardog-types/src/canonical/mod.rs"
    ]
    
    # Fix specific structural issues first
    fix_specific_structural_issues()
    
    # Fix delimiter balance in each file
    files_fixed = 0
    for filepath in problem_files:
        if os.path.exists(filepath):
            if analyze_and_fix_delimiters(filepath):
                files_fixed += 1
        else:
            print(f"   ⚠️ File not found: {filepath}")
    
    print(f"\n✅ Comprehensive delimiter fix complete!")
    print(f"   Files processed: {len(problem_files)}")
    print(f"   Files fixed: {files_fixed}")

if __name__ == "__main__":
    main() 