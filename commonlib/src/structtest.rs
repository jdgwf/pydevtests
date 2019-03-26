// use pyo3::prelude::*;

// #[pyclass]
pub struct StructTest {
    pub name: String
}

// #[pymethods]
impl StructTest {
    // #[new]
    // pub fn pynew(obj: &PyRawObject) {
    //     obj.init( StructTest{
    //         name: "".to_string()
    //         }
    //     )
    // }

    pub fn new() -> StructTest {
        StructTest{ name: "".to_string() }
    }

    pub fn say_hello( &self ) -> String {
        format!("Hello, {}", self.name )
    }
}