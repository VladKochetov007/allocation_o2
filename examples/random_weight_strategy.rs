use ndarray::ArrayD;
use pyo3::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

// Import from allocation_o2
use allocation_o2::allocation::traits::AllocationStrategy;
use allocation_o2::register_strategy;
use allocation_o2::allocation::py_bindings::{numpy_to_ndarray, ndarray_to_numpy};

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
    
    /// Predict allocation weights based on input data
    fn predict(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        // Convert Python/NumPy data to ndarray
        let input_array = numpy_to_ndarray(py, input)?;
        
        // Call the implementation
        let output_array = self.generate_random_weights(&input_array);
        
        // Convert back to Python/NumPy
        ndarray_to_numpy(py, output_array)
    }
}

// Implement the AllocationStrategy trait for RandomWeightStrategy
impl AllocationStrategy for RandomWeightStrategy {
    fn min_observations(&self) -> usize {
        self.min_observations
    }
    
    // Predict function that works directly with ndarray
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        self.generate_random_weights(input)
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
    // Use the register_strategy macro
    register_strategy!(m, RandomWeightStrategy);
    Ok(())
} 