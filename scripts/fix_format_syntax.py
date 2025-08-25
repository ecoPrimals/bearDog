#!/usr/bin/env python3
"""
Fix malformed format strings and syntax errors in security crate
"""

import re
import glob

def fix_syntax_errors(file_path):
    """Fix syntax errors in a file."""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix malformed format strings
    # Pattern: format!("text {)", var), })
    content = re.sub(
        r'format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\)',
        r'format!("\1}", \2))',
        content
    )
    
    # Pattern: format!("text {)", var), });
    content = re.sub(
        r'format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\);',
        r'format!("\1}", \2)));',
        content
    )
    
    # Pattern: BearDogError::something(format!("text {)", var), })
    content = re.sub(
        r'(BearDogError::\w+\(format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\)',
        r'\1format!("\2}", \3))',
        content
    )
    
    # Fix missing closing parentheses
    content = re.sub(
        r'BearDogError::(\w+)\(format!\("([^"]*)", ([^)]+)\),\s*\}\)',
        r'BearDogError::\1(format!("\2", \3))',
        content
    )
    
    # Write back if changed
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"✅ Fixed syntax errors in {file_path}")
        return True
    
    return False

def main():
    """Fix all syntax errors in security crate."""
    print("🔧 Fixing syntax errors in security crate...")
    
    files = glob.glob('crates/beardog-security/src/**/*.rs', recursive=True)
    fixed_count = 0
    
    for file_path in files:
        if fix_syntax_errors(file_path):
            fixed_count += 1
    
    print(f"🎉 Fixed syntax errors in {fixed_count} files")

if __name__ == "__main__":
    main() 