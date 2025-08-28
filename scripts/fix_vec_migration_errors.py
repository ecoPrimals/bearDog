#!/usr/bin/env python3
"""
Fix Vec Migration Errors
Fixes incorrect Vec<T, BearDogError> patterns to Vec<T> in Result<Vec<T>, BearDogError>
"""

import os
import re
import sys
from pathlib import Path

def fix_vec_errors(file_path):
    """Fix Vec<T, BearDogError> patterns in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix patterns that were incorrectly migrated
        fixes = [
            # Fix Vec<T, BearDogError> to Vec<T> in Result contexts
            (r'Result<Vec<([^,>]+), BearDogError>', r'Result<Vec<\1>, BearDogError>'),
            (r'Result<Option<([^,>]+), BearDogError>', r'Result<Option<\1>, BearDogError>'),
            
            # Fix more complex nested patterns
            (r'Vec<\(([^,)]+), ([^,)]+)\), BearDogError>', r'Vec<(\1, \2)>'),
            (r'Option<([^,>]+), BearDogError>', r'Option<\1>'),
            
            # Fix JoinHandle patterns
            (r'tokio::task::JoinHandle<\(\), BearDogError>', r'tokio::task::JoinHandle<()>'),
        ]
        
        # Apply fixes
        for pattern, replacement in fixes:
            content = re.sub(pattern, replacement, content)
        
        # Only write if content changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    """Main fix function"""
    print("🔧 Fixing Vec Migration Errors")
    print("=" * 40)
    
    # Find all Rust files in crates directory
    crates_dir = Path("crates")
    if not crates_dir.exists():
        print("❌ Error: 'crates' directory not found. Run from project root.")
        sys.exit(1)
    
    rust_files = list(crates_dir.rglob("*.rs"))
    print(f"📁 Found {len(rust_files)} Rust files")
    
    # Filter files that contain the problematic patterns
    files_to_fix = []
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                if ('Vec<' in content and ', BearDogError>' in content) or \
                   ('Option<' in content and ', BearDogError>' in content) or \
                   ('JoinHandle<(), BearDogError>' in content):
                    files_to_fix.append(file_path)
        except Exception:
            continue
    
    print(f"🎯 {len(files_to_fix)} files need fixing")
    
    if not files_to_fix:
        print("✅ No files need fixing!")
        return
    
    # Perform fixes
    fixed_count = 0
    for file_path in files_to_fix:
        if fix_vec_errors(file_path):
            fixed_count += 1
            print(f"✅ Fixed: {file_path}")
        else:
            print(f"⚪ No changes: {file_path}")
    
    print()
    print("📊 Fix Summary:")
    print(f"   • Files processed: {len(files_to_fix)}")
    print(f"   • Files fixed: {fixed_count}")
    print(f"   • Files unchanged: {len(files_to_fix) - fixed_count}")
    
    if fixed_count > 0:
        print()
        print("🎉 Fixes applied successfully!")
        print("💡 Next step: Run 'cargo build' to verify fixes")
    else:
        print("ℹ️  No files needed fixing.")

if __name__ == "__main__":
    main() 