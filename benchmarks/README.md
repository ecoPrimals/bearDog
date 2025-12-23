# 🎯 BearDog Performance Benchmarks

Comprehensive performance benchmarking suite for the BearDog Security Platform.

## 📊 Benchmark Categories

### 1. **HSM Operations** (`hsm_operations_benchmarks.rs`)

Production-critical cryptographic operations:

- **Key Generation**: RSA-2048/4096, ECDSA-P256/P384, Ed25519, AES-256
- **Symmetric Encryption**: AES-256-GCM at various data sizes (100B to 1MB)
- **Asymmetric Encryption**: RSA-2048 encryption/decryption
- **Digital Signatures**: ECDSA and Ed25519 signing/verification
- **Provider Dispatch**: Enum vs Box<dyn> overhead analysis
- **Memory Protection**: Lock/zeroize/guard operations
- **Concurrent Access**: Multi-threaded HSM operations
- **Key Lifecycle**: Generate, store, retrieve, rotate, delete

**Performance Targets**:
- AES-256-GCM: < 1µs per KB
- ECDSA P-256 signing: < 50µs
- Ed25519 signing: < 15µs
- Enum dispatch: < 5ns overhead

### 2. **Discovery Operations** (`discovery_benchmarks.rs`)

HSM discovery and detection:

- **Network Probing**: HTTP/TCP/HTTPS endpoint health checks
- **Platform Detection**: TPM (Windows/Linux), Secure Enclave (iOS), StrongBox (Android)
- **Cloud Discovery**: AWS KMS, Azure Key Vault, GCP Cloud KMS
- **USB Discovery**: YubiKey, Nitrokey enumeration
- **Discovery Caching**: Cache hit/miss, invalidation
- **Full Cycle**: Sequential vs parallel discovery
- **mDNS/DNS-SD**: Service discovery protocols

**Performance Targets**:
- Network probe (LAN): < 10ms
- Platform detection: < 50ms
- Cloud discovery: < 100ms
- Full discovery cycle (parallel): < 500ms

### 3. **Production Workloads** (`production_workload_benchmarks.rs`)

Real-world scenario testing:

- **API Request Handling**: Full lifecycle (auth → routing → crypto → response)
- **Multi-Tenant Operations**: Concurrent key access across tenants
- **Signing Service**: High-throughput message signing
- **Configuration Loading**: Env vars, TOML parsing, validation, hot reload
- **Error Handling**: Creation, propagation, context, logging
- **E2E Crypto Flows**: Encrypt-store-retrieve-decrypt, sign-verify
- **Rate Limiting**: Token bucket, sliding window
- **Metrics Collection**: Counters, histograms, structured logging

**Performance Targets**:
- API request latency (p95): < 10ms
- Signing throughput: > 10,000 ops/sec
- Config hot reload: < 100ms
- Error propagation overhead: < 10µs

### 4. **Legacy Benchmarks** (`hyperoptimized_benchmarks.rs`)

Historical benchmarks for regression testing:
- Error creation and handling
- Zero-copy optimizations
- String operations and interning
- Memory allocation patterns

## 🚀 Quick Start

### Run All Benchmarks

```bash
./scripts/run_benchmarks.sh
```

### Run Specific Category

```bash
# HSM operations only
./scripts/run_benchmarks.sh --hsm

# Discovery operations only
./scripts/run_benchmarks.sh --discovery

# Production workloads only
./scripts/run_benchmarks.sh --production
```

### Quick Development Run

```bash
# Reduced sample size for faster iteration
./scripts/run_benchmarks.sh --quick
```

### Compare with Baseline

```bash
# Save current results as baseline
cargo bench --package benchmarks -- --save-baseline main

# Run and compare
./scripts/run_benchmarks.sh --compare main
```

## 📈 Reading Results

### HTML Reports

After running benchmarks, view the interactive HTML reports:

```bash
open target/criterion/report/index.html
```

Individual benchmark reports:
- `target/criterion/hsm_operations_benchmarks/report/index.html`
- `target/criterion/discovery_benchmarks/report/index.html`
- `target/criterion/production_workload_benchmarks/report/index.html`

### Flamegraphs

Profile and visualize performance hotspots:

```bash
ls target/criterion/*/profile/flamegraph.svg
```

Open flamegraphs in your browser to identify bottlenecks.

### Terminal Output

Criterion provides detailed statistics:

```
test encrypt/1KB    time: [42.5 µs 43.2 µs 44.1 µs]
                    thrpt: [23.2 MB/s 23.7 MB/s 24.1 MB/s]
                    change: [-5.2% -3.8% -2.1%] (p < 0.05)
                    Performance has improved.
```

## 🔬 Methodology

### Criterion Configuration

- **Measurement Time**: 10 seconds per benchmark
- **Warm-up Time**: 3 seconds
- **Sample Size**: Auto-determined (statistically significant)
- **Profiler**: pprof with flamegraph generation

### Data Sizes

Benchmarks test realistic data sizes:
- **Small**: 100 bytes (API tokens, signatures)
- **Medium**: 1 KB (JSON documents)
- **Large**: 10 KB (small files)
- **XL**: 100 KB (large documents)
- **XXL**: 1 MB (file uploads)

### Network Latencies

Discovery benchmarks simulate realistic network conditions:
- **Local**: 1ms (same machine)
- **LAN**: 5ms (local network)
- **WAN**: 50ms (internet)
- **Slow**: 200ms (poor connection)

## 🎯 Performance Regression Testing

### CI/CD Integration

```bash
# Run in CI mode (exit on regression)
./scripts/run_benchmarks.sh --ci
```

### Regression Thresholds

- **Critical Path**: > 10% regression fails CI
- **Important Path**: > 25% regression fails CI
- **Non-critical**: > 50% regression warns

### GitHub Actions Example

```yaml
name: Performance Benchmarks

on:
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      - name: Run benchmarks
        run: ./scripts/run_benchmarks.sh --ci
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion/
```

## 📊 Interpreting Results

### Time/Iteration

Lower is better. Shows the average time per operation:
- **< 1µs**: Excellent (hot path ready)
- **1-10µs**: Good (acceptable for most operations)
- **10-100µs**: Acceptable (non-critical paths)
- **> 100µs**: Review (optimize if in hot path)

### Throughput

Higher is better. Shows operations per second or data processed:
- **Crypto**: MB/s (data throughput)
- **Signing**: ops/sec (signature throughput)
- **Discovery**: probes/sec (discovery rate)

### Change from Baseline

Indicates performance delta:
- **Negative %**: Performance improved (faster)
- **Positive %**: Performance regressed (slower)
- **< ±5%**: Within noise, likely insignificant
- **> ±10%**: Significant change, investigate

### Statistical Significance

Criterion reports p-values:
- **p < 0.05**: Statistically significant change
- **p > 0.05**: Change likely due to noise

## 🔧 Advanced Usage

### Custom Sample Size

```bash
BENCH_SAMPLES=50 cargo bench --package benchmarks
```

### Custom Measurement Time

```bash
BENCH_TIME=30 cargo bench --package benchmarks
```

### Profile Specific Benchmark

```bash
cargo bench --package benchmarks --bench hsm_operations_benchmarks -- encrypt/1KB
```

### Export CSV Data

```bash
cargo bench --package benchmarks -- --save-baseline baseline_name
# Results saved to: target/criterion/*/baseline_name/
```

## 📝 Adding New Benchmarks

1. **Create benchmark file**: `benches/my_new_benchmarks.rs`
2. **Add to Cargo.toml**:
   ```toml
   [[bench]]
   name = "my_new_benchmarks"
   harness = false
   ```
3. **Use Criterion framework**:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   
   fn my_benchmark(c: &mut Criterion) {
       c.bench_function("my_test", |b| {
           b.iter(|| {
               black_box(my_function());
           })
       });
   }
   
   criterion_group!(benches, my_benchmark);
   criterion_main!(benches);
   ```
4. **Run**: `cargo bench --package benchmarks --bench my_new_benchmarks`

## 🎓 Best Practices

1. **Use `black_box()`**: Prevent compiler optimizations from eliminating code
2. **Realistic Data**: Use production-like data sizes and patterns
3. **Warm-up**: Let JIT/caches stabilize before measurement
4. **Isolation**: Run on idle system for consistent results
5. **Statistical Rigor**: Use sufficient samples for significance
6. **Document**: Add comments explaining what's being measured
7. **Baseline**: Save baselines before major changes

## 🐛 Troubleshooting

### Benchmarks Run Slowly

- Reduce sample size: `--quick` flag
- Run specific benchmark: `--bench name`
- Increase system resources

### Inconsistent Results

- Close background applications
- Disable CPU frequency scaling
- Run multiple times and average
- Check thermal throttling

### Out of Memory

- Reduce data sizes in benchmarks
- Run benchmarks sequentially (not parallel)
- Increase system memory

## 📚 Resources

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Performance Profiling Guide](../docs/guides/PERFORMANCE_PROFILING.md)
- [Zero-Copy Optimization Patterns](../docs/architecture/ZERO_COPY_PATTERNS.md)

## 📄 License

This project is licensed under AGPL-3.0-only.

## 🤝 Contributing

This crate follows the BearDog canonical patterns and modernization guidelines.

For questions or contributions, see: `docs/CONTRIBUTING.md` 