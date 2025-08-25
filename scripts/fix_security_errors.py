#!/usr/bin/env python3
"""
Fix security crate error patterns to use canonical constructors
"""

import re
import glob

def fix_security_crate():
    """Fix all error patterns in the security crate."""
    
    # Get all Rust files in the security crate
    files = glob.glob('crates/beardog-security/src/**/*.rs', recursive=True)
    
    for file_path in files:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix old error patterns to new canonical constructors
        replacements = {
            # Crypto errors
            r'BearDogError::Crypto\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("crypto", \1)',
            
            # Encryption errors  
            r'BearDogError::Encryption\s*\{\s*operation:\s*([^,}]+),\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption(\1, \2)',
            r'BearDogError::Encryption\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("encryption", \1)',
            
            # Key derivation errors
            r'BearDogError::KeyDerivation\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::encryption("key_derivation", \1)',
            
            # Authentication errors
            r'BearDogError::Authentication\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::authentication(\1)',
            
            # Authorization errors
            r'BearDogError::Authorization\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::authentication(\1)',
            
            # Rate limit errors
            r'BearDogError::RateLimit\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::timeout(\1)',
            
            # Configuration errors
            r'BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::configuration(\1)',
            
            # Cryptographic errors
            r'BearDogError::Cryptographic\s*\{\s*operation:\s*([^}]+)\s*\}': r'BearDogError::encryption("crypto", \1)',
            
            # Invalid input errors
            r'BearDogError::InvalidInput\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::invalid_input(\1)',
            
            # Internal errors
            r'BearDogError::Internal\s*\{\s*message:\s*([^}]+)\s*\}': r'BearDogError::internal(\1)',
        }
        
        # Apply all replacements
        for pattern, replacement in replacements.items():
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"✅ Fixed {file_path}")

def fix_module_conflict():
    """Fix the memory_key_manager module conflict."""
    # Remove the standalone file to resolve the conflict
    import os
    if os.path.exists('crates/beardog-security/src/memory_key_manager.rs'):
        os.remove('crates/beardog-security/src/memory_key_manager.rs')
        print("✅ Removed conflicting memory_key_manager.rs file")

def fix_config_issue():
    """Fix the config issue in encryption.rs."""
    file_path = 'crates/beardog-security/src/encryption.rs'
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix the Option<u32> issue
    content = content.replace(
        'iterations: self.config.key_derivation_iterations,',
        'iterations: self.config.key_derivation_iterations.unwrap_or(10000),'
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    print("✅ Fixed config iteration issue")

if __name__ == "__main__":
    print("🔧 Fixing security crate error patterns...")
    fix_security_crate()
    fix_module_conflict()
    fix_config_issue()
    print("🎉 Security crate fixes complete!") 