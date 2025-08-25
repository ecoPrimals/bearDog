#!/usr/bin/env python3
"""
BearDog Idiomatic Migration Analyzer

**WEEK 2 IMPLEMENTATION** - Automated migration analysis and reporting

This script analyzes BearDogResult usage patterns across the codebase and generates
comprehensive migration reports for the idiomatic Result<T, E> evolution.
"""

import os
import re
import json
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass, asdict
from collections import defaultdict

@dataclass
class BearDogResultUsage:
    """Single BearDogResult usage analysis"""
    file_path: str
    line_number: int
    line_content: str
    function_name: Optional[str]
    suggested_domain: str
    confidence: str
    context_keywords: List[str]

@dataclass
class FileAnalysis:
    """Analysis results for a single file"""
    file_path: str
    beardog_result_count: int
    usages: List[BearDogResultUsage]
    suggested_domain: str
    migration_confidence: str
    complexity_score: int

@dataclass
class ModuleAnalysis:
    """Analysis results for a module/crate"""
    module_name: str
    total_files: int
    total_usages: int
    files: List[FileAnalysis]
    domain_distribution: Dict[str, int]
    migration_plan: Dict[str, List[str]]

class IdiomaticMigrationAnalyzer:
    """Comprehensive analyzer for BearDog idiomatic migration"""
    
    def __init__(self):
        # Domain inference patterns
        self.domain_patterns = {
            'security': [
                r'(?i)(auth|login|password|credential|encrypt|decrypt|crypto|hsm|key|security|threat)',
                r'(?i)(certificate|signature|token|session|permission|authorization)',
            ],
            'genetics': [
                r'(?i)(spawn|genetic|lineage|diversity|breed|mutation|evolution)',
                r'(?i)(primal|variant|generation|crossbreed)',
            ],
            'network': [
                r'(?i)(connect|network|endpoint|request|response|http|tcp|udp)',
                r'(?i)(timeout|latency|bandwidth|connection)',
            ],
            'workflow': [
                r'(?i)(workflow|process|approval|execute|step|pipeline)',
                r'(?i)(orchestration|coordination|execution)',
            ],
        }
        
        # Confidence assessment patterns
        self.high_confidence_patterns = [
            r'fn\s+\w*auth\w*.*BearDogResult',
            r'fn\s+\w*encrypt\w*.*BearDogResult',
            r'fn\s+\w*spawn\w*.*BearDogResult',
            r'fn\s+\w*hsm\w*.*BearDogResult',
        ]
        
        self.complexity_indicators = [
            r'match.*BearDogError::',
            r'\.map_err\(',
            r'impl.*From.*BearDogError',
            r'Box<dyn.*Error>',
        ]
    
    def analyze_file(self, file_path: Path) -> FileAnalysis:
        """Analyze a single Rust file for BearDogResult usage"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"Warning: Could not read {file_path}: {e}")
            return FileAnalysis(
                file_path=str(file_path),
                beardog_result_count=0,
                usages=[],
                suggested_domain="unknown",
                migration_confidence="low",
                complexity_score=0
            )
        
        lines = content.split('\n')
        usages = []
        
        # Find all BearDogResult usages
        for line_num, line in enumerate(lines, 1):
            if 'BearDogResult' in line:
                # Extract function context
                function_name = self._extract_function_name(lines, line_num - 1)
                
                # Determine domain from context
                context_window = self._get_context_window(lines, line_num - 1, 5)
                suggested_domain = self._infer_domain(context_window)
                
                # Assess confidence
                confidence = self._assess_confidence(line, context_window)
                
                # Extract keywords
                keywords = self._extract_keywords(context_window)
                
                usages.append(BearDogResultUsage(
                    file_path=str(file_path),
                    line_number=line_num,
                    line_content=line.strip(),
                    function_name=function_name,
                    suggested_domain=suggested_domain,
                    confidence=confidence,
                    context_keywords=keywords
                ))
        
        # Calculate complexity score
        complexity_score = sum(1 for pattern in self.complexity_indicators 
                             if re.search(pattern, content))
        
        # Determine overall file domain and confidence
        domain_counts = defaultdict(int)
        confidence_counts = defaultdict(int)
        
        for usage in usages:
            domain_counts[usage.suggested_domain] += 1
            confidence_counts[usage.confidence] += 1
        
        # Most common domain wins
        suggested_domain = max(domain_counts.items(), key=lambda x: x[1])[0] if domain_counts else "unknown"
        
        # Overall confidence assessment
        total_usages = len(usages)
        high_conf = confidence_counts.get('high', 0)
        
        if total_usages == 0:
            migration_confidence = "none"
        elif high_conf / total_usages > 0.8:
            migration_confidence = "high"
        elif high_conf / total_usages > 0.5:
            migration_confidence = "medium"
        else:
            migration_confidence = "low"
        
        return FileAnalysis(
            file_path=str(file_path),
            beardog_result_count=len(usages),
            usages=usages,
            suggested_domain=suggested_domain,
            migration_confidence=migration_confidence,
            complexity_score=complexity_score
        )
    
    def analyze_module(self, module_path: Path) -> ModuleAnalysis:
        """Analyze an entire module/crate for migration opportunities"""
        rust_files = list(module_path.rglob("*.rs"))
        file_analyses = []
        
        print(f"📊 Analyzing module: {module_path.name}")
        print(f"   Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            analysis = self.analyze_file(file_path)
            if analysis.beardog_result_count > 0:
                file_analyses.append(analysis)
        
        # Calculate module statistics
        total_usages = sum(f.beardog_result_count for f in file_analyses)
        domain_distribution = defaultdict(int)
        
        for file_analysis in file_analyses:
            domain_distribution[file_analysis.suggested_domain] += file_analysis.beardog_result_count
        
        # Create migration plan
        migration_plan = {
            'high_confidence': [f.file_path for f in file_analyses if f.migration_confidence == 'high'],
            'medium_confidence': [f.file_path for f in file_analyses if f.migration_confidence == 'medium'],
            'manual_review': [f.file_path for f in file_analyses if f.migration_confidence == 'low'],
        }
        
        return ModuleAnalysis(
            module_name=module_path.name,
            total_files=len(file_analyses),
            total_usages=total_usages,
            files=file_analyses,
            domain_distribution=dict(domain_distribution),
            migration_plan=migration_plan
        )
    
    def _extract_function_name(self, lines: List[str], line_index: int) -> Optional[str]:
        """Extract function name from context"""
        # Look backward for function definition
        for i in range(max(0, line_index - 10), line_index + 1):
            if i < len(lines):
                match = re.search(r'fn\s+(\w+)', lines[i])
                if match:
                    return match.group(1)
        return None
    
    def _get_context_window(self, lines: List[str], center: int, window_size: int) -> str:
        """Get context window around a line"""
        start = max(0, center - window_size)
        end = min(len(lines), center + window_size + 1)
        return ' '.join(lines[start:end])
    
    def _infer_domain(self, context: str) -> str:
        """Infer domain type from context"""
        for domain, patterns in self.domain_patterns.items():
            for pattern in patterns:
                if re.search(pattern, context):
                    return domain
        return "unknown"
    
    def _assess_confidence(self, line: str, context: str) -> str:
        """Assess migration confidence level"""
        # High confidence for clear patterns
        for pattern in self.high_confidence_patterns:
            if re.search(pattern, context):
                return "high"
        
        # Medium confidence for function returns
        if re.search(r'fn.*->.*BearDogResult', line):
            return "medium"
        
        # Low confidence for complex usage
        if any(re.search(pattern, context) for pattern in self.complexity_indicators):
            return "low"
        
        return "medium"
    
    def _extract_keywords(self, context: str) -> List[str]:
        """Extract relevant keywords from context"""
        keywords = []
        keyword_patterns = [
            r'\b(auth\w*)', r'\b(encrypt\w*)', r'\b(spawn\w*)', r'\b(hsm\w*)',
            r'\b(key\w*)', r'\b(security\w*)', r'\b(genetic\w*)', r'\b(network\w*)',
            r'\b(workflow\w*)', r'\b(crypto\w*)', r'\b(threat\w*)',
        ]
        
        for pattern in keyword_patterns:
            matches = re.findall(pattern, context, re.IGNORECASE)
            keywords.extend(matches)
        
        return list(set(keywords))
    
    def generate_migration_report(self, analysis: ModuleAnalysis, output_path: Path):
        """Generate comprehensive migration report"""
        report = {
            'module': analysis.module_name,
            'summary': {
                'total_files': analysis.total_files,
                'total_usages': analysis.total_usages,
                'domain_distribution': analysis.domain_distribution,
            },
            'migration_plan': analysis.migration_plan,
            'detailed_analysis': [asdict(f) for f in analysis.files],
            'recommendations': self._generate_recommendations(analysis),
        }
        
        with open(output_path, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"📋 Migration report generated: {output_path}")

    def _generate_recommendations(self, analysis: ModuleAnalysis) -> Dict:
        """Generate migration recommendations"""
        total_usages = analysis.total_usages
        high_conf_files = len(analysis.migration_plan['high_confidence'])
        medium_conf_files = len(analysis.migration_plan['medium_confidence'])
        manual_files = len(analysis.migration_plan['manual_review'])
        
        # Effort estimation (hours)
        automated_effort = high_conf_files * 0.5
        review_effort = medium_conf_files * 2.0
        manual_effort = manual_files * 4.0
        total_effort = automated_effort + review_effort + manual_effort
        
        return {
            'migration_strategy': 'phased_approach',
            'recommended_order': [
                'high_confidence_automated',
                'medium_confidence_reviewed', 
                'manual_migration'
            ],
            'effort_estimation': {
                'automated_hours': automated_effort,
                'review_hours': review_effort,
                'manual_hours': manual_effort,
                'total_hours': total_effort,
            },
            'pilot_readiness': 'ready' if high_conf_files > 0 else 'needs_preparation',
            'primary_domain': max(analysis.domain_distribution.items(), key=lambda x: x[1])[0] if analysis.domain_distribution else 'unknown',
        }

def main():
    parser = argparse.ArgumentParser(description='Analyze BearDog modules for idiomatic migration')
    parser.add_argument('module_path', help='Path to the module to analyze')
    parser.add_argument('--output', '-o', help='Output file for the report', default='migration_report.json')
    parser.add_argument('--verbose', '-v', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    analyzer = IdiomaticMigrationAnalyzer()
    module_path = Path(args.module_path)
    
    if not module_path.exists():
        print(f"❌ Error: Module path {module_path} does not exist")
        return 1
    
    print(f"🚀 Starting migration analysis for: {module_path}")
    print("=" * 60)
    
    # Analyze the module
    analysis = analyzer.analyze_module(module_path)
    
    # Display summary
    print(f"\n📊 **ANALYSIS SUMMARY**")
    print(f"Module: {analysis.module_name}")
    print(f"Files with BearDogResult: {analysis.total_files}")
    print(f"Total BearDogResult usages: {analysis.total_usages}")
    print(f"\n🎯 **DOMAIN DISTRIBUTION**")
    for domain, count in analysis.domain_distribution.items():
        percentage = (count / analysis.total_usages) * 100 if analysis.total_usages > 0 else 0
        print(f"  {domain}: {count} usages ({percentage:.1f}%)")
    
    print(f"\n🚀 **MIGRATION PLAN**")
    print(f"High Confidence (automated): {len(analysis.migration_plan['high_confidence'])} files")
    print(f"Medium Confidence (review): {len(analysis.migration_plan['medium_confidence'])} files") 
    print(f"Manual Review Required: {len(analysis.migration_plan['manual_review'])} files")
    
    # Generate detailed report
    output_path = Path(args.output)
    analyzer.generate_migration_report(analysis, output_path)
    
    # Verbose output
    if args.verbose:
        print(f"\n🔍 **DETAILED FILE ANALYSIS**")
        for file_analysis in analysis.files[:5]:  # Show first 5 files
            print(f"\nFile: {file_analysis.file_path}")
            print(f"  Usages: {file_analysis.beardog_result_count}")
            print(f"  Domain: {file_analysis.suggested_domain}")
            print(f"  Confidence: {file_analysis.migration_confidence}")
            print(f"  Complexity: {file_analysis.complexity_score}")
    
    print(f"\n🎉 **ANALYSIS COMPLETE**")
    print(f"Report saved to: {output_path}")
    
    return 0

if __name__ == "__main__":
    exit(main()) 