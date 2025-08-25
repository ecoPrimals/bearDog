#!/usr/bin/env python3
"""
Fix syntax errors in address_management.rs
"""

import re

def fix_address_management():
    """Fix all syntax errors in address_management.rs."""
    
    file_path = 'crates/beardog-security/src/address_management.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix all malformed format strings and parentheses
    fixes = [
        # Fix format string with wrong closing brace
        (r'format!\("([^"]*)\{)\)", ([^)]+)\),\s*\}\)', r'format!("\1}", \2))'),
        
        # Fix nested error constructors with wrong parentheses
        (r'BearDogError::(\w+)\(\s*format!\("([^"]*)", ([^)]+)\),\s*\}\)', r'BearDogError::\1(format!("\2", \3))'),
        
        # Fix specific patterns in this file
        (r'format!\("Invalid Ed25519 key material length: \{)\)", ([^)]+)\),\s*\}\)', r'format!("Invalid Ed25519 key material length: {}", \1))'),
        (r'format!\("Invalid derivation path component: \{part)\)", ([^)]+)\),\s*\}\)', r'format!("Invalid derivation path component: {part}", \1))'),
    ]
    
    for pattern, replacement in fixes:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    
    # Manual fixes for specific known issues
    content = content.replace(
        'format!("Invalid Ed25519 key material length: {)", key_material.len()),',
        'format!("Invalid Ed25519 key material length: {}", key_material.len())'
    )
    
    content = content.replace(
        'format!("Invalid derivation path component: {part)"),',
        'format!("Invalid derivation path component: {part}")'
    )
    
    # Fix any remaining }) patterns
    content = re.sub(r',\s*\}\)', ')', content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print("✅ Fixed address_management.rs syntax errors")
    else:
        print("ℹ️ No changes needed in address_management.rs")

if __name__ == "__main__":
    fix_address_management() 