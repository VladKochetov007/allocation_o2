"""
Example of using a custom Rust strategy for random weight allocation.
"""

import numpy as np
import matplotlib.pyplot as plt
import sys
import os
from pathlib import Path

# Add the project root to the path to import the packages
sys.path.insert(0, str(Path(__file__).parent.parent))

# Import the Python frontend package
from python_frontend.allocation_o2 import create_allocator_class

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
    n_assets = 5
    time_steps = 100
    
    # Generate random price data
    np.random.seed(42)  # For reproducibility
    prices = np.exp(np.cumsum(
        np.random.normal(0.0005, 0.01, (n_assets, time_steps)), 
        axis=1
    ))
    
    # Create allocator instances with different seeds
    # For the first allocator, we'll just skip the seed parameter to use the default None
    allocator1 = RandomWeightAllocator(seed=None)  # Random weights each time
    allocator2 = RandomWeightAllocator(seed=42)  # Fixed seed for reproducibility
    
    # Get weights for the assets
    weights1 = allocator1.predict(prices)
    weights2 = allocator1.predict(prices)  # Different weights each time
    weights3 = allocator2.predict(prices)
    weights4 = allocator2.predict(prices)  # Same weights with fixed seed
    
    # Print the weights
    print("\nRandom weights (no seed):")
    print(f"First call:  {weights1}")
    print(f"Second call: {weights2}")
    print("\nRandom weights (with seed=42):")
    print(f"First call:  {weights3}")
    print(f"Second call: {weights4}")
    
    # Verify that weights sum to 1.0
    print(f"\nSum of weights (should be close to 1.0):")
    print(f"No seed, first call:  {np.sum(weights1):.6f}")
    print(f"No seed, second call: {np.sum(weights2):.6f}")
    print(f"Seed=42, first call:  {np.sum(weights3):.6f}")
    print(f"Seed=42, second call: {np.sum(weights4):.6f}")
    
    # Plot the weights
    plt.figure(figsize=(12, 8))
    
    plt.subplot(2, 2, 1)
    plt.bar(range(n_assets), weights1)
    plt.title("Random Weights (No Seed) - First Call")
    plt.xlabel("Asset")
    plt.ylabel("Weight")
    
    plt.subplot(2, 2, 2)
    plt.bar(range(n_assets), weights2)
    plt.title("Random Weights (No Seed) - Second Call")
    plt.xlabel("Asset")
    plt.ylabel("Weight")
    
    plt.subplot(2, 2, 3)
    plt.bar(range(n_assets), weights3)
    plt.title("Random Weights (Seed=42) - First Call")
    plt.xlabel("Asset")
    plt.ylabel("Weight")
    
    plt.subplot(2, 2, 4)
    plt.bar(range(n_assets), weights4)
    plt.title("Random Weights (Seed=42) - Second Call")
    plt.xlabel("Asset")
    plt.ylabel("Weight")
    
    plt.tight_layout()
    plt.savefig("random_weights.png")
    plt.show()

if __name__ == "__main__":
    main() 