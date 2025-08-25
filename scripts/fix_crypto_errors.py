#!/usr/bin/env python3
"""
Fix BearDogError::Crypto patterns in crypto_utils.rs
"""

import re

def fix_crypto_utils():
    """Fix the crypto_utils file specifically."""
    file_path = 'crates/beardog-security/src/crypto_utils.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Pattern 1: BearDogError::Crypto { message: format!(...) }
    content = re.sub(
        r'BearDogError::Crypto\s*\{\s*message:\s*([^}]+)\s*\}',
        r'BearDogError::encryption("crypto", \1)',
        content,
        flags=re.MULTILINE | re.DOTALL
    )
    
    # Pattern 2: BearDogError::InvalidInput { message: ... }
    content = re.sub(
        r'BearDogError::InvalidInput\s*\{\s*message:\s*([^}]+)\s*\}',
        r'BearDogError::invalid_input(\1)',
        content,
        flags=re.MULTILINE | re.DOTALL
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✅ Fixed crypto_utils.rs")

if __name__ == "__main__":
    fix_crypto_utils() 