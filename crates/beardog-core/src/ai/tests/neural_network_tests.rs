// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for Neural Network module

use crate::ai::hybrid_intelligence::neural_networks::*;

#[test]
fn test_activation_function_variants() {
    let functions = [
        ActivationFunction::Sigmoid,
        ActivationFunction::Tanh,
        ActivationFunction::Relu,
        ActivationFunction::LeakyRelu,
        ActivationFunction::Elu,
        ActivationFunction::Selu,
        ActivationFunction::Softmax,
        ActivationFunction::Linear,
    ];

    assert_eq!(functions.len(), 8);
}

#[test]
fn test_activation_function_serialization() {
    let func = ActivationFunction::Relu;
    let serialized = serde_json::to_string(&func).unwrap();
    assert!(serialized.contains("Relu"));
}

#[test]
fn test_data_type_variants() {
    let types = [
        DataType::Float16,
        DataType::Float32,
        DataType::Float64,
        DataType::Int8,
        DataType::Int16,
        DataType::Int32,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_architecture_type_variants() {
    let types = [
        ArchitectureType::Feedforward,
        ArchitectureType::Convolutional,
        ArchitectureType::Recurrent,
        ArchitectureType::Transformer,
        ArchitectureType::Hybrid,
    ];

    assert_eq!(types.len(), 5);
}

#[test]
fn test_loss_function_variants() {
    let functions = [
        LossFunction::MeanSquaredError,
        LossFunction::MeanAbsoluteError,
        LossFunction::CrossEntropy,
        LossFunction::BinaryCrossEntropy,
        LossFunction::BinaryCrossentropy,
        LossFunction::CategoricalCrossEntropy,
        LossFunction::Huber,
    ];

    assert_eq!(functions.len(), 7);
}

#[test]
fn test_layer_type_variants() {
    let types = [
        LayerType::Dense,
        LayerType::Conv2d,
        LayerType::Conv1d,
        LayerType::Conv3d,
        LayerType::MaxPool2d,
        LayerType::AvgPool2d,
        LayerType::GlobalAvgPool,
        LayerType::Dropout,
        LayerType::Lstm,
        LayerType::Gru,
        LayerType::Attention,
        LayerType::MultiHeadAttention,
        LayerType::Embedding,
        LayerType::BatchNorm,
        LayerType::LayerNorm,
    ];

    assert_eq!(types.len(), 15);
}

#[test]
fn test_padding_type_variants() {
    let types = [
        PaddingType::Valid,
        PaddingType::Same,
        PaddingType::Custom(2),
    ];

    assert_eq!(types.len(), 3);
}

#[test]
fn test_rnn_cell_type_variants() {
    let types = [RnnCellType::SimpleRnn, RnnCellType::Lstm, RnnCellType::Gru];

    assert_eq!(types.len(), 3);
}

#[test]
fn test_weight_initialization_zeros() {
    let init = WeightInitialization::Zeros;
    assert_eq!(init, WeightInitialization::Zeros);
}

#[test]
fn test_weight_initialization_ones() {
    let init = WeightInitialization::Ones;
    assert_eq!(init, WeightInitialization::Ones);
}

#[test]
fn test_input_layer_config_creation() {
    let input_config = InputLayerConfig {
        shape: vec![28, 28, 1],
        input_shape: vec![28, 28, 1],
        data_type: DataType::Float32,
        normalization: Some("batch".to_string()),
    };

    assert_eq!(input_config.shape, vec![28, 28, 1]);
    assert_eq!(input_config.normalization, Some("batch".to_string()));
}

#[test]
fn test_output_layer_config_creation() {
    let output_config = OutputLayerConfig {
        units: 10,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::CategoricalCrossEntropy,
    };

    assert_eq!(output_config.units, 10);
}

#[test]
fn test_dense_layer_config_creation() {
    let dense_config = DenseLayerConfig {
        units: 128,
        use_bias: true,
        weight_init: WeightInitialization::Zeros,
        bias_init: WeightInitialization::Zeros,
    };

    assert_eq!(dense_config.units, 128);
    assert!(dense_config.use_bias);
}

#[test]
fn test_conv_layer_config_creation() {
    let conv_config = ConvLayerConfig {
        filters: 64,
        kernel_size: vec![3, 3],
        strides: vec![1, 1],
        padding: PaddingType::Same,
        dilation_rate: vec![1, 1],
        use_bias: true,
    };

    assert_eq!(conv_config.filters, 64);
    assert_eq!(conv_config.kernel_size, vec![3, 3]);
}

#[test]
fn test_pooling_layer_config_creation() {
    let pooling_config = PoolingLayerConfig {
        pool_size: vec![2, 2],
        strides: vec![2, 2],
        padding: PaddingType::Valid,
    };

    assert_eq!(pooling_config.pool_size, vec![2, 2]);
}

#[test]
fn test_rnn_layer_config_creation() {
    let rnn_config = RnnLayerConfig {
        units: 64,
        cell_type: RnnCellType::Lstm,
        return_sequences: true,
        dropout_rate: 0.2,
    };

    assert_eq!(rnn_config.units, 64);
    assert!(rnn_config.return_sequences);
}

#[test]
fn test_attention_layer_config_creation() {
    let attention_config = AttentionLayerConfig {
        num_heads: 8,
        key_dim: 64,
        value_dim: Some(64),
        dropout_rate: 0.1,
    };

    assert_eq!(attention_config.num_heads, 8);
    assert_eq!(attention_config.key_dim, 64);
}

#[test]
fn test_embedding_layer_config_creation() {
    let embedding_config = EmbeddingLayerConfig {
        input_dim: 10000,
        output_dim: 128,
        mask_zero: true,
        input_length: Some(100),
    };

    assert_eq!(embedding_config.input_dim, 10000);
    assert_eq!(embedding_config.output_dim, 128);
}

#[test]
fn test_layer_parameters_with_dense() {
    let params = LayerParameters {
        dense: Some(DenseLayerConfig {
            units: 256,
            use_bias: true,
            weight_init: WeightInitialization::Zeros,
            bias_init: WeightInitialization::Zeros,
        }),
        conv: None,
        pooling: None,
        rnn: None,
        attention: None,
        embedding: None,
    };

    assert!(params.dense.is_some());
    assert_eq!(params.dense.unwrap().units, 256);
}

#[test]
fn test_layer_parameters_with_conv() {
    let params = LayerParameters {
        dense: None,
        conv: Some(ConvLayerConfig {
            filters: 32,
            kernel_size: vec![5, 5],
            strides: vec![1, 1],
            padding: PaddingType::Same,
            dilation_rate: vec![1, 1],
            use_bias: true,
        }),
        pooling: None,
        rnn: None,
        attention: None,
        embedding: None,
    };

    assert!(params.conv.is_some());
    assert_eq!(params.conv.unwrap().filters, 32);
}

#[test]
fn test_layer_config_creation() {
    let layer_config = LayerConfig {
        layer_type: LayerType::Dense,
        parameters: LayerParameters {
            dense: Some(DenseLayerConfig {
                units: 64,
                use_bias: true,
                weight_init: WeightInitialization::Zeros,
                bias_init: WeightInitialization::Zeros,
            }),
            conv: None,
            pooling: None,
            rnn: None,
            attention: None,
            embedding: None,
        },
    };

    assert!(layer_config.parameters.dense.is_some());
}

#[test]
fn test_network_architecture_feedforward() {
    let input_layer = InputLayerConfig {
        shape: vec![784],
        input_shape: vec![784],
        data_type: DataType::Float32,
        normalization: None,
    };

    let output_layer = OutputLayerConfig {
        units: 10,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::CategoricalCrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Feedforward,
        input_layer,
        hidden_layers: vec![],
        output_layer,
        skip_connections: vec![],
    };

    // Verify architecture fields are accessible
    assert_eq!(architecture.output_layer.units, 10);
    assert_eq!(architecture.hidden_layers.len(), 0);
}

#[test]
fn test_network_architecture_convolutional() {
    let input_layer = InputLayerConfig {
        shape: vec![28, 28, 1],
        input_shape: vec![28, 28, 1],
        data_type: DataType::Float32,
        normalization: Some("batch".to_string()),
    };

    let conv_layer = LayerParameters {
        dense: None,
        conv: Some(ConvLayerConfig {
            filters: 32,
            kernel_size: vec![3, 3],
            strides: vec![1, 1],
            padding: PaddingType::Same,
            dilation_rate: vec![1, 1],
            use_bias: true,
        }),
        pooling: None,
        rnn: None,
        attention: None,
        embedding: None,
    };

    let output_layer = OutputLayerConfig {
        units: 10,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::CategoricalCrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Convolutional,
        input_layer,
        hidden_layers: vec![conv_layer],
        output_layer,
        skip_connections: vec![],
    };

    assert_eq!(architecture.hidden_layers.len(), 1);
}

#[test]
fn test_network_architecture_recurrent() {
    let input_layer = InputLayerConfig {
        shape: vec![100, 50],
        input_shape: vec![100, 50],
        data_type: DataType::Float32,
        normalization: None,
    };

    let rnn_layer = LayerParameters {
        dense: None,
        conv: None,
        pooling: None,
        rnn: Some(RnnLayerConfig {
            units: 128,
            cell_type: RnnCellType::Lstm,
            return_sequences: false,
            dropout_rate: 0.2,
        }),
        attention: None,
        embedding: None,
    };

    let output_layer = OutputLayerConfig {
        units: 2,
        activation: ActivationFunction::Sigmoid,
        loss_function: LossFunction::BinaryCrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Recurrent,
        input_layer,
        hidden_layers: vec![rnn_layer],
        output_layer,
        skip_connections: vec![],
    };

    // Verify RNN layer configuration
    assert!(architecture.hidden_layers[0].rnn.is_some());
    assert_eq!(
        architecture.hidden_layers[0].rnn.as_ref().unwrap().units,
        128
    );
}

#[test]
fn test_network_architecture_transformer() {
    let input_layer = InputLayerConfig {
        shape: vec![512, 768],
        input_shape: vec![512, 768],
        data_type: DataType::Float32,
        normalization: Some("layer".to_string()),
    };

    let attention_layer = LayerParameters {
        dense: None,
        conv: None,
        pooling: None,
        rnn: None,
        attention: Some(AttentionLayerConfig {
            num_heads: 12,
            key_dim: 64,
            value_dim: Some(64),
            dropout_rate: 0.1,
        }),
        embedding: None,
    };

    let output_layer = OutputLayerConfig {
        units: 30000,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::CrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Transformer,
        input_layer,
        hidden_layers: vec![attention_layer],
        output_layer,
        skip_connections: vec![],
    };

    // Verify attention layer configuration
    assert!(architecture.hidden_layers[0].attention.is_some());
    assert_eq!(
        architecture.hidden_layers[0]
            .attention
            .as_ref()
            .unwrap()
            .num_heads,
        12
    );
}

#[test]
fn test_training_params_default() {
    let params = TrainingParams::default();

    // Should have reasonable defaults
    assert_eq!(params.batch_size, 32);
    assert_eq!(params.epochs, 100);
    assert!(params.learning_rate > 0.0);
}

#[test]
fn test_optimizer_type_variants() {
    let optimizers = [
        OptimizerType::Sgd,
        OptimizerType::Adam,
        OptimizerType::AdamW,
        OptimizerType::RmsProp,
        OptimizerType::Adagrad,
        OptimizerType::Adadelta,
    ];

    assert_eq!(optimizers.len(), 6);
}

#[test]
fn test_optimizer_default() {
    let optimizer = Optimizer::default();

    // Verify optimizer has default values
    assert!(optimizer.parameters.is_empty());
}

#[test]
fn test_network_optimization_creation() {
    let optimization = NetworkOptimization {
        mixed_precision: true,
        gradient_clipping: None,
        batch_size_optimization: true,
        memory_optimization: true,
    };

    assert!(optimization.mixed_precision);
}

#[test]
fn test_network_regularization_creation() {
    let regularization = NetworkRegularization {
        dropout: None,
        batch_normalization: true,
        weight_decay: 0.01,
        early_stopping: None,
    };

    assert!((regularization.weight_decay - 0.01).abs() < 1e-6);
}

#[test]
fn test_multi_layer_network() {
    let input_layer = InputLayerConfig {
        shape: vec![224, 224, 3],
        input_shape: vec![224, 224, 3],
        data_type: DataType::Float32,
        normalization: Some("batch".to_string()),
    };

    let layer1 = LayerParameters {
        dense: None,
        conv: Some(ConvLayerConfig {
            filters: 64,
            kernel_size: vec![3, 3],
            strides: vec![1, 1],
            padding: PaddingType::Same,
            dilation_rate: vec![1, 1],
            use_bias: true,
        }),
        pooling: None,
        rnn: None,
        attention: None,
        embedding: None,
    };

    let layer2 = LayerParameters {
        dense: None,
        conv: None,
        pooling: Some(PoolingLayerConfig {
            pool_size: vec![2, 2],
            strides: vec![2, 2],
            padding: PaddingType::Valid,
        }),
        rnn: None,
        attention: None,
        embedding: None,
    };

    let output_layer = OutputLayerConfig {
        units: 1000,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::CategoricalCrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Convolutional,
        input_layer,
        hidden_layers: vec![layer1, layer2],
        output_layer,
        skip_connections: vec!["layer_0_to_layer_2".to_string()],
    };

    assert_eq!(architecture.hidden_layers.len(), 2);
    assert_eq!(architecture.skip_connections.len(), 1);
}

#[test]
fn test_embedding_network() {
    let input_layer = InputLayerConfig {
        shape: vec![100],
        input_shape: vec![100],
        data_type: DataType::Int32,
        normalization: None,
    };

    let embedding_layer = LayerParameters {
        dense: None,
        conv: None,
        pooling: None,
        rnn: None,
        attention: None,
        embedding: Some(EmbeddingLayerConfig {
            input_dim: 10000,
            output_dim: 256,
            mask_zero: true,
            input_length: Some(100),
        }),
    };

    let output_layer = OutputLayerConfig {
        units: 2,
        activation: ActivationFunction::Softmax,
        loss_function: LossFunction::BinaryCrossEntropy,
    };

    let architecture = NetworkArchitecture {
        architecture_type: ArchitectureType::Recurrent,
        input_layer,
        hidden_layers: vec![embedding_layer],
        output_layer,
        skip_connections: vec![],
    };

    assert!(architecture.hidden_layers[0].embedding.is_some());
}
