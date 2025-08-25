#!/usr/bin/env python3
"""
Final Syntax Cleanup Script

Fixes the remaining specific delimiter issues in the files that are still failing compilation.
"""

import os
import re

def fix_beardog_deploy():
    """Fix the beardog-deploy lib.rs file."""
    filepath = "crates/beardog-deploy/src/lib.rs"
    print(f"Fixing {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Count braces to identify the issue
        open_braces = content.count('{')
        close_braces = content.count('}')
        
        print(f"  Open braces: {open_braces}, Close braces: {close_braces}")
        
        # If there's an extra closing brace, remove one
        if close_braces > open_braces:
            # Find and remove the extra closing brace (likely at the end)
            lines = content.split('\n')
            for i in range(len(lines) - 1, -1, -1):
                if lines[i].strip() == '}':
                    lines.pop(i)
                    break
            content = '\n'.join(lines)
            
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  Removed extra closing brace")
            
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")

def fix_beardog_compliance():
    """Fix the beardog-compliance audit.rs file."""
    filepath = "crates/beardog-compliance/src/audit.rs"
    print(f"Fixing {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Add missing closing brace at the end if needed
        if not content.strip().endswith('}'):
            content = content.rstrip() + '\n}\n'
            
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  Added missing closing brace")
            
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")

def fix_beardog_utils():
    """Fix the beardog-utils safe_memory_enhanced.rs file."""
    filepath = "crates/beardog-utils/src/utils/safe_memory_enhanced.rs"
    print(f"Fixing {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Count braces to identify the issue
        open_braces = content.count('{')
        close_braces = content.count('}')
        
        print(f"  Open braces: {open_braces}, Close braces: {close_braces}")
        
        # Fix mismatched delimiters
        if open_braces != close_braces:
            # Add missing closing braces
            while content.count('{') > content.count('}'):
                content = content.rstrip() + '\n}\n'
                
            # Remove extra closing braces
            lines = content.split('\n')
            while content.count('}') > content.count('{'):
                for i in range(len(lines) - 1, -1, -1):
                    if lines[i].strip() == '}':
                        lines.pop(i)
                        break
                content = '\n'.join(lines)
                
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  Fixed delimiter mismatch")
            
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")

def main():
    print("🔧 Running final syntax cleanup...")
    
    fix_beardog_deploy()
    fix_beardog_compliance()
    fix_beardog_utils()
    
    print("\n✅ Final syntax cleanup complete!")

if __name__ == "__main__":
    main() 