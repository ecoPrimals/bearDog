use beardog_errors::BearDogError;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn benchmark_modernized_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("modernized_system");

    group.bench_function("error_handling", |b| {
        b.iter(|| {
            let result: Result<i32, &str> = black_box(Ok(42));
            let mapped = black_box(result.map(|x| x * 2));
            black_box(mapped.map_err(|e| BearDogError::system(format!("Error: {:?}", e))))
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_modernized_system);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modernization_report() {
        generate_performance_report();
    }
}
