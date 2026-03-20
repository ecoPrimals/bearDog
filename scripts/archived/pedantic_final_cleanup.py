#!/usr/bin/env python3
"""
🎯 PEDANTIC FINAL CLEANUP
Removes all erroneous documentation from use statements and expressions.
"""

import os
import re
import sys
from pathlib import Path

def clean_use_statements(content):
    """Remove documentation from use statements"""
    lines = content.split('\n')
    result_lines = []
    
    i = 0
    in_use_block = False
    
    while i < len(lines):
        line = lines[i]
        
        # Check if we're entering a use block
        if 'pub use ' in line and '{' in line:
            in_use_block = True
            result_lines.append(line)
        elif in_use_block and '}' in line:
            in_use_block = False
            result_lines.append(line)
        elif in_use_block and line.strip().startswith('///'):
            # Skip documentation in use blocks
            pass
        else:
            result_lines.append(line)
        
        i += 1
    
    return '\n'.join(result_lines)

def main():
    print("🎯 PEDANTIC FINAL CLEANUP")
    print("🧹 Removing erroneous documentation from use statements")
    
    # Find the problematic file
    file_path = Path("crates/beardog-core/src/ecosystem/mod.rs")
    
    if file_path.exists():
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Clean up the use statements
        cleaned_content = clean_use_statements(content)
        
        with open(file_path, 'w') as f:
            f.write(cleaned_content)
        
        print(f"✅ Cleaned up: {file_path}")
    
    print("🏆 PEDANTIC FINAL CLEANUP COMPLETE")

if __name__ == "__main__":
    main() 