#!/usr/bin/env python3
"""
Fix all BearDogError variant usages to use unified constructors
"""

import re
import subprocess
from pathlib import Path

def fix_error_variants():
    """Fix all error variant usages in the codebase"""
    
    # Find all files with BearDogError:: patterns
    result = subprocess.run([
        'grep', '-r', '--include=*.rs', 'BearDogError::', 'crates/'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        print("No BearDogError:: patterns found")
        return
    
    # Parse grep output to find files and patterns
    files_to_fix = set()
    for line in result.stdout.split('\n'):
        if ':' in line:
            file_path = line.split(':')[0]
            files_to_fix.add(file_path)
    
    print(f"Found {len(files_to_fix)} files to fix")
    
    # Fix each file
    for file_path in files_to_fix:
        fix_file(Path(file_path))

def fix_file(file_path: Path):
    """Fix error variant usages in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return
    
    original_content = content
    
    # Configuration errors
    content = re.sub(
        r'BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}',
        r'BearDogError::configuration(\1)',
        content
    )
    
    # NoSuitableProvider errors
    content = re.sub(
        r'BearDogError::NoSuitableProvider\s*\{\s*message:\s*([^}]+)\s*\}',
        r'BearDogError::no_suitable_provider(\1)',
        content
    )
    
    # UnsupportedOperation errors  
    content = re.sub(
        r'BearDogError::UnsupportedOperation\s*\{\s*operation:\s*([^}]+)\s*\}',
        r'BearDogError::unsupported_operation(\1)',
        content
    )
    
    # Save if changes were made
    if content != original_content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed {file_path}")
        except Exception as e:
            print(f"Error writing {file_path}: {e}")

if __name__ == "__main__":
    fix_error_variants() 