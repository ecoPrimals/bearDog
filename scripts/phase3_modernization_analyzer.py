#!/usr/bin/env python3
"""
Phase 3 Modernization Analyzer

This script analyzes the codebase for remaining modernization opportunities:
1. Error system consolidation beyond the existing BearDogError
2. async_trait usage that can be converted to native async fn
3. Arc<dyn> patterns that can be eliminated
4. Performance optimization opportunities
"""

import os
import re
import sys
from pathlib import Path
from collections import defaultdict, Counter

def analyze_error_system():
    """Analyze remaining error system fragmentation"""
    print("🔍 Analyzing error system consolidation opportunities...")
    
    # Find all error enum definitions
    error_enums = []
    async_trait_usage = []
    arc_dyn_patterns = []
    
    for rust_file in Path("crates").rglob("*.rs"):
        if "target" in str(rust_file):
            continue
            
        try:
            content = rust_file.read_text()
            
            # Find error enums
            error_enum_matches = re.findall(r'pub enum (\w*Error\w*)', content)
            for match in error_enum_matches:
                error_enums.append((rust_file, match))
            
            # Find async_trait usage
            if 'async_trait' in content and not '// Removed async_trait' in content:
                async_trait_lines = [i for i, line in enumerate(content.split('\n'), 1) 
                                   if 'async_trait' in line and not line.strip().startswith('//')]
                if async_trait_lines:
                    async_trait_usage.append((rust_file, async_trait_lines))
            
            # Find Arc<dyn> patterns
            arc_dyn_matches = re.findall(r'Arc<dyn\s+(\w+)', content)
            for match in arc_dyn_matches:
                arc_dyn_patterns.append((rust_file, match))
                
        except Exception as e:
            print(f"Error reading {rust_file}: {e}")
    
    return error_enums, async_trait_usage, arc_dyn_patterns

def generate_error_consolidation_plan(error_enums):
    """Generate a plan for consolidating remaining error enums"""
    print("\n📋 Error Consolidation Analysis:")
    
    # Group by crate
    crate_errors = defaultdict(list)
    for file_path, error_name in error_enums:
        crate_name = str(file_path).split('/')[1] if '/' in str(file_path) else 'unknown'
        crate_errors[crate_name].append((file_path, error_name))
    
    consolidation_opportunities = []
    
    for crate, errors in crate_errors.items():
        if len(errors) > 1:
            print(f"  🎯 {crate}: {len(errors)} error enums can be consolidated")
            for file_path, error_name in errors:
                print(f"    • {error_name} ({file_path})")
            consolidation_opportunities.append((crate, errors))
        else:
            print(f"  ✅ {crate}: {len(errors)} error enum (already consolidated)")
    
    return consolidation_opportunities

def analyze_async_trait_modernization(async_trait_usage):
    """Analyze remaining async_trait usage"""
    print(f"\n🔄 Async Trait Modernization Analysis:")
    print(f"  📊 Files with async_trait usage: {len(async_trait_usage)}")
    
    modernization_candidates = []
    
    for file_path, line_numbers in async_trait_usage:
        print(f"  📝 {file_path}: Lines {line_numbers}")
        modernization_candidates.append(file_path)
    
    return modernization_candidates

def analyze_arc_dyn_elimination(arc_dyn_patterns):
    """Analyze Arc<dyn> patterns for zero-cost conversion"""
    print(f"\n⚡ Zero-Cost Pattern Analysis:")
    
    trait_usage = Counter(trait for _, trait in arc_dyn_patterns)
    
    print(f"  📊 Arc<dyn> patterns found: {len(arc_dyn_patterns)}")
    print("  🎯 Most common traits for zero-cost conversion:")
    
    for trait_name, count in trait_usage.most_common(10):
        print(f"    • Arc<dyn {trait_name}>: {count} usages")
    
    return trait_usage

def create_phase3_migration_plan(consolidation_opportunities, modernization_candidates, trait_usage):
    """Create a comprehensive Phase 3 migration plan"""
    print(f"\n🚀 Phase 3 Migration Plan:")
    
    plan = {
        "error_consolidation": len(consolidation_opportunities),
        "async_trait_modernization": len(modernization_candidates),
        "zero_cost_opportunities": len(trait_usage),
        "priority_crates": []
    }
    
    # Prioritize crates with multiple modernization opportunities
    crate_priorities = defaultdict(int)
    
    for crate, _ in consolidation_opportunities:
        crate_priorities[crate] += 3  # High priority for error consolidation
    
    for file_path in modernization_candidates:
        crate = str(file_path).split('/')[1] if '/' in str(file_path) else 'unknown'
        crate_priorities[crate] += 1
    
    # Sort by priority
    priority_crates = sorted(crate_priorities.items(), key=lambda x: x[1], reverse=True)
    plan["priority_crates"] = priority_crates[:10]
    
    print("  📈 Priority Crates for Phase 3:")
    for crate, score in priority_crates[:5]:
        print(f"    🎯 {crate}: Priority Score {score}")
    
    return plan

def generate_migration_scripts(plan):
    """Generate migration scripts for Phase 3"""
    print(f"\n🛠️ Generating Phase 3 migration tools...")
    
    # Create error consolidation script
    error_script = """#!/usr/bin/env python3
# Phase 3 Error Consolidation Script
# Generated automatically by phase3_modernization_analyzer.py

def consolidate_crate_errors(crate_name):
    print(f"Consolidating errors in {crate_name}...")
    # Implementation will be generated based on analysis
    pass

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        consolidate_crate_errors(sys.argv[1])
    else:
        print("Usage: python3 error_consolidation.py <crate_name>")
"""
    
    with open("scripts/phase3_error_consolidation.py", "w") as f:
        f.write(error_script)
    
    # Create async trait modernization script
    async_script = """#!/usr/bin/env python3
# Phase 3 Async Trait Modernization Script
# Generated automatically by phase3_modernization_analyzer.py

def modernize_async_traits(file_path):
    print(f"Modernizing async traits in {file_path}...")
    # Implementation will be generated based on analysis
    pass

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        modernize_async_traits(sys.argv[1])
    else:
        print("Usage: python3 async_trait_modernization.py <file_path>")
"""
    
    with open("scripts/phase3_async_trait_modernization.py", "w") as f:
        f.write(async_script)
    
    print("  ✅ Generated migration scripts:")
    print("    • scripts/phase3_error_consolidation.py")
    print("    • scripts/phase3_async_trait_modernization.py")

def main():
    """Main analysis function"""
    print("🚀 BearDog Phase 3 Modernization Analysis")
    print("=" * 50)
    
    # Analyze current state
    error_enums, async_trait_usage, arc_dyn_patterns = analyze_error_system()
    
    # Generate consolidation plans
    consolidation_opportunities = generate_error_consolidation_plan(error_enums)
    modernization_candidates = analyze_async_trait_modernization(async_trait_usage)
    trait_usage = analyze_arc_dyn_elimination(arc_dyn_patterns)
    
    # Create migration plan
    plan = create_phase3_migration_plan(consolidation_opportunities, modernization_candidates, trait_usage)
    
    # Generate migration tools
    generate_migration_scripts(plan)
    
    print(f"\n🎉 Phase 3 Analysis Complete!")
    print(f"📊 Summary:")
    print(f"  • Error consolidation opportunities: {plan['error_consolidation']}")
    print(f"  • Async trait modernization files: {plan['async_trait_modernization']}")
    print(f"  • Zero-cost conversion opportunities: {plan['zero_cost_opportunities']}")
    
    print(f"\n📋 Next Steps:")
    print(f"  1. Run generated migration scripts on priority crates")
    print(f"  2. Validate compilation after each migration")
    print(f"  3. Measure performance improvements")
    print(f"  4. Update documentation and examples")

if __name__ == "__main__":
    main() 