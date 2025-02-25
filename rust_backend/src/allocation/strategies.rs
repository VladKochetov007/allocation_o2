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
    
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64> {
        // Simple equal weight strategy
        let shape = input.shape();
        let n_assets = shape[0];
        
        // Create equal weights
        let mut weights = ArrayD::zeros(vec![n_assets]);
        let weight = 1.0 / n_assets as f64;
        
        for w in weights.iter_mut() {
            *w = weight;
        }
        
        weights
    }
}