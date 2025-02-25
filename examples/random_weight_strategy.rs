use ndarray::ArrayD;
use pyo3::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Strategy that returns random weights
#[pyclass]
pub struct RandomWeightStrategy {
    #[pyo3(get, set)]
    pub min_observations: usize,
    
    #[pyo3(get, set)]
    pub seed: Option<u64>,
}

#[pymethods]
impl RandomWeightStrategy {
    #[new]
    fn new() -> Self {
        Self {
            min_observations: 1,
            seed: None,
        }
    }
    
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        // Convert numpy array to ndarray
        let numpy = py.import("numpy")?;
        let array: &PyAny = numpy.getattr("asarray")?.call1((input,))?;
        let array_f64 = array.getattr("astype")?.call1((numpy.getattr("float64")?,))?;
        
        // Get array shape and data
        let shape: Vec<usize> = array_f64.getattr("shape")?.extract()?;
        let flat_data: Vec<f64> = array_f64.getattr("ravel")?.call0()?.extract()?;
        
        // Create ndarray
        let input_array = ndarray::Array::from_shape_vec(ndarray::IxDyn(&shape), flat_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create ndarray: {}", e)))?;
        
        // Generate random weights
        let output_array = self.generate_random_weights(&input_array);
        
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

impl RandomWeightStrategy {
    // Helper method to generate random weights
    fn generate_random_weights(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Get number of assets from input shape
        let shape = input.shape();
        let n_assets = shape[0];
        
        // Create array for weights
        let mut weights = ArrayD::zeros(vec![n_assets]);
        
        // Initialize random number generator with seed if provided
        let mut rng = match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        // Generate random weights
        let dist = Uniform::from(0.0..1.0);
        let mut sum = 0.0;
        
        // First generate random positive values
        for w in weights.iter_mut() {
            let random_value = dist.sample(&mut rng);
            *w = random_value;
            sum += random_value;
        }
        
        // Then normalize to ensure they sum to 1.0
        if sum > 0.0 {
            for w in weights.iter_mut() {
                *w /= sum;
            }
        } else {
            // Fallback to equal weights if all random values were zero
            let equal_weight = 1.0 / n_assets as f64;
            for w in weights.iter_mut() {
                *w = equal_weight;
            }
        }
        
        weights
    }
}

// Register the module with Python
#[pymodule]
fn random_weight_strategy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RandomWeightStrategy>()?;
    Ok(())
} 