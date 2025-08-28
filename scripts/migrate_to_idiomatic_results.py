#!/usr/bin/env python3
"""
BearDog Result Migration Script
Migrates all BearDogResult<T> references to idiomatic Result<T, BearDogError> patterns
"""

import os
import re
import sys
from pathlib import Path

def migrate_file(file_path):
    """Migrate a single Rust file from BearDogResult to Result<T, BearDogError>"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Migration patterns
        migrations = [
            # Import statements
            (r'use beardog_errors::\{BearDogError, BearDogResult\};', 'use beardog_errors::BearDogError;'),
            (r'use beardog_errors::BearDogResult;', 'use beardog_errors::BearDogError;'),
            (r'pub use beardog_errors::BearDogResult;', 'pub use beardog_errors::BearDogError;'),
            
            # Function return types
            (r'-> BearDogResult<([^>]+)>', r'-> Result<\1, BearDogError>'),
            (r'-> beardog_errors::BearDogResult<([^>]+)>', r'-> Result<\1, BearDogError>'),
            
            # Variable declarations and type annotations
            (r'BearDogResult<([^>]+)>', r'Result<\1, BearDogError>'),
            (r'beardog_errors::BearDogResult<([^>]+)>', r'Result<\1, BearDogError>'),
            
            # Generic type parameters in complex expressions
            (r'Pin<Box<dyn std::future::Future<Output = BearDogResult<([^>]+)>>', 
             r'Pin<Box<dyn std::future::Future<Output = Result<\1, BearDogError>>'),
        ]
        
        # Apply migrations
        for pattern, replacement in migrations:
            content = re.sub(pattern, replacement, content)
        
        # Only write if content changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"Error migrating {file_path}: {e}")
        return False

def main():
    """Main migration function"""
    print("🚀 BearDog Result Migration Script")
    print("=" * 50)
    
    # Find all Rust files in crates directory
    crates_dir = Path("crates")
    if not crates_dir.exists():
        print("❌ Error: 'crates' directory not found. Run from project root.")
        sys.exit(1)
    
    rust_files = list(crates_dir.rglob("*.rs"))
    print(f"📁 Found {len(rust_files)} Rust files")
    
    # Filter files that contain BearDogResult
    files_to_migrate = []
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                if 'BearDogResult' in f.read():
                    files_to_migrate.append(file_path)
        except Exception:
            continue
    
    print(f"🎯 {len(files_to_migrate)} files need migration")
    
    if not files_to_migrate:
        print("✅ No files need migration!")
        return
    
    # Perform migration
    migrated_count = 0
    for file_path in files_to_migrate:
        if migrate_file(file_path):
            migrated_count += 1
            print(f"✅ Migrated: {file_path}")
        else:
            print(f"⚪ No changes: {file_path}")
    
    print()
    print("📊 Migration Summary:")
    print(f"   • Files processed: {len(files_to_migrate)}")
    print(f"   • Files migrated: {migrated_count}")
    print(f"   • Files unchanged: {len(files_to_migrate) - migrated_count}")
    
    if migrated_count > 0:
        print()
        print("🎉 Migration completed successfully!")
        print("💡 Next steps:")
        print("   1. Run 'cargo build --workspace' to check for any issues")
        print("   2. Fix any remaining compilation errors manually")
        print("   3. Update tests if needed")
    else:
        print("ℹ️  No files needed migration.")

if __name__ == "__main__":
    main() 