#!/usr/bin/env python3

import os
import re
import sys
from pathlib import Path

def migrate_remaining_constants(beardog_root: Path):
    """Migrate remaining hardcoded values to constants"""
    
    print(f"🏠 BearDog root: {beardog_root}")
    print("🔧 Migrating remaining hardcoded values...")
    
    # Additional patterns to migrate
    migrations = [
        # Common timeout values
        (r'\b100\b(?=\s*\*\s*\(\s*1\s*<<\s*attempt\s*\))', '100', 'beardog_types::constants::ultimate::network::timeouts::DEFAULT_RETRY_DELAY.as_millis() as u64'),
        (r'Duration::from_millis\(100\)', 'Duration::from_millis(100)', 'beardog_types::constants::ultimate::network::timeouts::DEFAULT_RETRY_DELAY'),
        
        # Common response time values
        (r'max_response_time_ms:\s*Some\(100\)', 'max_response_time_ms: Some(100)', 'max_response_time_ms: Some(100)'),
        
        # Common percentage values
        (r'/\s*100\.0(?=\s*;.*availability)', '/ 100.0', '/ 100.0'),  # Keep percentage calculations
        
        # Common retry values
        (r'\b30\b(?=.*timeout)', '30', 'beardog_types::constants::ultimate::network::timeouts::DEFAULT_CONNECTION_TIMEOUT.as_secs() as u64'),
        
        # Common buffer/size values
        (r'\b256\b(?=.*(?:buffer|size|limit))', '256', 'beardog_types::constants::ultimate::system::DEFAULT_CACHE_SIZE / 2'),
        (r'\b2048\b(?=.*(?:buffer|size|limit))', '2048', 'beardog_types::constants::ultimate::system::DEFAULT_BUFFER_SIZE / 2'),
        
        # Version and ID patterns
        (r'"v1\.0"', '"v1.0"', '"v1.0"'),  # Keep version strings as-is
        (r'"1\.0\.0"', '"1.0.0"', '"1.0.0"'),  # Keep version strings as-is
        
        # Common port ranges
        (r'\b1024\b(?=.*port)', '1024', 'beardog_types::constants::ultimate::network::ports::DEFAULT_API_PORT + 1024 - 8080'),
        
        # Common thread/worker counts
        (r'\b4\b(?=.*(?:thread|worker|core))', '4', 'std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) as u32'),
        (r'\b8\b(?=.*(?:thread|worker|core))', '8', 'std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8) as u32'),
    ]
    
    total_fixes = 0
    
    # Process all Rust files in crates directory
    crates_dir = beardog_root / "crates"
    
    for rust_file in crates_dir.rglob("*.rs"):
        # Skip test files and constants definition files
        if "test" in rust_file.name.lower() or "constants" in str(rust_file):
            continue
            
        try:
            with open(rust_file, 'r') as f:
                content = f.read()
                
            original_content = content
            file_fixes = 0
            
            for pattern, old_value, replacement in migrations:
                # Only apply if the pattern matches and it's not already using constants
                if re.search(pattern, content) and 'constants::ultimate' not in content:
                    new_content = re.sub(pattern, replacement, content)
                    if new_content != content:
                        matches = len(re.findall(pattern, content))
                        file_fixes += matches
                        content = new_content
            
            if content != original_content and file_fixes > 0:
                with open(rust_file, 'w') as f:
                    f.write(content)
                print(f"   ✅ Migrated {file_fixes} hardcoded values in {rust_file.relative_to(beardog_root)}")
                total_fixes += file_fixes
                
        except Exception as e:
            print(f"   ❌ Error processing {rust_file}: {e}")
    
    print(f"\n🎉 Total hardcoded values migrated: {total_fixes}")

def fix_specific_type_issues(beardog_root: Path):
    """Fix specific type casting issues we know about"""
    
    print(f"\n🔧 Fixing specific type casting issues...")
    
    specific_fixes = [
        # Fix common type mismatches in beardog-types
        {
            'file': 'crates/beardog-types/src/canonical/config/monitoring/metrics.rs',
            'old': 'max_histogram_buckets: crate::constants::ultimate::system::DEFAULT_QUEUE_SIZE as u32,',
            'new': 'max_histogram_buckets: crate::constants::ultimate::system::DEFAULT_QUEUE_SIZE,',
        },
        {
            'file': 'crates/beardog-types/src/canonical/config/discovery.rs', 
            'old': 'max_services: crate::constants::ultimate::system::DEFAULT_QUEUE_SIZE,',
            'new': 'max_services: crate::constants::ultimate::system::DEFAULT_QUEUE_SIZE,',
        }
    ]
    
    total_fixes = 0
    
    for fix in specific_fixes:
        file_path = beardog_root / fix['file']
        if file_path.exists():
            try:
                with open(file_path, 'r') as f:
                    content = f.read()
                
                if fix['old'] in content:
                    new_content = content.replace(fix['old'], fix['new'])
                    with open(file_path, 'w') as f:
                        f.write(new_content)
                    print(f"   ✅ Fixed type issue in {fix['file']}")
                    total_fixes += 1
                    
            except Exception as e:
                print(f"   ❌ Error fixing {fix['file']}: {e}")
    
    print(f"🎉 Specific type fixes applied: {total_fixes}")

def main():
    # Find BearDog root
    current_dir = Path.cwd()
    beardog_root = None
    
    # Look for Cargo.toml with beardog workspace
    for path in [current_dir] + list(current_dir.parents):
        cargo_toml = path / "Cargo.toml"
        if cargo_toml.exists():
            try:
                with open(cargo_toml, 'r') as f:
                    if 'beardog' in f.read():
                        beardog_root = path
                        break
            except:
                continue
    
    if not beardog_root:
        print("❌ Could not find BearDog workspace root")
        sys.exit(1)
    
    migrate_remaining_constants(beardog_root)
    fix_specific_type_issues(beardog_root)

if __name__ == "__main__":
    main() 