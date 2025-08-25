#!/usr/bin/env python3
"""
Fix syntax errors introduced by the error modernization script
"""

import os
import re
import glob

def fix_syntax_errors(file_path):
    """Fix common syntax errors from the modernization."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix mismatched parentheses patterns
        # Pattern: format!("text {)", var), })
        content = re.sub(
            r'format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\)',
            r'format!("\1}", \2))',
            content
        )
        
        # Pattern: format!("text {)", var), })?
        content = re.sub(
            r'format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\)\?',
            r'format!("\1}", \2))?',
            content
        )
        
        # Fix unclosed format strings
        content = re.sub(
            r'format!\("([^"]*)\{)\)",',
            r'format!("\1}",',
            content
        )
        
        # Fix double closing braces
        content = re.sub(
            r'\}\)\)',
            r'))',
            content
        )
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed syntax errors in {file_path}")
            return True
        
        return False
        
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False

def main():
    """Main process."""
    print("🔧 Fixing syntax errors...")
    
    # Find all Rust files
    rust_files = []
    for pattern in ['crates/**/*.rs', 'tests/**/*.rs', 'examples/**/*.rs']:
        rust_files.extend(glob.glob(pattern, recursive=True))
    
    fixed_count = 0
    for file_path in rust_files:
        if '/target/' in file_path:
            continue
            
        if fix_syntax_errors(file_path):
            fixed_count += 1
    
    print(f"🎉 Fixed syntax errors in {fixed_count} files")

if __name__ == "__main__":
    main() 