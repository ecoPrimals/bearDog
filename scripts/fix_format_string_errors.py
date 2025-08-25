#!/usr/bin/env python3
"""
Fix Malformed Format String Errors
Targets specific syntax errors from previous migration scripts
"""

import re
import subprocess
from pathlib import Path

def fix_format_string_errors():
    """Fix all remaining format string syntax errors"""
    
    # Find files with compilation errors
    error_patterns = [
        # Fix malformed format strings with missing closing braces
        (r'format!\("([^"]*)\{([^}]*)\)([^"]*)"', r'format!("\1{\2}\3"'),
        
        # Fix malformed BearDogError::not_found calls  
        (r'BearDogError::not_found\(format!\("([^"]*)\{([^}]*)\)([^"]*)"', r'BearDogError::not_found(format!("\1{\2}\3"))'),
        
        # Fix specific incident handler errors
        (r'format!\("Incident not found: \{incident_id\)"', r'format!("Incident not found: {}", incident_id)'),
        
        # Fix workflow errors
        (r'format!\("Workflow \{workflow_id\) not found"', r'format!("Workflow {} not found", workflow_id)'),
        
        # Fix key not found errors
        (r'format!\("Key \'\}\' not found", key_id\)', r'format!("Key {} not found", key_id)'),
        
        # Fix algorithm errors
        (r'format!\("Algorithm \{:\?\) not supported"', r'format!("Algorithm {:?} not supported"')
    ]
    
    files_fixed = 0
    
    # Find all Rust files
    result = subprocess.run([
        'find', 'crates/', '-name', '*.rs', '-type', 'f'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        print("Error finding Rust files")
        return
    
    rust_files = [f for f in result.stdout.strip().split('\n') if f]
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
            continue
        
        original_content = content
        
        # Apply all format string fixes
        for pattern, replacement in error_patterns:
            content = re.sub(pattern, replacement, content)
        
        # Additional specific fixes based on error messages
        specific_fixes = [
            # Fix incident handler
            ('Err(BearDogError::not_found(format!("Incident not found: {incident_id)"),\n            })', 
             'Err(BearDogError::not_found(format!("Incident not found: {}", incident_id)))'),
            
            # Fix workflow handler
            ('ok_or_else(|| BearDogError::not_found(format!("Workflow {workflow_id) not found"),\n            })',
             'ok_or_else(|| BearDogError::not_found(format!("Workflow {} not found", workflow_id)))'),
            
            # Fix Android provider
            ('ok_or_else(|| BearDogError::not_found(format!("Key \'}\' not found", key_id),\n                })',
             'ok_or_else(|| BearDogError::not_found(format!("Key {} not found", key_id)))'),
        ]
        
        for old_text, new_text in specific_fixes:
            content = content.replace(old_text, new_text)
        
        # Save changes if any were made
        if content != original_content:
            try:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                files_fixed += 1
                print(f"Fixed format strings in {file_path}")
            except Exception as e:
                print(f"Error writing {file_path}: {e}")
    
    print(f"✅ Fixed format string errors in {files_fixed} files")

if __name__ == "__main__":
    fix_format_string_errors() 