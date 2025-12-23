#!/usr/bin/env python3
"""
🔧 INTELLIGENT CONST FIXER 🔧
Removes const from functions that cannot be const due to runtime operations
"""

import os
import re
from pathlib import Path

class IntelligentConstFixer:
    def __init__(self, crates_dir="crates"):
        self.crates_dir = Path(crates_dir)
        self.fixes_applied = 0
        
        # Patterns that indicate a function cannot be const
        self.non_const_patterns = [
            r'std::env::var',
            r'HashMap::new\(\)',
            r'BTreeMap::new\(\)',
            r'Vec::new\(\)',
            r'String::new\(\)',
            r'\.to_string\(\)',
            r'Uuid::new_v4\(\)',
            r'Utc::now\(\)',
            r'SystemTime::now\(\)',
            r'\.load\(',
            r'\.store\(',
            r'is_x86_feature_detected!',
            r'Arc::new',
            r'Rc::new',
            r'Box::new',
            r'RwLock::new',
            r'Mutex::new',
            r'vec!\[',
            r'format!\(',
            r'println!\(',
            r'eprintln!\(',
            r'::default\(\)',
            r'\.clone\(\)',
            r'\.into_vec',
            r'for\s+\w+\s+in',
            r'\.unwrap_or_else',
            r'\.map_err',
            r'\.expect',
            r'\.unwrap\(\)',
            r'async\s+fn',
            r'\.await',
            r'tokio::',
            r'futures::',
            r'\.spawn\(',
            r'\.join\(',
        ]
    
    def should_remove_const(self, function_content: str) -> bool:
        """Check if a function should have const removed"""
        for pattern in self.non_const_patterns:
            if re.search(pattern, function_content, re.IGNORECASE):
                return True
        return False
    
    def fix_file(self, file_path: Path):
        """Fix const functions in a single file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Find all const functions and their bodies
            const_fn_pattern = r'((?:pub\s+)?const\s+fn\s+\w+[^{]*\{)'
            
            lines = content.split('\n')
            modified = False
            
            i = 0
            while i < len(lines):
                line = lines[i]
                
                # Check if this line starts a const function
                if re.search(r'(?:pub\s+)?const\s+fn\s+\w+', line):
                    # Find the function body to analyze
                    function_start = i
                    brace_count = 0
                    function_lines = []
                    
                    # Collect the entire function
                    j = i
                    while j < len(lines):
                        current_line = lines[j]
                        function_lines.append(current_line)
                        
                        # Count braces to find function end
                        brace_count += current_line.count('{')
                        brace_count -= current_line.count('}')
                        
                        if brace_count == 0 and '{' in current_line:
                            break
                        j += 1
                    
                    function_content = '\n'.join(function_lines)
                    
                    # Check if this function should not be const
                    if self.should_remove_const(function_content):
                        # Remove const from the function signature
                        new_line = re.sub(r'const\s+fn', 'fn', line)
                        if new_line != line:
                            lines[i] = new_line
                            modified = True
                            self.fixes_applied += 1
                            print(f"  🔧 Removed const from function in {file_path.name}:{i+1}")
                
                i += 1
            
            if modified:
                with open(file_path, 'w') as f:
                    f.write('\n'.join(lines))
                    
        except Exception as e:
            print(f"Error fixing {file_path}: {e}")
    
    def fix_all_files(self):
        """Fix all Rust files in the crates directory"""
        print("🔧 INTELLIGENT CONST FIXER ACTIVATED")
        print("🎯 Removing const from functions that cannot be const...")
        
        for rust_file in self.crates_dir.rglob("*.rs"):
            self.fix_file(rust_file)
        
        print(f"\n✅ INTELLIGENT CONST FIXES APPLIED: {self.fixes_applied}")
        return self.fixes_applied > 0

if __name__ == "__main__":
    fixer = IntelligentConstFixer()
    fixer.fix_all_files() 