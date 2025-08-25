#!/usr/bin/env python3
"""
Quick fix for Configuration error usages
"""

import re
from pathlib import Path

def fix_configuration_errors(file_path: Path):
    """Fix Configuration error usages in a file"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Replace Configuration variant usage with constructor
    patterns = [
        (r'beardog_errors::BearDogError::Configuration\s*\{\s*message:\s*([^}]+)\s*\}', 
         r'beardog_errors::BearDogError::configuration(\1)'),
    ]
    
    for old_pattern, new_pattern in patterns:
        content = re.sub(old_pattern, new_pattern, content)
    
    with open(file_path, 'w') as f:
        f.write(content)

if __name__ == "__main__":
    runtime_file = Path("crates/beardog-config/src/runtime.rs")
    fix_configuration_errors(runtime_file)
    print(f"Fixed Configuration errors in {runtime_file}") 