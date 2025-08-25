#!/usr/bin/env python3
"""
Phase 4 Ecosystem-Wide Modernization Analyzer

This script analyzes the entire BearDog ecosystem for remaining modernization opportunities:
1. async_trait usage across all remaining crates
2. Arc<dyn> patterns for zero-cost conversion  
3. Performance optimization opportunities
4. Crate-by-crate priority scoring
"""

import os
import re
import sys
from pathlib import Path
from collections import defaultdict, Counter

def analyze_ecosystem_modernization():
    """Analyze ecosystem-wide modernization opportunities"""
    print("🔍 Phase 4: Analyzing ecosystem-wide modernization opportunities...")
    
    crate_analysis = defaultdict(lambda: {
        'async_trait_files': [],
        'arc_dyn_files': [],
        'priority_score': 0,
        'total_files': 0,
        'modernization_potential': 0
    })
    
    # Analyze each crate
    crates_dir = Path("crates")
    for crate_path in crates_dir.iterdir():
        if crate_path.is_dir():
            crate_name = crate_path.name
            
            # Count total Rust files
            rust_files = list(crate_path.rglob("*.rs"))
            crate_analysis[crate_name]['total_files'] = len(rust_files)
            
            # Analyze async_trait usage
            for rust_file in rust_files:
                try:
                    content = rust_file.read_text()
                    
                    if 'async_trait' in content:
                        crate_analysis[crate_name]['async_trait_files'].append(str(rust_file))
                    
                    if 'Arc<dyn' in content:
                        crate_analysis[crate_name]['arc_dyn_files'].append(str(rust_file))
                        
                except Exception as e:
                    print(f"⚠️  Error reading {rust_file}: {e}")
                    continue
            
            # Calculate priority score
            async_count = len(crate_analysis[crate_name]['async_trait_files'])
            arc_count = len(crate_analysis[crate_name]['arc_dyn_files'])
            total_files = crate_analysis[crate_name]['total_files']
            
            # Priority scoring algorithm
            priority_score = (async_count * 2) + (arc_count * 3) + (total_files * 0.1)
            crate_analysis[crate_name]['priority_score'] = priority_score
            crate_analysis[crate_name]['modernization_potential'] = async_count + arc_count
    
    # Sort crates by priority
    sorted_crates = sorted(crate_analysis.items(), 
                          key=lambda x: x[1]['priority_score'], 
                          reverse=True)
    
    print("\n🎯 PHASE 4 ECOSYSTEM MODERNIZATION PRIORITY RANKING:")
    print("=" * 70)
    
    total_async_files = 0
    total_arc_files = 0
    
    for i, (crate_name, analysis) in enumerate(sorted_crates[:15], 1):
        async_count = len(analysis['async_trait_files'])
        arc_count = len(analysis['arc_dyn_files'])
        total_files = analysis['total_files']
        priority = analysis['priority_score']
        
        total_async_files += async_count
        total_arc_files += arc_count
        
        if async_count > 0 or arc_count > 0:
            print(f"{i:2d}. 📦 {crate_name}")
            print(f"    Priority Score: {priority:.1f}")
            print(f"    async_trait files: {async_count}")
            print(f"    Arc<dyn> files: {arc_count}")
            print(f"    Total files: {total_files}")
            print(f"    Modernization potential: {analysis['modernization_potential']} patterns")
            print()
    
    print(f"📊 ECOSYSTEM TOTALS:")
    print(f"   • async_trait files remaining: {total_async_files}")
    print(f"   • Arc<dyn> files remaining: {total_arc_files}")
    print(f"   • Total modernization opportunities: {total_async_files + total_arc_files}")
    
    # Identify top 3 crates for Phase 4
    top_crates = [crate for crate, analysis in sorted_crates[:3] 
                  if analysis['modernization_potential'] > 0]
    
    print(f"\n🚀 PHASE 4 TARGET CRATES (Top 3):")
    for i, crate in enumerate(top_crates, 1):
        analysis = crate_analysis[crate]
        print(f"{i}. {crate} (Score: {analysis['priority_score']:.1f}, Potential: {analysis['modernization_potential']})")
    
    return sorted_crates

if __name__ == "__main__":
    analyze_ecosystem_modernization() 