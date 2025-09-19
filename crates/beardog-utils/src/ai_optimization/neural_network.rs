// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;

pub struct SimpleNeuralNetwork {
    input_layer: Vec<f64>,
    hidden_layer: Vec<f64>,
    output_layer: Vec<f64>,
    weights_ih: Vec<Vec<f64>>, // Input to hidden weights
    weights_ho: Vec<Vec<f64>>, // Hidden to output weights
}

impl SimpleNeuralNetwork {
    /// Creates a new instance
    pub fn new(
        input_size: usize,
        hidden_size: usize,
        output_size: usize,
    ) -> Result<Self, BearDogError> {
        if input_size == 0 || hidden_size == 0 || output_size == 0 {
            return Err(BearDogError::invalid_input(
                "Layer sizes must be greater than 0",
            ));
        }

        // Initialize weights with small random values
        let mut weights_ih = Vec::with_capacity(input_size);
        for _ in 0..input_size {
            let row = vec![0.1; hidden_size]; // More efficient initialization
            weights_ih.push(row);
        }

        let mut weights_ho = Vec::with_capacity(hidden_size);
        for _ in 0..hidden_size {
            let row = vec![0.1; output_size]; // More efficient initialization
            weights_ho.push(row);
        }

        Ok(Self {
            input_layer: vec![0.0; input_size],
            hidden_layer: vec![0.0; hidden_size],
            output_layer: vec![0.0; output_size],
            weights_ih,
            weights_ho,
        })
    }

    pub fn forward(&mut self, inputs: &[f64]) -> Result<Vec<f64>, BearDogError> {
        if inputs.len() != self.input_layer.len() {
            return Err(BearDogError::invalid_input(&format!(
                "Expected {} inputs, got {}",
                self.input_layer.len(),
                inputs.len()
            )));
        }

        // Set input layer
        self.input_layer.copy_from_slice(inputs);

        // Calculate hidden layer
        for h_idx in 0..self.hidden_layer.len() {
            let mut sum = 0.0;
            for (i_idx, &input_val) in self.input_layer.iter().enumerate() {
                sum += input_val * self.weights_ih[i_idx][h_idx];
            }
            self.hidden_layer[h_idx] = Self::activation_function(sum);
        }

        // Calculate output layer
        for o_idx in 0..self.output_layer.len() {
            let mut sum = 0.0;
            for (h_idx, &hidden_val) in self.hidden_layer.iter().enumerate() {
                sum += hidden_val * self.weights_ho[h_idx][o_idx];
            }
            self.output_layer[o_idx] = Self::activation_function(sum);
        }

        Ok(self.output_layer.clone())
    }

    fn activation_function(input: f64) -> f64 {
        // Simple sigmoid activation
        1.0 / (1.0 + (-input).exp())
    }

    pub fn train(
        &mut self,
        inputs: &[f64],
        expected_outputs: &[f64],
        learning_rate: f64,
    ) -> Result<(), BearDogError> {
        if expected_outputs.len() != self.output_layer.len() {
            return Err(BearDogError::invalid_input(&format!(
                "Expected {} outputs, got {}",
                self.output_layer.len(),
                expected_outputs.len()
            )));
        }

        // Forward pass
        let outputs = self.forward(inputs)?;

        // Calculate output layer errors
        let mut output_errors = Vec::with_capacity(outputs.len());
        for (&expected, &actual) in expected_outputs.iter().zip(outputs.iter()) {
            output_errors.push(expected - actual);
        }

        // Simplified backpropagation (just update output weights)
        for (h_idx, &hidden_value) in self.hidden_layer.iter().enumerate() {
            for (o_idx, &error) in output_errors
                .iter()
                .enumerate()
                .take(self.output_layer.len())
            {
                let delta = learning_rate * error * hidden_value;
                self.weights_ho[h_idx][o_idx] += delta;
            }
        }

        Ok(())
    }

    /// Gets `prediction_confidence`
    /// Gets `prediction_confidence`
    #[must_use] pub fn get_prediction_confidence(&self) -> f64 {
        // Simple confidence measure based on output variance
        if self.output_layer.is_empty() {
            return 0.0;
        }

        let mean: f64 = self.output_layer.iter().sum::<f64>() / self.output_layer.len() as f64;
        let variance: f64 = self
            .output_layer
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / self.output_layer.len() as f64;

        // Lower variance = higher confidence
        1.0 / (1.0 + variance)
    }
}
