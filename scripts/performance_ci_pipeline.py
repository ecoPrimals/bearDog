#!/usr/bin/env python3
"""
BearDog Performance CI/CD Pipeline
Advanced performance regression testing and monitoring system
"""

import asyncio
import json
import logging
import os
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import statistics

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger('beardog-performance-ci')

@dataclass
class PerformanceMetrics:
    """Performance metrics data structure"""
    test_name: str
    execution_time_ms: float
    memory_usage_mb: float
    cpu_usage_percent: float
    throughput_ops_per_sec: float
    success_rate_percent: float
    timestamp: str
    commit_hash: str
    branch: str

@dataclass
class BenchmarkResult:
    """Benchmark execution result"""
    name: str
    baseline_time_ms: float
    current_time_ms: float
    regression_percent: float
    status: str  # 'PASS', 'WARN', 'FAIL'
    details: Dict

@dataclass
class PerformanceReport:
    """Comprehensive performance report"""
    timestamp: str
    commit_hash: str
    branch: str
    total_benchmarks: int
    passed_benchmarks: int
    failed_benchmarks: int
    warning_benchmarks: int
    overall_status: str
    benchmarks: List[BenchmarkResult]
    summary: Dict

class PerformanceCIPipeline:
    """Advanced performance CI/CD pipeline for BearDog"""
    
    def __init__(self, config_path: Optional[Path] = None):
        self.config = self.load_config(config_path)
        self.workspace_root = Path.cwd()
        self.results_dir = self.workspace_root / "performance_results"
        self.results_dir.mkdir(exist_ok=True)
        
        # Performance thresholds
        self.regression_threshold = 0.10  # 10% regression threshold
        self.warning_threshold = 0.05     # 5% warning threshold
        self.memory_threshold_mb = 1000   # 1GB memory threshold
        self.cpu_threshold_percent = 80   # 80% CPU threshold
        
        logger.info("🚀 Initialized BearDog Performance CI Pipeline")
    
    def load_config(self, config_path: Optional[Path]) -> Dict:
        """Load pipeline configuration"""
        default_config = {
            "benchmarks": {
                "compile_time": {"enabled": True, "timeout": 300},
                "test_execution": {"enabled": True, "timeout": 600},
                "memory_usage": {"enabled": True, "timeout": 120},
                "simd_operations": {"enabled": True, "timeout": 60},
                "crypto_performance": {"enabled": True, "timeout": 180},
                "api_throughput": {"enabled": True, "timeout": 240},
            },
            "thresholds": {
                "regression": 0.10,
                "warning": 0.05,
                "memory_mb": 1000,
                "cpu_percent": 80,
            },
            "reporting": {
                "generate_html": True,
                "generate_json": True,
                "upload_to_dashboard": False,
            }
        }
        
        if config_path and config_path.exists():
            with open(config_path, 'r') as f:
                user_config = json.load(f)
                default_config.update(user_config)
        
        return default_config
    
    async def run_pipeline(self) -> PerformanceReport:
        """Run the complete performance CI pipeline"""
        logger.info("🔧 Starting Performance CI Pipeline")
        start_time = time.time()
        
        # Get git information
        commit_hash = self.get_git_commit_hash()
        branch = self.get_git_branch()
        timestamp = datetime.now(timezone.utc).isoformat()
        
        # Run all benchmarks
        benchmark_results = []
        
        if self.config["benchmarks"]["compile_time"]["enabled"]:
            result = await self.benchmark_compile_time()
            benchmark_results.append(result)
        
        if self.config["benchmarks"]["test_execution"]["enabled"]:
            result = await self.benchmark_test_execution()
            benchmark_results.append(result)
        
        if self.config["benchmarks"]["memory_usage"]["enabled"]:
            result = await self.benchmark_memory_usage()
            benchmark_results.append(result)
        
        if self.config["benchmarks"]["simd_operations"]["enabled"]:
            result = await self.benchmark_simd_operations()
            benchmark_results.append(result)
        
        if self.config["benchmarks"]["crypto_performance"]["enabled"]:
            result = await self.benchmark_crypto_performance()
            benchmark_results.append(result)
        
        if self.config["benchmarks"]["api_throughput"]["enabled"]:
            result = await self.benchmark_api_throughput()
            benchmark_results.append(result)
        
        # Generate comprehensive report
        report = self.generate_performance_report(
            timestamp, commit_hash, branch, benchmark_results
        )
        
        # Save and publish results
        await self.save_results(report)
        
        total_time = time.time() - start_time
        logger.info(f"✅ Performance CI Pipeline completed in {total_time:.2f}s")
        logger.info(f"📊 Overall Status: {report.overall_status}")
        
        return report
    
    async def benchmark_compile_time(self) -> BenchmarkResult:
        """Benchmark Rust compilation time"""
        logger.info("⏱️ Benchmarking compilation time")
        
        # Clean build first
        await self.run_command("cargo clean")
        
        # Measure compile time
        start_time = time.time()
        result = await self.run_command("cargo build --workspace --release")
        compile_time = (time.time() - start_time) * 1000  # Convert to ms
        
        # Load baseline (if exists)
        baseline_time = self.load_baseline("compile_time", 60000.0)  # Default 60s
        
        regression = ((compile_time - baseline_time) / baseline_time) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="compile_time",
            baseline_time_ms=baseline_time,
            current_time_ms=compile_time,
            regression_percent=regression,
            status=status,
            details={
                "command": "cargo build --workspace --release",
                "success": result.returncode == 0,
                "output_lines": len(result.stdout.splitlines()) if result.stdout else 0,
            }
        )
    
    async def benchmark_test_execution(self) -> BenchmarkResult:
        """Benchmark test execution time"""
        logger.info("🧪 Benchmarking test execution time")
        
        start_time = time.time()
        result = await self.run_command("cargo test --workspace --lib --quiet")
        execution_time = (time.time() - start_time) * 1000
        
        baseline_time = self.load_baseline("test_execution", 30000.0)  # Default 30s
        regression = ((execution_time - baseline_time) / baseline_time) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="test_execution",
            baseline_time_ms=baseline_time,
            current_time_ms=execution_time,
            regression_percent=regression,
            status=status,
            details={
                "command": "cargo test --workspace --lib --quiet",
                "success": result.returncode == 0,
                "test_count": self.count_tests_from_output(result.stdout),
            }
        )
    
    async def benchmark_memory_usage(self) -> BenchmarkResult:
        """Benchmark memory usage during build"""
        logger.info("💾 Benchmarking memory usage")
        
        # Use time command to measure peak memory
        result = await self.run_command("/usr/bin/time -v cargo build --workspace 2>&1")
        
        # Parse memory usage from time output
        memory_kb = self.parse_memory_from_time_output(result.stdout)
        memory_mb = memory_kb / 1024.0 if memory_kb else 0
        
        baseline_memory = self.load_baseline("memory_usage", 500.0)  # Default 500MB
        regression = ((memory_mb - baseline_memory) / baseline_memory) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="memory_usage",
            baseline_time_ms=baseline_memory,
            current_time_ms=memory_mb,
            regression_percent=regression,
            status=status,
            details={
                "memory_mb": memory_mb,
                "memory_kb": memory_kb,
                "threshold_mb": self.memory_threshold_mb,
            }
        )
    
    async def benchmark_simd_operations(self) -> BenchmarkResult:
        """Benchmark SIMD operations performance"""
        logger.info("⚡ Benchmarking SIMD operations")
        
        start_time = time.time()
        result = await self.run_command(
            "cargo test --workspace simd --release -- --nocapture"
        )
        execution_time = (time.time() - start_time) * 1000
        
        baseline_time = self.load_baseline("simd_operations", 5000.0)  # Default 5s
        regression = ((execution_time - baseline_time) / baseline_time) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="simd_operations",
            baseline_time_ms=baseline_time,
            current_time_ms=execution_time,
            regression_percent=regression,
            status=status,
            details={
                "command": "cargo test --workspace simd --release",
                "success": result.returncode == 0,
                "simd_tests": self.count_simd_tests_from_output(result.stdout),
            }
        )
    
    async def benchmark_crypto_performance(self) -> BenchmarkResult:
        """Benchmark cryptographic operations performance"""
        logger.info("🔐 Benchmarking cryptographic performance")
        
        start_time = time.time()
        result = await self.run_command(
            "cargo test --workspace mathematical_certainty --release -- --nocapture"
        )
        execution_time = (time.time() - start_time) * 1000
        
        baseline_time = self.load_baseline("crypto_performance", 8000.0)  # Default 8s
        regression = ((execution_time - baseline_time) / baseline_time) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="crypto_performance",
            baseline_time_ms=baseline_time,
            current_time_ms=execution_time,
            regression_percent=regression,
            status=status,
            details={
                "command": "cargo test --workspace mathematical_certainty --release",
                "success": result.returncode == 0,
                "crypto_tests": self.count_crypto_tests_from_output(result.stdout),
            }
        )
    
    async def benchmark_api_throughput(self) -> BenchmarkResult:
        """Benchmark API throughput performance"""
        logger.info("🌐 Benchmarking API throughput")
        
        start_time = time.time()
        result = await self.run_command(
            "cargo test --workspace api --release -- --nocapture"
        )
        execution_time = (time.time() - start_time) * 1000
        
        baseline_time = self.load_baseline("api_throughput", 12000.0)  # Default 12s
        regression = ((execution_time - baseline_time) / baseline_time) * 100
        status = self.determine_status(regression)
        
        return BenchmarkResult(
            name="api_throughput",
            baseline_time_ms=baseline_time,
            current_time_ms=execution_time,
            regression_percent=regression,
            status=status,
            details={
                "command": "cargo test --workspace api --release",
                "success": result.returncode == 0,
                "api_tests": self.count_api_tests_from_output(result.stdout),
            }
        )
    
    def generate_performance_report(
        self, 
        timestamp: str, 
        commit_hash: str, 
        branch: str, 
        benchmarks: List[BenchmarkResult]
    ) -> PerformanceReport:
        """Generate comprehensive performance report"""
        
        passed = sum(1 for b in benchmarks if b.status == 'PASS')
        failed = sum(1 for b in benchmarks if b.status == 'FAIL')
        warning = sum(1 for b in benchmarks if b.status == 'WARN')
        
        overall_status = 'PASS'
        if failed > 0:
            overall_status = 'FAIL'
        elif warning > 0:
            overall_status = 'WARN'
        
        # Calculate summary statistics
        regressions = [b.regression_percent for b in benchmarks]
        summary = {
            "avg_regression": statistics.mean(regressions) if regressions else 0.0,
            "max_regression": max(regressions) if regressions else 0.0,
            "min_regression": min(regressions) if regressions else 0.0,
            "total_execution_time_ms": sum(b.current_time_ms for b in benchmarks),
            "performance_score": self.calculate_performance_score(benchmarks),
        }
        
        return PerformanceReport(
            timestamp=timestamp,
            commit_hash=commit_hash,
            branch=branch,
            total_benchmarks=len(benchmarks),
            passed_benchmarks=passed,
            failed_benchmarks=failed,
            warning_benchmarks=warning,
            overall_status=overall_status,
            benchmarks=benchmarks,
            summary=summary
        )
    
    async def save_results(self, report: PerformanceReport):
        """Save performance results to files"""
        timestamp_str = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        # Save JSON report
        json_path = self.results_dir / f"performance_report_{timestamp_str}.json"
        with open(json_path, 'w') as f:
            json.dump(asdict(report), f, indent=2, default=str)
        
        logger.info(f"📁 Saved JSON report: {json_path}")
        
        # Generate HTML report if enabled
        if self.config["reporting"]["generate_html"]:
            html_path = await self.generate_html_report(report, timestamp_str)
            logger.info(f"📄 Generated HTML report: {html_path}")
        
        # Update baselines for successful benchmarks
        await self.update_baselines(report)
    
    async def generate_html_report(self, report: PerformanceReport, timestamp_str: str) -> Path:
        """Generate HTML performance report"""
        html_content = self.create_html_report_content(report)
        
        html_path = self.results_dir / f"performance_report_{timestamp_str}.html"
        with open(html_path, 'w') as f:
            f.write(html_content)
        
        return html_path
    
    def create_html_report_content(self, report: PerformanceReport) -> str:
        """Create HTML content for performance report"""
        benchmarks_html = ""
        for benchmark in report.benchmarks:
            status_color = {
                'PASS': '#28a745',
                'WARN': '#ffc107', 
                'FAIL': '#dc3545'
            }.get(benchmark.status, '#6c757d')
            
            benchmarks_html += f"""
            <tr>
                <td>{benchmark.name}</td>
                <td>{benchmark.current_time_ms:.2f}ms</td>
                <td>{benchmark.baseline_time_ms:.2f}ms</td>
                <td style="color: {status_color};">{benchmark.regression_percent:+.2f}%</td>
                <td><span class="badge" style="background-color: {status_color};">{benchmark.status}</span></td>
            </tr>
            """
        
        return f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>BearDog Performance Report</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                .header {{ background: #f8f9fa; padding: 20px; border-radius: 5px; }}
                .summary {{ display: flex; gap: 20px; margin: 20px 0; }}
                .metric {{ background: white; padding: 15px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
                table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
                th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
                th {{ background-color: #f8f9fa; }}
                .badge {{ padding: 4px 8px; border-radius: 4px; color: white; font-size: 0.8em; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>🚀 BearDog Performance Report</h1>
                <p><strong>Timestamp:</strong> {report.timestamp}</p>
                <p><strong>Commit:</strong> {report.commit_hash} ({report.branch})</p>
                <p><strong>Overall Status:</strong> <span class="badge" style="background-color: {'#28a745' if report.overall_status == 'PASS' else '#ffc107' if report.overall_status == 'WARN' else '#dc3545'};">{report.overall_status}</span></p>
            </div>
            
            <div class="summary">
                <div class="metric">
                    <h3>Total Benchmarks</h3>
                    <h2>{report.total_benchmarks}</h2>
                </div>
                <div class="metric">
                    <h3>Passed</h3>
                    <h2 style="color: #28a745;">{report.passed_benchmarks}</h2>
                </div>
                <div class="metric">
                    <h3>Warnings</h3>
                    <h2 style="color: #ffc107;">{report.warning_benchmarks}</h2>
                </div>
                <div class="metric">
                    <h3>Failed</h3>
                    <h2 style="color: #dc3545;">{report.failed_benchmarks}</h2>
                </div>
            </div>
            
            <h2>📊 Benchmark Results</h2>
            <table>
                <thead>
                    <tr>
                        <th>Benchmark</th>
                        <th>Current Time</th>
                        <th>Baseline Time</th>
                        <th>Regression</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
                    {benchmarks_html}
                </tbody>
            </table>
            
            <h2>📈 Summary Statistics</h2>
            <ul>
                <li><strong>Average Regression:</strong> {report.summary['avg_regression']:+.2f}%</li>
                <li><strong>Maximum Regression:</strong> {report.summary['max_regression']:+.2f}%</li>
                <li><strong>Total Execution Time:</strong> {report.summary['total_execution_time_ms']:.2f}ms</li>
                <li><strong>Performance Score:</strong> {report.summary['performance_score']:.1f}/100</li>
            </ul>
        </body>
        </html>
        """
    
    async def update_baselines(self, report: PerformanceReport):
        """Update performance baselines for successful benchmarks"""
        baselines_file = self.results_dir / "performance_baselines.json"
        
        # Load existing baselines
        baselines = {}
        if baselines_file.exists():
            with open(baselines_file, 'r') as f:
                baselines = json.load(f)
        
        # Update baselines for PASS benchmarks
        for benchmark in report.benchmarks:
            if benchmark.status == 'PASS':
                baselines[benchmark.name] = {
                    "time_ms": benchmark.current_time_ms,
                    "timestamp": report.timestamp,
                    "commit_hash": report.commit_hash,
                }
        
        # Save updated baselines
        with open(baselines_file, 'w') as f:
            json.dump(baselines, f, indent=2)
        
        logger.info(f"📊 Updated performance baselines")
    
    def load_baseline(self, benchmark_name: str, default_value: float) -> float:
        """Load baseline value for a benchmark"""
        baselines_file = self.results_dir / "performance_baselines.json"
        
        if not baselines_file.exists():
            return default_value
        
        try:
            with open(baselines_file, 'r') as f:
                baselines = json.load(f)
                return baselines.get(benchmark_name, {}).get("time_ms", default_value)
        except Exception as e:
            logger.warning(f"Failed to load baseline for {benchmark_name}: {e}")
            return default_value
    
    def determine_status(self, regression_percent: float) -> str:
        """Determine benchmark status based on regression"""
        if regression_percent > self.regression_threshold * 100:
            return 'FAIL'
        elif regression_percent > self.warning_threshold * 100:
            return 'WARN'
        else:
            return 'PASS'
    
    def calculate_performance_score(self, benchmarks: List[BenchmarkResult]) -> float:
        """Calculate overall performance score (0-100)"""
        if not benchmarks:
            return 0.0
        
        scores = []
        for benchmark in benchmarks:
            if benchmark.status == 'PASS':
                # Bonus for improvement, penalty for regression
                improvement_bonus = max(0, -benchmark.regression_percent * 2)
                scores.append(min(100, 85 + improvement_bonus))
            elif benchmark.status == 'WARN':
                scores.append(70)
            else:  # FAIL
                scores.append(40)
        
        return statistics.mean(scores)
    
    async def run_command(self, command: str) -> subprocess.CompletedProcess:
        """Run shell command asynchronously"""
        logger.debug(f"Running command: {command}")
        
        process = await asyncio.create_subprocess_shell(
            command,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.STDOUT,
            cwd=self.workspace_root
        )
        
        stdout, _ = await process.communicate()
        
        return subprocess.CompletedProcess(
            args=command,
            returncode=process.returncode,
            stdout=stdout.decode('utf-8', errors='replace') if stdout else '',
            stderr=''
        )
    
    def get_git_commit_hash(self) -> str:
        """Get current git commit hash"""
        try:
            result = subprocess.run(
                ['git', 'rev-parse', 'HEAD'],
                capture_output=True,
                text=True,
                cwd=self.workspace_root
            )
            return result.stdout.strip()[:8] if result.returncode == 0 else 'unknown'
        except Exception:
            return 'unknown'
    
    def get_git_branch(self) -> str:
        """Get current git branch"""
        try:
            result = subprocess.run(
                ['git', 'rev-parse', '--abbrev-ref', 'HEAD'],
                capture_output=True,
                text=True,
                cwd=self.workspace_root
            )
            return result.stdout.strip() if result.returncode == 0 else 'unknown'
        except Exception:
            return 'unknown'
    
    def count_tests_from_output(self, output: str) -> int:
        """Count tests from cargo test output"""
        if not output:
            return 0
        
        # Look for "test result: ok. X passed" pattern
        import re
        match = re.search(r'test result: ok\. (\d+) passed', output)
        return int(match.group(1)) if match else 0
    
    def count_simd_tests_from_output(self, output: str) -> int:
        """Count SIMD tests from output"""
        if not output:
            return 0
        return output.count('test_simd') + output.count('simd_')
    
    def count_crypto_tests_from_output(self, output: str) -> int:
        """Count crypto tests from output"""
        if not output:
            return 0
        return output.count('crypto') + output.count('mathematical_certainty')
    
    def count_api_tests_from_output(self, output: str) -> int:
        """Count API tests from output"""
        if not output:
            return 0
        return output.count('api_') + output.count('test_api')
    
    def parse_memory_from_time_output(self, output: str) -> Optional[int]:
        """Parse memory usage from /usr/bin/time output"""
        if not output:
            return None
        
        # Look for "Maximum resident set size (kbytes): XXXX"
        import re
        match = re.search(r'Maximum resident set size \(kbytes\): (\d+)', output)
        return int(match.group(1)) if match else None

async def main():
    """Main entry point for performance CI pipeline"""
    print("🚀 BearDog Performance CI/CD Pipeline")
    print("=" * 50)
    
    pipeline = PerformanceCIPipeline()
    
    try:
        report = await pipeline.run_pipeline()
        
        print(f"\n📊 Performance Report Summary:")
        print(f"   Overall Status: {report.overall_status}")
        print(f"   Total Benchmarks: {report.total_benchmarks}")
        print(f"   Passed: {report.passed_benchmarks}")
        print(f"   Warnings: {report.warning_benchmarks}")
        print(f"   Failed: {report.failed_benchmarks}")
        print(f"   Performance Score: {report.summary['performance_score']:.1f}/100")
        
        # Exit with appropriate code
        if report.overall_status == 'FAIL':
            sys.exit(1)
        elif report.overall_status == 'WARN':
            sys.exit(2)
        else:
            sys.exit(0)
            
    except Exception as e:
        logger.error(f"Pipeline failed with error: {e}")
        sys.exit(3)

if __name__ == "__main__":
    asyncio.run(main()) 