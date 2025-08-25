#!/usr/bin/env python3
"""
Fix all //! to /// doc comment issues for proper Rust documentation
"""

import re
import subprocess
from pathlib import Path

def fix_doc_comments():
    """Fix all //! inner doc comments to /// outer doc comments"""
    
    # Find all Rust files in the crates directory
    result = subprocess.run([
        'find', 'crates/', '-name', '*.rs', '-type', 'f'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        print("Error finding Rust files")
        return
    
    files = [f for f in result.stdout.strip().split('\n') if f]
    total_fixed = 0
    
    for file_path in files:
        fixed_count = fix_file_doc_comments(Path(file_path))
        if fixed_count > 0:
            total_fixed += fixed_count
            print(f"Fixed {fixed_count} doc comments in {file_path}")
    
    print(f"\nTotal doc comments fixed: {total_fixed}")

def fix_file_doc_comments(file_path: Path) -> int:
    """Fix doc comments in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return 0
    
    original_content = content
    lines = content.split('\n')
    fixed_lines = []
    fixes_made = 0
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Check if this is an inner doc comment that should be outer
        if line.strip().startswith('//!'):
            # Look ahead to see what follows
            next_item_line = find_next_item(lines, i + 1)
            
            if next_item_line is not None and should_be_outer_doc(lines[next_item_line]):
                # Convert //! to ///
                fixed_line = line.replace('//!', '///', 1)
                fixed_lines.append(fixed_line)
                fixes_made += 1
            else:
                # Keep as regular comment
                fixed_line = line.replace('//!', '//', 1)
                fixed_lines.append(fixed_line)
                fixes_made += 1
        else:
            fixed_lines.append(line)
        
        i += 1
    
    # Save if changes were made
    if fixes_made > 0:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(fixed_lines))
        except Exception as e:
            print(f"Error writing {file_path}: {e}")
            return 0
    
    return fixes_made

def find_next_item(lines: list, start_idx: int) -> int:
    """Find the next non-comment, non-empty line that could be a Rust item"""
    for i in range(start_idx, len(lines)):
        line = lines[i].strip()
        if line and not line.startswith('//') and not line.startswith('#['):
            return i
    return None

def should_be_outer_doc(line: str) -> bool:
    """Check if a line represents a Rust item that should have outer docs"""
    line = line.strip()
    rust_items = [
        'pub fn', 'fn ', 'pub struct', 'struct ', 'pub enum', 'enum ',
        'pub trait', 'trait ', 'pub mod', 'mod ', 'pub use', 'use ',
        'impl ', 'pub impl', 'pub type', 'type ', 'pub const', 'const ',
        'pub static', 'static ', 'pub macro', 'macro '
    ]
    
    return any(line.startswith(item) for item in rust_items)

def fix_specific_test_issues():
    """Fix the specific test compilation issue in categories/mod.rs"""
    file_path = Path("crates/beardog-errors/src/categories/mod.rs")
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return
    
    # Fix the Option.map_err issue in test
    original_content = content
    
    # Replace the problematic test line
    content = re.sub(
        r'assert_eq!\(retry_config\.map\(\|config\| config\)\.map_err\(\|e\| \{',
        r'let result = retry_config.ok_or_else(|| "Config not found"); assert_eq!(result.map_err(|e| {',
        content
    )
    
    if content != original_content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed test compilation issue in {file_path}")
        except Exception as e:
            print(f"Error writing {file_path}: {e}")

if __name__ == "__main__":
    print("🔧 Fixing documentation comments...")
    fix_doc_comments()
    
    print("\n🧪 Fixing test compilation issues...")
    fix_specific_test_issues()
    
    print("\n✅ Documentation cleanup complete!") 