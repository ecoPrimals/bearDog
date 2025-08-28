#!/usr/bin/env python3
"""
Fix Import Issues After Unification Migration

This script fixes the import issues created by the unification migration.
"""

import os
import re
from pathlib import Path

class ImportFixer:
    def __init__(self):
        self.files_fixed = 0
        
    def fix_beardog_types_imports(self, file_path: Path) -> bool:
        """Fix beardog_types imports that should be beardog-types"""
        try:
            content = file_path.read_text()
            original_content = content
            
            # Fix beardog_types -> beardog-types in use statements
            content = re.sub(r'use beardog_types::', 'use beardog_types::', content)
            
            # Fix derive attributes on use statements
            content = re.sub(r'#\[derive\([^\]]+\)\]\s*pub use', 'pub use', content)
            
            # Remove derive attributes from use statements
            lines = content.split('\n')
            fixed_lines = []
            i = 0
            
            while i < len(lines):
                line = lines[i]
                
                # Check if this is a derive attribute followed by pub use
                if re.match(r'#\[derive\([^\]]+\)\]', line.strip()):
                    if i + 1 < len(lines) and 'pub use' in lines[i + 1]:
                        # Skip the derive attribute, keep the pub use
                        i += 1
                        fixed_lines.append(lines[i])
                    else:
                        fixed_lines.append(line)
                else:
                    fixed_lines.append(line)
                
                i += 1
            
            content = '\n'.join(fixed_lines)
            
            if content != original_content:
                file_path.write_text(content)
                return True
                
        except Exception as e:
            print(f"❌ Error fixing imports in {file_path}: {e}")
            return False
            
        return False
    
    def fix_missing_imports(self, file_path: Path) -> bool:
        """Add missing imports and fix module references"""
        try:
            content = file_path.read_text()
            original_content = content
            
            # Fix specific import issues
            fixes = [
                # Fix constants import path
                (r'use crate::constants::unified::network::limits::CONNECTION_POOL_SIZE',
                 'use beardog_types::constants::unified::CONNECTION_POOL_SIZE'),
                
                # Fix config imports
                (r'use adapters::KubernetesConfig',
                 'use beardog_types::canonical::configuration::KubernetesConfig'),
                
                (r'use routing::ModelConfig',
                 'use beardog_types::canonical::configuration::ModelConfig'),
                 
                (r'use routing::OAuth2Config', 
                 'use beardog_types::canonical::configuration::OAuth2Config'),
                 
                (r'use routing::RouterConfig',
                 'use beardog_types::canonical::configuration::RouterConfig'),
                 
                (r'use monitoring::PrometheusConfig',
                 'use beardog_types::canonical::monitoring::PrometheusConfig'),
                 
                # Fix type references
                (r'SongBirdHandoffConfig',
                 'HandoffConfig'),
            ]
            
            for pattern, replacement in fixes:
                if re.search(pattern, content):
                    content = re.sub(pattern, replacement, content)
                    print(f"✅ Fixed import: {pattern} -> {replacement}")
            
            if content != original_content:
                file_path.write_text(content)
                return True
                
        except Exception as e:
            print(f"❌ Error fixing missing imports in {file_path}: {e}")
            return False
            
        return False
    
    def process_file(self, file_path: Path) -> bool:
        """Process a single file for import fixes"""
        if not file_path.suffix == '.rs':
            return False
            
        modified = False
        
        try:
            modified |= self.fix_beardog_types_imports(file_path)
            modified |= self.fix_missing_imports(file_path)
            
            if modified:
                self.files_fixed += 1
                print(f"🔧 Fixed imports in {file_path}")
                
            return modified
            
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def run_fixes(self) -> bool:
        """Run all import fixes"""
        print("🔧 Fixing import issues after unification...")
        print("=" * 50)
        
        crates_dir = Path("crates")
        if not crates_dir.exists():
            print("❌ Error: crates/ directory not found")
            return False
            
        rust_files = list(crates_dir.rglob("*.rs"))
        print(f"📁 Processing {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            self.process_file(file_path)
        
        print(f"\n✅ Fixed imports in {self.files_fixed} files")
        return True

def main():
    fixer = ImportFixer()
    fixer.run_fixes()

if __name__ == "__main__":
    main() 