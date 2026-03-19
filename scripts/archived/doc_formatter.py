#!/usr/bin/env python3
"""
Documentation Formatter - Fix TEST_ metadata formatting
Date: November 28, 2025
Purpose: Automatically add backticks to test metadata in documentation
"""

import os
import re
import sys
from pathlib import Path

# Patterns to fix
PATTERNS = {
    r'/// TEST_CATEGORY:': r'/// `TEST_CATEGORY`:',
    r'/// TEST_DOMAIN:': r'/// `TEST_DOMAIN`:',
    r'/// TEST_PRIORITY:': r'/// `TEST_PRIORITY`:',
    r'//! TEST_CATEGORY:': r'//! `TEST_CATEGORY`:',
    r'//! TEST_DOMAIN:': r'//! `TEST_DOMAIN`:',
    r'//! TEST_PRIORITY:': r'//! `TEST_PRIORITY`:',
}

def fix_file(filepath):
    """Fix documentation formatting in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all patterns
        for old_pattern, new_pattern in PATTERNS.items():
            content = re.sub(old_pattern, new_pattern, content)
        
        # Only write if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    """Main execution"""
    crates_dir = Path('crates')
    tests_dir = Path('tests')
    
    fixed_count = 0
    total_files = 0
    
    print("🐻 BearDog Documentation Formatter")
    print("====================================")
    print()
    print("Fixing test metadata formatting...")
    print()
    
    # Process all Rust files
    for directory in [crates_dir, tests_dir]:
        if directory.exists():
            for rs_file in directory.rglob('*.rs'):
                total_files += 1
                if fix_file(rs_file):
                    fixed_count += 1
                    print(f"  ✓ Fixed: {rs_file}")
    
    print()
    print("====================================")
    print(f"Summary:")
    print(f"  Files processed: {total_files}")
    print(f"  Files fixed: {fixed_count}")
    print()
    
    if fixed_count > 0:
        print("✅ Documentation formatting improved!")
    else:
        print("✅ All documentation already properly formatted!")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

