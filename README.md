# AllocationO2
A high-performance tactical asset allocation library with Rust backend for financial portfolio optimization

## Overview

AllocationO2 is a hybrid library that combines the speed and efficiency of Rust with the flexibility and ecosystem of Python. It's designed for quantitative analysts, portfolio managers, and financial researchers who need fast and reliable asset allocation algorithms.

The name "AllocationO2" refers to the combination of asset allocation with O2 (PyO3), the framework that enables Rust-Python integration.

## Project Structure

The project consists of two main parts:

1. **Rust Backend** (`rust_backend/`): Core functionality implemented in Rust
   - Fast and efficient portfolio optimization algorithms
   - Asset allocation strategies with minimal overhead
   - High-performance portfolio analytics
   - Memory-safe implementation using Rust's ownership model

2. **Python Package** (`allocation_o2/`): Python interface using PyO3
   - User-friendly API for Python users
   - Seamless integration with NumPy and pandas
   - Data visualization and analysis capabilities
   - Easy integration with data sources and existing Python workflows

## Why Rust + Python?

- **Performance**: Rust provides near-C performance for computationally intensive operations
- **Memory Safety**: Rust's ownership model prevents memory leaks and race conditions
- **Python Ecosystem**: Access to Python's rich data science and finance libraries
- **Ease of Use**: Simple Python API for everyday use with the option to customize in Rust for performance

## Features

- High-performance asset allocation strategies implemented in Rust
- Pythonic interface for easy integration with data science workflows
- Efficient handling of large datasets
- Thread-safe implementation for parallel processing
- Support for various allocation strategies:
  - Equal Weight (simple but effective baseline)
  - Random Weight (with optional seed for reproducibility)
  - Minimum Variance (coming soon)
  - Maximum Sharpe Ratio (coming soon)
  - Risk Parity (coming soon)
  - Hierarchical Risk Parity (coming soon)
- Custom strategy development:
  - Python-based strategies for quick prototyping
  - Rust-based strategies for maximum performance
  - Simple compilation pipeline for Rust strategies

## Installation

### Requirements

- Rust (latest stable) - Install via [rustup](https://rustup.rs/)
- Python 3.10+ - We recommend using a virtual environment
- C compiler (for building Rust extensions):
  - Linux: GCC (usually pre-installed)
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft Visual C++ Build Tools

### From PyPI (Coming Soon)

```bash
pip install allocation-o2
```

### From Source

Clone the repository and install:

```bash
git clone https://github.com/VladKochetov007/allocation_o2
cd AllocationO2
make pip_install
```

### Development Mode

For development, install in editable mode:

```bash
make develop
```

This will install the package in development mode, allowing you to modify the code and see changes without reinstalling.

## API Overview

The library provides several key interfaces:

1. **Strategy Classes**: Python classes that implement allocation logic
2. **Allocator Factory**: Creates optimized allocator classes from strategy definitions
3. **Compilation Tools**: Utilities to compile Rust strategies
4. **Visualization Tools**: (Coming soon) Helpers to visualize allocations and performance

## Usage

### Basic Example with Python Strategy

```python
import numpy as np
from allocation_o2 import create_allocator_class

# Create your own allocation strategy
class MyAllocationStrategy:
    def __init__(self):
        self.min_observations = 1
        
    def predict(self, prices):
        """
        Implement your allocation strategy here.
        
        Parameters:
            prices: numpy array of shape (n_assets, n_observations)
                Historical price data for assets
                
        Returns:
            numpy array of shape (n_assets,)
                Portfolio weights that sum to 1.0
        """
        n_assets = prices.shape[0]
        return np.ones(n_assets) / n_assets  # Equal weight allocation

# Register your strategy with the allocator factory
MyAllocator = create_allocator_class(
    MyAllocationStrategy,
    param_info={
        "min_observations": (int, 1),  # (parameter_type, default_value)
    }
)

# Create an allocator instance
allocator = MyAllocator()

# Generate sample price data
prices = np.random.random((5, 100))  # 5 assets, 100 time steps

# Get allocation weights
weights = allocator.predict(prices)
print(weights)  # Array of weights, sum of which equals 1.0
```

### Advanced Example: Strategy with Parameters

```python
import numpy as np
from allocation_o2 import create_allocator_class

class WeightedMomentumStrategy:
    def __init__(self):
        self.min_observations = 20
        self.lookback_period = 60
        self.momentum_weight = 0.7
        
    def predict(self, prices):
        """Allocate based on recent momentum"""
        # Ensure we have enough data
        n_assets, n_obs = prices.shape
        if n_obs < self.min_observations:
            return np.ones(n_assets) / n_assets
            
        # Calculate returns over the lookback period
        lookback = min(self.lookback_period, n_obs - 1)
        returns = prices[:, -1] / prices[:, -lookback - 1] - 1
        
        # Normalize returns to get weights
        # Handle the case where all returns are the same
        if np.max(returns) - np.min(returns) < 1e-8:
            return np.ones(n_assets) / n_assets
            
        # Assign higher weights to assets with better returns
        weights = (returns - np.min(returns)) / (np.max(returns) - np.min(returns))
        weights = weights / np.sum(weights)
        
        return weights

# Register with more parameters
MomentumAllocator = create_allocator_class(
    WeightedMomentumStrategy,
    param_info={
        "min_observations": (int, 20),
        "lookback_period": (int, 60),
        "momentum_weight": (float, 0.7),
    }
)

# Create with custom parameters
allocator = MomentumAllocator(
    min_observations=15, 
    lookback_period=30
)

# The rest of the code remains the same...
```

### Custom Rust Strategies

For maximum performance, you can implement allocation strategies directly in Rust.

#### Compiling a Custom Rust Strategy

Use the command line interface to compile your custom Rust strategy:

```bash
python -m allocation_o2 compile path/to/your_strategy.rs
```

Or use the Makefile:

```bash
make compile_strategy STRATEGY=path/to/your_strategy.rs
```

This will compile your Rust file into a shared library (.so/.dll/.dylib depending on your OS) and place it in the same directory. You can specify an alternative output location:

```bash
python -m allocation_o2 compile path/to/your_strategy.rs -o path/to/output.so
```

#### Creating a Custom Rust Strategy

To create a custom Rust strategy, use the template in the examples directory (`examples/strategy_template.rs`) as a starting point. Your strategy must implement the `AllocationStrategy` trait and be registered with PyO3.

Example rust strategy:

```rust
use ndarray::ArrayD;
use pyo3::prelude::*;

// Import from allocation_o2
use allocation_o2::allocation::traits::AllocationStrategy;
use allocation_o2::register_strategy;
use allocation_o2::allocation::py_bindings::{numpy_to_ndarray, ndarray_to_numpy};

#[pyclass]
pub struct MyCustomStrategy {
    #[pyo3(get, set)]
    pub min_observations: usize,
    
    #[pyo3(get, set)]
    pub weight_factor: f64,
}

#[pymethods]
impl MyCustomStrategy {
    #[new]
    fn new() -> Self {
        Self {
            min_observations: 1,
            weight_factor: 1.0,
        }
    }
    
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        // Convert from Python (NumPy) to Rust (ndarray)
        let input_array = numpy_to_ndarray(py, input)?;
        // Call the implementation
        let output_array = self.predict_impl(&input_array);
        // Convert back to Python (NumPy)
        ndarray_to_numpy(py, output_array)
    }
}

impl AllocationStrategy for MyCustomStrategy {
    fn min_observations(&self) -> usize {
        self.min_observations
    }
    
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        self.predict_impl(input)
    }
}

impl MyCustomStrategy {
    fn predict_impl(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Your allocation strategy logic here
        // For demonstration, we'll use equal weights
        let shape = input.shape();
        let n_assets = shape[1];
        
        let mut weights = ArrayD::zeros(vec![shape[0], shape[1]]);
        let equal_weight = self.weight_factor / n_assets as f64;
        
        for w in weights.iter_mut() {
            *w = equal_weight;
        }
        
        // Normalize if needed
        if self.weight_factor != 1.0 {
            let sum = weights.sum();
            if sum > 0.0 {
                for w in weights.iter_mut() {
                    *w /= sum;
                }
            }
        }
        
        weights
    }
}

#[pymodule]
fn my_custom_strategy(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register our strategy with PyO3
    register_strategy!(m, MyCustomStrategy);
    Ok(())
}
```

#### Using a Compiled Rust Strategy

Once compiled, you can use your custom Rust strategy from Python:

```python
import numpy as np
from my_custom_strategy import MyCustomStrategy

# Create an instance of your Rust strategy
strategy = MyCustomStrategy()
strategy.min_observations = 10
strategy.weight_factor = 1.2

# Generate sample data
prices = np.random.random((5, 100))  # 5 assets, 100 time steps

# Get allocation weights
weights = strategy.predict(prices)
print(weights)
```

## Performance Benchmarks

Rust strategies typically outperform equivalent Python implementations by 10-100x, depending on the complexity of the algorithm and the dataset size.

Example benchmark (coming soon):
- Equal weight strategy with 100 assets, 1000 time points
  - Python implementation: ~XX ms
  - Rust implementation: ~XX ms

## Creating a wheel package

To create a wheel package for distribution:

```bash
make wheel
```

The resulting wheel file will be in the `dist/` directory.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
