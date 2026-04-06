// SPDX-License-Identifier: AGPL-3.0-or-later

//! Data preprocessing: normalization, feature selection, augmentation, and missing values.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Normalization strategies for feature scaling
///
/// Different methods for transforming numerical features to a common scale,
/// improving model training stability and convergence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max scaling to range [0, 1]: (x - min) / (max - min)
    MinMax,
    /// Z-score standardization to mean 0, std 1: (x - mean) / std
    ZScore,
    /// Robust scaling using median and IQR, less sensitive to outliers
    Robust,
    /// Unit vector normalization, scales to unit length
    UnitVector,
    /// No normalization applied, use raw feature values
    None,
}

/// Feature selection configuration for dimensionality reduction
///
/// Configures automatic selection of the most relevant features from the input dataset,
/// reducing dimensionality and improving model performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSelectionConfig {
    /// Algorithm used to identify and select important features
    pub method: FeatureSelectionMethod,
    /// Optional target number of features to select from the input dataset
    pub n_features: Option<u32>,
    /// Optional importance threshold for feature selection (features below threshold are removed)
    pub threshold: Option<f64>,
}

/// Feature selection methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeatureSelectionMethod {
    /// Univariate selection
    Univariate,
    /// Recursive feature elimination
    RecursiveElimination,
    /// L1-based selection
    L1Based,
    /// Tree-based selection
    TreeBased,
}

/// Data augmentation configuration for training dataset expansion
///
/// Configures techniques for artificially expanding the training dataset
/// with transformed variations to improve model generalization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAugmentationConfig {
    /// Collection of transformation techniques to apply (rotation, translation, scaling, noise, etc.)
    pub techniques: Vec<AugmentationTechnique>,
    /// Probability (0.0-1.0) that augmentation is applied to each training sample
    pub probability: f64,
    /// Technique-specific parameters controlling transformation intensity and behavior
    pub parameters: HashMap<String, f64>,
}

/// Augmentation techniques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AugmentationTechnique {
    /// Rotation
    Rotation,
    /// Translation
    Translation,
    /// Scaling
    Scaling,
    /// Flipping
    Flipping,
    /// Noise injection
    NoiseInjection,
    /// Cropping
    Cropping,
}

/// Missing value strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MissingValueStrategy {
    /// Remove rows with missing values
    DropRows,
    /// Remove columns with missing values
    DropColumns,
    /// Fill with mean
    FillMean,
    /// Fill with median
    FillMedian,
    /// Fill with mode
    FillMode,
    /// Forward fill
    ForwardFill,
    /// Backward fill
    BackwardFill,
    /// Interpolation
    Interpolate,
}
