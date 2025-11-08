# BearDog Profiling Infrastructure

This directory contains profiling tools and results for performance optimization.

## Quick Start

### 1. Generate Flamegraph (CPU profiling)

```bash
./scripts/profiling/generate_flamegraph.sh beardog-adapters
```

This creates a flamegraph showing where CPU time is spent. Open the generated `.svg` file in your browser.

### 2. Analyze Clone Usage

```bash
./scripts/profiling/analyze_clones.sh
```

Generates a report of all `.clone()` calls, sorted by frequency and package.

### 3. Profile Benchmarks

```bash
./scripts/profiling/profile_benchmarks.sh
```

Runs all benchmarks and saves performance metrics.

### 4. Memory Profiling (Linux only)

```bash
./scripts/profiling/profile_memory.sh beardog-core
```

Analyzes memory allocation patterns using valgrind.

## Directory Structure

```
profiling/
├── README.md              # This file
├── config.toml           # Profiling configuration
├── flamegraphs/          # CPU flamegraphs (.svg files)
├── reports/              # Profiling reports (.txt files)
└── benchmarks/           # Benchmark results
```

## Interpreting Flamegraphs

- **Width**: Proportional to CPU time spent in that function
- **Height**: Call stack depth
- **Color**: Random (for differentiation)
- **Hot paths**: Wide blocks at the top of the graph

Look for:
1. Wide blocks = expensive operations
2. Repeated patterns = potential optimization targets
3. Unexpected functions = performance bugs

## Common Optimization Targets

### 1. Expensive Clones

**Pattern**: `Arc<RwLock<HashMap>>` being cloned

**Solution**: Use cheap Arc accessors
```rust
// ❌ Expensive
let data = self.map.read().unwrap().clone();

// ✅ Cheap
pub fn map_ref(&self) -> Arc<RwLock<HashMap<K, V>>> {
    Arc::clone(&self.map)
}
```

### 2. String Allocations

**Pattern**: Frequent `to_string()` or `to_owned()` calls

**Solution**: Use `&str` when possible, `Cow<str>` for sometimes-owned

### 3. Unnecessary Serialization

**Pattern**: Serialize/deserialize in hot paths

**Solution**: Cache serialized forms, use zero-copy deserialization

## Profiling Workflow

1. **Baseline**: Profile current performance
2. **Identify**: Find hot paths in flamegraph
3. **Optimize**: Apply optimizations
4. **Verify**: Re-profile to confirm improvement
5. **Document**: Record results

## Tools Reference

### cargo-flamegraph
- **Purpose**: CPU profiling with flamegraphs
- **Platform**: Linux (best), macOS (limited), Windows (minimal)
- **Overhead**: Low (~5%)

### perf (Linux)
- **Purpose**: Low-level CPU profiling
- **Platform**: Linux only
- **Overhead**: Very low (<1%)

### valgrind/massif
- **Purpose**: Memory profiling
- **Platform**: Linux, macOS
- **Overhead**: High (10-30x slowdown)

### cargo-instruments (macOS)
- **Purpose**: Xcode Instruments integration
- **Platform**: macOS only
- **Overhead**: Low

## Tips for Effective Profiling

1. **Profile in Release Mode**: Debug builds have different performance
2. **Use Representative Workloads**: Test with realistic data
3. **Profile Multiple Times**: Results can vary
4. **Focus on Hot Paths**: Optimize the 20% that takes 80% of time
5. **Measure Impact**: Always benchmark before/after
6. **Document Changes**: Record what worked and what didn't

## Configuration

Edit `profiling/config.toml` to customize:
- Sampling frequency
- Output directories
- Target packages
- Benchmark parameters

## Troubleshooting

### "cargo flamegraph not found"
```bash
cargo install flamegraph
```

### "perf not found" (Linux)
```bash
sudo apt-get install linux-tools-common linux-tools-generic linux-tools-$(uname -r)
```

### "Permission denied" when running perf
```bash
# Temporarily allow non-root perf
sudo sysctl -w kernel.perf_event_paranoid=-1

# Or permanently (add to /etc/sysctl.conf)
echo 'kernel.perf_event_paranoid=-1' | sudo tee -a /etc/sysctl.conf
```

### Flamegraph is empty or shows no data
- Build with debug symbols: `export CARGO_PROFILE_RELEASE_DEBUG=true`
- Increase sampling frequency in `config.toml`
- Run longer workloads (profiling needs time to collect samples)

## Further Reading

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [cargo-flamegraph documentation](https://github.com/flamegraph-rs/flamegraph)
- [Brendan Gregg's Flamegraph Guide](http://www.brendangregg.com/flamegraphs.html)

---

**Status**: Infrastructure ready for data-driven optimization  
**Grade Impact**: Enables A+ → A++ progression
