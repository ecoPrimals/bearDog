#!/usr/bin/env python3
"""
Fix malformed format strings in Rust files
"""

import re
import glob

def fix_format_strings(file_path):
    """Fix malformed format strings in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix pattern: format!("text {)", var),
        # Should be: format!("text {}", var)
        content = re.sub(
            r'format!\("([^"]*)\{)\)", ([^,)]+)\),',
            r'format!("\1}", \2)',
            content,
            flags=re.MULTILINE
        )
        
        # Fix pattern: format!("text {)", var),\n            });
        # Should be: format!("text {}", var)));
        content = re.sub(
            r'format!\("([^"]*)\{)\)", ([^,)]+)\),\s*\}\);',
            r'format!("\1}", \2)));',
            content,
            flags=re.MULTILINE
        )
        
        # Fix pattern: .map_err(|e| Error(format!("text {e)"),
        # Should be: .map_err(|e| Error(format!("text {e}")))
        content = re.sub(
            r'\.map_err\(\|([^|]+)\| BearDogError::([^(]+)\(format!\("([^"]*\{[^}]*)\)"\),?\s*\}\)\?;',
            r'.map_err(|\1| BearDogError::\2(format!("\3}")))?;',
            content,
            flags=re.MULTILINE
        )
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed format strings in {file_path}")
            return True
        
        return False
        
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False

def main():
    """Main process."""
    print("🔧 Fixing format string syntax errors...")
    
    # Focus on the problematic files
    target_files = [
        'crates/beardog-security/src/crypto_utils.rs',
        'crates/beardog-security/src/encryption.rs',
        'crates/beardog-security/src/address_management.rs',
        'crates/beardog-security/src/zero_cost/crypto_operations.rs',
        'crates/beardog-security/src/zero_cost/security_levels.rs'
    ]
    
    fixed_count = 0
    for file_path in target_files:
        if fix_format_strings(file_path):
            fixed_count += 1
    
    print(f"🎉 Fixed format strings in {fixed_count} files")

if __name__ == "__main__":
    main() 