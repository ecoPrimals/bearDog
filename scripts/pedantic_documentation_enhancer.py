#!/usr/bin/env python3
"""
🔍 PEDANTIC DOCUMENTATION ENHANCER

This script systematically enhances documentation across the BearDog unified
configuration system to achieve pedantic-level documentation standards.

Features:
- Analyzes Rust source files for missing documentation
- Generates comprehensive documentation for structs, enums, and fields
- Maintains consistent documentation style and format
- Focuses on unified configuration modules for maximum impact

Usage:
    python3 scripts/pedantic_documentation_enhancer.py
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass

@dataclass
class DocumentationIssue:
    """Represents a documentation issue found in the code."""
    file_path: str
    line_number: int
    issue_type: str  # 'struct', 'enum', 'field', 'variant', 'function'
    item_name: str
    context: str

class PedanticDocumentationEnhancer:
    """
    Enhances documentation to pedantic standards for BearDog configurations.
    """
    
    def __init__(self, project_root: str):
        """Initialize the documentation enhancer.
        
        Args:
            project_root: Path to the BearDog project root directory
        """
        self.project_root = Path(project_root)
        self.unified_config_path = self.project_root / "crates" / "beardog-types" / "src" / "canonical" / "config" / "domains_unified"
        self.issues: List[DocumentationIssue] = []
        
        # Documentation templates for different item types
        self.templates = {
            'struct': self._generate_struct_doc,
            'enum': self._generate_enum_doc,
            'field': self._generate_field_doc,
            'variant': self._generate_variant_doc,
            'function': self._generate_function_doc,
        }
    
    def analyze_unified_configurations(self) -> List[DocumentationIssue]:
        """
        Analyze unified configuration files for missing documentation.
        
        Returns:
            List of documentation issues found
        """
        print("🔍 Analyzing unified configuration documentation...")
        
        config_files = [
            "mod.rs",
            "adapter.rs", 
            "security.rs",
            "ai.rs"
        ]
        
        for config_file in config_files:
            file_path = self.unified_config_path / config_file
            if file_path.exists():
                print(f"  📄 Analyzing {config_file}...")
                self._analyze_file(file_path)
            else:
                print(f"  ⚠️  File not found: {config_file}")
        
        print(f"📊 Found {len(self.issues)} documentation issues")
        return self.issues
    
    def _analyze_file(self, file_path: Path) -> None:
        """Analyze a single Rust file for documentation issues."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')
            
            # Look for structs, enums, and other items that need documentation
            for i, line in enumerate(lines):
                line_stripped = line.strip()
                
                # Check for structs
                if re.match(r'^pub struct \w+', line_stripped):
                    if not self._has_doc_comment(lines, i):
                        struct_name = re.search(r'pub struct (\w+)', line_stripped).group(1)
                        self.issues.append(DocumentationIssue(
                            file_path=str(file_path),
                            line_number=i + 1,
                            issue_type='struct',
                            item_name=struct_name,
                            context=line_stripped
                        ))
                
                # Check for enums
                elif re.match(r'^pub enum \w+', line_stripped):
                    if not self._has_doc_comment(lines, i):
                        enum_name = re.search(r'pub enum (\w+)', line_stripped).group(1)
                        self.issues.append(DocumentationIssue(
                            file_path=str(file_path),
                            line_number=i + 1,
                            issue_type='enum',
                            item_name=enum_name,
                            context=line_stripped
                        ))
                
                # Check for struct fields
                elif re.match(r'^\s*pub \w+:', line_stripped):
                    if not self._has_doc_comment(lines, i):
                        field_match = re.search(r'pub (\w+):', line_stripped)
                        if field_match:
                            field_name = field_match.group(1)
                            self.issues.append(DocumentationIssue(
                                file_path=str(file_path),
                                line_number=i + 1,
                                issue_type='field',
                                item_name=field_name,
                                context=line_stripped
                            ))
        
        except Exception as e:
            print(f"❌ Error analyzing {file_path}: {e}")
    
    def _has_doc_comment(self, lines: List[str], line_index: int) -> bool:
        """Check if the item at line_index has documentation comments."""
        # Look backwards for doc comments
        for i in range(line_index - 1, max(0, line_index - 10), -1):
            line = lines[i].strip()
            if line.startswith('///') or line.startswith('//!'):
                return True
            elif line.startswith('#[') or line == '':
                continue  # Skip attributes and empty lines
            else:
                break  # Stop at non-doc content
        return False
    
    def enhance_documentation(self) -> None:
        """
        Enhance documentation for all identified issues.
        """
        print("✨ Enhancing documentation to pedantic standards...")
        
        # Group issues by file for efficient processing
        issues_by_file = {}
        for issue in self.issues:
            if issue.file_path not in issues_by_file:
                issues_by_file[issue.file_path] = []
            issues_by_file[issue.file_path].append(issue)
        
        # Process each file
        for file_path, file_issues in issues_by_file.items():
            print(f"  📝 Enhancing {Path(file_path).name} ({len(file_issues)} issues)...")
            self._enhance_file_documentation(file_path, file_issues)
        
        print("✅ Documentation enhancement complete!")
    
    def _enhance_file_documentation(self, file_path: str, issues: List[DocumentationIssue]) -> None:
        """Enhance documentation for a specific file."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            # Sort issues by line number in reverse order to avoid line number shifts
            issues.sort(key=lambda x: x.line_number, reverse=True)
            
            # Add documentation for each issue
            for issue in issues:
                doc_lines = self._generate_documentation(issue)
                if doc_lines:
                    # Insert documentation before the item
                    insert_index = issue.line_number - 1
                    for doc_line in reversed(doc_lines):
                        lines.insert(insert_index, doc_line + '\n')
            
            # Write the enhanced file
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(lines)
                
        except Exception as e:
            print(f"❌ Error enhancing {file_path}: {e}")
    
    def _generate_documentation(self, issue: DocumentationIssue) -> List[str]:
        """Generate appropriate documentation for an issue."""
        if issue.issue_type in self.templates:
            return self.templates[issue.issue_type](issue)
        return []
    
    def _generate_struct_doc(self, issue: DocumentationIssue) -> List[str]:
        """Generate documentation for a struct."""
        struct_name = issue.item_name
        
        # Determine the purpose based on the struct name
        purpose = self._infer_struct_purpose(struct_name)
        
        return [
            f"/// **{struct_name}** - {purpose}",
            "///",
            f"/// This configuration struct provides settings for {purpose.lower()}.",
            "/// It implements the unified BearDog configuration interface for",
            "/// consistent validation, serialization, and environment integration.",
            "///",
            "/// # Purpose",
            f"/// - Configure {purpose.lower()} behavior and parameters",
            "/// - Provide type-safe configuration with validation",
            "/// - Support environment-specific overrides",
            "///",
            "/// # Examples",
            "/// ```rust",
            f"/// use beardog_types::canonical::config::domains_unified::{struct_name};",
            "///",
            f"/// let config = {struct_name}::default();",
            "/// // Configure specific settings as needed",
            "/// ```"
        ]
    
    def _generate_enum_doc(self, issue: DocumentationIssue) -> List[str]:
        """Generate documentation for an enum."""
        enum_name = issue.item_name
        purpose = self._infer_enum_purpose(enum_name)
        
        return [
            f"/// **{enum_name}** - {purpose}",
            "///",
            f"/// This enumeration defines the available options for {purpose.lower()}.",
            "/// Each variant represents a different configuration choice with",
            "/// specific behavior and characteristics.",
            "///",
            "/// # Variants",
            "/// See individual variant documentation for detailed descriptions.",
            "///",
            "/// # Examples",
            "/// ```rust",
            f"/// use beardog_types::canonical::config::domains_unified::{enum_name};",
            "///",
            f"/// let option = {enum_name}::default();",
            "/// ```"
        ]
    
    def _generate_field_doc(self, issue: DocumentationIssue) -> List[str]:
        """Generate documentation for a struct field."""
        field_name = issue.item_name
        description = self._infer_field_description(field_name)
        
        return [
            f"/// {description}",
            "///",
            f"/// This field controls {description.lower()} and affects the overall",
            "/// behavior of the configuration system."
        ]
    
    def _generate_variant_doc(self, issue: DocumentationIssue) -> List[str]:
        """Generate documentation for an enum variant."""
        variant_name = issue.item_name
        description = self._infer_variant_description(variant_name)
        
        return [
            f"/// {description}"
        ]
    
    def _generate_function_doc(self, issue: DocumentationIssue) -> List[str]:
        """Generate documentation for a function."""
        function_name = issue.item_name
        description = self._infer_function_description(function_name)
        
        return [
            f"/// {description}",
            "///",
            "/// # Arguments",
            "/// * Add specific argument documentation as needed",
            "///",
            "/// # Returns",
            "/// * Add return value documentation as needed",
            "///",
            "/// # Errors",
            "/// * Add error condition documentation as needed"
        ]
    
    def _infer_struct_purpose(self, struct_name: str) -> str:
        """Infer the purpose of a struct from its name."""
        name_lower = struct_name.lower()
        
        if 'config' in name_lower:
            if 'ai' in name_lower or 'ml' in name_lower:
                return "AI/ML configuration settings"
            elif 'security' in name_lower:
                return "Security configuration settings"
            elif 'adapter' in name_lower:
                return "Adapter configuration settings"
            elif 'network' in name_lower:
                return "Network configuration settings"
            elif 'monitoring' in name_lower:
                return "Monitoring configuration settings"
            else:
                return "Configuration settings"
        
        if 'optimization' in name_lower:
            return "Optimization configuration"
        elif 'validation' in name_lower:
            return "Validation configuration"
        elif 'performance' in name_lower:
            return "Performance configuration"
        else:
            return f"{struct_name} configuration"
    
    def _infer_enum_purpose(self, enum_name: str) -> str:
        """Infer the purpose of an enum from its name."""
        name_lower = enum_name.lower()
        
        if 'type' in name_lower:
            return "Type selection options"
        elif 'mode' in name_lower:
            return "Operation mode options"
        elif 'strategy' in name_lower:
            return "Strategy selection options"
        elif 'algorithm' in name_lower:
            return "Algorithm selection options"
        else:
            return f"{enum_name} options"
    
    def _infer_field_description(self, field_name: str) -> str:
        """Infer the description of a field from its name."""
        name_lower = field_name.lower()
        
        if 'enabled' in name_lower or 'enable' in name_lower:
            return f"Enable or disable {field_name.replace('enabled', '').replace('enable', '').strip('_')}"
        elif 'timeout' in name_lower:
            return f"Timeout duration for {field_name.replace('timeout', '').strip('_')} operations"
        elif 'threshold' in name_lower:
            return f"Threshold value for {field_name.replace('threshold', '').strip('_')} detection"
        elif 'rate' in name_lower:
            return f"Rate configuration for {field_name.replace('rate', '').strip('_')}"
        elif 'size' in name_lower:
            return f"Size configuration for {field_name.replace('size', '').strip('_')}"
        elif 'count' in name_lower:
            return f"Count limit for {field_name.replace('count', '').strip('_')}"
        else:
            return f"Configuration setting for {field_name.replace('_', ' ')}"
    
    def _infer_variant_description(self, variant_name: str) -> str:
        """Infer the description of an enum variant from its name."""
        return f"{variant_name} configuration option"
    
    def _infer_function_description(self, function_name: str) -> str:
        """Infer the description of a function from its name."""
        name_lower = function_name.lower()
        
        if name_lower.startswith('get_'):
            return f"Get {function_name[4:].replace('_', ' ')}"
        elif name_lower.startswith('set_'):
            return f"Set {function_name[4:].replace('_', ' ')}"
        elif name_lower.startswith('is_'):
            return f"Check if {function_name[3:].replace('_', ' ')}"
        elif name_lower.startswith('validate_'):
            return f"Validate {function_name[9:].replace('_', ' ')}"
        else:
            return f"Execute {function_name.replace('_', ' ')} operation"
    
    def generate_report(self) -> str:
        """Generate a comprehensive documentation enhancement report."""
        report = [
            "# 📚 PEDANTIC DOCUMENTATION ENHANCEMENT REPORT",
            "",
            f"**Analysis Date**: {self._get_current_timestamp()}",
            f"**Total Issues Found**: {len(self.issues)}",
            "",
            "## 📊 Issue Summary",
            ""
        ]
        
        # Count issues by type
        issue_counts = {}
        for issue in self.issues:
            issue_counts[issue.issue_type] = issue_counts.get(issue.issue_type, 0) + 1
        
        for issue_type, count in sorted(issue_counts.items()):
            report.append(f"- **{issue_type.title()}s**: {count}")
        
        report.extend([
            "",
            "## 📁 Issues by File",
            ""
        ])
        
        # Group by file
        issues_by_file = {}
        for issue in self.issues:
            file_name = Path(issue.file_path).name
            if file_name not in issues_by_file:
                issues_by_file[file_name] = []
            issues_by_file[file_name].append(issue)
        
        for file_name, file_issues in sorted(issues_by_file.items()):
            report.append(f"### {file_name}")
            report.append(f"**Issues**: {len(file_issues)}")
            report.append("")
            
            for issue in file_issues[:5]:  # Show first 5 issues
                report.append(f"- Line {issue.line_number}: {issue.issue_type} `{issue.item_name}`")
            
            if len(file_issues) > 5:
                report.append(f"- ... and {len(file_issues) - 5} more")
            
            report.append("")
        
        return "\n".join(report)
    
    def _get_current_timestamp(self) -> str:
        """Get current timestamp for reporting."""
        from datetime import datetime
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")

def main():
    """Main entry point for the pedantic documentation enhancer."""
    print("🔍 PEDANTIC DOCUMENTATION ENHANCER")
    print("=" * 50)
    
    # Get project root (assume script is in scripts/ directory)
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    # Initialize enhancer
    enhancer = PedanticDocumentationEnhancer(str(project_root))
    
    # Analyze configurations
    issues = enhancer.analyze_unified_configurations()
    
    if not issues:
        print("✅ No documentation issues found! Configuration is already pedantic-perfect.")
        return 0
    
    # Generate report
    report = enhancer.generate_report()
    
    # Save report
    report_path = project_root / "PEDANTIC_DOCUMENTATION_REPORT.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(f"📋 Report saved to: {report_path}")
    
    # Ask user if they want to enhance documentation
    response = input("\n🚀 Enhance documentation automatically? (y/N): ").strip().lower()
    
    if response in ['y', 'yes']:
        enhancer.enhance_documentation()
        print("✨ Documentation enhanced to pedantic standards!")
        return 0
    else:
        print("📋 Analysis complete. Review the report and enhance manually if desired.")
        return 0

if __name__ == "__main__":
    sys.exit(main()) 