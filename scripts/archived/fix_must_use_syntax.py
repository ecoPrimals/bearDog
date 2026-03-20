#!/usr/bin/env python3
"""
Fix Must-Use Syntax Errors

This script fixes syntax errors caused by incorrectly placed #[must_use] attributes.
"""

import re
import subprocess
import sys
from pathlib import Path

def fix_must_use_syntax_errors():
    """Fix syntax errors in must_use attributes."""
    
    # Get compilation errors
    result = subprocess.run([
        'cargo', 'check', '--workspace', '--message-format=short'
    ], capture_output=True, text=True)
    
    errors = result.stderr
    fixed_count = 0
    
    # Pattern to match must_use syntax errors
    error_pattern = r'error\[E0658\]: attributes on expressions are experimental\s*-->\s*([^:]+):(\d+):\d+'
    
    for match in re.finditer(error_pattern, errors):
        file_path = match.group(1)
        line_num = int(match.group(2))
        
        if fix_must_use_in_file(file_path, line_num):
            fixed_count += 1
            print(f"✅ Fixed must_use syntax error in {file_path}:{line_num}")
    
    return fixed_count

def fix_must_use_in_file(file_path, line_num):
    """Fix must_use syntax error in a specific file."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Check if the line contains #[must_use]
        target_idx = line_num - 1
        if target_idx >= len(lines) or '#[must_use]' not in lines[target_idx]:
            return False
        
        # Remove the incorrectly placed #[must_use]
        lines[target_idx] = lines[target_idx].replace('#[must_use]', '').strip()
        if not lines[target_idx]:
            lines.pop(target_idx)
        else:
            lines[target_idx] += '\n'
        
        # Write back to file
        with open(file_path, 'w') as f:
            f.writelines(lines)
        
        return True
    except Exception as e:
        print(f"Error fixing {file_path}:{line_num}: {e}")
        return False

def main():
    """Main function."""
    print("🔧 Fixing Must-Use Syntax Errors...")
    
    fixed_count = fix_must_use_syntax_errors()
    print(f"🎯 Fixed {fixed_count} syntax errors")
    
    # Verify compilation
    result = subprocess.run(['cargo', 'check', '--workspace'], capture_output=True, text=True)
    if result.returncode == 0:
        print("✅ All syntax errors fixed - compilation successful!")
    else:
        remaining_errors = result.stderr.count('error[E0658]: attributes on expressions are experimental')
        if remaining_errors > 0:
            print(f"⚠️  {remaining_errors} syntax errors still remain")
        else:
            print("✅ No more must_use syntax errors!")

if __name__ == '__main__':
    main() 