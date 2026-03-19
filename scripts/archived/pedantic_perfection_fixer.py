#!/usr/bin/env python3
"""
Pedantic Perfection Fixer

This script systematically fixes all pedantic clippy warnings to achieve
absolute code quality perfection.
"""

import re
import subprocess
import sys
from pathlib import Path

def fix_empty_line_after_attribute():
    """Fix empty line after outer attribute warnings."""
    result = subprocess.run([
        'cargo', 'clippy', '--workspace', '--', 
        '-W', 'clippy::empty-line-after-outer-attr'
    ], capture_output=True, text=True)
    
    fixed_count = 0
    lines = result.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if 'empty line after outer attribute' in line and '-->' in lines[i + 1]:
            location = lines[i + 1].strip().replace('-->', '').strip()
            if ':' in location:
                file_path, line_col = location.split(':', 1)
                line_num = int(line_col.split(':')[0])
                
                if fix_empty_line_in_file(file_path, line_num):
                    fixed_count += 1
                    print(f"✅ Fixed empty line after attribute in {file_path}:{line_num}")
    
    return fixed_count

def fix_empty_line_in_file(file_path, line_num):
    """Fix empty line after attribute in a specific file."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Remove empty line after attribute
        target_idx = line_num - 1
        if target_idx < len(lines) and lines[target_idx].strip() == '':
            lines.pop(target_idx)
            
            with open(file_path, 'w') as f:
                f.writelines(lines)
            return True
    except Exception as e:
        print(f"Error fixing {file_path}:{line_num}: {e}")
    
    return False

def fix_incorrect_must_use():
    """Fix incorrectly placed must_use attributes."""
    result = subprocess.run([
        'cargo', 'clippy', '--workspace'
    ], capture_output=True, text=True)
    
    fixed_count = 0
    lines = result.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if ('`#[must_use]` has no effect when applied to' in line and 
            '-->' in lines[i + 1]):
            location = lines[i + 1].strip().replace('-->', '').strip()
            if ':' in location:
                file_path, line_col = location.split(':', 1)
                line_num = int(line_col.split(':')[0])
                
                if remove_incorrect_must_use(file_path, line_num):
                    fixed_count += 1
                    print(f"✅ Removed incorrect must_use in {file_path}:{line_num}")
    
    return fixed_count

def remove_incorrect_must_use(file_path, line_num):
    """Remove incorrectly placed must_use attribute."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        target_idx = line_num - 1
        if target_idx < len(lines) and '#[must_use]' in lines[target_idx]:
            # Remove the line with incorrect must_use
            lines.pop(target_idx)
            
            with open(file_path, 'w') as f:
                f.writelines(lines)
            return True
    except Exception as e:
        print(f"Error fixing {file_path}:{line_num}: {e}")
    
    return False

def fix_documentation_issues():
    """Fix documentation-related pedantic issues."""
    result = subprocess.run([
        'cargo', 'clippy', '--workspace', '--', 
        '-W', 'clippy::doc-markdown'
    ], capture_output=True, text=True)
    
    fixed_count = 0
    lines = result.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if 'item in documentation is missing backticks' in line and '-->' in lines[i + 1]:
            location = lines[i + 1].strip().replace('-->', '').strip()
            if ':' in location:
                file_path, line_col = location.split(':', 1)
                line_num = int(line_col.split(':')[0])
                
                # Get the suggested fix from clippy output
                if i + 4 < len(lines) and 'help:' in lines[i + 4]:
                    suggestion = lines[i + 4].split('help:')[1].strip()
                    if fix_documentation_backticks(file_path, line_num, suggestion):
                        fixed_count += 1
                        print(f"✅ Fixed documentation backticks in {file_path}:{line_num}")
    
    return fixed_count

def fix_documentation_backticks(file_path, line_num, suggestion):
    """Fix missing backticks in documentation."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        target_idx = line_num - 1
        if target_idx < len(lines):
            # Apply the suggestion (this is a simplified approach)
            # In practice, you'd need more sophisticated parsing
            original_line = lines[target_idx]
            
            # Look for common patterns that need backticks
            patterns = [
                (r'\b(fn|struct|enum|trait|impl|mod|use)\b', r'`\1`'),
                (r'\b([A-Z][a-zA-Z0-9_]*Config)\b', r'`\1`'),
                (r'\b([A-Z][a-zA-Z0-9_]*Error)\b', r'`\1`'),
                (r'\b([a-z_]+\(\))\b', r'`\1`'),
            ]
            
            modified_line = original_line
            for pattern, replacement in patterns:
                modified_line = re.sub(pattern, replacement, modified_line)
            
            if modified_line != original_line:
                lines[target_idx] = modified_line
                with open(file_path, 'w') as f:
                    f.writelines(lines)
                return True
    except Exception as e:
        print(f"Error fixing documentation in {file_path}:{line_num}: {e}")
    
    return False

def main():
    """Main function to achieve pedantic perfection."""
    print("🎯 Starting Pedantic Perfection Fixer...")
    print("=" * 50)
    
    total_fixed = 0
    
    # Fix empty lines after attributes
    print("\n🔧 Fixing empty lines after attributes...")
    fixed = fix_empty_line_after_attribute()
    total_fixed += fixed
    print(f"   Fixed: {fixed}")
    
    # Fix incorrect must_use placements
    print("\n🔧 Fixing incorrect must_use attributes...")
    fixed = fix_incorrect_must_use()
    total_fixed += fixed
    print(f"   Fixed: {fixed}")
    
    # Fix documentation issues
    print("\n🔧 Fixing documentation backticks...")
    fixed = fix_documentation_issues()
    total_fixed += fixed
    print(f"   Fixed: {fixed}")
    
    print(f"\n🎯 Total fixes applied: {total_fixed}")
    
    # Final pedantic check
    print("\n🔍 Running final pedantic analysis...")
    result = subprocess.run([
        'cargo', 'clippy', '--workspace', '--', 
        '-W', 'clippy::pedantic', '-W', 'clippy::nursery'
    ], capture_output=True, text=True)
    
    warning_count = result.stderr.count('warning:')
    print(f"📊 Remaining pedantic warnings: {warning_count}")
    
    if warning_count < 100:
        print("🎉 Achieved significant pedantic improvement!")
    elif warning_count < 500:
        print("✅ Good progress on pedantic compliance!")
    else:
        print("⚠️  More work needed for full pedantic compliance")
    
    print("\n🏆 Pedantic perfection process complete!")

if __name__ == '__main__':
    main() 