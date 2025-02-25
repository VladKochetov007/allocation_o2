"""
Example of using a custom Rust strategy for random weight allocation.
"""

import numpy as np
import sys
from pathlib import Path

# Add the project root to the path to import the packages
sys.path.insert(0, str(Path(__file__).parent.parent))

# Import the Python frontend package
from allocation_o2 import create_allocator_class

# Try to import the Rust module
try:
    # This assumes the compiled library is in the Python path
    from random_weight_strategy import RandomWeightStrategy
    
    print("Successfully imported RandomWeightStrategy from Rust")
except ImportError as e:
    print(f"Failed to import RandomWeightStrategy: {e}")
    print("Make sure you've built the Rust example with:")
    print("make build_examples")
    sys.exit(1)

# Create a Python allocator class from the Rust strategy
RandomWeightAllocator = create_allocator_class(
    RandomWeightStrategy,
    param_info={
        "min_observations": (int, 1),
        "seed": (int, None),  # Optional seed for reproducibility
    },
    input_shape_desc="[n_assets, time_steps]",
    output_shape_desc="[n_assets]"
)

def main():
    # Create simulated price data for 5 assets over 100 time steps
    n_assets = 2
    time_steps = 10
    
    # Generate random price data
    np.random.seed(42)  # For reproducibility
    prices = np.exp(np.cumsum(
        np.random.normal(0.0005, 0.01, (time_steps, n_assets)), 
        axis=1
    ))
    
    # Create allocator instances with different seeds
    # For the first allocator, we'll just skip the seed parameter to use the default None
    allocator1 = RandomWeightAllocator(seed=None)  # Random weights each time
    # Get weights for the assets
    weights1 = allocator1.predict(prices)
    # Print the weights
    print("\nRandom weights (no seed):")
    print(f"First call:  {weights1}")
    
    # Verify that weights sum to 1.0
    print(f"\nSum of weights (should be all close to 1.0):")
    sum_weights = np.sum(weights1, axis=1)
    print(f"No seed, first call:  {sum_weights}")

if __name__ == "__main__":
    main() 