#!/usr/bin/env python3
"""
Automated JSON Macro Refactoring Tool
Refactors serde_json::json!() macros to explicit JSON construction
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def find_json_macros(content: str) -> List[Tuple[int, int]]:
    """Find all json!() macro positions in the content."""
    matches = []
    for match in re.finditer(r'serde_json::json!|json!', content):
        start = match.start()
        # Find matching braces
        brace_count = 0
        i = content.find('(', start)
        if i == -1:
            continue
        i += 1
        start_pos = i
        while i < len(content):
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
                if brace_count == 0:
                    # Found closing brace
                    end_pos = i + 2  # Include })
                    matches.append((match.start(), end_pos))
                    break
            i += 1
    return matches

def refactor_simple_string_value(json_str: str) -> str:
    """Refactor simple string values like json!("value")."""
    match = re.match(r'(serde_json::)?json!\("([^"]+)"\)', json_str)
    if match:
        value = match.group(2)
        return f'serde_json::Value::String("{value}".to_string())'
    return None

def refactor_simple_object(json_str: str) -> str:
    """Refactor simple object patterns."""
    # This is a simplified version - handles basic cases
    # For complex nested objects, manual refactoring is better
    if '{{' in json_str:
        return None  # Skip nested objects for now
    
    # Extract content between braces
    match = re.search(r'json!\(\s*\{([^}]+)\}\s*\)', json_str)
    if not match:
        return None
    
    content = match.group(1).strip()
    
    # Parse simple key-value pairs
    lines = []
    lines.append('{')
    lines.append('    use serde_json::{Map, Value};')
    lines.append('    let mut obj = Map::new();')
    
    # Split by comma but respect strings
    pairs = []
    current = ""
    in_string = False
    for char in content + ',':
        if char == '"':
            in_string = not in_string
        if char == ',' and not in_string:
            if current.strip():
                pairs.append(current.strip())
            current = ""
        else:
            current += char
    
    for pair in pairs:
        pair = pair.strip()
        if not pair:
            continue
        
        # Parse "key": value
        match = re.match(r'"([^"]+)"\s*:\s*(.+)', pair)
        if not match:
            return None  # Complex pattern, skip
        
        key = match.group(1)
        value = match.group(2).strip()
        
        # Determine value type
        if value.startswith('"') and value.endswith('"'):
            # String value
            val_content = value[1:-1]
            lines.append(f'    obj.insert("{key}".to_string(), Value::String("{val_content}".to_string()));')
        elif value in ('true', 'false'):
            # Boolean
            lines.append(f'    obj.insert("{key}".to_string(), Value::Bool({value}));')
        elif value.isdigit():
            # Number
            lines.append(f'    obj.insert("{key}".to_string(), Value::Number({value}.into()));')
        else:
            return None  # Complex value, skip
    
    lines.append('    Value::Object(obj)')
    lines.append('}')
    
    return '\n'.join(lines)

def main():
    """Main refactoring function."""
    if len(sys.argv) < 2:
        print("Usage: python3 refactor_json_macro.py <file>")
        sys.exit(1)
    
    file_path = Path(sys.argv[1])
    if not file_path.exists():
        print(f"File not found: {file_path}")
        sys.exit(1)
    
    content = file_path.read_text()
    original = content
    
    # Find all json! macros
    macros = find_json_macros(content)
    print(f"Found {len(macros)} json! macro usages in {file_path}")
    
    # Refactor from end to start to preserve positions
    for start, end in reversed(macros):
        json_str = content[start:end]
        
        # Try simple refactorings
        refactored = refactor_simple_string_value(json_str)
        if not refactored:
            refactored = refactor_simple_object(json_str)
        
        if refactored:
            content = content[:start] + refactored + content[end:]
            print(f"  Refactored: {json_str[:50]}...")
        else:
            print(f"  Skipped (complex): {json_str[:50]}...")
    
    if content != original:
        # Backup original
        backup_path = file_path.with_suffix(file_path.suffix + '.bak')
        backup_path.write_text(original)
        
        # Write refactored
        file_path.write_text(content)
        print(f"\n✅ Refactored {file_path}")
        print(f"📦 Backup saved to {backup_path}")
    else:
        print("\n⚠️  No simple refactorings found - manual refactoring needed")

if __name__ == '__main__':
    main()

