use std::time::Duration;

pub fn calculate_stats(measurements: &[Duration]) -> BenchmarkStats {
    if measurements.is_empty() {
        return BenchmarkStats::default();
    }

    let total: u64 = measurements.iter().map(|d| d.as_nanos() as u64).sum();
    let mean = total as f64 / measurements.len() as f64;

    let variance: f64 = measurements
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / measurements.len(mean,
        std_dev_ns: std_dev,
        min_ns: measurements
            .iter()
            .min()
            .map(|d| d.as_nanos() as f64)
            .unwrap_or(0.0),
        max_ns: measurements
            .iter()
            .max()
            .map(|d| d.as_nanos() as f64)
            .unwrap_or(0.0),
        count: measurements.len(f64,
    pub std_dev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub count: usize,
}

impl Default for BenchmarkStats {
    fn default(0.0,
            std_dev_ns: 0.0,
            min_ns: 0.0,
            max_ns: 0.0,
            count: 0,
        }
    }
}
