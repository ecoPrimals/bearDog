#!/usr/bin/env python3
"""
Final cleanup script to fix all remaining syntax errors
"""

import re
import subprocess
from pathlib import Path

def fix_syntax_errors():
    """Fix all remaining syntax errors in the codebase"""
    
    fixes = [
        # Fix NotFound variant usages
        (r'BearDogError::NotFound\s*\{\s*message:\s*([^}]+)\s*\}', r'BearDogError::not_found(\1)'),
        (r'BearDogError::NotFound\s*\{\s*resource:\s*([^}]+)\s*\}', r'BearDogError::not_found(\1)'),
        
        # Fix format string delimiters
        (r'format!\([^)]*\{[^}]*\)[^)]*\)', fix_format_string),
        (r'format!\([^)]*\{[^}]*\}[^)]*\}', fix_malformed_format),
        
        # Fix doc comments 
        (r'^//! (.*)', r'/// \1'),
        
        # Fix workflow storage errors
        (r'BearDogError::configuration\(format!\(\s*"([^"]*)",\s*([^)]+)\s*\}\)\)', r'BearDogError::configuration(format!("\1", \2))'),
    ]
    
    # Find all Rust files
    result = subprocess.run([
        'find', 'crates/', '-name', '*.rs', '-type', 'f'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        print("Error finding Rust files")
        return
    
    files = result.stdout.strip().split('\n')
    
    for file_path in files:
        if not file_path:
            continue
        fix_file(Path(file_path), fixes)

def fix_format_string(match):
    """Fix malformed format strings"""
    content = match.group(0)
    # Simple fix for common issues
    content = content.replace('{e)', '{e}')
    content = content.replace('{)', '}')
    return content

def fix_malformed_format(match):
    """Fix malformed format strings with extra braces"""
    content = match.group(0) 
    # Remove extra closing braces
    if content.count('}') > content.count('{'):
        content = content.rstrip('}') + ')'
    return content

def fix_file(file_path: Path, fixes):
    """Fix syntax errors in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return
    
    original_content = content
    
    # Apply all fixes
    for pattern, replacement in fixes:
        if callable(replacement):
            content = re.sub(pattern, replacement, content)
        else:
            content = re.sub(pattern, replacement, content)
    
    # Manual fixes for known problematic patterns
    content = fix_specific_issues(content, file_path)
    
    # Save if changes were made
    if content != original_content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed {file_path}")
        except Exception as e:
            print(f"Error writing {file_path}: {e}")

def fix_specific_issues(content: str, file_path: Path) -> str:
    """Fix specific known syntax issues"""
    
    # Fix zero_cost_storage workflow issues
    if 'zero_cost_storage.rs' in str(file_path):
        content = re.sub(
            r'BearDogError::configuration\(format!\(\s*"([^"]*)",\s*([^)]+)\s*\}\)\)',
            r'BearDogError::configuration(format!("\1", \2))',
            content
        )
        content = re.sub(
            r'\}\)\);\s*\}\);',
            r')))',
            content
        )
    
    # Fix iOS secure enclave issues  
    if 'safe_secure_enclave_replacement.rs' in str(file_path):
        content = re.sub(
            r'format!\("Algorithm \{:\?\) not supported"',
            r'format!("Algorithm {:?} not supported"',
            content
        )
    
    # Fix doc comments
    if content.startswith('//!') and 'pub use' in content:
        lines = content.split('\n')
        fixed_lines = []
        for line in lines:
            if line.startswith('//!') and not any(lines[i+1:i+3] for i in range(len(lines)) if lines[i] == line and i < len(lines)-2 and any(l.strip().startswith(('pub ', 'use ', 'mod ')) for l in lines[i+1:i+3])):
                fixed_lines.append(line.replace('//!', '//'))
            else:
                fixed_lines.append(line)
        content = '\n'.join(fixed_lines)
    
    return content

if __name__ == "__main__":
    fix_syntax_errors() 