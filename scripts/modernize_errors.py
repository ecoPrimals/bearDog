#!/usr/bin/env python3
"""
BearDog Error Modernization Script

This script systematically modernizes all deprecated error patterns to the canonical form.
"""

import os
import re
import glob
from pathlib import Path

# Error pattern mappings from old to new canonical forms
ERROR_PATTERNS = {
    # Authentication errors
    r'BearDogError::Authentication\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::authentication(\1)',
    
    # Authorization errors  
    r'BearDogError::Authorization\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::authorization(\1)',
    
    # Rate limit errors
    r'BearDogError::RateLimit\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::rate_limit(\1)',
    
    # Configuration errors
    r'BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::configuration(\1)',
    
    # Cryptographic errors
    r'BearDogError::Cryptographic\s*\{\s*operation:\s*([^}]+)\s*\}': r'BearDogError::encryption("crypto", \1)',
    
    # Invalid input errors
    r'BearDogError::InvalidInput\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::invalid_input(\1)',
    
    # Internal errors
    r'BearDogError::Internal\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::internal(\1)',
    
    # Encryption errors
    r'BearDogError::Encryption\s*\{\s*operation:\s*([^,}]+),\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption(\1, \2)',
    r'BearDogError::Encryption\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("encryption", \1)',
    
    # Key derivation errors
    r'BearDogError::KeyDerivation\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("key_derivation", \1)',
    r'BearDogError::KeyDerivation\(([^)]+)\)': r'BearDogError::encryption("key_derivation", \1)',
    
    # Crypto errors
    r'BearDogError::Crypto\s*\{\s*operation:\s*([^,}]+),\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption(\1, \2)',
    r'BearDogError::Crypto\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("crypto", \1)',
    
    # IO errors
    r'BearDogError::IoError\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::storage(\1)',
}

def modernize_file(file_path):
    """Modernize error patterns in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = 0
        
        # Apply each pattern transformation
        for old_pattern, new_pattern in ERROR_PATTERNS.items():
            matches = re.findall(old_pattern, content, re.MULTILINE | re.DOTALL)
            if matches:
                content = re.sub(old_pattern, new_pattern, content, flags=re.MULTILINE | re.DOTALL)
                changes_made += len(matches)
        
        # Write back if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Modernized {changes_made} error patterns in {file_path}")
            return changes_made
        
        return 0
        
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return 0

def main():
    """Main modernization process."""
    print("🔧 Starting BearDog Error Modernization...")
    
    # Find all Rust files in the crates directory
    rust_files = []
    for pattern in ['crates/**/*.rs', 'tests/**/*.rs', 'examples/**/*.rs']:
        rust_files.extend(glob.glob(pattern, recursive=True))
    
    total_changes = 0
    files_modified = 0
    
    for file_path in rust_files:
        # Skip target directories and generated files
        if '/target/' in file_path or 'generated' in file_path:
            continue
            
        changes = modernize_file(file_path)
        if changes > 0:
            total_changes += changes
            files_modified += 1
    
    print(f"\n🎉 Modernization Complete!")
    print(f"📊 Files modified: {files_modified}")
    print(f"📊 Total error patterns updated: {total_changes}")
    
    if total_changes > 0:
        print(f"\n🚀 Next steps:")
        print(f"1. Run 'cargo check --workspace' to verify compilation")
        print(f"2. Run 'cargo test' to ensure tests pass")
        print(f"3. Review changes with 'git diff'")

if __name__ == "__main__":
    main() 