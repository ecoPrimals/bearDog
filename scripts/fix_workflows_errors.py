#!/usr/bin/env python3
"""
BearDog Workflows Error Pattern Fixer
Fixes multiline error patterns to use canonical constructors
"""

import os
import re
import glob

def fix_error_patterns(file_path):
    """Fix error patterns in a single file"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix InvalidInput patterns
        content = re.sub(
            r'BearDogError::InvalidInput\s*\{\s*message:\s*([^}]+)\s*\}',
            r'BearDogError::invalid_input(\1)',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Fix Configuration patterns
        content = re.sub(
            r'BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}',
            r'BearDogError::configuration(\1)',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Fix Workflow patterns  
        content = re.sub(
            r'BearDogError::Workflow\s*\{\s*message:\s*([^}]+)\s*\}',
            r'BearDogError::validation(\1)',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Fix NotFound patterns
        content = re.sub(
            r'BearDogError::NotFound\s*\{\s*message:\s*([^}]+)\s*\}',
            r'BearDogError::not_found(\1)',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Fix Internal patterns
        content = re.sub(
            r'BearDogError::Internal\s*\{\s*message:\s*([^}]+)\s*\}',
            r'BearDogError::internal(\1)',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            print(f"No changes: {file_path}")
            return False
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function"""
    print("🔄 BearDog Workflows Error Pattern Fixer")
    print("========================================")
    
    # Find all Rust files in workflows crate
    pattern = "crates/beardog-workflows/src/**/*.rs"
    files = glob.glob(pattern, recursive=True)
    
    if not files:
        print("No Rust files found in workflows crate")
        return
    
    fixed_count = 0
    for file_path in files:
        if fix_error_patterns(file_path):
            fixed_count += 1
    
    print(f"\n✅ Processing complete: {fixed_count}/{len(files)} files updated")

if __name__ == "__main__":
    main() 