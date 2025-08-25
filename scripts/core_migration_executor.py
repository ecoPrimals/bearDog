#!/usr/bin/env python3
"""
BearDog Core Module Migration Executor
Handles mixed-domain migration with sophisticated domain inference and validation
"""

import re
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple

class CoreMigrationExecutor:
    def __init__(self):
        self.core_path = Path("crates/beardog-core")
        self.domain_confidence_threshold = 0.7
        self.mixed_domain_patterns = self._initialize_mixed_patterns()
        self.migration_stats = {
            'functions_analyzed': 0,
            'high_confidence_migrations': 0,
            'medium_confidence_migrations': 0,
            'manual_review_required': 0,
            'domain_distribution': {}
        }
    
    def _initialize_mixed_patterns(self) -> Dict:
        """Initialize patterns for mixed-domain detection"""
        return {
            'network_security': {
                'patterns': [
                    r'(authenticate|verify|validate).*?(connection|network|socket)',
                    r'(encrypt|decrypt).*?(transmission|transfer|network)',
                    r'(secure|tls|ssl).*?(channel|connection|stream)'
                ],
                'target_error': 'NetworkSecurityError',
                'confidence_boost': 0.2
            },
            'genetics_security': {
                'patterns': [
                    r'(spawn|genetic).*?(auth|secure|validate)',
                    r'(lineage|breed).*?(permission|access|auth)',
                    r'(genetic).*?(signature|verify|validate)'
                ],
                'target_error': 'GeneticsSecurityError', 
                'confidence_boost': 0.15
            },
            'workflow_network': {
                'patterns': [
                    r'(workflow|process).*?(network|connection|api)',
                    r'(execute|run).*?(remote|distributed|cluster)',
                    r'(task|job).*?(network|transmission|sync)'
                ],
                'target_error': 'WorkflowNetworkError',
                'confidence_boost': 0.15
            }
        }
    
    def execute_core_migration(self, dry_run: bool = False) -> Dict:
        """Execute sophisticated core module migration"""
        print("🏗️  **BEARDOG-CORE MIXED-DOMAIN MIGRATION**")
        print("=" * 60)
        
        if not self.core_path.exists():
            raise FileNotFoundError(f"Core module path {self.core_path} not found")
        
        # Load previous analysis
        analysis_file = Path("core_phase2_analysis.json")
        if analysis_file.exists():
            with open(analysis_file, 'r') as f:
                previous_analysis = json.load(f)
            print(f"📊 Loaded previous analysis: {previous_analysis.get('total_functions', 0)} functions")
        else:
            print("⚠️  No previous analysis found, proceeding with discovery")
            previous_analysis = {}
        
        # Find all Rust files
        rust_files = list(self.core_path.rglob("*.rs"))
        print(f"📁 Found {len(rust_files)} Rust files in core module")
        
        results = {
            'files_processed': 0,
            'functions_migrated': 0,
            'high_confidence_functions': [],
            'medium_confidence_functions': [],
            'manual_review_functions': [],
            'domain_specific_migrations': {},
            'migration_summary': {}
        }
        
        for file_path in rust_files:
            try:
                file_result = self._migrate_core_file(file_path, dry_run)
                results['files_processed'] += 1
                
                # Aggregate results
                results['functions_migrated'] += file_result['functions_migrated']
                results['high_confidence_functions'].extend(file_result.get('high_confidence', []))
                results['medium_confidence_functions'].extend(file_result.get('medium_confidence', []))
                results['manual_review_functions'].extend(file_result.get('manual_review', []))
                
                # Track domain-specific migrations
                for domain, functions in file_result.get('domain_migrations', {}).items():
                    if domain not in results['domain_specific_migrations']:
                        results['domain_specific_migrations'][domain] = []
                    results['domain_specific_migrations'][domain].extend(functions)
                
            except Exception as e:
                print(f"❌ Error processing {file_path}: {e}")
        
        self._generate_core_migration_report(results)
        return results
    
    def _migrate_core_file(self, file_path: Path, dry_run: bool) -> Dict:
        """Migrate a single core file with sophisticated domain analysis"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
        except Exception as e:
            return {'error': str(e), 'functions_migrated': 0}
        
        original_content = content
        file_result = {
            'file': str(file_path),
            'functions_migrated': 0,
            'high_confidence': [],
            'medium_confidence': [],
            'manual_review': [],
            'domain_migrations': {}
        }
        
        # Find all function signatures with BearDogResult
        function_pattern = r'fn\s+(\w+)[^{]*->\s*BearDogResult<([^>]+)>'
        functions = list(re.finditer(function_pattern, content))
        
        for func_match in functions:
            function_name = func_match.group(1)
            return_type = func_match.group(2)
            
            # Analyze domain confidence
            domain_analysis = self._analyze_function_domain(
                function_name, 
                func_match.group(0), 
                content,
                file_path
            )
            
            migration_plan = self._create_migration_plan(
                function_name,
                return_type,
                domain_analysis
            )
            
            if dry_run:
                print(f"  📋 DRY RUN: Would migrate {function_name} → {migration_plan['target_result_type']}")
                print(f"      Domain: {migration_plan['primary_domain']} (confidence: {migration_plan['confidence']:.2f})")
            else:
                # Execute migration
                success = self._execute_function_migration(
                    content, func_match, migration_plan, file_path
                )
                
                if success:
                    file_result['functions_migrated'] += 1
                    print(f"  🔄 Migrated {function_name} → {migration_plan['target_result_type']}")
            
            # Categorize by confidence
            if migration_plan['confidence'] >= 0.8:
                file_result['high_confidence'].append({
                    'function': function_name,
                    'domain': migration_plan['primary_domain'],
                    'confidence': migration_plan['confidence']
                })
            elif migration_plan['confidence'] >= 0.5:
                file_result['medium_confidence'].append({
                    'function': function_name,
                    'domain': migration_plan['primary_domain'],
                    'confidence': migration_plan['confidence']
                })
            else:
                file_result['manual_review'].append({
                    'function': function_name,
                    'reason': 'Low confidence domain detection',
                    'confidence': migration_plan['confidence']
                })
            
            # Track domain migrations
            domain = migration_plan['primary_domain']
            if domain not in file_result['domain_migrations']:
                file_result['domain_migrations'][domain] = []
            file_result['domain_migrations'][domain].append(function_name)
        
        return file_result
    
    def _analyze_function_domain(self, func_name: str, func_signature: str, file_content: str, file_path: Path) -> Dict:
        """Sophisticated domain analysis with mixed-domain detection"""
        
        # Base domain scores
        domain_scores = {
            'security': 0.0,
            'network': 0.0,
            'genetics': 0.0,
            'workflow': 0.0,
            'system': 0.0
        }
        
        # Analyze function name
        name_lower = func_name.lower()
        
        # Security indicators
        security_keywords = ['auth', 'encrypt', 'decrypt', 'secure', 'verify', 'validate', 'permission', 'access', 'token', 'key', 'sign', 'hash']
        domain_scores['security'] += sum(0.1 for kw in security_keywords if kw in name_lower)
        
        # Network indicators
        network_keywords = ['connect', 'send', 'recv', 'socket', 'http', 'tcp', 'udp', 'api', 'request', 'response', 'client', 'server']
        domain_scores['network'] += sum(0.1 for kw in network_keywords if kw in name_lower)
        
        # Genetics indicators
        genetics_keywords = ['spawn', 'genetic', 'lineage', 'breed', 'evolve', 'mutate', 'diversity', 'generation']
        domain_scores['genetics'] += sum(0.1 for kw in genetics_keywords if kw in name_lower)
        
        # Workflow indicators
        workflow_keywords = ['execute', 'process', 'workflow', 'task', 'job', 'run', 'schedule', 'queue']
        domain_scores['workflow'] += sum(0.1 for kw in workflow_keywords if kw in name_lower)
        
        # System indicators
        system_keywords = ['config', 'init', 'setup', 'start', 'stop', 'status', 'health', 'monitor']
        domain_scores['system'] += sum(0.1 for kw in system_keywords if kw in name_lower)
        
        # Analyze function context (surrounding code)
        func_context = self._extract_function_context(file_content, func_signature)
        
        # Check for mixed-domain patterns
        mixed_domain_boost = 0.0
        detected_mixed_domain = None
        
        for pattern_name, pattern_info in self.mixed_domain_patterns.items():
            for pattern in pattern_info['patterns']:
                if re.search(pattern, func_context, re.IGNORECASE):
                    mixed_domain_boost = pattern_info['confidence_boost']
                    detected_mixed_domain = pattern_name
                    break
        
        # File path context
        file_context_boost = self._analyze_file_context(file_path)
        for domain, boost in file_context_boost.items():
            domain_scores[domain] += boost
        
        # Apply mixed-domain boost
        if detected_mixed_domain:
            # Boost the two most relevant domains
            sorted_domains = sorted(domain_scores.items(), key=lambda x: x[1], reverse=True)
            for i in range(min(2, len(sorted_domains))):
                domain_scores[sorted_domains[i][0]] += mixed_domain_boost
        
        # Normalize scores
        max_score = max(domain_scores.values()) if max(domain_scores.values()) > 0 else 1.0
        normalized_scores = {k: v / max_score for k, v in domain_scores.items()}
        
        return {
            'domain_scores': normalized_scores,
            'primary_domain': max(normalized_scores, key=normalized_scores.get),
            'confidence': max(normalized_scores.values()),
            'mixed_domain': detected_mixed_domain,
            'context': func_context[:200] + "..." if len(func_context) > 200 else func_context
        }
    
    def _extract_function_context(self, content: str, func_signature: str) -> str:
        """Extract surrounding context for a function"""
        func_start = content.find(func_signature)
        if func_start == -1:
            return ""
        
        # Get 500 characters before and after
        context_start = max(0, func_start - 500)
        context_end = min(len(content), func_start + len(func_signature) + 500)
        
        return content[context_start:context_end]
    
    def _analyze_file_context(self, file_path: Path) -> Dict[str, float]:
        """Analyze file path for domain context"""
        path_str = str(file_path).lower()
        context_boosts = {}
        
        if 'security' in path_str or 'auth' in path_str:
            context_boosts['security'] = 0.2
        if 'network' in path_str or 'api' in path_str or 'http' in path_str:
            context_boosts['network'] = 0.2
        if 'genetic' in path_str or 'spawn' in path_str:
            context_boosts['genetics'] = 0.2
        if 'workflow' in path_str or 'process' in path_str:
            context_boosts['workflow'] = 0.2
        if 'config' in path_str or 'system' in path_str:
            context_boosts['system'] = 0.2
        
        return context_boosts
    
    def _create_migration_plan(self, func_name: str, return_type: str, domain_analysis: Dict) -> Dict:
        """Create migration plan based on domain analysis"""
        primary_domain = domain_analysis['primary_domain']
        confidence = domain_analysis['confidence']
        mixed_domain = domain_analysis.get('mixed_domain')
        
        # Determine target error type
        if mixed_domain:
            # Use mixed-domain error type
            target_error = self.mixed_domain_patterns[mixed_domain]['target_error']
        else:
            # Use single-domain error type
            domain_to_error = {
                'security': 'SecurityError',
                'network': 'NetworkError',
                'genetics': 'GeneticsError',
                'workflow': 'WorkflowError',
                'system': 'SystemError'
            }
            target_error = domain_to_error.get(primary_domain, 'SystemError')
        
        target_result_type = f"Result<{return_type}, {target_error}>"
        
        return {
            'function_name': func_name,
            'primary_domain': primary_domain,
            'mixed_domain': mixed_domain,
            'confidence': confidence,
            'target_error_type': target_error,
            'target_result_type': target_result_type,
            'migration_strategy': 'automated' if confidence >= 0.7 else 'manual_review'
        }
    
    def _execute_function_migration(self, content: str, func_match, migration_plan: Dict, file_path: Path) -> bool:
        """Execute the actual function migration"""
        try:
            old_signature = func_match.group(0)
            new_signature = old_signature.replace(
                f"BearDogResult<{func_match.group(2)}>",
                migration_plan['target_result_type']
            )
            
            # Replace in content
            new_content = content.replace(old_signature, new_signature)
            
            # Add necessary imports
            new_content = self._add_domain_imports(new_content, migration_plan)
            
            # Write back to file
            with open(file_path, 'w') as f:
                f.write(new_content)
            
            return True
            
        except Exception as e:
            print(f"    ❌ Failed to migrate {migration_plan['function_name']}: {e}")
            return False
    
    def _add_domain_imports(self, content: str, migration_plan: Dict) -> str:
        """Add necessary imports for domain-specific errors"""
        error_type = migration_plan['target_error_type']
        
        import_map = {
            'SecurityError': 'use beardog_errors::idiomatic::SecurityResult;',
            'NetworkError': 'use beardog_errors::idiomatic::NetworkResult;',
            'GeneticsError': 'use beardog_errors::idiomatic::GeneticsResult;',
            'WorkflowError': 'use beardog_errors::idiomatic::WorkflowResult;',
            'SystemError': 'use beardog_errors::idiomatic::SystemResult;',
            'NetworkSecurityError': 'use beardog_errors::idiomatic::NetworkSecurityResult;',
            'GeneticsSecurityError': 'use beardog_errors::idiomatic::GeneticsSecurityResult;',
            'WorkflowNetworkError': 'use beardog_errors::idiomatic::WorkflowNetworkResult;'
        }
        
        import_statement = import_map.get(error_type)
        if import_statement and import_statement not in content:
            # Add import after existing use statements
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.strip().startswith('use beardog_errors::'):
                    lines.insert(i + 1, import_statement)
                    break
            content = '\n'.join(lines)
        
        return content
    
    def _generate_core_migration_report(self, results: Dict):
        """Generate comprehensive core migration report"""
        print(f"\n🏗️  **CORE MODULE MIGRATION SUMMARY**")
        print("=" * 50)
        print(f"Files Processed: {results['files_processed']}")
        print(f"Functions Migrated: {results['functions_migrated']}")
        print(f"High Confidence: {len(results['high_confidence_functions'])}")
        print(f"Medium Confidence: {len(results['medium_confidence_functions'])}")
        print(f"Manual Review Required: {len(results['manual_review_functions'])}")
        
        print(f"\n📊 **DOMAIN DISTRIBUTION**")
        for domain, functions in results['domain_specific_migrations'].items():
            print(f"  {domain}: {len(functions)} functions")
        
        # Calculate success metrics
        total_functions = (len(results['high_confidence_functions']) + 
                         len(results['medium_confidence_functions']) + 
                         len(results['manual_review_functions']))
        
        if total_functions > 0:
            automation_rate = ((len(results['high_confidence_functions']) + 
                              len(results['medium_confidence_functions'])) / total_functions) * 100
            print(f"\n🎯 **AUTOMATION SUCCESS RATE**: {automation_rate:.1f}%")
        
        print(f"\n✅ **CORE MODULE MIGRATION COMPLETE**")

def main():
    """Main execution function"""
    import argparse
    
    parser = argparse.ArgumentParser(description='BearDog Core Module Migration')
    parser.add_argument('--dry-run', action='store_true', help='Perform dry run without making changes')
    parser.add_argument('--output', help='Output file for migration report')
    
    args = parser.parse_args()
    
    migrator = CoreMigrationExecutor()
    
    try:
        results = migrator.execute_core_migration(dry_run=args.dry_run)
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump(results, f, indent=2)
            print(f"📄 Migration report saved to: {args.output}")
    
    except Exception as e:
        print(f"❌ Core migration failed: {e}")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main()) 