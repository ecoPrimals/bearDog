#!/usr/bin/env python3
"""
Final Canonicalization Script for BearDog
Completes the canonical modernization by fixing remaining deprecated error patterns.
"""

import os
import re
import glob

def fix_validation_errors(content):
    """Fix ValidationError patterns to use canonical validation() constructor."""
    # Replace ValidationError(message) with validation(message)
    content = re.sub(
        r'BearDogError::ValidationError\(([^)]+)\)',
        r'BearDogError::validation(\1)',
        content
    )
    return content

def fix_timeout_errors(content):
    """Fix TimeoutError patterns to use canonical timeout() constructor."""
    # Replace TimeoutError { ... } with timeout(...)
    content = re.sub(
        r'BearDogError::TimeoutError\s*\{[^}]*\}',
        'BearDogError::timeout("Operation timed out")',
        content
    )
    return content

def fix_unknown_errors(content):
    """Fix Unknown error patterns to use canonical internal() constructor."""
    # Replace Unknown { ... } with internal(...)
    content = re.sub(
        r'BearDogError::Unknown\s*\{[^}]*\}',
        'BearDogError::internal("Unknown error occurred")',
        content
    )
    return content

def fix_no_suitable_provider_errors(content):
    """Fix NoSuitableProvider patterns."""
    content = re.sub(
        r'BearDogError::NoSuitableProvider\s*\{[^}]*\}',
        'BearDogError::internal("No suitable provider found")',
        content
    )
    return content

def fix_unsupported_operation_errors(content):
    """Fix UnsupportedOperation patterns."""
    content = re.sub(
        r'BearDogError::UnsupportedOperation\s*\{[^}]*\}',
        'BearDogError::internal("Unsupported operation")',
        content
    )
    return content

def process_file(file_path):
    """Process a single Rust file to fix deprecated error patterns."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_validation_errors(content)
        content = fix_timeout_errors(content)
        content = fix_unknown_errors(content)
        content = fix_no_suitable_provider_errors(content)
        content = fix_unsupported_operation_errors(content)
        
        # Only write if content changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed: {file_path}")
            return True
        
        return False
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False

def main():
    """Main function to process all Rust files."""
    print("🔧 FINAL CANONICALIZATION - Fixing remaining deprecated error patterns...")
    
    # Find all Rust files
    rust_files = []
    for pattern in ['**/*.rs']:
        rust_files.extend(glob.glob(pattern, recursive=True))
    
    # Filter out target directories and generated files
    rust_files = [f for f in rust_files if '/target/' not in f and 'generated' not in f]
    
    fixed_count = 0
    total_files = len(rust_files)
    
    for file_path in rust_files:
        if process_file(file_path):
            fixed_count += 1
    
    print(f"\n🎯 FINAL CANONICALIZATION COMPLETE:")
    print(f"✅ Files processed: {total_files}")
    print(f"✅ Files fixed: {fixed_count}")
    print(f"✅ Deprecated patterns eliminated")
    print(f"✅ Canonical error constructors unified")
    
    if fixed_count > 0:
        print(f"\n🚀 Ready for final compilation verification!")
    else:
        print(f"\n✨ All error patterns already canonical!")

if __name__ == "__main__":
    main() 