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
   - Integration with data sources

## Features

- High-performance asset allocation strategies implemented in Rust
- Pythonic interface for easy integration with data science workflows
- Support for various allocation strategies:
  - Equal Weight
  - Random Weight (with optional seed for reproducibility)
  - More strategies coming soon...

## Installation

### Requirements

- Rust (latest stable)
- Python 3.8+
- C compiler (for building Rust extensions)

### From Source

Clone the repository and install:

```bash
git clone https://github.com/yourusername/AllocationO2.git
cd AllocationO2
make pip_install
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
from allocation_o2 import create_allocator_class

# Создайте свою стратегию аллокации
class MyAllocationStrategy:
    def __init__(self):
        self.min_observations = 1
        
    def predict(self, prices):
        # Ваша логика аллокации
        n_assets = prices.shape[0]
        return np.ones(n_assets) / n_assets

# Создайте класс аллокатора
MyAllocator = create_allocator_class(
    MyAllocationStrategy,
    param_info={
        "min_observations": (int, 1),
    }
)

# Создайте экземпляр аллокатора
allocator = MyAllocator()

# Сгенерируйте данные цен
prices = np.random.random((5, 100))  # 5 активов, 100 временных шагов

# Получите веса аллокации
weights = allocator.predict(prices)
print(weights)  # Массив весов, сумма которых равна 1.0
```

### Examples

Examples are not included in the package installation and should be created by the user themselves. The repository contains examples for reference:

```bash
# Run random weight example (only from source code)
make run_random_example
```

## Creating a wheel package

To create a wheel package for distribution:

```bash
make wheel
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
