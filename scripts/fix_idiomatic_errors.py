#!/usr/bin/env python3
"""
Fix Idiomatic Error System Compilation Issues
Resolves import errors, duplicate derives, and missing types
"""

import re
from pathlib import Path

def fix_beardog_errors_compilation():
    """Fix compilation issues in beardog-errors crate"""
    print("🔧 **FIXING IDIOMATIC ERROR COMPILATION ISSUES**")
    print("=" * 60)
    
    # Fix compatibility.rs import issues
    compatibility_file = Path("crates/beardog-errors/src/idiomatic/compatibility.rs")
    fix_compatibility_imports(compatibility_file)
    
    # Fix lib.rs import issues
    lib_file = Path("crates/beardog-errors/src/lib.rs")
    fix_lib_imports(lib_file)
    
    # Fix domain_errors.rs duplicate derives
    domain_errors_file = Path("crates/beardog-errors/src/idiomatic/domain_errors.rs")
    fix_duplicate_derives(domain_errors_file)
    
    # Fix improved_results.rs duplicate derives
    improved_results_file = Path("crates/beardog-errors/src/improved_results.rs")
    fix_improved_results_duplicates(improved_results_file)
    
    print("✅ **IDIOMATIC ERROR FIXES COMPLETE**")

def fix_compatibility_imports(file_path: Path):
    """Fix import issues in compatibility.rs"""
    print(f"🔄 Fixing imports in {file_path}")
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Add missing SecurityError import
        if "use crate::categories::SecurityError;" not in content:
            # Find the imports section
            lines = content.split('\n')
            import_index = 0
            
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    import_index = i + 1
                elif line.strip() and not line.strip().startswith('use ') and import_index > 0:
                    break
            
            # Insert the missing import
            lines.insert(import_index, "use crate::categories::SecurityError;")
            content = '\n'.join(lines)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"  ✅ Fixed SecurityError imports in {file_path.name}")
    
    except Exception as e:
        print(f"  ❌ Error fixing {file_path}: {e}")

def fix_lib_imports(file_path: Path):
    """Fix import issues in lib.rs"""
    print(f"🔄 Fixing imports in {file_path}")
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix SecurityError re-export to use categories
        if "SecurityError," in content and "idiomatic::SecurityError" in content:
            content = content.replace(
                "SecurityError,",
                "// SecurityError, // Use categories::SecurityError instead"
            )
        
        # Add missing type imports
        missing_imports = [
            "use beardog_types::canonical::hsm::{HsmCapabilities, KeyMetadata};",
        ]
        
        lines = content.split('\n')
        import_added = False
        
        for import_line in missing_imports:
            if import_line not in content:
                # Find where to insert the import
                for i, line in enumerate(lines):
                    if line.strip().startswith('use beardog_types::') and not import_added:
                        lines.insert(i + 1, import_line)
                        import_added = True
                        break
        
        if import_added:
            content = '\n'.join(lines)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"  ✅ Fixed imports in {file_path.name}")
    
    except Exception as e:
        print(f"  ❌ Error fixing {file_path}: {e}")

def fix_duplicate_derives(file_path: Path):
    """Fix duplicate derive macros in domain_errors.rs"""
    print(f"🔄 Fixing duplicate derives in {file_path}")
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Remove duplicate GeneticsError definition
        # Find the duplicate definition pattern
        duplicate_pattern = r'#\[derive\(Error, Debug, Clone, Serialize, Deserialize\)\]\s*pub enum GeneticsError \{[^}]*\}'
        matches = list(re.finditer(duplicate_pattern, content, re.DOTALL))
        
        if len(matches) > 1:
            # Remove all but the first definition
            for match in matches[1:]:
                content = content.replace(match.group(0), "")
        
        # Clean up extra whitespace
        content = re.sub(r'\n\n\n+', '\n\n', content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"  ✅ Fixed duplicate derives in {file_path.name}")
    
    except Exception as e:
        print(f"  ❌ Error fixing {file_path}: {e}")

def fix_improved_results_duplicates(file_path: Path):
    """Fix duplicate derives in improved_results.rs"""
    print(f"🔄 Fixing duplicate derives in {file_path}")
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix ProviderInfo duplicates
        provider_info_pattern = r'#\[derive\(Debug, Clone, Serialize, Deserialize\)\]\s*pub struct ProviderInfo \{[^}]*\}'
        matches = list(re.finditer(provider_info_pattern, content, re.DOTALL))
        
        if len(matches) > 1:
            for match in matches[1:]:
                content = content.replace(match.group(0), "")
        
        # Fix KeyType duplicates
        key_type_pattern = r'#\[derive\(Debug, Clone, Serialize, Deserialize\)\]\s*pub enum KeyType \{[^}]*\}'
        matches = list(re.finditer(key_type_pattern, content, re.DOTALL))
        
        if len(matches) > 1:
            for match in matches[1:]:
                content = content.replace(match.group(0), "")
        
        # Clean up extra whitespace
        content = re.sub(r'\n\n\n+', '\n\n', content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"  ✅ Fixed duplicate derives in {file_path.name}")
    
    except Exception as e:
        print(f"  ❌ Error fixing {file_path}: {e}")

if __name__ == "__main__":
    fix_beardog_errors_compilation() 