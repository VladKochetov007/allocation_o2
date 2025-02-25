"""
Factory for creating allocator classes from Rust strategies.
"""

from inspect import Parameter, Signature
from typing import Any, Protocol, Type, TypeVar, cast
import numpy as np
from numpy.typing import NDArray

from .capital_allocator import CapitalAllocator

try:
    # Import the Rust module
    from .allocation_o2 import NativeAllocator
except ImportError:
    # Fallback for development/testing
    class NativeAllocator:  # type: ignore
        def __init__(self, strategy_class, config=None):
            self.strategy_class = strategy_class
            self.config = config or {}
            
        def predict(self, x):
            return np.zeros((x.shape[0], x.shape[1]))
            
        @property
        def min_observations(self) -> int:
            return 1


class RustStrategy(Protocol):
    """Protocol for Rust-based trading strategies."""
    def min_observations(self) -> int: ...
    def predict(self, prices: NDArray[np.float64]) -> NDArray[np.float64]: ...


T = TypeVar('T', bound=RustStrategy)


def create_allocator_class(
    strategy_class: Type[T],
    param_info: dict[str, tuple[type, Any | None]] | None = None,
    input_shape_desc: str | None = None,
    output_shape_desc: str | None = None,
) -> Type[CapitalAllocator]:
    """
    Creates a wrapper class for a Rust strategy with Pythonic initialization interface.
    
    Args:
        strategy_class: The Rust strategy class to wrap
        param_info: Optional dictionary mapping parameter names to their types and default values
                   Format: {"param_name": (param_type, default_value)}
                   If None, all parameters will be treated as float with no defaults
        input_shape_desc: Optional description of expected input tensor shape
        output_shape_desc: Optional description of expected output tensor shape
    
    Returns:
        A new class that wraps the Rust strategy with proper parameter handling
    
    Example:
        ```python
        # Create allocator class with proper type hints
        MeanReversionAllocator = create_allocator_class(
            MeanReversionStrategy,
            param_info={
                "window": (int, None),           # usize in Rust
                "threshold": (float, 0.02),      # f64 in Rust
                "min_observations": (int, 15),   # usize in Rust
            },
            input_shape_desc="[time_steps, n_assets]",
            output_shape_desc="[n_assets]"
        )

        # Initialize allocator with parameters
        allocator = MeanReversionAllocator(
            window=10,
            threshold=0.02,  # can skip this as it has default value
            min_observations=15  # can skip this as it has default value
        )
        ```
    """
    
    class WrappedAllocator(CapitalAllocator):
        def __init__(self, **kwargs):
            # Convert kwargs to config dict
            config = {}
            
            # If no param_info provided, treat all params as float
            params = param_info or {k: (float, None) for k in kwargs}
            
            for param_name, (param_type, default) in params.items():
                if param_name in kwargs:
                    value = kwargs[param_name]
                    # Type conversion if needed
                    if value is None:
                        # If value is None, pass it directly without type conversion
                        config[param_name] = None
                    else:
                        # Convert to the specified type
                        config[param_name] = param_type(value)
                elif default is not None:
                    config[param_name] = default
                else:
                    raise ValueError(f"Missing required parameter: {param_name}")
            
            # Initialize native allocator with strategy class and config
            self._native_allocator = NativeAllocator(strategy_class, config)
        
        def predict(self, x: NDArray[np.float64]) -> NDArray[np.float64]:
            """
            Predict allocation weights based on input data.
            
            Args:
                x: Input tensor with shape matching input_shape_desc
                
            Returns:
                Allocation weights tensor with shape matching output_shape_desc
            """
            return self._native_allocator.predict(x)
        
        @property
        def min_observations(self) -> int:
            """
            Minimum number of observations required for prediction.
            
            Returns:
                int: Minimum number of observations
            """
            return self._native_allocator.min_observations
    
    # Set proper signature for better IDE support
    if param_info:
        params = []
        for name, (type_, default) in param_info.items():
            kind = Parameter.KEYWORD_ONLY
            default = Parameter.empty if default is None else default
            params.append(Parameter(name, kind, annotation=type_, default=default))
        
        WrappedAllocator.__signature__ = Signature(params)  # type: ignore
    
    # Set proper name and docstring
    WrappedAllocator.__name__ = f"{strategy_class.__name__}Allocator"
    WrappedAllocator.__qualname__ = WrappedAllocator.__name__
    
    # Build docstring with shape information
    shape_info = ""
    if input_shape_desc or output_shape_desc:
        shape_info = "\n\nTensor shapes:\n"
        if input_shape_desc:
            shape_info += f"    Input:  {input_shape_desc}\n"
        if output_shape_desc:
            shape_info += f"    Output: {output_shape_desc}\n"
    
    WrappedAllocator.__doc__ = f"""
    Pythonic wrapper for {strategy_class.__name__} Rust strategy.
    
    All parameters are keyword-only arguments.{shape_info}
    """
    
    return cast(Type[CapitalAllocator], WrappedAllocator) 