#!/usr/bin/env python3
"""
BearDog Test Coverage Analysis and Restoration Script

Systematically analyzes disabled tests and provides recommendations for restoration.
"""

import os
import re
import subprocess
from pathlib import Path
from typing import List, Dict, Tuple
import json

class TestCoverageAnalyzer:
    def __init__(self, workspace_root: str = "."):
        self.workspace_root = Path(workspace_root)
        self.tests_dir = self.workspace_root / "tests"
        
    def analyze_test_status(self) -> Dict[str, int]:
        """Analyze current test status"""
        active_tests = list(self.tests_dir.glob("**/*.rs"))
        disabled_tests = list(self.tests_dir.glob("**/*.disabled"))
        
        return {
            "active": len(active_tests),
            "disabled": len(disabled_tests),
            "total": len(active_tests) + len(disabled_tests),
            "coverage_percentage": (len(active_tests) / (len(active_tests) + len(disabled_tests))) * 100
        }
    
    def categorize_disabled_tests(self) -> Dict[str, List[str]]:
        """Categorize disabled tests by type"""
        disabled_tests = list(self.tests_dir.glob("**/*.disabled"))
        categories = {
            "security": [],
            "integration": [],
            "core": [],
            "performance": [],
            "chaos": [],
            "e2e": [],
            "api": [],
            "hsm": [],
            "genetic": [],
            "other": []
        }
        
        for test_file in disabled_tests:
            test_name = test_file.name.lower()
            categorized = False
            
            for category in categories.keys():
                if category in test_name:
                    categories[category].append(str(test_file))
                    categorized = True
                    break
            
            if not categorized:
                categories["other"].append(str(test_file))
        
        return categories
    
    def analyze_test_dependencies(self, test_file: Path) -> List[str]:
        """Analyze dependencies of a test file"""
        dependencies = []
        try:
            with open(test_file, 'r') as f:
                content = f.read()
                
            # Extract use statements
            use_statements = re.findall(r'use\s+([^;]+);', content)
            for use_stmt in use_statements:
                if 'beardog' in use_stmt:
                    dependencies.append(use_stmt.strip())
                    
        except Exception as e:
            print(f"Error analyzing {test_file}: {e}")
            
        return dependencies
    
    def get_restoration_priority(self, categories: Dict[str, List[str]]) -> List[Tuple[str, int]]:
        """Get restoration priority based on importance and complexity"""
        priority_weights = {
            "core": 10,      # Highest priority - core functionality
            "security": 9,   # Critical security tests
            "api": 8,        # API functionality tests
            "integration": 7, # Integration tests
            "performance": 6, # Performance validation
            "genetic": 5,    # Genetic algorithm tests
            "hsm": 4,        # HSM-specific tests
            "e2e": 3,        # End-to-end tests
            "chaos": 2,      # Chaos engineering tests
            "other": 1       # Other tests
        }
        
        priorities = []
        for category, tests in categories.items():
            if tests:  # Only include categories with tests
                weight = priority_weights.get(category, 1)
                priorities.append((category, weight, len(tests)))
        
        # Sort by priority weight (descending)
        priorities.sort(key=lambda x: x[1], reverse=True)
        return [(cat, count) for cat, weight, count in priorities]
    
    def suggest_next_tests_to_restore(self, categories: Dict[str, List[str]], count: int = 5) -> List[str]:
        """Suggest next tests to restore based on priority"""
        priorities = self.get_restoration_priority(categories)
        suggestions = []
        
        for category, _ in priorities:
            if len(suggestions) >= count:
                break
                
            category_tests = categories[category]
            # Sort by file size (smaller files first, likely simpler)
            category_tests.sort(key=lambda x: os.path.getsize(x) if os.path.exists(x) else 0)
            
            for test_file in category_tests[:2]:  # Take up to 2 from each category
                if len(suggestions) < count:
                    suggestions.append(test_file)
        
        return suggestions
    
    def run_coverage_report(self) -> Dict[str, any]:
        """Run comprehensive coverage analysis"""
        print("🔍 Analyzing BearDog Test Coverage...")
        
        status = self.analyze_test_status()
        categories = self.categorize_disabled_tests()
        priorities = self.get_restoration_priority(categories)
        suggestions = self.suggest_next_tests_to_restore(categories)
        
        report = {
            "status": status,
            "categories": {k: len(v) for k, v in categories.items()},
            "priorities": priorities,
            "next_suggestions": suggestions,
            "analysis_timestamp": subprocess.check_output(['date']).decode().strip()
        }
        
        return report
    
    def print_report(self, report: Dict[str, any]):
        """Print formatted coverage report"""
        print("\n" + "="*60)
        print("📊 BEARDOG TEST COVERAGE ANALYSIS REPORT")
        print("="*60)
        
        status = report["status"]
        print(f"\n🎯 CURRENT STATUS:")
        print(f"   Active Tests: {status['active']}")
        print(f"   Disabled Tests: {status['disabled']}")
        print(f"   Total Tests: {status['total']}")
        print(f"   Coverage: {status['coverage_percentage']:.1f}%")
        
        print(f"\n📋 DISABLED TESTS BY CATEGORY:")
        for category, count in report["categories"].items():
            if count > 0:
                print(f"   {category.capitalize()}: {count} tests")
        
        print(f"\n🚀 RESTORATION PRIORITIES:")
        for category, count in report["priorities"]:
            print(f"   {category.capitalize()}: {count} tests")
        
        print(f"\n💡 NEXT SUGGESTED RESTORATIONS:")
        for i, suggestion in enumerate(report["next_suggestions"], 1):
            filename = Path(suggestion).name
            print(f"   {i}. {filename}")
        
        print(f"\n📈 PROGRESS TOWARD 90% GOAL:")
        current = status['coverage_percentage']
        target = 90.0
        remaining = target - current
        tests_needed = int((remaining / 100) * status['total'])
        print(f"   Current: {current:.1f}%")
        print(f"   Target: {target}%")
        print(f"   Need to restore: ~{tests_needed} more tests")
        
        print("="*60)

def main():
    analyzer = TestCoverageAnalyzer()
    report = analyzer.run_coverage_report()
    analyzer.print_report(report)
    
    # Save detailed report
    with open("test_coverage_report.json", "w") as f:
        json.dump(report, f, indent=2)
    
    print(f"📄 Detailed report saved to: test_coverage_report.json")

if __name__ == "__main__":
    main() 