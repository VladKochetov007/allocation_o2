use ndarray::ArrayD;

/// Trait for allocation strategies
/// 
/// Implementations only need to provide:
/// 1. `min_observations` - minimum number of observations needed
/// 2. `predict` - the core allocation logic
pub trait AllocationStrategy: Send + Sync {
    /// Get minimum number of observations required for prediction
    fn min_observations(&self) -> usize;
    
    /// Predict allocation weights based on input data
    /// 
    /// # Arguments
    /// * `input` - Input array with shape [n_assets, n_features]
    /// 
    /// # Returns
    /// * Array with shape [n_assets] containing weights that sum to 1.0
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64>;
} 