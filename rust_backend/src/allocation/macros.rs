/// Macro for registering a strategy with Python
/// 
/// This macro automatically implements the necessary boilerplate
/// to expose a strategy to Python through PyO3.
#[macro_export]
macro_rules! register_strategy {
    ($module:expr, $strategy:ty) => {
        $module.add_class::<$strategy>()?;
    };
    
    // For multiple strategies
    ($module:expr, $strategy:ty, $($more:ty),+) => {
        $module.add_class::<$strategy>()?;
        $crate::register_strategy!($module, $($more),+);
    };
} 