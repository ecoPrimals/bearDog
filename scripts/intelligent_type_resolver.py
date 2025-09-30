#!/usr/bin/env python3

import os
import re
import sys
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple

def get_compilation_errors(beardog_root: Path) -> str:
    """Get detailed compilation errors from cargo check"""
    cmd = ["cargo", "check", "-p", "beardog-types", "--no-default-features"]
    try:
        result = subprocess.run(cmd, cwd=beardog_root, capture_output=True, text=True)
        return result.stderr
    except Exception as e:
        print(f"Error running cargo check: {e}")
        return ""

def parse_detailed_errors(error_output: str) -> List[Dict]:
    """Parse detailed type errors with file context"""
    errors = []
    lines = error_output.split('\n')
    
    current_error = {}
    for i, line in enumerate(lines):
        # Match file location
        if " --> " in line and ".rs:" in line:
            match = re.search(r'([^/]+\.rs):(\d+):(\d+)', line)
            if match:
                current_error = {
                    'file': match.group(1),
                    'line': int(match.group(2)),
                    'column': int(match.group(3))
                }
        
        # Match expected vs found types
        elif "expected" in line and "found" in line:
            match = re.search(r'expected `([^`]+)`, found `([^`]+)`', line)
            if match and current_error:
                current_error['expected'] = match.group(1)
                current_error['found'] = match.group(2)
                
                # Look for the actual code line in subsequent lines
                for j in range(i+1, min(i+10, len(lines))):
                    if lines[j].strip() and not lines[j].startswith(' '):
                        break
                    if 'crate::constants::ultimate::system::' in lines[j]:
                        current_error['code_line'] = lines[j].strip()
                        break
                
                errors.append(current_error.copy())
    
    return errors

def analyze_field_types(beardog_root: Path) -> Dict[str, str]:
    """Analyze struct field types to understand expected types"""
    field_types = {}
    
    types_dir = beardog_root / "crates" / "beardog-types" / "src"
    
    for rust_file in types_dir.rglob("*.rs"):
        try:
            with open(rust_file, 'r') as f:
                content = f.read()
            
            # Find struct field definitions
            struct_matches = re.finditer(r'pub struct\s+(\w+)\s*{([^}]+)}', content, re.MULTILINE | re.DOTALL)
            
            for struct_match in struct_matches:
                struct_name = struct_match.group(1)
                fields_content = struct_match.group(2)
                
                # Parse individual fields
                field_matches = re.finditer(r'pub\s+(\w+):\s*([^,\n]+)[,\n]', fields_content)
                
                for field_match in field_matches:
                    field_name = field_match.group(1)
                    field_type = field_match.group(2).strip()
                    
                    # Clean up type (remove Option, etc.)
                    clean_type = re.sub(r'Option<([^>]+)>', r'\1', field_type)
                    clean_type = re.sub(r'Duration', 'Duration', clean_type)
                    
                    field_types[field_name] = clean_type
                    
        except Exception as e:
            continue
    
    return field_types

def get_appropriate_cast(expected_type: str, found_type: str, field_name: str = None) -> str:
    """Determine the appropriate type cast"""
    
    # Duration fields need special handling
    if expected_type == 'Duration' or 'Duration::from_' in expected_type:
        if found_type == 'usize':
            return 'as u64'
        return ''
    
    # Numeric type conversions
    type_map = {
        ('usize', 'u32'): 'as u32',
        ('usize', 'u64'): 'as u64', 
        ('usize', 'f64'): 'as f64',
        ('usize', 'f32'): 'as f32',
        ('u32', 'usize'): '',  # Remove cast
        ('u64', 'f64'): 'as f64',
        ('u64', 'u32'): 'as u32',
    }
    
    # Field-specific overrides
    if field_name:
        # Fields that should remain usize
        if field_name in ['max_services', 'max_entries', 'max_genetics_stored', 'memory_buffer_size', 
                         'max_data_points', 'min_data_points', 'max_histogram_buckets']:
            if found_type == 'u32':
                return ''  # Remove cast
        
        # Fields that need u32
        if field_name in ['max_generations', 'max_concurrent_operations', 'batch_size', 
                         'operations_per_second', 'retry_delay_ms', 'pool_size']:
            if found_type == 'usize':
                return 'as u32'
    
    return type_map.get((found_type, expected_type), '')

def apply_intelligent_fixes(beardog_root: Path):
    """Apply intelligent type casting fixes based on error analysis"""
    
    print(f"🏠 BearDog root: {beardog_root}")
    print("🧠 Applying intelligent type casting fixes...")
    
    # Get errors and analyze field types
    error_output = get_compilation_errors(beardog_root)
    errors = parse_detailed_errors(error_output)
    field_types = analyze_field_types(beardog_root)
    
    print(f"📊 Analyzing {len(errors)} type errors...")
    print(f"📋 Found {len(field_types)} field type definitions")
    
    # Group errors by file
    files_to_fix = {}
    for error in errors:
        file_path = None
        for rust_file in (beardog_root / "crates" / "beardog-types" / "src").rglob("*.rs"):
            if rust_file.name == error['file']:
                file_path = rust_file
                break
        
        if file_path:
            if str(file_path) not in files_to_fix:
                files_to_fix[str(file_path)] = []
            files_to_fix[str(file_path)].append(error)
    
    total_fixes = 0
    
    # Apply fixes file by file
    for file_path, file_errors in files_to_fix.items():
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            original_content = content
            file_fixes = 0
            
            for error in file_errors:
                expected = error['expected']
                found = error['found']
                
                # Extract field name from context if possible
                field_name = None
                if 'code_line' in error:
                    field_match = re.search(r'(\w+):\s*crate::constants', error['code_line'])
                    if field_match:
                        field_name = field_match.group(1)
                
                # Get appropriate cast
                cast = get_appropriate_cast(expected, found, field_name)
                
                if cast:
                    # Apply the cast
                    if cast == '':  # Remove existing cast
                        pattern = r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)\s+as\s+\w+'
                        replacement = r'\1'
                    else:
                        pattern = r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)(?!\s+as\s)'
                        replacement = f'\\1 {cast}'
                    
                    new_content = re.sub(pattern, replacement, content, count=1)
                    if new_content != content:
                        content = new_content
                        file_fixes += 1
            
            if content != original_content:
                with open(file_path, 'w') as f:
                    f.write(content)
                
                rel_path = Path(file_path).relative_to(beardog_root)
                print(f"   ✅ Applied {file_fixes} intelligent fixes to {rel_path}")
                total_fixes += file_fixes
                
        except Exception as e:
            print(f"   ❌ Error processing {file_path}: {e}")
    
    print(f"\n🎉 Total intelligent fixes applied: {total_fixes}")

def main():
    current_dir = Path.cwd()
    beardog_root = None
    
    for path in [current_dir] + list(current_dir.parents):
        cargo_toml = path / "Cargo.toml"
        if cargo_toml.exists():
            try:
                with open(cargo_toml, 'r') as f:
                    if 'beardog' in f.read():
                        beardog_root = path
                        break
            except:
                continue
    
    if not beardog_root:
        print("❌ Could not find BearDog workspace root")
        sys.exit(1)
    
    apply_intelligent_fixes(beardog_root)

if __name__ == "__main__":
    main() 