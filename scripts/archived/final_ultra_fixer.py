#!/usr/bin/env python3
"""
🛠️ FINAL ULTRA FIXER 🛠️
Fixes all remaining compilation issues for ultra-pedantic perfection
"""

import os
import re
from pathlib import Path

class FinalUltraFixer:
    def __init__(self, crates_dir="crates"):
        self.crates_dir = Path(crates_dir)
        self.fixes_applied = 0
        
    def remove_const_from_trait_impls(self):
        """Remove const from trait implementation functions"""
        print("🔧 Removing const from trait implementation functions...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                # Find trait implementations and remove const from their functions
                lines = content.split('\n')
                in_trait_impl = False
                modified = False
                
                for i, line in enumerate(lines):
                    # Detect trait implementation blocks
                    if re.search(r'impl\s+\w+\s+for\s+\w+', line) or re.search(r'impl.*Default.*for', line):
                        in_trait_impl = True
                    elif in_trait_impl and line.strip() == '}':
                        in_trait_impl = False
                    
                    # Remove const from functions in trait implementations
                    if in_trait_impl and 'const fn' in line:
                        new_line = re.sub(r'const\s+fn', 'fn', line)
                        if new_line != line:
                            lines[i] = new_line
                            modified = True
                            self.fixes_applied += 1
                            print(f"  🔧 Removed const from trait impl in {rust_file.name}:{i+1}")
                
                if modified:
                    with open(rust_file, 'w') as f:
                        f.write('\n'.join(lines))
                        
            except Exception as e:
                print(f"Error processing {rust_file}: {e}")
    
    def remove_invalid_copy_derives(self):
        """Remove Copy derives from structs that cannot implement Copy"""
        print("🔧 Removing invalid Copy derives...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                # Find structs with Copy derives that have non-Copy fields
                struct_pattern = r'(#\[derive\([^)]*Copy[^)]*\)\][^{]*pub struct \w+[^{]*\{[^}]*\})'
                
                modified = False
                for match in re.finditer(struct_pattern, content, re.MULTILINE | re.DOTALL):
                    struct_def = match.group(1)
                    
                    # Check if struct contains non-Copy types
                    non_copy_indicators = [
                        'String', 'Vec<', 'HashMap<', 'BTreeMap<', 'HashSet<', 'BTreeSet<',
                        'Box<', 'Arc<', 'Rc<', 'Mutex<', 'RwLock<', 'RefCell<',
                        'Config>', 'Settings>', 'Manager>', 'Engine>', 'Handler>',
                        'Context>', 'Builder>', 'Factory>', 'Registry>', 'Service>',
                        'Option<String>', 'Option<Vec', 'Option<HashMap'
                    ]
                    
                    if any(indicator in struct_def for indicator in non_copy_indicators):
                        # Remove Copy from the derive
                        new_struct = re.sub(r'Copy,\s*', '', struct_def)
                        new_struct = re.sub(r',\s*Copy', '', new_struct)
                        new_struct = re.sub(r'Copy', '', new_struct)
                        
                        # Clean up empty derives or trailing commas
                        new_struct = re.sub(r'#\[derive\(\s*,\s*', '#[derive(', new_struct)
                        new_struct = re.sub(r',\s*\)\]', ')]', new_struct)
                        new_struct = re.sub(r'\(\s*\)', '()', new_struct)
                        
                        if new_struct != struct_def:
                            content = content.replace(struct_def, new_struct)
                            modified = True
                            self.fixes_applied += 1
                            print(f"  🔧 Removed Copy derive from struct in {rust_file.name}")
                
                if modified:
                    with open(rust_file, 'w') as f:
                        f.write(content)
                        
            except Exception as e:
                print(f"Error processing Copy derives in {rust_file}: {e}")
    
    def remove_remaining_invalid_const(self):
        """Remove const from functions that still cannot be const"""
        print("🔧 Removing remaining invalid const functions...")
        
        invalid_const_patterns = [
            r'\.contains\(',
            r'\.get\(',
            r'matches!\(',
            r'match\s+\w+\s*\{',
            r'RangeInclusive.*contains',
            r'Self::new\(',
            r'Vec::<.*>::',
            r'HashMap::<.*>::',
            r'\.deref\(',
            r'==.*ComplianceStatus',
            r'schema_version\(\)',
        ]
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                lines = content.split('\n')
                modified = False
                
                for i, line in enumerate(lines):
                    if 'const fn' in line:
                        # Look ahead to see if this function has invalid operations
                        function_content = []
                        brace_count = 0
                        j = i
                        
                        while j < len(lines):
                            current_line = lines[j]
                            function_content.append(current_line)
                            
                            brace_count += current_line.count('{')
                            brace_count -= current_line.count('}')
                            
                            if brace_count == 0 and '{' in current_line:
                                break
                            j += 1
                        
                        function_text = '\n'.join(function_content)
                        
                        # Check if function has invalid const operations
                        has_invalid_ops = any(
                            re.search(pattern, function_text) 
                            for pattern in invalid_const_patterns
                        )
                        
                        if has_invalid_ops:
                            new_line = re.sub(r'const\s+fn', 'fn', line)
                            if new_line != line:
                                lines[i] = new_line
                                modified = True
                                self.fixes_applied += 1
                                print(f"  🔧 Removed const from invalid function in {rust_file.name}:{i+1}")
                
                if modified:
                    with open(rust_file, 'w') as f:
                        f.write('\n'.join(lines))
                        
            except Exception as e:
                print(f"Error removing invalid const in {rust_file}: {e}")
    
    def remove_duplicate_inline_attributes(self):
        """Remove duplicate inline attributes"""
        print("🔧 Removing duplicate inline attributes...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                # Remove duplicate #[inline] attributes
                lines = content.split('\n')
                modified = False
                
                for i in range(len(lines) - 1):
                    if '#[inline]' in lines[i] and '#[inline]' in lines[i + 1]:
                        lines[i + 1] = lines[i + 1].replace('#[inline]', '').strip()
                        if not lines[i + 1]:
                            lines[i + 1] = ''
                        modified = True
                        self.fixes_applied += 1
                
                if modified:
                    with open(rust_file, 'w') as f:
                        f.write('\n'.join(lines))
                        
            except Exception as e:
                print(f"Error removing duplicate inlines in {rust_file}: {e}")
    
    def remove_inline_from_trait_prototypes(self):
        """Remove inline from trait function prototypes"""
        print("🔧 Removing inline from trait function prototypes...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                
                # Remove #[inline] from trait function prototypes
                lines = content.split('\n')
                modified = False
                in_trait = False
                
                for i, line in enumerate(lines):
                    if re.search(r'trait\s+\w+', line):
                        in_trait = True
                    elif in_trait and line.strip() == '}':
                        in_trait = False
                    
                    if in_trait and '#[inline]' in line and 'fn ' in lines[i + 1] if i + 1 < len(lines) else False:
                        # This is an inline attribute before a trait function prototype
                        lines[i] = re.sub(r'\s*#\[inline\]\s*', '', line)
                        if not lines[i].strip():
                            lines[i] = ''
                        modified = True
                        self.fixes_applied += 1
                
                if modified:
                    with open(rust_file, 'w') as f:
                        f.write('\n'.join(lines))
                        
            except Exception as e:
                print(f"Error removing trait inline attributes in {rust_file}: {e}")
    
    def run_final_fixes(self):
        """Run all final fixes"""
        print("🎯 FINAL ULTRA FIXER ACTIVATED")
        print("=" * 60)
        
        self.remove_const_from_trait_impls()
        self.remove_invalid_copy_derives()
        self.remove_remaining_invalid_const()
        self.remove_duplicate_inline_attributes()
        self.remove_inline_from_trait_prototypes()
        
        print(f"\n✅ FINAL ULTRA FIXES APPLIED: {self.fixes_applied}")
        return self.fixes_applied > 0

if __name__ == "__main__":
    fixer = FinalUltraFixer()
    fixer.run_final_fixes() 