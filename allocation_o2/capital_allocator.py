"""
Capital allocation models abstract base class.
"""

from abc import ABC, abstractmethod
import copy
from numpy.typing import NDArray
import numpy as np


class CapitalAllocator(ABC):
    """
    Abstract base class for capital allocation models.
    
    Capital allocators predict weights for tradable assets based on input data.
    """
    
    @abstractmethod
    def predict(self, x: NDArray[np.float64]) -> NDArray[np.float64]:
        """
        Predict allocation weights based on input data.

        Args:
            - `x` (np.ndarray): Array of data. shape: `(>=min_observations, *n_information)` \
                `n_information` is number of features. Usually `n_information` = `n_tradable`
        Returns:
            np.ndarray: Array of predicted weights. shape: `(time_steps, n_tradable)` where time_steps matches input. 
                Sum of each abs(row) is 1.
        """
        pass

    @property
    @abstractmethod
    def min_observations(self) -> int:  # pragma: no cover
        """
        Minimum number of observations required for prediction.
        
        Returns:
            int: Minimum number of observations
        """
        ...

    def __call__(self, x: NDArray[np.float64]) -> NDArray[np.float64]:
        return self.predict(x)
    
    def __mul__(self, other: float) -> 'CapitalAllocator':
        def model_prediction(x: NDArray[np.float64]) -> NDArray[np.float64]:
            return self.predict(x) * other
        model = copy.deepcopy(self)
        model.predict = model_prediction
        return model
    
    def __rmul__(self, other: float) -> 'CapitalAllocator':
        return self * other
    
    def __truediv__(self, other: float) -> 'CapitalAllocator':
        return self * (1 / other)
    
    def __rtruediv__(self, other: float) -> 'CapitalAllocator':
        return self * (1 / other)
    
    def __neg__(self) -> 'CapitalAllocator':
        def model_prediction(x: NDArray[np.float64]) -> NDArray[np.float64]:
            return -self.predict(x)
        model = copy.deepcopy(self)
        model.predict = model_prediction
        return model 