#!/usr/bin/env python3
"""
Quick script to fix common syntax errors in test files.
These were introduced by a previous automated tool.
"""

import re
import sys
from pathlib import Path

def fix_malformed_to_string(content):
    """Fix .to_string(ARG, patterns"""
    
    # Pattern 1: .to_string(None, -> .to_string(), with target_ecosystem: None,
    content = re.sub(
        r'(\s+source_ecosystem:\s+\w+[\w:]*\.to_string)\(None,',
        r'\1(),\n            target_ecosystem: None,',
        content
    )
    
    # Pattern 2: .to_string(8443, -> .to_string(), with port: 8443,
    content = re.sub(
        r'(\s+listen_address:\s+"[^"]+")\.to_string\((\d+),',
        r'\1.to_string(),\n            port: \2,',
        content
    )
    
    # Pattern 3: .to_string(30, after log_level -> add interval_seconds
    content = re.sub(
        r'(\s+log_level:\s+"[^"]+")\.to_string\((\d+),',
        r'\1.to_string(),\n            interval_seconds: \2,',
        content
    )
    
    # Pattern 4: .to_string(0.X, -> .to_string(), with strength/confidence/threshold
    content = re.sub(
        r'(\s+\w+):\s+("[^"]+")\.to_string\((0\.\d+),',
        r'\1: \2.to_string(),\n            strength: \3,',
        content
    )
    
    # Pattern 5: .to_string(true, or .to_string(false,
    content = re.sub(
        r'(\s+\w+):\s+("[^"]+")\.to_string\((true|false),',
        r'\1: \2.to_string(),\n            enabled: \3,',
        content
    )
    
    # Pattern 6: .to_string(SomeType::Variant, -> .to_string(), type: SomeType::Variant,
    content = re.sub(
        r'(\s+\w+):\s+("[^"]+")\.to_string\((\w+::\w+),',
        r'\1: \2.to_string(),\n            type_field: \3,',
        content
    )
    
    return content

def fix_malformed_map_err(content):
    """Fix .map_err(|e| BearDogError::xyz(...))?; patterns"""
    
    # Fix missing closing paren before )?;
    content = re.sub(
        r'\.map_err\(\|e\|\s+BearDogError::(\w+)\(([^)]*)\)\)?;',
        r'.map_err(|e| BearDogError::\1(\2))?;',
        content
    )
    
    return content

def fix_prefix_literals(content):
    """Fix prefix literals like R"..." -> r"..." """
    
    # Fix capital R prefix
    content = re.sub(r'R"', r'r"', content)
    
    # Fix string prefix issues (remove invalid prefixes)
    content = re.sub(r'(\s+)"([^"]+)"\s+unknown prefix', r'\1"\2"', content)
    
    return content

def fix_unclosed_delimiters(content):
    """Attempt to fix obvious unclosed delimiter patterns"""
    
    # Fix vec![ patterns missing closing bracket
    content = re.sub(
        r'(vec!\[\s*"[^"]+".to_string\(\)),(\s+\w+:)',
        r'\1],\2',
        content
    )
    
    return content

def process_file(filepath):
    """Process a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original = content
        
        content = fix_malformed_to_string(content)
        content = fix_malformed_map_err(content)
        content = fix_prefix_literals(content)
        content = fix_unclosed_delimiters(content)
        
        if content != original:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✓ Fixed: {filepath}")
            return True
        else:
            print(f"  Skipped (no changes): {filepath}")
            return False
            
    except Exception as e:
        print(f"✗ Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    """Main entry point"""
    test_dir = Path(__file__).parent.parent / "tests"
    
    if not test_dir.exists():
        print(f"Error: Test directory not found: {test_dir}", file=sys.stderr)
        sys.exit(1)
    
    test_files = list(test_dir.rglob("*.rs"))
    
    print(f"Found {len(test_files)} test files")
    print("Fixing syntax errors...")
    print()
    
    fixed = 0
    for test_file in test_files:
        if process_file(test_file):
            fixed += 1
    
    print()
    print(f"Summary: Fixed {fixed} / {len(test_files)} files")

if __name__ == "__main__":
    main() 