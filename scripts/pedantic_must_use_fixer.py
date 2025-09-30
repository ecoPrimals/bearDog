#!/usr/bin/env python3
"""
Pedantic Must-Use Attribute Fixer

This script systematically adds #[must_use] attributes to methods that clippy
identifies as candidates, ensuring pedantic compliance.
"""

import re
import subprocess
import sys
from pathlib import Path

def get_must_use_candidates():
    """Get all must_use candidates from clippy."""
    try:
        result = subprocess.run([
            'cargo', 'clippy', '--workspace', '--', 
            '-W', 'clippy::must-use-candidate'
        ], capture_output=True, text=True, cwd='.')
        
        candidates = []
        lines = result.stderr.split('\n')
        
        for i, line in enumerate(lines):
            if 'this method could have a `#[must_use]` attribute' in line:
                # Get the file location from the next line
                if i + 1 < len(lines) and '-->' in lines[i + 1]:
                    location = lines[i + 1].strip().replace('-->', '').strip()
                    if ':' in location:
                        file_path, line_col = location.split(':', 1)
                        line_num = int(line_col.split(':')[0])
                        candidates.append((file_path, line_num))
        
        return candidates
    except Exception as e:
        print(f"Error getting clippy output: {e}")
        return []

def add_must_use_attribute(file_path, line_num):
    """Add #[must_use] attribute to a specific method."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Find the method definition (line_num is 1-indexed)
        target_line_idx = line_num - 1
        
        if target_line_idx >= len(lines):
            return False
            
        # Look for the method line and find the preceding comment/doc
        method_line = lines[target_line_idx].strip()
        
        # Find where to insert the #[must_use] attribute
        insert_idx = target_line_idx
        
        # Look backwards for documentation comments
        while insert_idx > 0 and (lines[insert_idx - 1].strip().startswith('///') or 
                                  lines[insert_idx - 1].strip().startswith('///')):
            insert_idx -= 1
        
        # Check if #[must_use] already exists
        for check_idx in range(max(0, insert_idx - 3), target_line_idx + 1):
            if '#[must_use]' in lines[check_idx]:
                return False  # Already has must_use
        
        # Get the indentation from the method line
        method_indent = len(lines[target_line_idx]) - len(lines[target_line_idx].lstrip())
        indent = ' ' * method_indent
        
        # Insert the #[must_use] attribute
        lines.insert(target_line_idx, f'{indent}#[must_use]\n')
        
        # Write back to file
        with open(file_path, 'w') as f:
            f.writelines(lines)
        
        return True
    except Exception as e:
        print(f"Error processing {file_path}:{line_num}: {e}")
        return False

def main():
    """Main function to fix all must_use candidates."""
    print("🔧 Starting Pedantic Must-Use Attribute Fixer...")
    
    candidates = get_must_use_candidates()
    print(f"📊 Found {len(candidates)} must_use candidates")
    
    fixed_count = 0
    for file_path, line_num in candidates:
        if add_must_use_attribute(file_path, line_num):
            print(f"✅ Added #[must_use] to {file_path}:{line_num}")
            fixed_count += 1
        else:
            print(f"⏭️  Skipped {file_path}:{line_num} (already has attribute or error)")
    
    print(f"\n🎯 Fixed {fixed_count} out of {len(candidates)} candidates")
    
    # Run clippy again to verify
    print("\n🔍 Verifying fixes...")
    result = subprocess.run([
        'cargo', 'clippy', '--workspace', '--', 
        '-W', 'clippy::must-use-candidate'
    ], capture_output=True, text=True)
    
    remaining = result.stderr.count('this method could have a `#[must_use]` attribute')
    print(f"📈 Remaining must_use candidates: {remaining}")
    
    if remaining == 0:
        print("🎉 All must_use candidates fixed!")
    else:
        print(f"⚠️  {remaining} candidates still need manual review")

if __name__ == '__main__':
    main() 