use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{
    event::TwxEvent as RustTwxEvent, property::TwxProperty as RustTwxProperty,
    service::TwxService as RustTwxService,
};

#[pyclass(name = "TwxEvent")]
#[derive(Clone, Debug)]
pub struct PyTwxEvent {
    pub(crate) inner: RustTwxEvent,
}

#[pymethods]
impl PyTwxEvent {
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        match serde_json::from_str::<RustTwxEvent>(json_str) {
            Ok(event) => Ok(PyTwxEvent { inner: event }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Event JSON deserialization error: {}",
                e
            ))),
        }
    }

    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        // Try to interpret bytes as UTF-8 JSON string
        match std::str::from_utf8(data) {
            Ok(json_str) => Self::from_json(json_str),
            Err(_) => Err(PyValueError::new_err("Event data is not valid UTF-8 JSON")),
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let json_str = self.to_json()?;
        Ok(PyBytes::new_bound(py, json_str.as_bytes()))
    }

    fn get_name(&self) -> String {
        self.inner.name.clone()
    }

    fn get_description(&self) -> String {
        self.inner.description.clone()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
    }

    fn __str__(&self) -> String {
        format!(
            "TwxEvent(name='{}', description='{}')",
            self.inner.name, self.inner.description
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "TwxEvent(name='{}', description='{}')",
            self.inner.name, self.inner.description
        )
    }
}

#[pyclass(name = "TwxService")]
#[derive(Clone, Debug)]
pub struct PyTwxService {
    pub(crate) inner: RustTwxService,
}

#[pymethods]
impl PyTwxService {
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        match serde_json::from_str::<RustTwxService>(json_str) {
            Ok(service) => Ok(PyTwxService { inner: service }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Service JSON deserialization error: {}",
                e
            ))),
        }
    }

    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        // Try to interpret bytes as UTF-8 JSON string
        match std::str::from_utf8(data) {
            Ok(json_str) => Self::from_json(json_str),
            Err(_) => Err(PyValueError::new_err(
                "Service data is not valid UTF-8 JSON",
            )),
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let json_str = self.to_json()?;
        Ok(PyBytes::new_bound(py, json_str.as_bytes()))
    }

    fn get_name(&self) -> String {
        self.inner.name.clone()
    }

    fn get_description(&self) -> String {
        self.inner.description.clone()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
    }

    fn __str__(&self) -> String {
        format!(
            "TwxService(name='{}', description='{}')",
            self.inner.name, self.inner.description
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "TwxService(name='{}', description='{}')",
            self.inner.name, self.inner.description
        )
    }
}

#[pyclass(name = "TwxProperty")]
#[derive(Clone, Debug)]
pub struct PyTwxProperty {
    pub(crate) inner: RustTwxProperty,
}

#[pymethods]
impl PyTwxProperty {
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        match serde_json::from_str::<RustTwxProperty>(json_str) {
            Ok(property) => Ok(PyTwxProperty { inner: property }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Property JSON deserialization error: {}",
                e
            ))),
        }
    }

    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        // Try to interpret bytes as UTF-8 JSON string
        match std::str::from_utf8(data) {
            Ok(json_str) => Self::from_json(json_str),
            Err(_) => Err(PyValueError::new_err(
                "Property data is not valid UTF-8 JSON",
            )),
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let json_str = self.to_json()?;
        Ok(PyBytes::new_bound(py, json_str.as_bytes()))
    }

    fn get_name(&self) -> String {
        self.inner.name.clone()
    }

    fn get_base_type(&self) -> String {
        format!("{:?}", self.inner.basetype)
    }

    fn get_push_threshold(&self) -> f64 {
        self.inner.push_threshold
    }

    fn should_read_edge_value(&self) -> bool {
        self.inner.should_read_edge_value()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
    }

    fn __str__(&self) -> String {
        format!(
            "TwxProperty(name='{}', base_type='{:?}', threshold={})",
            self.inner.name, self.inner.basetype, self.inner.push_threshold
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "TwxProperty(name='{}', base_type='{:?}', threshold={})",
            self.inner.name, self.inner.basetype, self.inner.push_threshold
        )
    }
}
