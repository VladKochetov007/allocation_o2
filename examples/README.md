# Custom Allocation Strategies

This directory contains examples of custom allocation strategies implemented in Rust.

## Available Examples

- `random_weight_strategy` - A simple strategy that generates random weights for assets
- `template_strategy` - A template for creating your own custom strategies

## Creating Your Own Strategy

To create your own allocation strategy, follow these steps:

1. Copy `strategy_template.rs` to a new file with a descriptive name, e.g., `my_strategy.rs`
2. Rename the struct and module names to match your strategy
3. Implement your allocation logic in the `predict_impl` method
4. Add your strategy to `rust_backend/Cargo.toml`:

```toml
[[example]]
name = "my_strategy"
path = "../examples/my_strategy.rs"
crate-type = ["cdylib"]
```

5. Build your strategy:

```bash
make build_examples
```

## Strategy Implementation Guide

When implementing your own strategy, you only need to focus on two main aspects:

1. **Parameters**: Add any parameters your strategy needs as fields in your struct with `#[pyo3(get, set)]` attributes
2. **Allocation Logic**: Implement your core allocation logic in the `predict_impl` method

The boilerplate for converting between Python/NumPy arrays and Rust ndarrays is handled automatically.

### Input and Output Format

- **Input**: Typically a 2D array with shape `[n_observations, n_assets]` where:
  - `n_observations` is the number of observation points or samples
  - `n_assets` is the number of assets in the portfolio
  - Arbitrary dimensions are supported as long as the assets dimension is correctly identified
  
- **Output**: Returns an array with the same shape as the input, where weights sum to 1.0 along the assets dimension:
  - For 2D input: Returns shape `[n_observations, n_assets]` where each row is a separate allocation
  - For 1D input (backward compatibility): Returns shape `[n_assets]` containing single portfolio weights

## Using Your Strategy from Python

Once built, you can use your strategy from Python like this:

```python
from allocation_o2 import create_allocator_class
from my_strategy import MyStrategy

# Create a Python allocator class
MyAllocator = create_allocator_class(
    MyStrategy,
    param_info={
        "min_observations": (int, 1),
        "your_parameter": (float, 0.5),  # Default value
    }
)

# Create an instance and use it
allocator = MyAllocator(your_parameter=0.7)
weights = allocator.predict(data)
``` 