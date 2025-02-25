use pyo3::prelude::*;

mod allocation;
mod portfolio;
mod optimization;

/// A Python module implemented in Rust.
#[pymodule]
fn allocation_o2(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
