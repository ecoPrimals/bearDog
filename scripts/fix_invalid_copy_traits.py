#!/usr/bin/env python3
"""
🔧 INVALID COPY TRAIT FIXER
Removes Copy traits from types that have non-Copy fields.
"""

import os
import re
import sys
from pathlib import Path

class InvalidCopyFixer:
    def __init__(self):
        self.fixes_applied = 0
        self.files_updated = 0
        
        # Non-Copy types that prevent Copy implementation
        self.non_copy_types = [
            'String', 'Vec', 'HashMap', 'Arc', 'Rc', 'Box', 'PathBuf',
            'Option<String>', 'Option<Arc', 'serde_json::Value',
            'BiometricHash', 'HumanEntropySource', 'UniversalHsmManager'
        ]
    
    def fix_invalid_copy_traits(self, content, file_path):
        """Remove Copy traits from types with non-Copy fields"""
        lines = content.split('\n')
        result_lines = []
        updated = False
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Check if this is a derive macro with Copy
            if line.strip().startswith('#[derive(') and 'Copy' in line:
                # Look ahead to see if this type has non-Copy fields
                if self._has_non_copy_fields(lines, i):
                    # Remove Copy from the derive macro
                    new_line = self._remove_copy_from_derive(line)
                    if new_line != line:
                        result_lines.append(new_line)
                        updated = True
                        self.fixes_applied += 1
                        print(f"  ✅ Removed invalid Copy trait from line {i+1}")
                    else:
                        result_lines.append(line)
                else:
                    result_lines.append(line)
            else:
                result_lines.append(line)
            
            i += 1
        
        if updated:
            self.files_updated += 1
        
        return '\n'.join(result_lines)
    
    def _has_non_copy_fields(self, lines, derive_line_index):
        """Check if the type following this derive has non-Copy fields"""
        # Find the struct/enum definition
        i = derive_line_index + 1
        while i < len(lines) and not (lines[i].strip().startswith('pub struct ') or lines[i].strip().startswith('pub enum ')):
            i += 1
        
        if i >= len(lines):
            return False
        
        # Check if it's an enum with String variants
        if 'pub enum ' in lines[i]:
            # Look for enum variants with non-Copy types
            brace_count = 0
            j = i
            while j < len(lines):
                line = lines[j].strip()
                if '{' in line:
                    brace_count += line.count('{')
                if '}' in line:
                    brace_count -= line.count('}')
                    if brace_count == 0:
                        break
                
                # Check for variants with non-Copy types
                if brace_count > 0 and '(' in line and ')' in line:
                    variant_content = line[line.find('('):line.find(')')+1]
                    if any(non_copy_type in variant_content for non_copy_type in self.non_copy_types):
                        return True
                
                j += 1
        
        # Check if it's a struct with non-Copy fields
        elif 'pub struct ' in lines[i]:
            brace_count = 0
            j = i
            while j < len(lines):
                line = lines[j].strip()
                if '{' in line:
                    brace_count += line.count('{')
                if '}' in line:
                    brace_count -= line.count('}')
                    if brace_count == 0:
                        break
                
                # Check for fields with non-Copy types
                if brace_count > 0 and ':' in line and 'pub ' in line:
                    field_type = line.split(':')[1].strip().rstrip(',')
                    if any(non_copy_type in field_type for non_copy_type in self.non_copy_types):
                        return True
                
                j += 1
        
        return False
    
    def _remove_copy_from_derive(self, line):
        """Remove Copy from a derive macro"""
        # Handle various formats of Copy in derive
        patterns = [
            (r', Copy', ''),
            (r'Copy, ', ''),
            (r'\(Copy\)', '()'),
            (r'\(([^)]*), Copy\)', r'(\1)'),
            (r'\(Copy, ([^)]*)\)', r'(\1)')
        ]
        
        result = line
        for pattern, replacement in patterns:
            result = re.sub(pattern, replacement, result)
        
        # Clean up any double commas or empty parentheses
        result = re.sub(r',\s*,', ',', result)
        result = re.sub(r'\(\s*\)', '()', result)
        result = re.sub(r'#\[derive\(\s*\)\]', '', result)
        
        return result
    
    def process_file(self, file_path):
        """Process a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = self.fix_invalid_copy_traits(content, file_path)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True
            return False
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def process_directory(self, directory):
        """Process all Rust files in directory"""
        print(f"🔧 Fixing invalid Copy traits in {directory}...")
        
        rust_files = list(Path(directory).rglob("*.rs"))
        print(f"📊 Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            # Skip test files
            if 'test' in str(file_path).lower():
                continue
            
            self.process_file(file_path)
        
        print(f"\n🔧 INVALID COPY TRAIT FIXES COMPLETE:")
        print(f"   📊 Fixes applied: {self.fixes_applied}")
        print(f"   📁 Files updated: {self.files_updated}")

def main():
    print("🔧 INVALID COPY TRAIT FIXER")
    print("🎯 Mission: Remove Copy traits from types with non-Copy fields")
    print("📋 Principle: Only types with all Copy fields can implement Copy")
    
    fixer = InvalidCopyFixer()
    fixer.process_directory("crates/beardog-core/src/")
    
    if fixer.fixes_applied > 0:
        print(f"\n✅ SUCCESS: Fixed {fixer.fixes_applied} invalid Copy traits!")
    else:
        print("\n📊 No invalid Copy traits found")

if __name__ == "__main__":
    main() 