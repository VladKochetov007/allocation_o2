# AllocationO2
Tactical asset allocation library with Rust backend

## Project Structure

The project consists of two main parts:

1. **Rust Backend** (`rust_backend/`): Core functionality implemented in Rust
   - Fast and efficient portfolio optimization algorithms
   - Asset allocation strategies
   - Portfolio analytics

2. **Python Package** (`allocation_o2/`): Python interface using PyO3
   - User-friendly API for Python users
   - Data visualization and analysis

## Features

- High-performance asset allocation strategies implemented in Rust
- Ability to compile custom Rust strategy files
- Pythonic interface for easy integration with data science workflows

## Installation

### Requirements

- Rust (latest stable)
- Python 3.10+
- C compiler (for building Rust extensions)

### From Source

Clone the repository and install:

```bash
git clone https://github.com/VladKochetov007/allocation_o2
cd AllocationO2
make install
```

### From PyPI

```bash
pip install allocation-o2
```

### Development Mode

For development, install in editable mode:

```bash
make develop
```

## Usage

### Basic Example

```python
import numpy as np
from numpy.typing import NDArray
from allocation_o2 import create_allocator_class

# Create your own allocation strategy
class MyAllocationStrategy:
    @property
    def min_observations(self) -> int:
        return 1
        
    def predict(self, x: NDArray[np.float64]) -> NDArray[np.float64]:
        # Your allocation strategy logic here
        n_assets = x.shape[0]
        return np.ones(n_assets) / n_assets

# Create your own allocation strategy
MyAllocator = create_allocator_class(
    MyAllocationStrategy,
    param_info={
        "min_observations": (int, 1),
    }
)

# Create an allocator instance
allocator = MyAllocator()

# Generate price data
prices = np.random.random((5, 100))  # 5 assets, 100 time steps

# Get allocation weights
weights = allocator.predict(prices)
print(weights)  # Array of weights, sum of which equals 1.0
```

### Custom Rust Strategies

You can create custom allocation strategies directly in Rust for better performance.

#### Compiling a Custom Rust Strategy

Use the command line interface to compile your custom Rust strategy:

```bash
python -m allocation_o2 compile path/to/your_strategy.rs
```

This will compile your Rust file into a shared library (.so) and place it in the same directory. You can specify an alternative output location:

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
}

#[pymethods]
impl MyCustomStrategy {
    #[new]
    fn new() -> Self {
        Self {
            min_observations: 1,
        }
    }
    
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        let input_array = numpy_to_ndarray(py, input)?;
        let output_array = self.predict_impl(&input_array);
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
        let equal_weight = 1.0 / n_assets as f64;
        
        for w in weights.iter_mut() {
            *w = equal_weight;
        }
        
        weights
    }
}

#[pymodule]
fn my_custom_strategy(_py: Python, m: &PyModule) -> PyResult<()> {
    register_strategy!(m, MyCustomStrategy);
    Ok(())
}
```

### Examples

Examples are not included in the package installation and should be created by the user themselves. The repository contains examples for reference:

```bash
# Run random weight example (only from source code)
make build_examples
```

## Creating a wheel package

To create a wheel package for distribution:

```bash
make wheel
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
