#!/usr/bin/env python3
"""
BearDog Technical Debt Cleanup Tool

This tool identifies and helps clean up technical debt including:
1. Deprecated code and compatibility layers
2. Unused imports and dead code
3. TODO/FIXME/HACK comments
4. Large files that need refactoring
5. Duplicate code patterns

Usage:
    python3 technical_debt_cleaner.py --scan
    python3 technical_debt_cleaner.py --clean --type deprecated
    python3 technical_debt_cleaner.py --refactor --max-lines 2000
"""

import os
import re
import argparse
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
from dataclasses import dataclass
from datetime import datetime
from collections import defaultdict

@dataclass
class DebtItem:
    """Represents a technical debt item"""
    file_path: str
    line_number: int
    debt_type: str  # 'deprecated', 'todo', 'unused_import', 'large_file', 'duplicate'
    severity: str   # 'low', 'medium', 'high', 'critical'
    description: str
    context: str
    suggested_action: str
    estimated_effort: str  # 'trivial', 'small', 'medium', 'large'

@dataclass
class DebtReport:
    """Technical debt analysis report"""
    timestamp: str
    total_files_scanned: int
    total_debt_items: int
    debt_by_type: Dict[str, int]
    debt_by_severity: Dict[str, int]
    files_needing_refactor: List[str]
    critical_issues: List[str]
    cleanup_recommendations: List[str]

class TechnicalDebtCleaner:
    """Main technical debt cleanup tool"""
    
    def __init__(self, project_root: str, max_file_lines: int = 2000):
        self.project_root = Path(project_root)
        self.max_file_lines = max_file_lines
        self.debt_items: List[DebtItem] = []
        
        # Patterns for different types of technical debt
        self.deprecated_patterns = [
            r'#\[deprecated.*?\]',
            r'// ?DEPRECATED',
            r'// ?TODO.*deprecated',
            r'LegacyUnifiedProvider',
            r'LegacyHsmProvider',
            r'LegacySecurityProvider',
            r'EnhancedError.*Legacy',
            r'ErrorContext.*Legacy',
        ]
        
        self.todo_patterns = [
            r'// ?TODO',
            r'// ?FIXME',
            r'// ?HACK',
            r'// ?XXX',
            r'// ?NOTE.*fix',
            r'// ?BUG',
            r'unimplemented!',
            r'todo!',
        ]
        
        self.compatibility_patterns = [
            r'compat',
            r'_shim',
            r'_wrapper',
            r'_bridge',
            r'_adapter.*legacy',
            r'backward_compatibility',
            r'migration.*helper',
        ]
        
        self.duplicate_code_indicators = [
            r'// ?DUPLICATE',
            r'// ?COPY.*FROM',
            r'// ?SAME.*AS',
        ]
    
    def scan_codebase(self) -> DebtReport:
        """Scan the entire codebase for technical debt"""
        print("🔍 Scanning codebase for technical debt...")
        
        rust_files = list(self.project_root.rglob("*.rs"))
        python_files = list(self.project_root.rglob("*.py"))
        all_files = rust_files + python_files
        
        total_files = len(all_files)
        
        for file_path in all_files:
            self._scan_file(file_path)
        
        # Additional analysis
        self._analyze_file_sizes(rust_files)
        self._analyze_unused_imports(rust_files)
        self._analyze_duplicate_patterns(rust_files)
        
        return self._generate_report(total_files)
    
    def _scan_file(self, file_path: Path) -> None:
        """Scan a single file for technical debt"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            for line_num, line in enumerate(lines, 1):
                self._scan_line(str(file_path), line_num, line.strip())
                
        except (UnicodeDecodeError, IOError) as e:
            print(f"⚠️ Warning: Could not read {file_path}: {e}")
    
    def _scan_line(self, file_path: str, line_num: int, line: str) -> None:
        """Scan a single line for technical debt indicators"""
        if not line.strip():
            return
        
        # Check for deprecated code
        for pattern in self.deprecated_patterns:
            if re.search(pattern, line, re.IGNORECASE):
                severity = 'high' if 'deprecated' in pattern.lower() else 'medium'
                self.debt_items.append(DebtItem(
                    file_path=file_path,
                    line_number=line_num,
                    debt_type='deprecated',
                    severity=severity,
                    description=f'Deprecated code found: {pattern}',
                    context=line,
                    suggested_action='Remove deprecated code and update to new implementation',
                    estimated_effort='small'
                ))
        
        # Check for TODO/FIXME/HACK comments
        for pattern in self.todo_patterns:
            if re.search(pattern, line, re.IGNORECASE):
                severity = self._determine_todo_severity(line)
                self.debt_items.append(DebtItem(
                    file_path=file_path,
                    line_number=line_num,
                    debt_type='todo',
                    severity=severity,
                    description=f'TODO/FIXME found: {pattern}',
                    context=line,
                    suggested_action=self._suggest_todo_action(line),
                    estimated_effort=self._estimate_todo_effort(line)
                ))
        
        # Check for compatibility layers
        for pattern in self.compatibility_patterns:
            if re.search(pattern, line, re.IGNORECASE):
                self.debt_items.append(DebtItem(
                    file_path=file_path,
                    line_number=line_num,
                    debt_type='compatibility',
                    severity='medium',
                    description=f'Compatibility layer found: {pattern}',
                    context=line,
                    suggested_action='Evaluate if compatibility layer is still needed',
                    estimated_effort='medium'
                ))
        
        # Check for duplicate code indicators
        for pattern in self.duplicate_code_indicators:
            if re.search(pattern, line, re.IGNORECASE):
                self.debt_items.append(DebtItem(
                    file_path=file_path,
                    line_number=line_num,
                    debt_type='duplicate',
                    severity='medium',
                    description=f'Duplicate code indicator: {pattern}',
                    context=line,
                    suggested_action='Refactor to eliminate code duplication',
                    estimated_effort='medium'
                ))
    
    def _analyze_file_sizes(self, rust_files: List[Path]) -> None:
        """Analyze file sizes and identify files that need refactoring"""
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    line_count = sum(1 for _ in f)
                
                if line_count > self.max_file_lines:
                    severity = 'critical' if line_count > self.max_file_lines * 1.5 else 'high'
                    self.debt_items.append(DebtItem(
                        file_path=str(file_path),
                        line_number=1,
                        debt_type='large_file',
                        severity=severity,
                        description=f'File too large: {line_count} lines (max: {self.max_file_lines})',
                        context=f'File has {line_count} lines',
                        suggested_action='Refactor into smaller, focused modules',
                        estimated_effort='large' if line_count > self.max_file_lines * 2 else 'medium'
                    ))
                    
            except (UnicodeDecodeError, IOError):
                continue
    
    def _analyze_unused_imports(self, rust_files: List[Path]) -> None:
        """Analyze files for potentially unused imports"""
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Find all use statements
                use_statements = re.findall(r'use\s+([^;]+);', content)
                
                for line_num, line in enumerate(content.split('\n'), 1):
                    if line.strip().startswith('use ') and '::' in line:
                        # Simple heuristic: check if imported items are used
                        imported_items = self._extract_imported_items(line)
                        for item in imported_items:
                            if item and len(item) > 2:  # Skip very short names
                                # Count occurrences (excluding the import line)
                                occurrences = content.count(item) - line.count(item)
                                if occurrences == 0:
                                    self.debt_items.append(DebtItem(
                                        file_path=str(file_path),
                                        line_number=line_num,
                                        debt_type='unused_import',
                                        severity='low',
                                        description=f'Potentially unused import: {item}',
                                        context=line.strip(),
                                        suggested_action='Remove unused import or verify usage',
                                        estimated_effort='trivial'
                                    ))
                                    
            except (UnicodeDecodeError, IOError):
                continue
    
    def _analyze_duplicate_patterns(self, rust_files: List[Path]) -> None:
        """Analyze for duplicate code patterns"""
        # Simple duplicate detection based on function signatures
        function_signatures = defaultdict(list)
        
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                
                for line_num, line in enumerate(lines, 1):
                    # Look for function definitions
                    if re.match(r'\s*(pub\s+)?(async\s+)?fn\s+\w+\s*\(', line):
                        # Extract function signature (simplified)
                        signature = re.sub(r'\s+', ' ', line.strip())
                        function_signatures[signature].append((str(file_path), line_num))
                        
            except (UnicodeDecodeError, IOError):
                continue
        
        # Report potential duplicates
        for signature, locations in function_signatures.items():
            if len(locations) > 1:
                for file_path, line_num in locations:
                    self.debt_items.append(DebtItem(
                        file_path=file_path,
                        line_number=line_num,
                        debt_type='duplicate',
                        severity='low',
                        description=f'Potentially duplicate function signature',
                        context=signature,
                        suggested_action='Review for actual duplication and refactor if needed',
                        estimated_effort='small'
                    ))
    
    def _extract_imported_items(self, use_statement: str) -> List[str]:
        """Extract individual items from a use statement"""
        items = []
        
        # Handle different use statement formats
        if '{' in use_statement and '}' in use_statement:
            # use path::{item1, item2, item3}
            match = re.search(r'\{([^}]+)\}', use_statement)
            if match:
                items_str = match.group(1)
                items = [item.strip() for item in items_str.split(',')]
        else:
            # use path::item
            match = re.search(r'use\s+(?:[^:]+::)*([^:;\s]+)', use_statement)
            if match:
                items = [match.group(1)]
        
        return [item for item in items if item and not item.startswith('*')]
    
    def _determine_todo_severity(self, line: str) -> str:
        """Determine severity of TODO/FIXME based on content"""
        line_lower = line.lower()
        
        if any(word in line_lower for word in ['critical', 'urgent', 'security', 'bug', 'crash']):
            return 'critical'
        elif any(word in line_lower for word in ['important', 'performance', 'memory']):
            return 'high'
        elif any(word in line_lower for word in ['hack', 'fixme', 'broken']):
            return 'medium'
        else:
            return 'low'
    
    def _suggest_todo_action(self, line: str) -> str:
        """Suggest action for TODO/FIXME items"""
        line_lower = line.lower()
        
        if 'implement' in line_lower:
            return 'Implement the missing functionality'
        elif 'fix' in line_lower or 'bug' in line_lower:
            return 'Fix the identified issue'
        elif 'optimize' in line_lower or 'performance' in line_lower:
            return 'Optimize the code for better performance'
        elif 'refactor' in line_lower:
            return 'Refactor the code for better structure'
        elif 'test' in line_lower:
            return 'Add or improve tests'
        else:
            return 'Address the TODO item'
    
    def _estimate_todo_effort(self, line: str) -> str:
        """Estimate effort required for TODO/FIXME items"""
        line_lower = line.lower()
        
        if any(word in line_lower for word in ['rewrite', 'redesign', 'major']):
            return 'large'
        elif any(word in line_lower for word in ['refactor', 'implement', 'add']):
            return 'medium'
        elif any(word in line_lower for word in ['fix', 'update', 'change']):
            return 'small'
        else:
            return 'trivial'
    
    def _generate_report(self, total_files: int) -> DebtReport:
        """Generate comprehensive debt report"""
        debt_by_type = defaultdict(int)
        debt_by_severity = defaultdict(int)
        
        for item in self.debt_items:
            debt_by_type[item.debt_type] += 1
            debt_by_severity[item.severity] += 1
        
        # Identify files needing refactoring
        files_needing_refactor = []
        for item in self.debt_items:
            if item.debt_type == 'large_file':
                files_needing_refactor.append(f"{item.file_path} ({item.description})")
        
        # Identify critical issues
        critical_issues = []
        for item in self.debt_items:
            if item.severity == 'critical':
                rel_path = Path(item.file_path).relative_to(self.project_root)
                critical_issues.append(f"{rel_path}:{item.line_number} - {item.description}")
        
        # Generate cleanup recommendations
        recommendations = self._generate_cleanup_recommendations(debt_by_type, debt_by_severity)
        
        return DebtReport(
            timestamp=datetime.now().isoformat(),
            total_files_scanned=total_files,
            total_debt_items=len(self.debt_items),
            debt_by_type=dict(debt_by_type),
            debt_by_severity=dict(debt_by_severity),
            files_needing_refactor=files_needing_refactor,
            critical_issues=critical_issues,
            cleanup_recommendations=recommendations
        )
    
    def _generate_cleanup_recommendations(self, debt_by_type: Dict[str, int], 
                                        debt_by_severity: Dict[str, int]) -> List[str]:
        """Generate cleanup recommendations based on debt analysis"""
        recommendations = [
            "🧹 Technical Debt Cleanup Recommendations:",
            "",
        ]
        
        # Priority recommendations based on severity
        if debt_by_severity.get('critical', 0) > 0:
            recommendations.extend([
                "🚨 **CRITICAL PRIORITY:**",
                f"   - Address {debt_by_severity['critical']} critical issues immediately",
                "   - These may impact system stability or security",
                "",
            ])
        
        if debt_by_severity.get('high', 0) > 0:
            recommendations.extend([
                "🔥 **HIGH PRIORITY:**",
                f"   - Address {debt_by_severity['high']} high-priority issues",
                "   - Schedule for next sprint or release cycle",
                "",
            ])
        
        # Type-specific recommendations
        if debt_by_type.get('deprecated', 0) > 0:
            recommendations.extend([
                "📦 **Deprecated Code Cleanup:**",
                f"   - Remove {debt_by_type['deprecated']} deprecated code instances",
                "   - Update to use new unified APIs",
                "   - Run migration scripts where available",
                "",
            ])
        
        if debt_by_type.get('large_file', 0) > 0:
            recommendations.extend([
                "📄 **File Size Refactoring:**",
                f"   - Refactor {debt_by_type['large_file']} large files",
                "   - Break into smaller, focused modules",
                "   - Maintain single responsibility principle",
                "",
            ])
        
        if debt_by_type.get('todo', 0) > 0:
            recommendations.extend([
                "✅ **TODO/FIXME Resolution:**",
                f"   - Address {debt_by_type['todo']} TODO/FIXME items",
                "   - Prioritize by severity and business impact",
                "   - Convert to proper issues/tickets for tracking",
                "",
            ])
        
        if debt_by_type.get('unused_import', 0) > 0:
            recommendations.extend([
                "🧽 **Import Cleanup:**",
                f"   - Remove {debt_by_type['unused_import']} potentially unused imports",
                "   - Use `cargo fix` for automated cleanup",
                "   - Review and verify before removal",
                "",
            ])
        
        recommendations.extend([
            "📈 **Ongoing Maintenance:**",
            "   - Set up automated linting to prevent new debt",
            "   - Regular debt review in team retrospectives",
            "   - Establish coding standards and guidelines",
            "   - Use pre-commit hooks for quality checks",
        ])
        
        return recommendations
    
    def print_report(self, report: DebtReport) -> None:
        """Print a formatted technical debt report"""
        print("\n" + "="*80)
        print("🧹 BEARDOG TECHNICAL DEBT CLEANUP REPORT")
        print("="*80)
        print(f"📅 Generated: {report.timestamp}")
        print(f"📁 Files scanned: {report.total_files_scanned}")
        print(f"⚠️  Total debt items: {report.total_debt_items}")
        
        if report.critical_issues:
            print("\n🚨 CRITICAL ISSUES (Immediate Attention Required):")
            for issue in report.critical_issues[:10]:  # Show top 10
                print(f"   {issue}")
            if len(report.critical_issues) > 10:
                print(f"   ... and {len(report.critical_issues) - 10} more critical issues")
        
        print("\n📊 DEBT BREAKDOWN BY TYPE:")
        for debt_type, count in sorted(report.debt_by_type.items()):
            print(f"   📂 {debt_type.replace('_', ' ').title()}: {count} items")
        
        print("\n⚡ DEBT BREAKDOWN BY SEVERITY:")
        severity_order = ['critical', 'high', 'medium', 'low']
        for severity in severity_order:
            count = report.debt_by_severity.get(severity, 0)
            if count > 0:
                icon = {'critical': '🚨', 'high': '🔥', 'medium': '⚠️', 'low': '📝'}[severity]
                print(f"   {icon} {severity.title()}: {count} items")
        
        if report.files_needing_refactor:
            print("\n📄 FILES NEEDING REFACTORING:")
            for file_info in report.files_needing_refactor[:10]:  # Show top 10
                print(f"   📝 {file_info}")
            if len(report.files_needing_refactor) > 10:
                print(f"   ... and {len(report.files_needing_refactor) - 10} more files")
        
        if report.cleanup_recommendations:
            print("\n" + "\n".join(report.cleanup_recommendations))
        
        print("\n" + "="*80)
    
    def generate_cleanup_script(self, debt_type: str, output_file: str) -> None:
        """Generate a cleanup script for specific debt type"""
        type_items = [item for item in self.debt_items if item.debt_type == debt_type]
        
        if not type_items:
            print(f"✅ No {debt_type} items found")
            return
        
        script_content = self._generate_cleanup_script_content(debt_type, type_items)
        
        with open(output_file, 'w') as f:
            f.write(script_content)
        
        print(f"📝 Generated cleanup script: {output_file}")
        print(f"   - Type: {debt_type}")
        print(f"   - Items to address: {len(type_items)}")
    
    def _generate_cleanup_script_content(self, debt_type: str, items: List[DebtItem]) -> str:
        """Generate cleanup script content"""
        script_lines = [
            f"#!/bin/bash",
            f"# BearDog {debt_type.title()} Cleanup Script",
            f"# Generated on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            f"#",
            f"# This script helps clean up {debt_type} technical debt.",
            f"",
            f"set -e",
            f"",
            f"echo \"🧹 Starting {debt_type} cleanup...\"",
            f"",
        ]
        
        if debt_type == 'unused_import':
            script_lines.extend([
                "# Use cargo fix to remove unused imports automatically",
                "echo \"🔧 Running cargo fix for unused imports...\"",
                "cargo fix --allow-dirty --allow-staged",
                "",
                "# Manual review items:",
            ])
            
            for item in items[:20]:  # Limit to first 20
                rel_path = Path(item.file_path).relative_to(self.project_root)
                script_lines.append(f"# {rel_path}:{item.line_number} - {item.description}")
        
        elif debt_type == 'deprecated':
            script_lines.extend([
                "# Deprecated code removal",
                "echo \"📦 Removing deprecated code...\"",
                "",
            ])
            
            files_to_clean = set(item.file_path for item in items)
            for file_path in files_to_clean:
                rel_path = Path(file_path).relative_to(self.project_root)
                script_lines.extend([
                    f"echo \"📝 Cleaning {rel_path}...\"",
                    f"# Manual review required for: {rel_path}",
                    "",
                ])
        
        script_lines.extend([
            f"echo \"✅ {debt_type.title()} cleanup script complete!\"",
            f"echo \"🔍 Please review all changes before committing\"",
        ])
        
        return "\n".join(script_lines)

def main():
    parser = argparse.ArgumentParser(description='BearDog Technical Debt Cleanup Tool')
    parser.add_argument('--scan', action='store_true', help='Scan codebase for technical debt')
    parser.add_argument('--clean', help='Generate cleanup script for specific debt type')
    parser.add_argument('--type', help='Type of debt to clean (deprecated, todo, unused_import, large_file, duplicate)')
    parser.add_argument('--max-lines', type=int, default=2000, help='Maximum lines per file before flagging for refactor')
    parser.add_argument('--output', default='cleanup_script.sh', help='Output file for cleanup script')
    parser.add_argument('--project-root', default='.', help='Project root directory')
    
    args = parser.parse_args()
    
    cleaner = TechnicalDebtCleaner(args.project_root, args.max_lines)
    
    if args.scan:
        print("🔍 Starting comprehensive technical debt scan...")
        report = cleaner.scan_codebase()
        cleaner.print_report(report)
        
        # Save detailed report to JSON
        report_file = f"debt_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(report_file, 'w') as f:
            json.dump({
                'timestamp': report.timestamp,
                'total_files_scanned': report.total_files_scanned,
                'total_debt_items': report.total_debt_items,
                'debt_by_type': report.debt_by_type,
                'debt_by_severity': report.debt_by_severity,
                'files_needing_refactor': report.files_needing_refactor,
                'critical_issues': report.critical_issues,
                'debt_items': [{
                    'file_path': item.file_path,
                    'line_number': item.line_number,
                    'debt_type': item.debt_type,
                    'severity': item.severity,
                    'description': item.description,
                    'context': item.context,
                    'suggested_action': item.suggested_action,
                    'estimated_effort': item.estimated_effort
                } for item in cleaner.debt_items]
            }, f, indent=2)
        
        print(f"\n💾 Detailed report saved to: {report_file}")
    
    elif args.clean and args.type:
        print(f"🛠️ Generating cleanup script for {args.type} debt...")
        # First scan to find debt items
        cleaner.scan_codebase()
        cleaner.generate_cleanup_script(args.type, args.output)
    
    else:
        parser.print_help()

if __name__ == '__main__':
    main() 