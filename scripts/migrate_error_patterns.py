#!/usr/bin/env python3
"""
BearDog Error Pattern Migration Script

Migrates old struct-style BearDogError patterns to canonical constructor patterns.
This is part of Phase 2 of the canonical modernization plan.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Tuple, Dict

class ErrorPatternMigrator:
    def __init__(self):
        self.migration_patterns = self._initialize_migration_patterns()
        self.files_processed = 0
        self.patterns_migrated = 0
        
    def _initialize_migration_patterns(self) -> List[Tuple[str, str, str]]:
        """Initialize error pattern migrations: (old_pattern, new_pattern, description)"""
        return [
            # Configuration errors
            (r'BearDogError::Configuration\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::configuration(\1)', 
             'Configuration error migration'),
            
            # Authentication errors  
            (r'BearDogError::Authentication\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::authentication(\1)',
             'Authentication error migration'),
            
            # Internal errors
            (r'BearDogError::Internal\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::internal(\1)',
             'Internal error migration'),
            
            # Network errors
            (r'BearDogError::Network\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::network(\1)',
             'Network error migration'),
            
            # Validation errors
            (r'BearDogError::Validation\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::validation(\1)',
             'Validation error migration'),
            
            # Authorization errors
            (r'BearDogError::Authorization\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::invalid_input(\1)',
             'Authorization error migration'),
            
            # Encryption errors (special case with operation field)
            (r'BearDogError::Encryption\s*{\s*operation:\s*([^,]+),\s*message:\s*([^}]+)\s*}',
             r'BearDogError::encryption(\1, \2)',
             'Encryption error migration'),
            
            # Timeout errors
            (r'BearDogError::OperationTimeout\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::timeout(\1)',
             'Timeout error migration'),
            
            # Not found errors
            (r'BearDogError::NotFound\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::not_found(\1)',
             'Not found error migration'),
            
            # Generic business logic errors
            (r'BearDogError::InvalidGenetics\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::validation(\1)',
             'Invalid genetics error migration'),
            
            (r'BearDogError::SpawnRejected\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::validation(\1)',
             'Spawn rejected error migration'),
            
            # Compliance errors
            (r'BearDogError::Compliance\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::validation(\1)',
             'Compliance error migration'),
            
            # Threat detection errors
            (r'BearDogError::ThreatDetection\s*{\s*message:\s*([^}]+)\s*}',
             r'BearDogError::internal(\1)',
             'Threat detection error migration'),
            
            # HSM errors
            (r'BearDogError::HsmError\s*{\s*([^}]+)\s*}',
             r'BearDogError::internal("HSM operation failed")',
             'HSM error migration'),
        ]
    
    def migrate_file(self, filepath: Path) -> bool:
        """Migrate error patterns in a single file. Returns True if changes were made."""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            changes_made = False
            
            for old_pattern, new_pattern, description in self.migration_patterns:
                matches = re.findall(old_pattern, content)
                if matches:
                    print(f"  🔄 {description}: {len(matches)} instances")
                    content = re.sub(old_pattern, new_pattern, content)
                    changes_made = True
                    self.patterns_migrated += len(matches)
            
            if changes_made:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"  ✅ Updated {filepath}")
                return True
            else:
                print(f"  ⏭️  No changes needed for {filepath}")
                return False
                
        except Exception as e:
            print(f"  ❌ Error processing {filepath}: {e}")
            return False
    
    def find_target_files(self) -> List[Path]:
        """Find files that likely contain error patterns to migrate."""
        target_patterns = [
            "tests/**/*.rs",
            "crates/**/src/**/*.rs",
            "examples/**/*.rs"
        ]
        
        files = []
        for pattern in target_patterns:
            files.extend(Path(".").glob(pattern))
        
        # Filter out target directories and other build artifacts
        filtered_files = []
        for file in files:
            if 'target' not in str(file) and file.suffix == '.rs':
                filtered_files.append(file)
        
        return filtered_files
    
    def migrate_all(self) -> None:
        """Migrate error patterns in all target files."""
        print("🔄 BearDog Error Pattern Migration")
        print("=" * 50)
        
        target_files = self.find_target_files()
        print(f"📁 Found {len(target_files)} Rust files to check")
        
        files_changed = 0
        
        for filepath in target_files:
            print(f"\n📄 Processing {filepath}")
            if self.migrate_file(filepath):
                files_changed += 1
            self.files_processed += 1
        
        print(f"\n🎯 Migration Summary")
        print(f"Files processed: {self.files_processed}")
        print(f"Files changed: {files_changed}")
        print(f"Error patterns migrated: {self.patterns_migrated}")
        
        if files_changed > 0:
            print(f"\n🔧 Testing compilation...")
            result = os.system("cargo check --workspace --message-format=short 2>/dev/null")
            if result == 0:
                print("✅ Compilation successful after migration!")
            else:
                print("⚠️  Compilation issues remain - may need manual fixes")
        
        print(f"\n✨ Phase 2 error migration {'completed' if self.patterns_migrated > 0 else 'found no patterns to migrate'}")

def main():
    """Main entry point for error pattern migration."""
    if len(sys.argv) > 1 and sys.argv[1] == '--dry-run':
        print("🔍 Dry run mode - would migrate the following patterns:")
        migrator = ErrorPatternMigrator()
        for old, new, desc in migrator.migration_patterns:
            print(f"  {desc}: {old} → {new}")
        return
    
    migrator = ErrorPatternMigrator()
    migrator.migrate_all()

if __name__ == "__main__":
    main() 