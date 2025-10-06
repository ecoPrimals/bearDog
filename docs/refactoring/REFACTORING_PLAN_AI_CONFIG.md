# AI Config Refactoring Plan

## Current Issues
- 1756 lines in single file (76% over 1000 line limit)
- Flat struct hierarchy
- No builder patterns
- Limited validation
- Many optional fields without clear semantics

## Modern Idiomatic Improvements

### 1. Builder Pattern for Complex Configs
```rust
AiConfig::builder()
    .hybrid_intelligence(|h| h.oversight_level(0.8))
    .training(TrainingConfig::default())
    .build()?
```

### 2. Newtypes for Type Safety
```rust
struct OversightLevel(f64);  // 0.0-1.0, validated
struct LearningRate(f64);     // Positive, validated
struct BatchSize(NonZeroUsize);
```

### 3. Smaller, Focused Modules
- `ai_config/core.rs` - Main config + builder (~200 lines)
- `ai_config/hybrid.rs` - Human-AI collaboration (~150 lines)
- `ai_config/training.rs` - Training configs (~200 lines)
- `ai_config/inference.rs` - Inference configs (~150 lines)  
- `ai_config/neural.rs` - Neural network types (~400 lines)
- `ai_config/management.rs` - Model management (~200 lines)
- `ai_config/types.rs` - Common newtypes and enums (~150 lines)

### 4. Better Validation
- Validate at construction, not runtime
- Use type system to prevent invalid states
- Clear error messages

### 5. Modern Patterns
- `#[non_exhaustive]` on enums for forward compatibility
- Const functions where possible
- Better derives (Eq, Hash where appropriate)
- Serde with validation

## Implementation Order
1. Create newtypes for validated values
2. Add builder patterns
3. Split into logical modules
4. Add comprehensive validation
5. Update tests and documentation

