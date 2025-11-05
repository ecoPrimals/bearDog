//! Anomaly Detection Test Types

use beardog_errors::BearDogError;

pub struct AnomalyDetector {
    samples: Vec<f64>,
    baseline: Option<f64>,
    std_dev: f64,
    threshold: f64,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            baseline: None,
            std_dev: 0.0,
            threshold: 2.0,
        }
    }

    pub fn add_sample(&mut self, value: f64) {
        self.samples.push(value);
    }

    pub fn establish_baseline(&mut self) {
        if self.samples.is_empty() {
            return;
        }

        let mean = self.samples.iter().sum::<f64>() / self.samples.len() as f64;
        self.baseline = Some(mean);

        let variance = self.samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / self.samples.len() as f64;

        self.std_dev = variance.sqrt();
    }

    pub fn has_baseline(&self) -> bool {
        self.baseline.is_some()
    }

    pub fn baseline(&self) -> f64 {
        self.baseline.unwrap_or(0.0)
    }

    pub fn std_dev(&self) -> f64 {
        self.std_dev
    }

    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold;
    }

    pub fn is_anomaly(&self, value: f64) -> Result<bool, BearDogError> {
        let baseline = self
            .baseline
            .ok_or_else(|| BearDogError::system("No baseline established".to_string()))?;

        let deviation = (value - baseline).abs();
        let threshold_value = self.threshold * self.std_dev;

        Ok(deviation > threshold_value)
    }
}
