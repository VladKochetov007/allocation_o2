use pyo3::prelude::*;

pub mod allocation;

use allocation::{NativeAllocator, EqualWeightStrategy};

/// A Python module implemented in Rust.
#[pymodule]
fn allocation_o2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NativeAllocator>()?;
    m.add_class::<EqualWeightStrategy>()?;
    Ok(())
}
