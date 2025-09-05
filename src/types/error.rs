use pyo3::prelude::*;

#[pyclass(name = "AlwaysOnError")]
#[derive(Debug)]
pub struct PyAlwaysOnError {
    pub(crate) message: String,
}

#[pymethods]
impl PyAlwaysOnError {
    #[new]
    fn new(message: String) -> Self {
        PyAlwaysOnError { message }
    }

    fn __str__(&self) -> String {
        self.message.clone()
    }

    fn __repr__(&self) -> String {
        format!("AlwaysOnError('{}')", self.message)
    }
}
