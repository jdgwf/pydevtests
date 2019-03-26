extern crate pyo3;

use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

use commonlib::structtest::StructTest;

#[pymodule]
fn libpylib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<StructTest>()?;
    Ok(())
}