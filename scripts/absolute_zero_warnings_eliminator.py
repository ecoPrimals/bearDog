#!/usr/bin/env python3
"""
🎯 ABSOLUTE ZERO WARNINGS ELIMINATOR
Removes ALL erroneous documentation to achieve perfect compilation hygiene.
"""

import os
import re
import sys
from pathlib import Path

class AbsoluteZeroWarningsEliminator:
    def __init__(self):
        self.fixes_applied = 0
        self.files_updated = 0
        
    def eliminate_erroneous_docs(self, content, file_path):
        """Remove all erroneous documentation that causes warnings"""
        lines = content.split('\n')
        result_lines = []
        updated = False
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Skip erroneous doc comments on expressions and fields
            if self._is_erroneous_doc_comment(lines, i):
                # Skip this line - it's an erroneous doc comment
                updated = True
                self.fixes_applied += 1
                i += 1
                continue
            
            result_lines.append(line)
            i += 1
        
        if updated:
            self.files_updated += 1
        
        return '\n'.join(result_lines)
    
    def _is_erroneous_doc_comment(self, lines, i):
        """Check if this is an erroneous doc comment that should be removed"""
        line = lines[i].strip()
        
        # Must be a doc comment
        if not line.startswith('///'):
            return False
        
        # Look at the next non-empty line
        j = i + 1
        while j < len(lines) and lines[j].strip() == '':
            j += 1
        
        if j >= len(lines):
            return False
        
        next_line = lines[j].strip()
        
        # Patterns that indicate erroneous doc comments
        erroneous_patterns = [
            # Expression fields in struct literals
            r'^\w+,$',  # field_name,
            r'^\w+\s*$',  # standalone expressions
            # Pattern matching fields
            r'^\w+,\s*$',  # pattern fields
            # Return statements and expressions
            r'^(true|false|None)$',
            r'^self$',
            r'^\w+_\w+$',  # snake_case identifiers
            # Constants and expressions
            r'^[A-Z_]+$',  # CONSTANTS
            # Function calls and complex expressions
            r'^\w+\s*\(',  # function calls
            r'.*\.\w+.*',  # method calls
        ]
        
        # Check if the doc comment describes a "variant" (which is wrong for expressions)
        if 'variant' in line:
            for pattern in erroneous_patterns:
                if re.match(pattern, next_line):
                    return True
        
        # Check for other common erroneous patterns
        if any(phrase in line.lower() for phrase in ['variant', 'associated type']) and \
           any(re.match(pattern, next_line) for pattern in erroneous_patterns):
            return True
        
        return False
    
    def fix_unused_variables(self, content, file_path):
        """Fix unused variable warnings by prefixing with underscore"""
        # Fix unused service parameter
        content = re.sub(
            r'fn deregister_service\(&self, service: &ServiceInfo\)',
            r'fn deregister_service(&self, _service: &ServiceInfo)',
            content
        )
        
        # Fix unused network_config variable
        content = re.sub(
            r'let network_config = ',
            r'let _network_config = ',
            content
        )
        
        return content
    
    def process_file(self, file_path):
        """Process a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = self.eliminate_erroneous_docs(content, file_path)
            content = self.fix_unused_variables(content, file_path)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True
            return False
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def process_directory(self, directory):
        """Process all Rust files in directory"""
        print(f"🎯 ABSOLUTE ZERO WARNINGS ELIMINATION in {directory}...")
        
        rust_files = list(Path(directory).rglob("*.rs"))
        print(f"📊 Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            # Skip test files
            if 'test' in str(file_path).lower():
                continue
            
            self.process_file(file_path)
        
        print(f"\n🎯 ABSOLUTE ZERO WARNINGS ELIMINATION COMPLETE:")
        print(f"   📊 Fixes applied: {self.fixes_applied}")
        print(f"   📁 Files updated: {self.files_updated}")

def main():
    print("🎯 ABSOLUTE ZERO WARNINGS ELIMINATOR")
    print("🚀 Mission: Eliminate ALL warnings for perfect compilation")
    print("📋 Principle: Zero tolerance for any compiler warnings")
    
    eliminator = AbsoluteZeroWarningsEliminator()
    eliminator.process_directory("crates/beardog-core/src/")
    
    if eliminator.fixes_applied > 0:
        print(f"\n✅ SUCCESS: Eliminated {eliminator.fixes_applied} warning sources!")
    else:
        print("\n📊 No warning sources found")

if __name__ == "__main__":
    main() 