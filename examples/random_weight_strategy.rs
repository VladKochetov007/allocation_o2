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
    ///
    /// # Arguments
    /// * `input` - Input NumPy array. Expected shape is [n_observations, n_assets],
    ///   though arbitrary dimensions are supported as long as the assets
    ///   dimension is correctly specified.
    ///
    /// # Returns
    /// * NumPy array with the same shape as input, containing weights that sum to 1.0
    ///   along the assets dimension.
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
    /// Helper method to generate random weights
    ///
    /// # Arguments
    /// * `input` - Input array. Expected shape is [n_observations, n_assets],
    ///   though arbitrary dimensions are supported as long as the assets
    ///   dimension is correctly specified.
    ///
    /// # Returns
    /// * Array with the same shape as input, containing weights that sum to 1.0
    ///   along the assets dimension.
    fn generate_random_weights(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Get shape information
        let shape = input.shape();
        
        // Initialize random number generator with seed if provided
        let mut rng = match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        // Distribution for random weights
        let dist = Uniform::from(0.0..1.0);
        
        if shape.len() < 2 {
            // Handle the case of 1D array (just assets, no observations)
            let n_assets = shape[0];
            
            // Create array for weights
            let mut weights = ArrayD::zeros(vec![n_assets]);
            
            // Generate random weights
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
            
            return weights;
        }
        
        // For 2D and higher arrays, assume shape [n_observations, n_assets, ...]
        let n_observations = shape[0];
        let n_assets = shape[1];
        
        // Create output array with the same shape as input
        let weights = ArrayD::zeros(shape.to_vec());
        
        // Reshape view for easier iteration
        let mut weights_view = weights.into_shape((n_observations, n_assets)).unwrap();
        
        // Generate random weights for each observation
        for obs_idx in 0..n_observations {
            let mut sum = 0.0;
            
            // First generate random positive values
            for asset_idx in 0..n_assets {
                let random_value = dist.sample(&mut rng);
                weights_view[[obs_idx, asset_idx]] = random_value;
                sum += random_value;
            }
            
            // Then normalize to ensure they sum to 1.0
            if sum > 0.0 {
                for asset_idx in 0..n_assets {
                    weights_view[[obs_idx, asset_idx]] /= sum;
                }
            } else {
                // Fallback to equal weights if all random values were zero
                let equal_weight = 1.0 / n_assets as f64;
                for asset_idx in 0..n_assets {
                    weights_view[[obs_idx, asset_idx]] = equal_weight;
                }
            }
        }
        
        // Convert back to original shape if needed
        let weights = weights_view.into_shape(shape).unwrap();
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