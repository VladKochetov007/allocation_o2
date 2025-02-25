use ndarray::{Array, ArrayD, IxDyn};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

pub mod strategies;
pub use strategies::{EqualWeightStrategy};

/// Trait for allocation strategies
pub trait AllocationStrategy: Send + Sync {
    /// Get minimum number of observations required for prediction
    fn min_observations(&self) -> usize;
    
    /// Predict allocation weights based on input data
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64>;
}

/// Python wrapper for Rust allocation strategies
#[pyclass]
pub struct NativeAllocator {
    strategy: Box<dyn AllocationStrategy>,
}

#[pymethods]
impl NativeAllocator {
    #[new]
    fn new(py: Python, strategy_class: PyObject, config: Option<&PyDict>) -> PyResult<Self> {
        // Convert Python config dict to Rust HashMap
        let config_map: HashMap<String, PyObject> = if let Some(dict) = config {
            dict.extract()?
        } else {
            HashMap::new()
        };
        
        // Create strategy instance by calling the Python class
        let strategy_instance = strategy_class.call0(py)?;
        
        // Configure the strategy with parameters
        for (key, value) in config_map {
            strategy_instance.setattr(py, key.as_str(), value)?;
        }
        
        // For now, we'll just use the Python strategy directly
        // In a real implementation, you would extract the strategy from Python
        let strategy: Box<dyn AllocationStrategy> = Box::new(PyStrategy {
            py_obj: strategy_instance,
        });
        
        Ok(Self { strategy })
    }
    
    /// Get minimum number of observations required for prediction
    #[getter]
    fn min_observations(&self) -> usize {
        self.strategy.min_observations()
    }
    
    /// Predict allocation weights based on input data
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        // Convert numpy array to ndarray
        let numpy = py.import("numpy")?;
        let array: &PyAny = numpy.getattr("asarray")?.call1((input,))?;
        let array_f64 = array.getattr("astype")?.call1((numpy.getattr("float64")?,))?;
        
        // Get array shape and data
        let shape: Vec<usize> = array_f64.getattr("shape")?.extract()?;
        let flat_data: Vec<f64> = array_f64.getattr("ravel")?.call0()?.extract()?;
        
        // Create ndarray
        let input_array = Array::from_shape_vec(IxDyn(&shape), flat_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create ndarray: {}", e)))?;
        
        // Call strategy predict method
        let output_array = self.strategy.predict(&input_array);
        
        // Convert back to numpy array
        let output_shape: Vec<usize> = output_array.shape().to_vec();
        let output_flat: Vec<f64> = output_array.into_raw_vec();
        
        // Create numpy array from flat data and reshape
        let output_np = numpy.getattr("array")?.call1((output_flat,))?;
        let reshaped = output_np.getattr("reshape")?.call1((output_shape,))?;
        
        Ok(reshaped.into())
    }
    
    fn __call__(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        self.predict(py, input)
    }
}

// Wrapper for Python strategy objects
struct PyStrategy {
    py_obj: PyObject,
}

impl AllocationStrategy for PyStrategy {
    fn min_observations(&self) -> usize {
        // Get min_observations from Python object
        Python::with_gil(|py| {
            self.py_obj.getattr(py, "min_observations")
                .and_then(|attr| attr.extract::<usize>(py))
                .unwrap_or(1)
        })
    }
    
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Call predict method on Python object
        Python::with_gil(|py| {
            // Convert ndarray to numpy array
            let numpy = py.import("numpy").unwrap();
            let shape = input.shape().to_vec();
            let flat_data = input.clone().into_raw_vec();
            
            // Create numpy array from flat data and reshape
            let array = numpy.getattr("array").unwrap().call1((flat_data,)).unwrap();
            let reshaped = array.getattr("reshape").unwrap().call1((shape,)).unwrap();
            
            // Call predict method
            let result = self.py_obj.call_method1(py, "predict", (reshaped,)).unwrap();
            
            // Convert result back to ndarray
            let result_array = result.extract::<&PyAny>(py).unwrap();
            let result_shape: Vec<usize> = result_array.getattr("shape").unwrap().extract().unwrap();
            let result_flat: Vec<f64> = result_array.getattr("ravel").unwrap().call0().unwrap().extract().unwrap();
            
            Array::from_shape_vec(IxDyn(&result_shape), result_flat).unwrap()
        })
    }
} 