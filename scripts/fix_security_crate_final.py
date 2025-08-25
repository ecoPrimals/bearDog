#!/usr/bin/env python3
"""
Comprehensive Security Crate Modernization Script
Fixes all error patterns, configuration issues, and modernizes to canonical patterns
"""

import re
import glob
import os

def fix_error_patterns(file_path, content):
    """Fix all error pattern issues in a file."""
    
    # Pattern 1: Old error variants to new constructors
    replacements = [
        # Crypto errors
        (r'BearDogError::Crypto\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::encryption("crypto", \1)'),
        
        # Encryption errors  
        (r'BearDogError::Encryption\s*\{\s*operation:\s*([^,}]+),\s*message:\s*([^}]+)\s*\}', r'BearDogError::encryption(\1, \2)'),
        (r'BearDogError::Encryption\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::encryption("encryption", \1)'),
        
        # Key derivation errors
        (r'BearDogError::KeyDerivation\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::encryption("key_derivation", \1)'),
        
        # Authentication errors
        (r'BearDogError::Authentication\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::authentication(\1)'),
        
        # Authorization errors
        (r'BearDogError::Authorization\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::authentication(\1)'),
        
        # Rate limit errors
        (r'BearDogError::RateLimit\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::timeout(\1)'),
        
        # Configuration errors
        (r'BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::configuration(\1)'),
        
        # Cryptographic errors
        (r'BearDogError::Cryptographic\s*\{\s*operation:\s*([^}]+)\s*\}', r'BearDogError::encryption("crypto", \1)'),
        
        # Invalid input errors
        (r'BearDogError::InvalidInput\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::invalid_input(\1)'),
        
        # Internal errors
        (r'BearDogError::Internal\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::internal(\1)'),
    ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    
    return content

def fix_configuration_issues(file_path, content):
    """Fix configuration-related issues."""
    
    # Fix Option<u32> unwrapping
    content = content.replace(
        'iterations: self.config.key_derivation_iterations,',
        'iterations: self.config.key_derivation_iterations.unwrap_or(10000),'
    )
    
    return content

def fix_pattern_matching(file_path, content):
    """Fix pattern matching for new error structure."""
    
    # Fix error pattern matching in types/mod.rs
    if 'types/mod.rs' in file_path:
        # Update match patterns to use new unified structure
        old_patterns = [
            (r'beardog_errors::BearDogError::Authentication \{ message \}', 
             r'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Authentication { message })'),
            (r'beardog_errors::BearDogError::Authorization \{ message \}',
             r'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Authorization { message })'),
            (r'beardog_errors::BearDogError::RateLimit \{ message \}',
             r'beardog_errors::BearDogError::System(beardog_errors::SystemError::Timeout { message })'),
            (r'beardog_errors::BearDogError::Configuration \{ message \}',
             r'beardog_errors::BearDogError::System(beardog_errors::SystemError::Configuration { message })'),
            (r'beardog_errors::BearDogError::Cryptographic \{ operation \}',
             r'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Encryption { operation, message: operation })'),
        ]
        
        for old, new in old_patterns:
            content = re.sub(old, new, content)
    
    return content

def add_missing_implementations(file_path, content):
    """Add missing implementations like MemoryKeyManager::new."""
    
    if 'memory_key_manager/mod.rs' in file_path:
        # Check if new() method exists
        if 'impl MemoryKeyManager' in content and 'fn new(' not in content:
            # Add new() method implementation
            impl_block = re.search(r'(impl MemoryKeyManager \{[^}]*)\}', content, re.DOTALL)
            if impl_block:
                new_impl = impl_block.group(1) + '''
    /// Create a new memory key manager
    pub async fn new(config: MemoryKeyManagerConfig) -> BearDogResult<Self> {
        Ok(Self {
            config,
            keys: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(MemoryKeyManagerMetrics::default())),
        })
    }
}'''
                content = content.replace(impl_block.group(0), new_impl)
    
    return content

def fix_security_crate():
    """Fix all issues in the security crate."""
    
    # Get all Rust files in the security crate
    files = glob.glob('crates/beardog-security/src/**/*.rs', recursive=True)
    
    for file_path in files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Apply all fixes
            content = fix_error_patterns(file_path, content)
            content = fix_configuration_issues(file_path, content)
            content = fix_pattern_matching(file_path, content)
            content = add_missing_implementations(file_path, content)
            
            # Write back if changed
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"✅ Fixed {file_path}")
        
        except Exception as e:
            print(f"❌ Error fixing {file_path}: {e}")

def main():
    print("🔧 Starting comprehensive security crate modernization...")
    fix_security_crate()
    print("🎉 Security crate modernization complete!")

if __name__ == "__main__":
    main() 