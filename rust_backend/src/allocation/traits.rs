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
    /// * `input` - Input array. Expected shape is typically [n_observations, n_assets],
    ///   though implementations may support arbitrary dimensions as long as the assets
    ///   dimension is correctly identified.
    /// 
    /// # Returns
    /// * Array with the same shape as input, containing weights that sum to 1.0
    ///   along the assets dimension.
    fn predict(&self, input: &ArrayD<f64>) -> ArrayD<f64>;
} 