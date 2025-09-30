#!/usr/bin/env python3
"""
🎯 PEDANTIC COPY OPTIMIZER
Systematically adds Copy traits to all eligible types for maximum performance.
"""

import os
import re
import sys
from pathlib import Path

class CopyOptimizer:
    def __init__(self):
        self.patterns_fixed = 0
        self.files_updated = 0
        
        # Types that should get Copy trait (simple enums and structs with only Copy fields)
        self.copy_patterns = [
            # Enums with only unit variants
            (r'#\[derive\(([^)]*)\)\]\s*pub enum (\w+) \{\s*(?:\s*///[^\n]*\n\s*)*(\w+),\s*(?:\s*///[^\n]*\n\s*)*(\w+),', 'simple_enum'),
            
            # Structs with only primitive fields
            (r'#\[derive\(([^)]*)\)\]\s*pub struct (\w+) \{\s*(?:\s*///[^\n]*\n\s*)*pub \w+: (?:u8|u16|u32|u64|i8|i16|i32|i64|f32|f64|bool|usize|isize),', 'primitive_struct'),
        ]
    
    def add_copy_trait(self, content, file_path):
        """Add Copy trait to eligible types"""
        updated = False
        lines = content.split('\n')
        result_lines = []
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Look for derive macros that don't already have Copy
            if '#[derive(' in line and 'Copy' not in line:
                # Check if this is an enum with only unit variants
                if 'pub enum' in lines[i + 1] if i + 1 < len(lines) else False:
                    # Look ahead to see if it's a simple enum
                    j = i + 2
                    is_simple_enum = True
                    variant_count = 0
                    
                    while j < len(lines) and not lines[j].strip().startswith('}'):
                        line_content = lines[j].strip()
                        if line_content and not line_content.startswith('///') and not line_content.startswith('//'):
                            if '(' in line_content or '{' in line_content:
                                is_simple_enum = False
                                break
                            if line_content.endswith(',') or line_content == '}':
                                variant_count += 1
                        j += 1
                    
                    if is_simple_enum and variant_count > 0:
                        # Add Copy to the derive macro
                        derive_content = line[line.find('(')+1:line.find(')')]
                        if 'Clone' in derive_content:
                            new_derive = line.replace('Clone', 'Clone, Copy')
                            result_lines.append(new_derive)
                            updated = True
                            self.patterns_fixed += 1
                        else:
                            result_lines.append(line)
                    else:
                        result_lines.append(line)
                
                # Check if this is a struct with only primitive fields
                elif 'pub struct' in lines[i + 1] if i + 1 < len(lines) else False:
                    # Look ahead to see if it has only primitive fields
                    j = i + 2
                    is_primitive_struct = True
                    field_count = 0
                    
                    while j < len(lines) and not lines[j].strip().startswith('}'):
                        line_content = lines[j].strip()
                        if line_content.startswith('pub ') and ':' in line_content:
                            field_type = line_content.split(':')[1].strip().rstrip(',')
                            # Check if it's a primitive type
                            primitives = ['u8', 'u16', 'u32', 'u64', 'i8', 'i16', 'i32', 'i64', 
                                        'f32', 'f64', 'bool', 'usize', 'isize']
                            if not any(prim in field_type for prim in primitives):
                                is_primitive_struct = False
                                break
                            field_count += 1
                        j += 1
                    
                    if is_primitive_struct and field_count > 0:
                        # Add Copy to the derive macro
                        derive_content = line[line.find('(')+1:line.find(')')]
                        if 'Clone' in derive_content and 'Copy' not in derive_content:
                            new_derive = line.replace('Clone', 'Clone, Copy')
                            result_lines.append(new_derive)
                            updated = True
                            self.patterns_fixed += 1
                        else:
                            result_lines.append(line)
                    else:
                        result_lines.append(line)
                else:
                    result_lines.append(line)
            else:
                result_lines.append(line)
            
            i += 1
        
        if updated:
            self.files_updated += 1
            print(f"  ✅ Added Copy traits: {file_path}")
        
        return '\n'.join(result_lines)
    
    def optimize_file(self, file_path):
        """Optimize a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = self.add_copy_trait(content, file_path)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True
            return False
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def optimize_directory(self, directory):
        """Optimize all Rust files in directory"""
        print(f"🔍 Scanning {directory} for Copy optimization opportunities...")
        
        rust_files = list(Path(directory).rglob("*.rs"))
        print(f"📊 Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            # Skip test files and examples for now
            if 'test' in str(file_path).lower() or 'example' in str(file_path).lower():
                continue
            
            self.optimize_file(file_path)
        
        print(f"\n🎯 COPY OPTIMIZATION COMPLETE:")
        print(f"   📊 Patterns fixed: {self.patterns_fixed}")
        print(f"   📁 Files updated: {self.files_updated}")

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 pedantic_copy_optimizer.py <directory>")
        sys.exit(1)
    
    directory = sys.argv[1]
    if not os.path.exists(directory):
        print(f"❌ Directory not found: {directory}")
        sys.exit(1)
    
    print("🚀 PEDANTIC COPY OPTIMIZER")
    print("🎯 Mission: Maximum performance through Copy trait optimization")
    print("📋 Principle: Zero-cost abstractions with optimal copying")
    
    optimizer = CopyOptimizer()
    optimizer.optimize_directory(directory)
    
    if optimizer.patterns_fixed > 0:
        print(f"\n✅ SUCCESS: Optimized {optimizer.patterns_fixed} types for better performance!")
    else:
        print("\n📊 No additional Copy optimizations found")

if __name__ == "__main__":
    main() 