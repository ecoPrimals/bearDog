#!/usr/bin/env python3
"""
Final Syntax Cleanup Script
Fixes remaining syntax errors from the BearDogResult migration
"""

import os
import re
import sys
from pathlib import Path

def fix_syntax_errors(file_path):
    """Fix remaining syntax errors in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix patterns that caused syntax errors
        fixes = [
            # Fix extra '>' characters in function signatures
            (r'BearDogError>>', r'BearDogError>'),
            (r', BearDogError>>;', r', BearDogError>;'),
            (r', BearDogError>>> \+', r', BearDogError>> +'),
            
            # Fix HashMap patterns with BearDogError as second type param
            (r'HashMap<([^,>]+), ([^,>]+), BearDogError>', r'HashMap<\1, \2>'),
            
            # Fix duplicate BearDogError imports
            (r'use beardog_errors::BearDogError;\s*use beardog_errors::BearDogError;', 'use beardog_errors::BearDogError;'),
            
            # Add missing BearDogError imports where needed
            (r'^(?!.*use beardog_errors::BearDogError)(.*)BearDogError', r'use beardog_errors::BearDogError;\n\1BearDogError', re.MULTILINE),
            
            # Fix Result patterns that got mangled
            (r'Result<([^,>]+), BearDogError>, E>', r'Result<\1, BearDogError>'),
            
            # Fix specific patterns that broke
            (r'Result<Vec<([^>]+)>, BearDogError>, E>', r'Result<Vec<\1>, BearDogError>'),
            (r'Result<Option<([^>]+)>, BearDogError>, E>', r'Result<Option<\1>, BearDogError>'),
        ]
        
        # Apply fixes
        for pattern, replacement in fixes:
            if len(fixes) > 6:  # Skip the complex regex for now
                content = re.sub(pattern, replacement, content)
            else:
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE if 'MULTILINE' in str(fixes) else 0)
        
        # Only write if content changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def add_missing_imports(file_path):
    """Add missing BearDogError imports where needed"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if file uses BearDogError but doesn't import it
        if 'BearDogError' in content and 'use beardog_errors::BearDogError' not in content:
            # Find the right place to add the import (after other use statements)
            lines = content.split('\n')
            insert_index = 0
            
            # Find last use statement
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    insert_index = i + 1
            
            # Insert the import
            lines.insert(insert_index, 'use beardog_errors::BearDogError;')
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines))
            return True
        
        return False
        
    except Exception as e:
        print(f"Error adding imports to {file_path}: {e}")
        return False

def main():
    """Main cleanup function"""
    print("🔧 Final Syntax Cleanup Script")
    print("=" * 40)
    
    # Find all Rust files with compilation errors
    crates_dir = Path("crates")
    if not crates_dir.exists():
        print("❌ Error: 'crates' directory not found. Run from project root.")
        sys.exit(1)
    
    # Target specific problematic files first
    problem_files = [
        "crates/beardog-traits/src/canonical.rs",
        "crates/beardog-workflows/src/workflows/canonical/approval.rs",
        "crates/beardog-compliance/src/audit.rs",
        "crates/beardog-compliance/src/compliance/handlers.rs",
        "crates/beardog-workflows/src/workflows/storage/mod.rs",
        "crates/beardog-workflows/src/workflows/zero_cost_storage.rs",
        "crates/beardog-workflows/src/workflows/zero_cost_traits.rs",
        "crates/beardog-threat/src/threat/handlers/analysis.rs",
        "crates/beardog-threat/src/threat/handlers/incident.rs",
        "crates/beardog-workflows/src/workflows/notification/mod.rs",
        "crates/beardog-workflows/src/lib.rs",
        "crates/beardog-threat/src/threat/mod.rs",
    ]
    
    # Filter to existing files
    existing_files = [Path(f) for f in problem_files if Path(f).exists()]
    
    print(f"🎯 Targeting {len(existing_files)} problematic files")
    
    if not existing_files:
        print("✅ No target files found!")
        return
    
    # Perform fixes
    fixed_count = 0
    import_count = 0
    
    for file_path in existing_files:
        fixed = fix_syntax_errors(file_path)
        imports_added = add_missing_imports(file_path)
        
        if fixed or imports_added:
            fixed_count += 1
            status = "✅ Fixed"
            if imports_added:
                status += " + imports"
            print(f"{status}: {file_path}")
        else:
            print(f"⚪ No changes: {file_path}")
    
    print()
    print("📊 Cleanup Summary:")
    print(f"   • Files processed: {len(existing_files)}")
    print(f"   • Files fixed: {fixed_count}")
    
    if fixed_count > 0:
        print()
        print("🎉 Cleanup completed!")
        print("💡 Next step: Test compilation with 'cargo build'")
    else:
        print("ℹ️  No files needed fixing.")

if __name__ == "__main__":
    main() 