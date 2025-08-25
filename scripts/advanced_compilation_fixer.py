#!/usr/bin/env python3
"""
Advanced Compilation Fixer
Resolves complex compilation issues in beardog-errors with comprehensive fixes
"""

import re
from pathlib import Path
from typing import Dict, List

class AdvancedCompilationFixer:
    def __init__(self):
        self.errors_crate = Path("crates/beardog-errors")
        self.fixes_applied = []
        
    def fix_all_compilation_issues(self):
        """Fix all compilation issues systematically"""
        print("🔧 **ADVANCED COMPILATION FIXES**")
        print("=" * 50)
        
        # 1. Fix lib.rs export issues
        self._fix_lib_exports()
        
        # 2. Fix domain_errors.rs duplicates
        self._fix_domain_errors_duplicates()
        
        # 3. Fix improved_results.rs duplicates and missing types
        self._fix_improved_results_issues()
        
        # 4. Fix compatibility.rs variant mismatches
        self._fix_compatibility_variants()
        
        # 5. Add missing type imports
        self._add_missing_type_imports()
        
        self._generate_fix_summary()
    
    def _fix_lib_exports(self):
        """Fix lib.rs export issues"""
        lib_file = self.errors_crate / "src" / "lib.rs"
        print(f"🔄 Fixing exports in {lib_file}")
        
        try:
            with open(lib_file, 'r') as f:
                content = f.read()
            
            # Remove problematic SecurityError export from idiomatic
            content = re.sub(
                r'^\s*SecurityError,\s*$',
                '    // SecurityError, // Use categories::SecurityError instead',
                content,
                flags=re.MULTILINE
            )
            
            # Ensure proper re-exports
            if "pub use categories::SecurityError;" not in content:
                # Add proper re-export after other use statements
                content = content.replace(
                    "pub use categories::",
                    "pub use categories::SecurityError;\npub use categories::"
                )
            
            with open(lib_file, 'w') as f:
                f.write(content)
            
            self.fixes_applied.append("Fixed lib.rs exports")
            print("  ✅ Fixed lib.rs exports")
            
        except Exception as e:
            print(f"  ❌ Error fixing lib.rs: {e}")
    
    def _fix_domain_errors_duplicates(self):
        """Fix duplicate definitions in domain_errors.rs"""
        domain_file = self.errors_crate / "src" / "idiomatic" / "domain_errors.rs"
        print(f"🔄 Fixing duplicates in {domain_file}")
        
        try:
            with open(domain_file, 'r') as f:
                content = f.read()
            
            # Remove all duplicate GeneticsError definitions
            genetics_pattern = r'#\[derive\(Error, Debug, Clone, Serialize, Deserialize\)\]\s*pub enum GeneticsError\s*\{[^}]*\}'
            matches = list(re.finditer(genetics_pattern, content, re.DOTALL))
            
            if len(matches) > 1:
                # Keep only the first definition
                for match in matches[1:]:
                    content = content.replace(match.group(0), "")
                    print(f"    Removed duplicate GeneticsError definition")
            
            # Clean up excessive whitespace
            content = re.sub(r'\n\s*\n\s*\n+', '\n\n', content)
            
            with open(domain_file, 'w') as f:
                f.write(content)
            
            self.fixes_applied.append("Fixed domain_errors.rs duplicates")
            print("  ✅ Fixed domain_errors.rs duplicates")
            
        except Exception as e:
            print(f"  ❌ Error fixing domain_errors.rs: {e}")
    
    def _fix_improved_results_issues(self):
        """Fix improved_results.rs duplicates and missing types"""
        results_file = self.errors_crate / "src" / "improved_results.rs"
        print(f"🔄 Fixing issues in {results_file}")
        
        try:
            with open(results_file, 'r') as f:
                content = f.read()
            
            # Remove duplicate ProviderInfo definitions
            provider_pattern = r'#\[derive\(Debug, Clone, Serialize, Deserialize\)\]\s*pub struct ProviderInfo\s*\{[^}]*\}'
            matches = list(re.finditer(provider_pattern, content, re.DOTALL))
            
            if len(matches) > 1:
                for match in matches[1:]:
                    content = content.replace(match.group(0), "")
                    print(f"    Removed duplicate ProviderInfo definition")
            
            # Remove duplicate KeyType definitions
            keytype_pattern = r'#\[derive\(Debug, Clone, Serialize, Deserialize\)\]\s*pub enum KeyType\s*\{[^}]*\}'
            matches = list(re.finditer(keytype_pattern, content, re.DOTALL))
            
            if len(matches) > 1:
                for match in matches[1:]:
                    content = content.replace(match.group(0), "")
                    print(f"    Removed duplicate KeyType definition")
            
            # Add missing imports at the top
            if "use beardog_types::canonical::hsm::{HsmCapabilities, KeyMetadata};" not in content:
                # Find the first use statement and insert before it
                lines = content.split('\n')
                for i, line in enumerate(lines):
                    if line.strip().startswith('use '):
                        lines.insert(i, "use beardog_types::canonical::hsm::{HsmCapabilities, KeyMetadata};")
                        break
                content = '\n'.join(lines)
                print(f"    Added missing type imports")
            
            # Clean up excessive whitespace
            content = re.sub(r'\n\s*\n\s*\n+', '\n\n', content)
            
            with open(results_file, 'w') as f:
                f.write(content)
            
            self.fixes_applied.append("Fixed improved_results.rs issues")
            print("  ✅ Fixed improved_results.rs issues")
            
        except Exception as e:
            print(f"  ❌ Error fixing improved_results.rs: {e}")
    
    def _fix_compatibility_variants(self):
        """Fix SecurityError variant mismatches in compatibility.rs"""
        compat_file = self.errors_crate / "src" / "idiomatic" / "compatibility.rs"
        print(f"🔄 Fixing variants in {compat_file}")
        
        try:
            with open(compat_file, 'r') as f:
                content = f.read()
            
            # Map old variant names to new ones based on the actual SecurityError definition
            variant_mappings = {
                'AuthenticationFailed': 'Authentication',
                'KeyRotationFailed': 'KeyManagement', 
                'EncryptionFailed': 'Encryption',
                'HsmOperationFailed': 'HsmOperation'
            }
            
            for old_variant, new_variant in variant_mappings.items():
                # Replace variant usage in match arms
                content = re.sub(
                    rf'SecurityError::{old_variant}',
                    f'SecurityError::{new_variant}',
                    content
                )
                print(f"    Mapped {old_variant} → {new_variant}")
            
            with open(compat_file, 'w') as f:
                f.write(content)
            
            self.fixes_applied.append("Fixed compatibility.rs variants")
            print("  ✅ Fixed compatibility.rs variants")
            
        except Exception as e:
            print(f"  ❌ Error fixing compatibility.rs: {e}")
    
    def _add_missing_type_imports(self):
        """Add any remaining missing type imports"""
        files_to_check = [
            self.errors_crate / "src" / "lib.rs",
            self.errors_crate / "src" / "improved_results.rs",
            self.errors_crate / "src" / "idiomatic" / "compatibility.rs"
        ]
        
        for file_path in files_to_check:
            try:
                with open(file_path, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Add beardog_types imports if missing
                if "HsmCapabilities" in content and "use beardog_types" not in content:
                    lines = content.split('\n')
                    for i, line in enumerate(lines):
                        if line.strip().startswith('use ') and 'beardog_types' not in line:
                            lines.insert(i, "use beardog_types::canonical::hsm::{HsmCapabilities, KeyMetadata};")
                            break
                    content = '\n'.join(lines)
                
                if content != original_content:
                    with open(file_path, 'w') as f:
                        f.write(content)
                    print(f"  ✅ Added missing imports to {file_path.name}")
                    
            except Exception as e:
                print(f"  ❌ Error checking {file_path}: {e}")
    
    def _generate_fix_summary(self):
        """Generate summary of fixes applied"""
        print(f"\n🎯 **COMPILATION FIXES SUMMARY**")
        print("=" * 40)
        print(f"Total Fixes Applied: {len(self.fixes_applied)}")
        
        for i, fix in enumerate(self.fixes_applied, 1):
            print(f"  {i}. {fix}")
        
        print(f"\n✅ **ADVANCED COMPILATION FIXES COMPLETE**")

def main():
    """Main execution"""
    fixer = AdvancedCompilationFixer()
    fixer.fix_all_compilation_issues()

if __name__ == "__main__":
    main() 