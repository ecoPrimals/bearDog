#!/usr/bin/env python3
"""
BearDog Code Evolution Automation
Systematically evolves the codebase to modern, idiomatic Rust patterns
"""

import subprocess
import re
from pathlib import Path
from typing import List, Tuple

class BeardogEvolution:
    def __init__(self, repo_root: Path):
        self.repo_root = repo_root
        self.stats = {
            'unwraps_fixed': 0,
            'expects_fixed': 0,
            'unsafe_evolved': 0,
            'docs_added': 0,
            'hardcoding_removed': 0
        }
    
    def evolve_unwraps_in_file(self, file_path: Path) -> int:
        """Replace .unwrap() with proper error handling in production code"""
        if 'tests/' in str(file_path) or '/test' in str(file_path):
            return 0  # Skip test files
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        original = content
        
        # Pattern: .lock().unwrap() -> .lock().map_err(...)
        content = re.sub(
            r'\.lock\(\)\.unwrap\(\)',
            r'.lock().map_err(|e| BearDogError::system(format!("Lock poisoned: {}", e)))?',
            content
        )
        
        # Pattern: .expect("msg") -> .map_err(...)
        content = re.sub(
            r'\.expect\("([^"]+)"\)',
            r'.map_err(|e| BearDogError::system(format!("\1: {}", e)))?',
            content
        )
        
        if content != original:
            with open(file_path, 'w') as f:
                f.write(content)
            return content.count('.map_err(') - original.count('.map_err(')
        return 0
    
    def add_missing_docs(self):
        """Add documentation to items missing docs"""
        result = subprocess.run(
            ['cargo', 'clippy', '--', '-W', 'missing_docs'],
            cwd=self.repo_root,
            capture_output=True,
            text=True
        )
        
        # Parse clippy output to find missing docs
        missing_docs = re.findall(r'missing documentation for (.+)', result.stderr)
        print(f"Found {len(missing_docs)} missing documentation items")
        return len(missing_docs)
    
    def run_fmt(self):
        """Run cargo fmt"""
        subprocess.run(['cargo', 'fmt'], cwd=self.repo_root)
        print("✅ Formatted all code")
    
    def run_tests(self) -> bool:
        """Run test suite"""
        result = subprocess.run(
            ['cargo', 'test', '--workspace'],
            cwd=self.repo_root,
            capture_output=True
        )
        return result.returncode == 0
    
    def measure_coverage(self):
        """Measure test coverage with llvm-cov"""
        subprocess.run([
            'cargo', 'llvm-cov', '--all-features', '--workspace',
            '--html', '--output-dir', 'coverage/html'
        ], cwd=self.repo_root)
        print("✅ Coverage report generated in coverage/html/")
    
    def report(self):
        """Print evolution report"""
        print("\n" + "="*60)
        print("🚀 BEARDOG EVOLUTION REPORT")
        print("="*60)
        for key, value in self.stats.items():
            print(f"  {key}: {value}")
        print("="*60 + "\n")

if __name__ == '__main__':
    repo = Path('/home/eastgate/Development/ecoPrimals/beardog')
    evolution = BeardogEvolution(repo)
    
    print("🐻 Starting BearDog Evolution Process...")
    
    # 1. Format code
    evolution.run_fmt()
    
    # 2. Add documentation
    missing = evolution.add_missing_docs()
    
    # 3. Fix unwraps in production code
    rust_files = list(repo.glob('crates/**/src/**/*.rs'))
    for file in rust_files:
        fixed = evolution.evolve_unwraps_in_file(file)
        evolution.stats['unwraps_fixed'] += fixed
    
    # 4. Run tests
    print("\n🧪 Running test suite...")
    if evolution.run_tests():
        print("✅ All tests passing!")
    else:
        print("⚠️  Some tests failing - review required")
    
    # 5. Measure coverage
    print("\n📊 Measuring test coverage...")
    evolution.measure_coverage()
    
    # 6. Report
    evolution.report()

