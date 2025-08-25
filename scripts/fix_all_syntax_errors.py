#!/usr/bin/env python3
"""
Comprehensive Syntax Error Fix Script

Fixes all unclosed delimiter issues identified during cargo fmt.
This addresses the remaining compilation blockers in the canonical modernization.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Tuple, Dict

class SyntaxErrorFixer:
    def __init__(self):
        self.files_fixed = 0
        self.errors_fixed = 0
        
    def fix_file(self, filepath: str) -> bool:
        """Fix syntax errors in a single file."""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix specific patterns based on the cargo fmt output
            content = self._fix_struct_definitions(content)
            content = self._fix_impl_blocks(content)
            content = self._fix_pub_use_statements(content)
            content = self._fix_function_definitions(content)
            content = self._fix_enum_definitions(content)
            content = self._fix_test_modules(content)
            content = self._fix_match_expressions(content)
            
            if content != original_content:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                self.files_fixed += 1
                self.errors_fixed += content.count('}') - original_content.count('}')
                print(f"Fixed: {filepath}")
                return True
                
        except Exception as e:
            print(f"Error fixing {filepath}: {e}")
            
        return False
    
    def _fix_struct_definitions(self, content: str) -> str:
        """Fix unclosed struct definitions."""
        # Fix struct definitions missing closing braces
        patterns = [
            # pub struct Name { ... missing }
            (r'(pub struct \w+\s*{[^}]*?)(\n\s*impl\s+)', r'\1}\n\n\2'),
            # pub struct Name { field: Type, missing }
            (r'(pub struct \w+\s*{[^}]*?,)\s*(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod))', r'\1\n}\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_impl_blocks(self, content: str) -> str:
        """Fix unclosed impl blocks."""
        patterns = [
            # impl Type { ... missing }
            (r'(impl[^{]*{[^}]*?)(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod|\#))', r'\1}\n\n\2'),
            # impl blocks with methods but no closing brace
            (r'(impl[^{]*{\s*(?:pub\s+)?fn[^}]*?}[^}]*?)(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod))', r'\1}\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_pub_use_statements(self, content: str) -> str:
        """Fix unclosed pub use statements."""
        patterns = [
            # pub use crate::module::{ ... missing }
            (r'(pub use [^{]*{[^}]*?)(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod|use))', r'\1};\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_function_definitions(self, content: str) -> str:
        """Fix unclosed function definitions."""
        patterns = [
            # Functions missing closing braces
            (r'((?:pub\s+)?(?:async\s+)?fn\s+\w+[^{]*{[^}]*?)(\n\s*(?:pub\s+)?(?:async\s+)?fn\s+)', r'\1}\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_enum_definitions(self, content: str) -> str:
        """Fix unclosed enum definitions."""
        patterns = [
            # enum Name { ... missing }
            (r'((?:pub\s+)?enum\s+\w+\s*{[^}]*?)(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod))', r'\1}\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_test_modules(self, content: str) -> str:
        """Fix unclosed test modules."""
        patterns = [
            # #[cfg(test)] mod tests { ... missing }
            (r'(#\[cfg\(test\)\]\s*mod\s+\w+\s*{[^}]*?)(\n\s*$)', r'\1}\n'),
            # mod tests { ... missing }
            (r'(mod\s+tests\s*{[^}]*?)(\n\s*$)', r'\1}\n'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def _fix_match_expressions(self, content: str) -> str:
        """Fix unclosed match expressions."""
        patterns = [
            # match expr { ... missing }
            (r'(match\s+[^{]*{[^}]*?)(\n\s*(?:pub\s+)?(?:struct|impl|enum|fn|mod))', r'\1}\n\n\2'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
        return content
    
    def fix_all_files(self):
        """Fix all Rust files in the crates directory."""
        print("🔧 Starting comprehensive syntax error fixes...")
        
        # Get all Rust files in crates directory
        rust_files = []
        for root, dirs, files in os.walk("crates"):
            for file in files:
                if file.endswith(".rs"):
                    rust_files.append(os.path.join(root, file))
        
        print(f"Found {len(rust_files)} Rust files to check")
        
        # Fix each file
        for filepath in rust_files:
            self.fix_file(filepath)
        
        print(f"\n✅ Syntax fix complete!")
        print(f"   Files fixed: {self.files_fixed}")
        print(f"   Errors fixed: {self.errors_fixed}")

def main():
    fixer = SyntaxErrorFixer()
    fixer.fix_all_files()

if __name__ == "__main__":
    main() 