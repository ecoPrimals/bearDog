#!/usr/bin/env python3
"""
Canonical Modernization Analyzer
Identifies fragmented code, duplicate definitions, and deprecation opportunities
"""

import re
import ast
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple, Set, Optional
from collections import defaultdict, Counter

class CanonicalModernizationAnalyzer:
    def __init__(self):
        self.crates_path = Path("crates")
        self.fragments = defaultdict(list)
        self.duplicates = defaultdict(list)
        self.deprecations = []
        self.constants = defaultdict(list)
        self.types = defaultdict(list)
        
    def analyze_codebase(self) -> Dict:
        """Comprehensive analysis of codebase for modernization opportunities"""
        print("🔍 **CANONICAL MODERNIZATION ANALYSIS**")
        print("=" * 60)
        
        results = {
            'duplicate_types': {},
            'fragmented_constants': {},
            'deprecated_patterns': [],
            'consolidation_opportunities': {},
            'modernization_priority': [],
            'statistics': {}
        }
        
        # Find all Rust files
        rust_files = list(self.crates_path.rglob("*.rs"))
        print(f"📁 Analyzing {len(rust_files)} Rust files")
        
        # Analyze each file
        for file_path in rust_files:
            try:
                self._analyze_file(file_path)
            except Exception as e:
                print(f"⚠️  Error analyzing {file_path}: {e}")
        
        # Process findings
        results['duplicate_types'] = self._identify_duplicate_types()
        results['fragmented_constants'] = self._identify_fragmented_constants()
        results['deprecated_patterns'] = self._identify_deprecated_patterns()
        results['consolidation_opportunities'] = self._identify_consolidation_opportunities()
        results['modernization_priority'] = self._calculate_modernization_priority()
        results['statistics'] = self._generate_statistics()
        
        self._generate_report(results)
        return results
    
    def _analyze_file(self, file_path: Path):
        """Analyze a single file for modernization opportunities"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception:
            return
        
        # Analyze type definitions
        self._analyze_types(file_path, content)
        
        # Analyze constants
        self._analyze_constants(file_path, content)
        
        # Analyze deprecated patterns
        self._analyze_deprecated_patterns(file_path, content)
        
        # Analyze fragments
        self._analyze_fragments(file_path, content)
    
    def _analyze_types(self, file_path: Path, content: str):
        """Analyze type definitions for duplicates and fragments"""
        
        # Find struct definitions
        struct_pattern = r'#\[derive\([^\]]*\)\]\s*(?:pub\s+)?struct\s+(\w+)\s*\{'
        for match in re.finditer(struct_pattern, content):
            struct_name = match.group(1)
            self.types[struct_name].append({
                'file': str(file_path),
                'type': 'struct',
                'line': content[:match.start()].count('\n') + 1,
                'definition': self._extract_definition(content, match)
            })
        
        # Find enum definitions
        enum_pattern = r'#\[derive\([^\]]*\)\]\s*(?:pub\s+)?enum\s+(\w+)\s*\{'
        for match in re.finditer(enum_pattern, content):
            enum_name = match.group(1)
            self.types[enum_name].append({
                'file': str(file_path),
                'type': 'enum',
                'line': content[:match.start()].count('\n') + 1,
                'definition': self._extract_definition(content, match)
            })
        
        # Find type aliases
        type_alias_pattern = r'(?:pub\s+)?type\s+(\w+)\s*=\s*([^;]+);'
        for match in re.finditer(type_alias_pattern, content):
            alias_name = match.group(1)
            alias_target = match.group(2).strip()
            self.types[alias_name].append({
                'file': str(file_path),
                'type': 'alias',
                'line': content[:match.start()].count('\n') + 1,
                'target': alias_target,
                'definition': match.group(0)
            })
    
    def _analyze_constants(self, file_path: Path, content: str):
        """Analyze constant definitions for duplicates"""
        
        # Find const declarations
        const_pattern = r'(?:pub\s+)?const\s+(\w+)\s*:\s*([^=]+)\s*=\s*([^;]+);'
        for match in re.finditer(const_pattern, content):
            const_name = match.group(1)
            const_type = match.group(2).strip()
            const_value = match.group(3).strip()
            
            self.constants[const_name].append({
                'file': str(file_path),
                'line': content[:match.start()].count('\n') + 1,
                'type': const_type,
                'value': const_value,
                'definition': match.group(0)
            })
    
    def _analyze_deprecated_patterns(self, file_path: Path, content: str):
        """Analyze deprecated patterns and technical debt"""
        
        deprecated_patterns = [
            (r'\.unwrap\(\)', 'unwrap() usage - should use safe error handling'),
            (r'\.expect\([^)]+\)', 'expect() usage - should use safe error handling'),
            (r'panic!\(', 'panic! usage - should use Result<T, E>'),
            (r'todo!\(', 'TODO marker - incomplete implementation'),
            (r'unimplemented!\(', 'unimplemented! marker - missing functionality'),
            (r'#\[allow\(dead_code\)\]', 'dead_code allowance - potential cleanup needed'),
            (r'#\[allow\(unused_variables\)\]', 'unused_variables allowance - cleanup needed'),
            (r'BearDogResult<', 'Legacy BearDogResult - should migrate to Result<T, E>'),
            (r'use\s+std::error::Error', 'Generic Error usage - should use domain-specific errors'),
        ]
        
        for pattern, description in deprecated_patterns:
            for match in re.finditer(pattern, content):
                self.deprecations.append({
                    'file': str(file_path),
                    'line': content[:match.start()].count('\n') + 1,
                    'pattern': pattern,
                    'description': description,
                    'code': self._get_context_lines(content, match.start(), 3)
                })
    
    def _analyze_fragments(self, file_path: Path, content: str):
        """Analyze code fragments that could be consolidated"""
        
        # Look for similar error handling patterns
        error_patterns = [
            r'Err\(BearDogError::\w+\([^)]+\)\)',
            r'\.map_err\(\|[^|]*\|\s*BearDogError::\w+',
            r'BearDogError::\w+\s*\{[^}]*\}',
        ]
        
        for pattern in error_patterns:
            matches = list(re.finditer(pattern, content))
            if len(matches) > 1:  # Multiple similar patterns in same file
                self.fragments['error_handling'].append({
                    'file': str(file_path),
                    'pattern': pattern,
                    'count': len(matches),
                    'lines': [content[:m.start()].count('\n') + 1 for m in matches]
                })
    
    def _identify_duplicate_types(self) -> Dict:
        """Identify duplicate type definitions"""
        duplicates = {}
        
        for type_name, definitions in self.types.items():
            if len(definitions) > 1:
                # Check if definitions are actually different
                unique_defs = set()
                for defn in definitions:
                    if defn['type'] == 'alias':
                        unique_defs.add(defn['target'])
                    else:
                        # Normalize whitespace for comparison
                        normalized = re.sub(r'\s+', ' ', defn['definition'])
                        unique_defs.add(normalized)
                
                if len(unique_defs) > 1:
                    duplicates[type_name] = {
                        'count': len(definitions),
                        'definitions': definitions,
                        'consolidation_target': self._suggest_consolidation_target(type_name, definitions)
                    }
        
        return duplicates
    
    def _identify_fragmented_constants(self) -> Dict:
        """Identify fragmented constant definitions"""
        fragmented = {}
        
        for const_name, definitions in self.constants.items():
            if len(definitions) > 1:
                # Group by value to find true duplicates
                value_groups = defaultdict(list)
                for defn in definitions:
                    value_groups[defn['value']].append(defn)
                
                if len(value_groups) > 1:  # Same name, different values
                    fragmented[const_name] = {
                        'count': len(definitions),
                        'value_groups': dict(value_groups),
                        'consolidation_needed': True
                    }
        
        return fragmented
    
    def _identify_deprecated_patterns(self) -> List:
        """Process and prioritize deprecated patterns"""
        # Group by pattern type
        pattern_groups = defaultdict(list)
        for dep in self.deprecations:
            pattern_groups[dep['description']].append(dep)
        
        # Sort by frequency and impact
        prioritized = []
        for description, items in pattern_groups.items():
            priority = self._calculate_deprecation_priority(description, len(items))
            prioritized.append({
                'description': description,
                'count': len(items),
                'priority': priority,
                'instances': items[:10]  # Limit to first 10 instances
            })
        
        return sorted(prioritized, key=lambda x: x['priority'], reverse=True)
    
    def _identify_consolidation_opportunities(self) -> Dict:
        """Identify major consolidation opportunities"""
        opportunities = {}
        
        # HSM-related consolidation
        hsm_files = [f for f in self.types.keys() if 'hsm' in f.lower() or 'Hsm' in f]
        if hsm_files:
            opportunities['hsm_types'] = {
                'description': 'HSM-related types scattered across multiple files',
                'types': hsm_files,
                'target_location': 'crates/beardog-types/src/canonical/hsm/',
                'impact': 'high'
            }
        
        # Configuration consolidation
        config_files = [f for f in self.types.keys() if 'config' in f.lower() or 'Config' in f]
        if config_files:
            opportunities['config_types'] = {
                'description': 'Configuration types fragmented across modules',
                'types': config_files,
                'target_location': 'crates/beardog-types/src/canonical/configuration/',
                'impact': 'high'
            }
        
        # Error type consolidation
        error_files = [f for f in self.types.keys() if 'error' in f.lower() or 'Error' in f]
        if error_files:
            opportunities['error_types'] = {
                'description': 'Error types should use idiomatic domain-specific errors',
                'types': error_files,
                'target_location': 'crates/beardog-errors/src/idiomatic/',
                'impact': 'critical'
            }
        
        return opportunities
    
    def _calculate_modernization_priority(self) -> List:
        """Calculate modernization priorities"""
        priorities = []
        
        # High priority: Error handling modernization
        beardog_result_count = sum(1 for dep in self.deprecations if 'BearDogResult' in dep['description'])
        if beardog_result_count > 0:
            priorities.append({
                'area': 'Error Handling Modernization',
                'priority': 'critical',
                'instances': beardog_result_count,
                'description': 'Complete migration to idiomatic Result<T, E> patterns',
                'estimated_effort': 'high'
            })
        
        # Medium priority: Type consolidation
        duplicate_count = len(self._identify_duplicate_types())
        if duplicate_count > 5:
            priorities.append({
                'area': 'Type Consolidation',
                'priority': 'high',
                'instances': duplicate_count,
                'description': 'Consolidate duplicate type definitions',
                'estimated_effort': 'medium'
            })
        
        # Low priority: Code cleanup
        cleanup_count = sum(1 for dep in self.deprecations if any(x in dep['description'] 
                           for x in ['unwrap', 'todo', 'dead_code']))
        if cleanup_count > 0:
            priorities.append({
                'area': 'Code Cleanup',
                'priority': 'medium',
                'instances': cleanup_count,
                'description': 'Remove deprecated patterns and technical debt',
                'estimated_effort': 'low'
            })
        
        return sorted(priorities, key=lambda x: {'critical': 3, 'high': 2, 'medium': 1}[x['priority']], reverse=True)
    
    def _suggest_consolidation_target(self, type_name: str, definitions: List) -> str:
        """Suggest best location for consolidated type"""
        
        # Prefer canonical locations
        canonical_files = [d for d in definitions if 'canonical' in d['file']]
        if canonical_files:
            return canonical_files[0]['file']
        
        # Prefer beardog-types locations
        types_files = [d for d in definitions if 'beardog-types' in d['file']]
        if types_files:
            return types_files[0]['file']
        
        # Default to first definition
        return definitions[0]['file']
    
    def _calculate_deprecation_priority(self, description: str, count: int) -> int:
        """Calculate priority score for deprecation cleanup"""
        base_score = count
        
        # High priority patterns
        if any(x in description.lower() for x in ['beardog_result', 'unwrap', 'panic']):
            base_score *= 3
        
        # Medium priority patterns  
        elif any(x in description.lower() for x in ['todo', 'unimplemented']):
            base_score *= 2
        
        return base_score
    
    def _extract_definition(self, content: str, match) -> str:
        """Extract full definition from match"""
        start = match.start()
        
        # Find the opening brace
        brace_pos = content.find('{', start)
        if brace_pos == -1:
            return match.group(0)
        
        # Count braces to find the end
        brace_count = 0
        pos = brace_pos
        while pos < len(content):
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
                if brace_count == 0:
                    return content[start:pos+1]
            pos += 1
        
        return match.group(0)
    
    def _get_context_lines(self, content: str, pos: int, context: int) -> List[str]:
        """Get context lines around a position"""
        lines = content[:pos].split('\n')
        line_num = len(lines) - 1
        all_lines = content.split('\n')
        
        start = max(0, line_num - context)
        end = min(len(all_lines), line_num + context + 1)
        
        return all_lines[start:end]
    
    def _generate_statistics(self) -> Dict:
        """Generate analysis statistics"""
        return {
            'total_types_analyzed': len(self.types),
            'duplicate_types_found': len([t for t in self.types.values() if len(t) > 1]),
            'total_constants_analyzed': len(self.constants),
            'fragmented_constants': len([c for c in self.constants.values() if len(c) > 1]),
            'deprecated_patterns_found': len(self.deprecations),
            'files_analyzed': len(set(dep['file'] for dep in self.deprecations))
        }
    
    def _generate_report(self, results: Dict):
        """Generate comprehensive modernization report"""
        print(f"\n📊 **MODERNIZATION ANALYSIS RESULTS**")
        print("=" * 50)
        
        stats = results['statistics']
        print(f"Types Analyzed: {stats['total_types_analyzed']}")
        print(f"Duplicate Types: {stats['duplicate_types_found']}")
        print(f"Constants Analyzed: {stats['total_constants_analyzed']}")
        print(f"Fragmented Constants: {stats['fragmented_constants']}")
        print(f"Deprecated Patterns: {stats['deprecated_patterns_found']}")
        print(f"Files Requiring Attention: {stats['files_analyzed']}")
        
        print(f"\n🎯 **TOP MODERNIZATION PRIORITIES**")
        for priority in results['modernization_priority'][:3]:
            print(f"  {priority['priority'].upper()}: {priority['area']} ({priority['instances']} instances)")
        
        print(f"\n🔧 **CONSOLIDATION OPPORTUNITIES**")
        for name, opp in results['consolidation_opportunities'].items():
            print(f"  {opp['impact'].upper()}: {opp['description']}")
        
        print(f"\n✅ **ANALYSIS COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Canonical Modernization Analysis')
    parser.add_argument('--output', help='Output file for analysis results')
    parser.add_argument('--format', choices=['json', 'markdown'], default='json', help='Output format')
    
    args = parser.parse_args()
    
    analyzer = CanonicalModernizationAnalyzer()
    results = analyzer.analyze_codebase()
    
    if args.output:
        if args.format == 'json':
            with open(args.output, 'w') as f:
                json.dump(results, f, indent=2)
        else:
            # Generate markdown report
            with open(args.output, 'w') as f:
                f.write("# Canonical Modernization Analysis Report\n\n")
                f.write(f"**Generated**: {results.get('timestamp', 'Unknown')}\n\n")
                # Add detailed markdown formatting here
        
        print(f"📄 Analysis report saved to: {args.output}")
    
    return 0

if __name__ == "__main__":
    exit(main()) 