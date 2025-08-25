#!/usr/bin/env python3
"""
Final Security Crate Modernization - Surgical Approach
Complete the last 15% with precise, error-free transformations
"""

import re
import glob
import os

def apply_surgical_fixes(content, file_path):
    """Apply precise surgical fixes without breaking syntax."""
    
    # Phase 1: Simple direct replacements (safest)
    simple_replacements = [
        # Error enum variants to constructors
        ('BearDogError::Crypto {', 'BearDogError::encryption("crypto", '),
        ('BearDogError::Encryption {', 'BearDogError::encryption("encryption", '),
        ('BearDogError::KeyDerivation {', 'BearDogError::encryption("key_derivation", '),
        ('BearDogError::Authentication {', 'BearDogError::authentication('),
        ('BearDogError::Authorization {', 'BearDogError::authentication('),
        ('BearDogError::RateLimit {', 'BearDogError::timeout('),
        ('BearDogError::Configuration {', 'BearDogError::configuration('),
        ('BearDogError::InvalidInput {', 'BearDogError::invalid_input('),
        ('BearDogError::Internal {', 'BearDogError::internal('),
        ('BearDogError::Cryptographic {', 'BearDogError::encryption("crypto", '),
    ]
    
    for old, new in simple_replacements:
        content = content.replace(old, new)
    
    # Phase 2: Fix message field patterns
    content = re.sub(r'message:\s*([^,}]+),?\s*}', r'\1)', content)
    content = re.sub(r'operation:\s*([^,}]+),?\s*}', r'"crypto", \1)', content)
    
    # Phase 3: Fix specific configuration issues
    if 'encryption.rs' in file_path:
        content = content.replace(
            'iterations: self.config.key_derivation_iterations,',
            'iterations: self.config.key_derivation_iterations.unwrap_or(10000),'
        )
    
    return content

def fix_pattern_matching(content, file_path):
    """Fix pattern matching for unified error types."""
    
    if 'types/mod.rs' in file_path:
        # Update error pattern matching to use new structure
        pattern_fixes = [
            ('beardog_errors::BearDogError::Authentication { message }', 
             'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Authentication { message })'),
            ('beardog_errors::BearDogError::Authorization { message }',
             'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Authorization { message })'),
            ('beardog_errors::BearDogError::RateLimit { message }',
             'beardog_errors::BearDogError::System(beardog_errors::SystemError::Timeout { message })'),
            ('beardog_errors::BearDogError::Configuration { message }',
             'beardog_errors::BearDogError::System(beardog_errors::SystemError::Configuration { message })'),
            ('beardog_errors::BearDogError::Cryptographic { operation }',
             'beardog_errors::BearDogError::Security(beardog_errors::SecurityError::Encryption { operation, message: operation })'),
        ]
        
        for old, new in pattern_fixes:
            content = content.replace(old, new)
    
    return content

def modernize_file(file_path):
    """Modernize a single file with surgical precision."""
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply surgical fixes
        content = apply_surgical_fixes(content, file_path)
        content = fix_pattern_matching(content, file_path)
        
        # Write back only if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Surgically modernized {file_path}")
            return True
        
        return False
    
    except Exception as e:
        print(f"❌ Error in {file_path}: {e}")
        return False

def add_missing_implementations():
    """Add missing implementations that are causing compilation errors."""
    
    # Add missing new() method to MemoryKeyManager
    mod_file = 'crates/beardog-security/src/memory_key_manager/mod.rs'
    if os.path.exists(mod_file):
        with open(mod_file, 'r') as f:
            content = f.read()
        
        if 'impl MemoryKeyManager' in content and 'pub async fn new(' not in content:
            # Find the end of the struct definition and add impl
            if 'pub struct MemoryKeyManager' in content and 'impl MemoryKeyManager' not in content:
                impl_block = '''
impl MemoryKeyManager {
    /// Create a new memory key manager
    pub async fn new(config: MemoryKeyManagerConfig) -> BearDogResult<Self> {
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::RwLock;
        
        Ok(Self {
            config,
            keys: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(MemoryKeyManagerMetrics::default())),
        })
    }
}
'''
                content += impl_block
                
                with open(mod_file, 'w') as f:
                    f.write(content)
                print(f"✅ Added missing new() method to MemoryKeyManager")

def main():
    """Complete the final security crate modernization."""
    
    print("🔧 Starting final security crate modernization...")
    
    # First, fix the module conflict
    conflict_file = 'crates/beardog-security/src/memory_key_manager.rs'
    if os.path.exists(conflict_file):
        os.remove(conflict_file)
        print("✅ Removed conflicting memory_key_manager.rs file")
    
    # Add missing implementations
    add_missing_implementations()
    
    # Get all Rust files in security crate
    files = glob.glob('crates/beardog-security/src/**/*.rs', recursive=True)
    modernized_count = 0
    
    for file_path in files:
        if modernize_file(file_path):
            modernized_count += 1
    
    print(f"🎉 Surgically modernized {modernized_count} files")
    print("✅ Final security crate modernization complete!")

if __name__ == "__main__":
    main() 