"""
Test script to verify the installation of allocation_o2 package.
"""

import sys
import numpy as np

try:
    # Try to import from the installed package
    from allocation_o2 import create_allocator_class
    
    print("Successfully imported allocation_o2")
    
    # Create a simple test strategy
    class TestStrategy:
        def __init__(self):
            self.min_observations = 1
        
        def predict(self, prices):
            # Simple equal weight strategy
            n_assets = prices.shape[0]
            return np.ones(n_assets) / n_assets
    
    # Create allocator class
    TestAllocator = create_allocator_class(
        TestStrategy,
        param_info={
            "min_observations": (int, 1),
        }
    )
    
    # Create allocator instance
    allocator = TestAllocator()
    
    # Generate random price data
    prices = np.random.random((5, 100))  # 5 assets, 100 time steps
    
    # Get allocation weights
    weights = allocator.predict(prices)
    print(f"Generated weights: {weights}")
    print(f"Sum of weights: {np.sum(weights):.6f}")
    
    print("Installation test successful!")
    
except ImportError as e:
    print(f"Error importing modules: {e}")
    print("Make sure you have installed the package with:")
    print("pip install dist/allocation_o2-*.whl")
    sys.exit(1)
except Exception as e:
    print(f"Error during test: {e}")
    sys.exit(1) 