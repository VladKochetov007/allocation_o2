use ndarray::ArrayD;
use pyo3::prelude::*;

// Import from allocation_o2
use allocation_o2::allocation::traits::AllocationStrategy;
use allocation_o2::register_strategy;
use allocation_o2::allocation::py_bindings::{numpy_to_ndarray, ndarray_to_numpy};

/// Template for a custom allocation strategy
/// 
/// This is a starting point for creating your own allocation strategies.
/// You only need to focus on the core allocation logic in the predict method.
#[pyclass]
pub struct TemplateStrategy {
    #[pyo3(get, set)]
    pub min_observations: usize,
    
    // Add your custom parameters here
    #[pyo3(get, set)]
    pub example_parameter: f64,
}

#[pymethods]
impl TemplateStrategy {
    #[new]
    fn new() -> Self {
        Self {
            min_observations: 1,
            example_parameter: 0.5,
        }
    }
    
    /// Python-exposed predict method
    /// 
    /// This method handles conversion between Python/NumPy arrays and Rust ndarrays.
    /// You generally don't need to modify this - it calls your AllocationStrategy implementation.
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        // Convert Python/NumPy data to ndarray
        let input_array = numpy_to_ndarray(py, input)?;
        
        // Call the implementation
        let output_array = self.predict_impl(&input_array);
        
        // Convert back to Python/NumPy
        ndarray_to_numpy(py, output_array)
    }
}

// Implement the AllocationStrategy trait
impl AllocationStrategy for TemplateStrategy {
    fn min_observations(&self) -> usize {
        self.min_observations
    }
    
    // Core allocation logic that works directly with ndarray
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        self.predict_impl(input)
    }
}

impl TemplateStrategy {
    // Implementation of your allocation strategy
    fn predict_impl(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Get shape information
        let shape = input.shape();
        
        // For 2D arrays with shape [n_observations, n_assets]
        let n_observations = shape[0];
        let n_assets = shape[1];
        
        // Create output array with the same shape
        let mut weights = ArrayD::zeros(vec![n_observations, n_assets]);
        
        // TODO: Implement your allocation strategy here
        
        // For this template, we'll use equal weights for each observation
        let equal_weight = 1.0 / n_assets as f64;
        for w in weights.iter_mut() {
            *w = equal_weight;
        }
        
        weights
    }
}

// Register the module with Python
#[pymodule]
fn template_strategy(_py: Python, m: &PyModule) -> PyResult<()> {
    // Use the register_strategy macro
    register_strategy!(m, TemplateStrategy);
    Ok(())
} 