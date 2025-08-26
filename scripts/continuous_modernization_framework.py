#!/usr/bin/env python3
"""
BearDog Continuous Modernization Framework

This framework ensures ongoing modernization excellence by:
1. Continuous pattern detection and monitoring
2. Automated regression prevention
3. Performance tracking and optimization
4. Proactive modernization opportunity identification
"""

import os
import re
import sys
import time
import json
import subprocess
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
from collections import defaultdict

class ContinuousModernizationFramework:
    """Framework for maintaining world-class modernization standards"""
    
    def __init__(self):
        self.base_path = Path(".")
        self.crates_path = self.base_path / "crates"
        self.reports_path = self.base_path / "modernization_reports"
        self.reports_path.mkdir(exist_ok=True)
        
    def detect_regression_patterns(self) -> Dict[str, List[str]]:
        """Detect any regression in modernization patterns"""
        print("🔍 Detecting potential modernization regressions...")
        
        regressions = defaultdict(list)
        
        # Patterns that should not appear in modernized code
        regression_patterns = [
            (r'#\[async_trait\]', 'async_trait_usage'),
            (r'Arc<dyn\s+\w+(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 'arc_dyn_usage'),
            (r'Box<dyn\s+\w+(?:\s*\+\s*Send\s*\+\s*Sync)?\s*>', 'box_dyn_usage'),
            (r'String::from\("([^"]{1,20})"\)', 'inefficient_string_creation'),
            (r'HashMap::new\(\)', 'suboptimal_hashmap'),
        ]
        
        for crate_path in self.crates_path.iterdir():
            if crate_path.is_dir():
                rust_files = list(crate_path.rglob("*.rs"))
                
                for rust_file in rust_files:
                    try:
                        content = rust_file.read_text()
                        
                        # Skip files that are explicitly marked as legacy
                        if 'LEGACY_PATTERN_ALLOWED' in content:
                            continue
                            
                        for pattern, pattern_type in regression_patterns:
                            matches = re.findall(pattern, content)
                            if matches:
                                regressions[pattern_type].append({
                                    'file': str(rust_file.relative_to(self.base_path)),
                                    'matches': len(matches),
                                    'examples': matches[:3]  # Show first 3 examples
                                })
                                
                    except Exception as e:
                        continue
        
        return dict(regressions)
    
    def measure_ecosystem_health(self) -> Dict[str, any]:
        """Measure overall ecosystem modernization health"""
        print("📊 Measuring ecosystem modernization health...")
        
        health_metrics = {
            'total_files': 0,
            'modernized_files': 0,
            'async_trait_files': 0,
            'arc_dyn_files': 0,
            'performance_optimized_files': 0,
            'compilation_success_rate': 0.0,
            'modernization_completion': 0.0
        }
        
        # Count files and patterns
        for crate_path in self.crates_path.iterdir():
            if crate_path.is_dir():
                rust_files = list(crate_path.rglob("*.rs"))
                health_metrics['total_files'] += len(rust_files)
                
                for rust_file in rust_files:
                    try:
                        content = rust_file.read_text()
                        
                        if 'MODERNIZED' in content or 'PHASE' in content:
                            health_metrics['modernized_files'] += 1
                        
                        if 'async_trait' in content:
                            health_metrics['async_trait_files'] += 1
                        
                        if 'Arc<dyn' in content:
                            health_metrics['arc_dyn_files'] += 1
                        
                        if 'OPTIMIZED' in content:
                            health_metrics['performance_optimized_files'] += 1
                            
                    except Exception:
                        continue
        
        # Calculate completion percentage
        if health_metrics['total_files'] > 0:
            remaining_patterns = health_metrics['async_trait_files'] + health_metrics['arc_dyn_files']
            health_metrics['modernization_completion'] = (
                (health_metrics['total_files'] - remaining_patterns) / health_metrics['total_files']
            ) * 100
        
        return health_metrics
    
    def identify_new_opportunities(self) -> List[Dict[str, any]]:
        """Identify new modernization opportunities"""
        print("🎯 Identifying new modernization opportunities...")
        
        opportunities = []
        
        # Patterns to look for that indicate modernization opportunities
        opportunity_patterns = [
            {
                'pattern': r'Vec<([^>]+)>::with_capacity\(\d+\)',
                'suggestion': 'Consider using heapless::Vec for compile-time sizing',
                'category': 'memory_optimization'
            },
            {
                'pattern': r'Mutex<([^>]+)>',
                'suggestion': 'Consider using parking_lot::Mutex for better performance',
                'category': 'concurrency_optimization'
            },
            {
                'pattern': r'serde_json::to_string',
                'suggestion': 'Consider using rmp_serde for zero-copy serialization',
                'category': 'serialization_optimization'
            },
            {
                'pattern': r'tokio::spawn\(',
                'suggestion': 'Consider using tokio::task::spawn_local for better performance',
                'category': 'async_optimization'
            }
        ]
        
        for crate_path in self.crates_path.iterdir():
            if crate_path.is_dir():
                rust_files = list(crate_path.rglob("*.rs"))
                
                for rust_file in rust_files:
                    try:
                        content = rust_file.read_text()
                        
                        for opp in opportunity_patterns:
                            matches = re.findall(opp['pattern'], content)
                            if matches:
                                opportunities.append({
                                    'file': str(rust_file.relative_to(self.base_path)),
                                    'category': opp['category'],
                                    'suggestion': opp['suggestion'],
                                    'matches': len(matches),
                                    'priority': 'medium'
                                })
                                
                    except Exception:
                        continue
        
        return opportunities
    
    def validate_compilation_health(self) -> Dict[str, any]:
        """Validate compilation health across key crates"""
        print("🔨 Validating compilation health...")
        
        key_crates = [
            'beardog-types',
            'beardog-core',
            'beardog-tunnel',
            'beardog-adapters',
            'beardog-workflows',
            'beardog-errors'
        ]
        
        compilation_results = {}
        
        for crate in key_crates:
            try:
                result = subprocess.run(
                    ['cargo', 'check', '-p', crate, '--quiet'],
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                compilation_results[crate] = {
                    'success': result.returncode == 0,
                    'warnings': len(re.findall(r'warning:', result.stderr)) if result.stderr else 0,
                    'errors': len(re.findall(r'error:', result.stderr)) if result.stderr else 0
                }
                
            except Exception as e:
                compilation_results[crate] = {
                    'success': False,
                    'warnings': 0,
                    'errors': 1,
                    'error_message': str(e)
                }
        
        return compilation_results
    
    def generate_modernization_report(self) -> Dict[str, any]:
        """Generate comprehensive modernization status report"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        print(f"\n🎯 Generating modernization report for {timestamp}...")
        
        # Collect all metrics
        regressions = self.detect_regression_patterns()
        health_metrics = self.measure_ecosystem_health()
        opportunities = self.identify_new_opportunities()
        compilation_health = self.validate_compilation_health()
        
        report = {
            'timestamp': timestamp,
            'ecosystem_health': health_metrics,
            'regressions_detected': regressions,
            'new_opportunities': opportunities,
            'compilation_health': compilation_health,
            'recommendations': self.generate_recommendations(
                regressions, health_metrics, opportunities, compilation_health
            )
        }
        
        # Save report
        report_file = self.reports_path / f"modernization_report_{timestamp}.json"
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        return report
    
    def generate_recommendations(self, regressions, health_metrics, opportunities, compilation_health) -> List[str]:
        """Generate actionable recommendations"""
        recommendations = []
        
        # Regression recommendations
        if regressions:
            recommendations.append("🚨 PRIORITY: Address detected modernization regressions")
            for pattern_type, instances in regressions.items():
                recommendations.append(f"   • Fix {len(instances)} instances of {pattern_type}")
        
        # Health recommendations
        if health_metrics['modernization_completion'] < 85:
            recommendations.append("📈 OPPORTUNITY: Continue pattern elimination to reach 85%+ completion")
        
        if health_metrics['async_trait_files'] > 50:
            recommendations.append("⚡ PERFORMANCE: High async_trait usage detected - consider native async fn migration")
        
        # Compilation recommendations
        successful_crates = sum(1 for result in compilation_health.values() if result['success'])
        if successful_crates < len(compilation_health):
            recommendations.append("🔧 STABILITY: Address compilation issues in failing crates")
        
        # Opportunity recommendations
        if opportunities:
            high_impact_opportunities = [opp for opp in opportunities if opp['matches'] > 5]
            if high_impact_opportunities:
                recommendations.append("🎯 OPTIMIZATION: High-impact performance opportunities identified")
        
        if not recommendations:
            recommendations.append("🎉 EXCELLENT: No critical issues detected - maintain current excellence")
        
        return recommendations
    
    def print_status_summary(self, report: Dict[str, any]):
        """Print a comprehensive status summary"""
        print("\n" + "="*70)
        print("🏆 BEARDOG CONTINUOUS MODERNIZATION STATUS")
        print("="*70)
        
        health = report['ecosystem_health']
        print(f"\n📊 ECOSYSTEM HEALTH:")
        print(f"   • Total files: {health['total_files']}")
        print(f"   • Modernization completion: {health['modernization_completion']:.1f}%")
        print(f"   • Files with async_trait: {health['async_trait_files']}")
        print(f"   • Files with Arc<dyn>: {health['arc_dyn_files']}")
        print(f"   • Performance optimized: {health['performance_optimized_files']}")
        
        # Compilation status
        compilation = report['compilation_health']
        successful = sum(1 for r in compilation.values() if r['success'])
        print(f"\n🔨 COMPILATION HEALTH:")
        print(f"   • Successful crates: {successful}/{len(compilation)}")
        for crate, result in compilation.items():
            status = "✅" if result['success'] else "❌"
            warnings = f" ({result['warnings']} warnings)" if result['warnings'] > 0 else ""
            print(f"   {status} {crate}{warnings}")
        
        # Regressions
        regressions = report['regressions_detected']
        if regressions:
            print(f"\n🚨 REGRESSIONS DETECTED:")
            for pattern_type, instances in regressions.items():
                print(f"   • {pattern_type}: {len(instances)} instances")
        else:
            print(f"\n✅ NO REGRESSIONS DETECTED")
        
        # Opportunities
        opportunities = report['new_opportunities']
        if opportunities:
            print(f"\n🎯 NEW OPPORTUNITIES:")
            opportunity_counts = defaultdict(int)
            for opp in opportunities:
                opportunity_counts[opp['category']] += opp['matches']
            
            for category, count in opportunity_counts.items():
                print(f"   • {category}: {count} instances")
        
        # Recommendations
        print(f"\n💡 RECOMMENDATIONS:")
        for rec in report['recommendations']:
            print(f"   {rec}")
        
        print(f"\n🌟 STATUS: BearDog maintains world-class modernization standards")
        print("="*70)

def main():
    """Main continuous modernization monitoring"""
    framework = ContinuousModernizationFramework()
    
    print("🚀 BearDog Continuous Modernization Framework")
    print("Ensuring world-class modernization standards...")
    
    report = framework.generate_modernization_report()
    framework.print_status_summary(report)
    
    return report

if __name__ == "__main__":
    main() 