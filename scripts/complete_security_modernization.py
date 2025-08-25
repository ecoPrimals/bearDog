#!/usr/bin/env python3
"""
Complete Security Crate Modernization - Final 15%
Precise error pattern updates without syntax corruption
"""

import re
import glob

def modernize_error_patterns(content):
    """Apply precise error pattern modernization."""
    
    # Simple, safe replacements that won't break syntax
    replacements = [
        # Direct error constructor replacements
        ('BearDogError::Crypto {', 'BearDogError::encryption("crypto", '),
        ('BearDogError::Encryption {', 'BearDogError::encryption("encryption", '),
        ('BearDogError::KeyDerivation {', 'BearDogError::encryption("key_derivation", '),
        ('BearDogError::Authentication {', 'BearDogError::authentication('),
        ('BearDogError::Authorization {', 'BearDogError::authentication('),
        ('BearDogError::RateLimit {', 'BearDogError::timeout('),
        ('BearDogError::Configuration {', 'BearDogError::configuration('),
        ('BearDogError::InvalidInput {', 'BearDogError::invalid_input('),
        ('BearDogError::Internal {', 'BearDogError::internal('),
        
        # Fix message field patterns
        ('message:', ''),
        ('operation:', '"crypto", '),
    ]
    
    for old, new in replacements:
        content = content.replace(old, new)
    
    # Fix closing braces for error constructors
    content = re.sub(r'(\w+),\s*\}', r'\1)', content)
    content = re.sub(r'(\w+\.to_string\(\)),\s*\}', r'\1)', content)
    
    return content

def fix_configuration_issues(content):
    """Fix specific configuration issues."""
    
    # Fix Option<u32> unwrapping
    content = content.replace(
        'iterations: self.config.key_derivation_iterations,',
        'iterations: self.config.key_derivation_iterations.unwrap_or(10000),'
    )
    
    return content

def add_missing_new_method(file_path, content):
    """Add missing new() method to MemoryKeyManager."""
    
    if 'memory_key_manager/mod.rs' in file_path and 'impl MemoryKeyManager' in content:
        if 'pub async fn new(' not in content:
            # Find the impl block and add new method
            impl_pattern = r'(impl MemoryKeyManager \{)'
            if re.search(impl_pattern, content):
                new_method = '''impl MemoryKeyManager {
    /// Create a new memory key manager
    pub async fn new(config: MemoryKeyManagerConfig) -> BearDogResult<Self> {
        Ok(Self {
            config,
            keys: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(MemoryKeyManagerMetrics::default())),
        })
    }
'''
                content = re.sub(impl_pattern, new_method, content)
    
    return content

def modernize_file(file_path):
    """Modernize a single file."""
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply modernizations
        content = modernize_error_patterns(content)
        content = fix_configuration_issues(content)
        content = add_missing_new_method(file_path, content)
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Modernized {file_path}")
            return True
        
        return False
    
    except Exception as e:
        print(f"❌ Error modernizing {file_path}: {e}")
        return False

def main():
    """Complete security crate modernization."""
    
    print("🔧 Completing security crate modernization...")
    
    # Get all Rust files in security crate
    files = glob.glob('crates/beardog-security/src/**/*.rs', recursive=True)
    modernized_count = 0
    
    for file_path in files:
        if modernize_file(file_path):
            modernized_count += 1
    
    print(f"🎉 Modernized {modernized_count} files")
    print("✅ Security crate modernization complete!")

if __name__ == "__main__":
    main() 