#!/usr/bin/env python3

import os
import re
import sys
import subprocess
from pathlib import Path

def get_compilation_errors(beardog_root: Path):
    """Get compilation errors from cargo check"""
    
    cmd = ["cargo", "check", "-p", "beardog-types", "--no-default-features"]
    try:
        result = subprocess.run(cmd, cwd=beardog_root, capture_output=True, text=True)
        return result.stderr
    except Exception as e:
        print(f"Error running cargo check: {e}")
        return ""

def parse_type_errors(error_output):
    """Parse type errors from compilation output"""
    
    errors = []
    lines = error_output.split('\n')
    
    current_error = {}
    for line in lines:
        # Match file and line number
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
                errors.append(current_error.copy())
    
    return errors

def fix_type_errors_systematically(beardog_root: Path):
    """Fix type errors by analyzing compilation output and applying targeted fixes"""
    
    print(f"🏠 BearDog root: {beardog_root}")
    print("🔧 Surgically fixing type casting issues...")
    
    # Get compilation errors
    error_output = get_compilation_errors(beardog_root)
    errors = parse_type_errors(error_output)
    
    print(f"📊 Found {len(errors)} type errors to fix")
    
    # Group errors by file for batch processing
    files_to_fix = {}
    for error in errors:
        file_path = None
        # Find the full path to the file
        for rust_file in (beardog_root / "crates" / "beardog-types" / "src").rglob("*.rs"):
            if rust_file.name == error['file']:
                file_path = rust_file
                break
        
        if file_path:
            if str(file_path) not in files_to_fix:
                files_to_fix[str(file_path)] = []
            files_to_fix[str(file_path)].append(error)
    
    total_fixes = 0
    
    # Apply systematic fixes
    for file_path, file_errors in files_to_fix.items():
        try:
            with open(file_path, 'r') as f:
                lines = f.readlines()
            
            file_fixes = 0
            
            # Sort errors by line number (reverse order to avoid line number shifts)
            file_errors.sort(key=lambda x: x['line'], reverse=True)
            
            for error in file_errors:
                line_idx = error['line'] - 1
                if line_idx < len(lines):
                    line = lines[line_idx]
                    
                    # Apply specific type casting based on expected vs found types
                    new_line = apply_type_cast_fix(line, error['expected'], error['found'])
                    
                    if new_line != line:
                        lines[line_idx] = new_line
                        file_fixes += 1
            
            if file_fixes > 0:
                with open(file_path, 'w') as f:
                    f.writelines(lines)
                
                rel_path = Path(file_path).relative_to(beardog_root)
                print(f"   ✅ Fixed {file_fixes} type issues in {rel_path}")
                total_fixes += file_fixes
                
        except Exception as e:
            print(f"   ❌ Error fixing {file_path}: {e}")
    
    print(f"\n🎉 Total type fixes applied: {total_fixes}")

def apply_type_cast_fix(line, expected_type, found_type):
    """Apply appropriate type casting fix"""
    
    # Common patterns and their fixes
    fixes = [
        # usize to u32 conversion
        (r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)(?=\s*,)', expected_type == 'u32' and found_type == 'usize', r'\1 as u32'),
        
        # usize to u64 conversion  
        (r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)(?=\s*,)', expected_type == 'u64' and found_type == 'usize', r'\1 as u64'),
        
        # usize to f64 conversion
        (r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)(?=\s*,)', expected_type == 'f64' and found_type == 'usize', r'\1 as f64'),
        
        # u32 to usize conversion
        (r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)\s+as\s+u32(?=\s*,)', expected_type == 'usize' and found_type == 'u32', r'\1'),
        
        # u64 to f64 conversion
        (r'(crate::constants::ultimate::system::DEFAULT_[A-Z_]+)\s+as\s+u64(?=\s*,)', expected_type == 'f64' and found_type == 'u64', r'\1 as f64'),
        
        # Duration::from_secs fixes
        (r'Duration::from_secs\((crate::constants::ultimate::system::DEFAULT_[A-Z_]+)\)', expected_type == 'u64' and found_type == 'usize', r'Duration::from_secs(\1 as u64)'),
    ]
    
    for pattern, condition, replacement in fixes:
        if condition and re.search(pattern, line):
            return re.sub(pattern, replacement, line)
    
    return line

def main():
    # Find BearDog root
    current_dir = Path.cwd()
    beardog_root = None
    
    # Look for Cargo.toml with beardog workspace
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
    
    fix_type_errors_systematically(beardog_root)

if __name__ == "__main__":
    main() 