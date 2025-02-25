use ndarray::ArrayD;
use pyo3::prelude::*;
use crate::allocation::traits::AllocationStrategy;

/// Example equal weight strategy
#[pyclass]
pub struct EqualWeightStrategy {
    #[pyo3(get, set)]
    pub min_observations: usize,
}

#[pymethods]
impl EqualWeightStrategy {
    #[new]
    pub fn new() -> Self {
        Self {
            min_observations: 1,
        }
    }
}

impl AllocationStrategy for EqualWeightStrategy {
    fn min_observations(&self) -> usize {
        self.min_observations
    }
    
    /// Predict allocation weights based on input data
    ///
    /// # Arguments
    /// * `input` - Input array. Expected shape is [n_observations, n_assets],
    ///   though arbitrary dimensions are supported as long as the assets
    ///   dimension is correctly specified.
    ///
    /// # Returns
    /// * Array with the same shape as input, containing weights that sum to 1.0
    ///   along the assets dimension.
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Get shape information
        let shape = input.shape();
        if shape.len() < 2 {
            // Handle the case of 1D array (just assets, no observations)
            let n_assets = shape[0];
            
            // Create equal weights
            let mut weights = ArrayD::zeros(vec![n_assets]);
            let weight = 1.0 / n_assets as f64;
            
            for w in weights.iter_mut() {
                *w = weight;
            }
            
            return weights;
        }
        
        // For 2D and higher arrays, assume shape [n_observations, n_assets, ...]
        let _n_observations = shape[0];
        let n_assets = shape[1];
        
        // Create output array with the same shape as input
        let mut weights = ArrayD::zeros(shape.to_vec());
        let weight = 1.0 / n_assets as f64;
        
        // Fill with equal weights
        for w in weights.iter_mut() {
            *w = weight;
        }
        
        weights
    }
}