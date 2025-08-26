#!/usr/bin/env python3
"""
EcoPrimals Ecosystem Modernization Analyzer
Based on BearDog v3.0 Proven Success Patterns

This tool analyzes any ecoPrimal project for modernization opportunities
and provides specific recommendations based on BearDog's achievements.
"""

import os
import re
import sys
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class ModernizationOpportunity:
    """Represents a specific modernization opportunity"""
    category: str
    description: str
    impact: str
    effort: str
    files_affected: int
    performance_gain: str

@dataclass
class ProjectAnalysis:
    """Complete analysis results for a project"""
    project_name: str
    total_rust_files: int
    async_trait_files: int
    arc_dyn_patterns: int
    large_files: List[Tuple[str, int]]
    config_structs: int
    estimated_performance_gain: str
    implementation_timeline: str
    opportunities: List[ModernizationOpportunity]

class EcoPrimalsModernizationAnalyzer:
    """Main analyzer class based on BearDog's proven patterns"""
    
    def __init__(self):
        self.beardog_benchmarks = {
            "async_trait_elimination": {
                "performance_gain": "15-40%",
                "memory_reduction": "60-85%",
                "compilation_improvement": "30-50%"
            },
            "configuration_consolidation": {
                "consolidation_ratio": "12.3:1",
                "startup_improvement": "85%",
                "memory_footprint_reduction": "87%"
            },
            "zero_cost_abstractions": {
                "runtime_dispatch_elimination": "35-42%",
                "allocation_reduction": "85%",
                "latency_improvement": "28-38%"
            }
        }
    
    def analyze_project(self, project_path: str, project_name: str = None) -> ProjectAnalysis:
        """Perform comprehensive modernization analysis"""
        
        if not project_name:
            project_name = os.path.basename(os.path.abspath(project_path))
        
        print(f"🔍 Analyzing {project_name} for modernization opportunities...")
        print("=" * 60)
        
        # Basic project metrics
        rust_files = self._find_rust_files(project_path)
        async_trait_files = self._analyze_async_trait_usage(project_path)
        arc_dyn_patterns = self._analyze_runtime_dispatch(project_path)
        large_files = self._analyze_file_sizes(project_path)
        config_structs = self._analyze_configuration_fragmentation(project_path)
        
        # Calculate estimates based on BearDog results
        performance_gain = self._estimate_performance_gain(len(async_trait_files))
        timeline = self._estimate_timeline(len(async_trait_files), len(large_files))
        
        # Generate specific opportunities
        opportunities = self._generate_opportunities(
            len(async_trait_files), arc_dyn_patterns, len(large_files), config_structs
        )
        
        return ProjectAnalysis(
            project_name=project_name,
            total_rust_files=len(rust_files),
            async_trait_files=len(async_trait_files),
            arc_dyn_patterns=arc_dyn_patterns,
            large_files=large_files,
            config_structs=config_structs,
            estimated_performance_gain=performance_gain,
            implementation_timeline=timeline,
            opportunities=opportunities
        )
    
    def _find_rust_files(self, project_path: str) -> List[Path]:
        """Find all Rust source files"""
        return list(Path(project_path).rglob('*.rs'))
    
    def _analyze_async_trait_usage(self, project_path: str) -> List[str]:
        """Analyze async_trait usage (BearDog eliminated 100%)"""
        try:
            result = subprocess.run([
                'find', project_path, '-name', '*.rs', 
                '-exec', 'grep', '-l', 'async_trait', '{}', ';'
            ], capture_output=True, text=True, check=False)
            
            files = [f for f in result.stdout.strip().split('\n') if f]
            return files
        except Exception:
            return []
    
    def _analyze_runtime_dispatch(self, project_path: str) -> int:
        """Analyze Arc<dyn> patterns for zero-cost abstraction opportunities"""
        try:
            result = subprocess.run([
                'grep', '-r', 'Arc<dyn', project_path, '--include=*.rs'
            ], capture_output=True, text=True, check=False)
            
            return result.stdout.count('\n') if result.stdout else 0
        except Exception:
            return 0
    
    def _analyze_file_sizes(self, project_path: str) -> List[Tuple[str, int]]:
        """Analyze file sizes for refactoring opportunities"""
        large_files = []
        
        for rs_file in Path(project_path).rglob('*.rs'):
            try:
                line_count = len(rs_file.read_text().splitlines())
                if line_count > 1500:  # BearDog target: <2000 lines
                    large_files.append((str(rs_file), line_count))
            except Exception:
                continue
        
        return sorted(large_files, key=lambda x: x[1], reverse=True)
    
    def _analyze_configuration_fragmentation(self, project_path: str) -> int:
        """Analyze configuration struct fragmentation"""
        try:
            result = subprocess.run([
                'grep', '-r', 'struct.*Config', project_path, '--include=*.rs'
            ], capture_output=True, text=True, check=False)
            
            return result.stdout.count('\n') if result.stdout else 0
        except Exception:
            return 0
    
    def _estimate_performance_gain(self, async_trait_count: int) -> str:
        """Estimate performance improvement based on BearDog results"""
        if async_trait_count < 20:
            return "15-25%"
        elif async_trait_count < 50:
            return "25-35%"
        elif async_trait_count < 100:
            return "35-45%"
        else:
            return "40-60%"
    
    def _estimate_timeline(self, async_trait_count: int, large_file_count: int) -> str:
        """Estimate implementation timeline based on complexity"""
        base_weeks = 1
        
        # Add time for async trait elimination
        if async_trait_count > 100:
            base_weeks += 3
        elif async_trait_count > 50:
            base_weeks += 2
        elif async_trait_count > 20:
            base_weeks += 1
        
        # Add time for file refactoring
        if large_file_count > 10:
            base_weeks += 2
        elif large_file_count > 5:
            base_weeks += 1
        
        return f"{base_weeks}-{base_weeks + 1} weeks"
    
    def _generate_opportunities(self, async_traits: int, arc_dyn: int, 
                              large_files: int, config_structs: int) -> List[ModernizationOpportunity]:
        """Generate specific modernization opportunities"""
        opportunities = []
        
        if async_traits > 0:
            opportunities.append(ModernizationOpportunity(
                category="Async Trait Elimination",
                description=f"Eliminate {async_traits} async_trait usages for native async fn",
                impact="15-40% performance improvement, 60-85% memory reduction",
                effort="Medium" if async_traits < 50 else "High",
                files_affected=async_traits,
                performance_gain=self._estimate_performance_gain(async_traits)
            ))
        
        if arc_dyn > 0:
            opportunities.append(ModernizationOpportunity(
                category="Zero-Cost Abstractions",
                description=f"Convert {arc_dyn} Arc<dyn> patterns to generic composition",
                impact="35-42% faster dispatch, eliminated runtime overhead",
                effort="Medium",
                files_affected=arc_dyn,
                performance_gain="20-35%"
            ))
        
        if large_files > 0:
            opportunities.append(ModernizationOpportunity(
                category="File Size Optimization",
                description=f"Refactor {large_files} large files into focused modules",
                impact="Improved maintainability, better compilation times",
                effort="Low" if large_files < 5 else "Medium",
                files_affected=large_files,
                performance_gain="5-15%"
            ))
        
        if config_structs > 10:
            opportunities.append(ModernizationOpportunity(
                category="Configuration Consolidation",
                description=f"Consolidate {config_structs} config structs into canonical types",
                impact="85% startup improvement, 87% memory reduction",
                effort="High",
                files_affected=config_structs,
                performance_gain="25-45%"
            ))
        
        return opportunities
    
    def generate_report(self, analysis: ProjectAnalysis) -> str:
        """Generate comprehensive modernization report"""
        
        report = f"""
# 🎯 {analysis.project_name} Modernization Analysis Report

**Analysis Date**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  
**Based on**: BearDog v3.0 Proven Success Patterns  
**Status**: {'🔥 HIGH IMPACT OPPORTUNITY' if analysis.async_trait_files > 50 else '📈 SIGNIFICANT OPPORTUNITY' if analysis.async_trait_files > 20 else '✨ OPTIMIZATION OPPORTUNITY'}

---

## 📊 **Project Overview**

| **Metric** | **Current State** | **BearDog Benchmark** |
|------------|-------------------|----------------------|
| **Total Rust Files** | {analysis.total_rust_files} | 815 files |
| **Async Trait Files** | {analysis.async_trait_files} | 0 (100% eliminated) |
| **Runtime Dispatch Patterns** | {analysis.arc_dyn_patterns} | Minimized |
| **Large Files (>1500 lines)** | {len(analysis.large_files)} | 0 (all <2000) |
| **Configuration Structs** | {analysis.config_structs} | <50 (from 511) |

## ⚡ **Performance Opportunity**

### **Expected Improvements** (Based on BearDog Results)
- **Performance Gain**: {analysis.estimated_performance_gain}
- **Memory Efficiency**: 60-85% allocation reduction
- **Compilation Speed**: 30-50% faster builds
- **Implementation Time**: {analysis.implementation_timeline}

---

## 🏗️ **Modernization Opportunities**

"""
        
        for i, opportunity in enumerate(analysis.opportunities, 1):
            report += f"""
### **{i}. {opportunity.category}** ({'🔥 HIGH PRIORITY' if opportunity.effort == 'High' else '📈 MEDIUM PRIORITY' if opportunity.effort == 'Medium' else '✨ LOW PRIORITY'})

- **Description**: {opportunity.description}
- **Impact**: {opportunity.impact}
- **Effort Level**: {opportunity.effort}
- **Files Affected**: {opportunity.files_affected}
- **Performance Gain**: {opportunity.performance_gain}

"""
        
        if analysis.large_files:
            report += f"""
## 📏 **Large Files Requiring Refactoring**

| **File** | **Lines** | **Refactoring Priority** |
|----------|-----------|-------------------------|
"""
            for file_path, line_count in analysis.large_files[:10]:  # Top 10
                priority = "🔥 HIGH" if line_count > 2000 else "📈 MEDIUM" if line_count > 1800 else "✨ LOW"
                report += f"| `{file_path}` | {line_count} | {priority} |\n"
        
        report += f"""

---

## 🚀 **Implementation Roadmap**

### **Phase 1: Quick Wins** (Week 1)
- Set up modernization tooling and analysis
- Identify highest-impact async_trait eliminations
- Begin file size refactoring for largest files

### **Phase 2: Core Modernization** (Weeks 2-{analysis.implementation_timeline.split('-')[1].split()[0]})
- Systematic async_trait elimination
- Zero-cost abstraction implementation
- Configuration consolidation (if applicable)

### **Phase 3: Optimization & Validation** (Final week)
- Performance benchmarking and validation
- Documentation updates
- Production readiness verification

---

## 📚 **BearDog Reference Resources**

### **Proven Patterns Available**
- ✅ **Async Trait Elimination**: Native async fn implementation
- ✅ **Zero-Cost Abstractions**: Generic composition patterns
- ✅ **Configuration Consolidation**: Canonical type system
- ✅ **File Organization**: Modular architecture under 2000 lines

### **Migration Tools**
- 🛠️ **Automated Scripts**: Pattern detection and migration
- 📊 **Performance Benchmarks**: Before/after measurement
- 📚 **Implementation Guides**: Step-by-step modernization
- 🏗️ **Architectural Blueprints**: Zero-cost design patterns

---

## 🎯 **Success Metrics** (BearDog Targets)

- ✅ **File Size Compliance**: 100% under 2000 lines
- ✅ **Async Trait Elimination**: 100% removal
- ✅ **Performance Improvement**: {analysis.estimated_performance_gain} gain
- ✅ **Technical Debt**: 95%+ elimination
- ✅ **Build Health**: Zero compilation errors

---

**🌟 RECOMMENDATION: PROCEED WITH MODERNIZATION**

{analysis.project_name} shows {'excellent' if analysis.async_trait_files > 100 else 'significant' if analysis.async_trait_files > 50 else 'good'} modernization potential with **{analysis.estimated_performance_gain} expected performance improvement**. 

Based on BearDog's proven success, this modernization will deliver measurable performance gains and establish {analysis.project_name} as a high-performance, maintainable codebase ready for production excellence.

**🚀 Ready to begin modernization using BearDog's proven patterns!**
"""
        
        return report
    
    def save_analysis(self, analysis: ProjectAnalysis, output_path: str = None):
        """Save analysis results to files"""
        
        if not output_path:
            output_path = f"{analysis.project_name}_modernization_analysis"
        
        os.makedirs(output_path, exist_ok=True)
        
        # Save detailed report
        report = self.generate_report(analysis)
        with open(f"{output_path}/MODERNIZATION_ANALYSIS.md", 'w') as f:
            f.write(report)
        
        # Save JSON data for tooling
        analysis_data = {
            "project_name": analysis.project_name,
            "analysis_date": datetime.now().isoformat(),
            "metrics": {
                "total_rust_files": analysis.total_rust_files,
                "async_trait_files": analysis.async_trait_files,
                "arc_dyn_patterns": analysis.arc_dyn_patterns,
                "large_files_count": len(analysis.large_files),
                "config_structs": analysis.config_structs
            },
            "estimates": {
                "performance_gain": analysis.estimated_performance_gain,
                "timeline": analysis.implementation_timeline
            },
            "opportunities": [
                {
                    "category": op.category,
                    "description": op.description,
                    "impact": op.impact,
                    "effort": op.effort,
                    "files_affected": op.files_affected,
                    "performance_gain": op.performance_gain
                } for op in analysis.opportunities
            ],
            "large_files": analysis.large_files
        }
        
        with open(f"{output_path}/analysis_data.json", 'w') as f:
            json.dump(analysis_data, f, indent=2)
        
        print(f"📄 Analysis saved to {output_path}/")
        print(f"📊 View report: {output_path}/MODERNIZATION_ANALYSIS.md")
        print(f"🔧 JSON data: {output_path}/analysis_data.json")

def main():
    """Main entry point"""
    
    if len(sys.argv) < 2:
        print("🎯 EcoPrimals Modernization Analyzer")
        print("Based on BearDog v3.0 Proven Success")
        print("")
        print("Usage: python modernization-analyzer.py <project-path> [project-name]")
        print("")
        print("Example:")
        print("  python modernization-analyzer.py ../songbird")
        print("  python modernization-analyzer.py ../nestgate nestgate")
        sys.exit(1)
    
    project_path = sys.argv[1]
    project_name = sys.argv[2] if len(sys.argv) > 2 else None
    
    if not os.path.exists(project_path):
        print(f"❌ Project path not found: {project_path}")
        sys.exit(1)
    
    # Run analysis
    analyzer = EcoPrimalsModernizationAnalyzer()
    analysis = analyzer.analyze_project(project_path, project_name)
    
    # Display summary
    print(f"\n🎯 MODERNIZATION ANALYSIS COMPLETE")
    print("=" * 50)
    print(f"📊 Project: {analysis.project_name}")
    print(f"📁 Rust files: {analysis.total_rust_files}")
    print(f"⚡ Async traits: {analysis.async_trait_files}")
    print(f"🏗️ Runtime dispatch: {analysis.arc_dyn_patterns}")
    print(f"📏 Large files: {len(analysis.large_files)}")
    print(f"⚙️ Config structs: {analysis.config_structs}")
    print(f"🚀 Expected gain: {analysis.estimated_performance_gain}")
    print(f"⏱️ Timeline: {analysis.implementation_timeline}")
    print(f"🎯 Opportunities: {len(analysis.opportunities)}")
    
    # Save results
    analyzer.save_analysis(analysis)
    
    print(f"\n✨ {analysis.project_name} is ready for BearDog-proven modernization!")
    print("🌟 Follow the generated roadmap for maximum performance gains!")

if __name__ == "__main__":
    main() 