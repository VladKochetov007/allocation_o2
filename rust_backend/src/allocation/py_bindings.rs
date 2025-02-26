use ndarray::{Array, ArrayD, IxDyn};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::allocation::traits::AllocationStrategy;

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
        
        // Create strategy instance by calling the Python class with config parameters
        let strategy_instance = if !config_map.is_empty() {
            // Create a new dictionary for constructor arguments
            let kwargs = PyDict::new(py);
            for (key, value) in &config_map {
                kwargs.set_item(key.as_str(), value.clone_ref(py))?;
            }
            strategy_class.call(py, (), Some(kwargs))?
        } else {
            strategy_class.call0(py)?
        };
        
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
        let input_array = numpy_to_ndarray(py, input)?;
        
        // Call strategy predict method
        let output_array = self.strategy.predict(&input_array);
        
        // Convert back to numpy array
        let output = ndarray_to_numpy(py, output_array)?;
        
        Ok(output)
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
            // Convert ndarray to numpy
            let numpy_input = ndarray_to_numpy(py, input.clone()).unwrap();
            
            // Call predict method
            let result = self.py_obj.call_method1(py, "predict", (numpy_input,)).unwrap();
            
            // Convert result back to ndarray
            let result_any = result.extract::<&PyAny>(py).unwrap();
            numpy_to_ndarray(py, result_any).unwrap()
        })
    }
}

/// Convert NumPy array to ndarray
pub fn numpy_to_ndarray(py: Python, array: &PyAny) -> PyResult<ArrayD<f64>> {
    let numpy = py.import("numpy")?;
    let array: &PyAny = numpy.getattr("asarray")?.call1((array,))?;
    let array_f64 = array.getattr("astype")?.call1((numpy.getattr("float64")?,))?;
    
    // Get array shape and data
    let shape: Vec<usize> = array_f64.getattr("shape")?.extract()?;
    let flat_data: Vec<f64> = array_f64.getattr("ravel")?.call0()?.extract()?;
    
    // Create ndarray
    Array::from_shape_vec(IxDyn(&shape), flat_data)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create ndarray: {}", e)))
}

/// Convert ndarray to NumPy array
pub fn ndarray_to_numpy(py: Python, array: ArrayD<f64>) -> PyResult<PyObject> {
    let numpy = py.import("numpy")?;
    
    // Get shape and data
    let shape: Vec<usize> = array.shape().to_vec();
    let flat_data: Vec<f64> = array.into_raw_vec();
    
    // Create numpy array from flat data and reshape
    let array_np = numpy.getattr("array")?.call1((flat_data,))?;
    let reshaped = array_np.getattr("reshape")?.call1((shape,))?;
    
    Ok(reshaped.into())
} 