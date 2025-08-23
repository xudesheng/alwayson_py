#![allow(non_local_definitions)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::uninlined_format_args)]

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{
    base::BaseType as RustBaseType, event::TwxEvent as RustTwxEvent,
    message::tw_message::TwxMsg as RustTwxMsg, primitive::TwPrim as RustTwPrim,
    property::TwxProperty as RustTwxProperty, service::TwxService as RustTwxService, BytesStream,
};
use bytes::{Bytes, BytesMut};

#[pyclass(name = "BaseType")]
#[derive(Clone, Debug)]
pub struct PyBaseType {
    inner: RustBaseType,
}

#[pymethods]
impl PyBaseType {
    #[new]
    fn new(type_name: &str) -> PyResult<Self> {
        let base_type = match type_name.to_uppercase().as_str() {
            "BOOLEAN" => RustBaseType::BOOLEAN,
            "INTEGER" => RustBaseType::INTEGER,
            "LONG" => RustBaseType::LONG,
            "NUMBER" => RustBaseType::NUMBER,
            "STRING" => RustBaseType::STRING,
            "DATETIME" => RustBaseType::DATETIME,
            "BLOB" => RustBaseType::BLOB,
            "LOCATION" => RustBaseType::LOCATION,
            "INFOTABLE" => RustBaseType::INFOTABLE,
            "VARIANT" => RustBaseType::VARIANT,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Invalid base type: {type_name}"
                )))
            }
        };
        Ok(PyBaseType { inner: base_type })
    }

    #[classattr]
    const BOOLEAN: &'static str = "BOOLEAN";
    #[classattr]
    const INTEGER: &'static str = "INTEGER";
    #[classattr]
    const LONG: &'static str = "LONG";
    #[classattr]
    const NUMBER: &'static str = "NUMBER";
    #[classattr]
    const STRING: &'static str = "STRING";
    #[classattr]
    const DATETIME: &'static str = "DATETIME";
    #[classattr]
    const BLOB: &'static str = "BLOB";
    #[classattr]
    const LOCATION: &'static str = "LOCATION";
    #[classattr]
    const INFOTABLE: &'static str = "INFOTABLE";
    #[classattr]
    const VARIANT: &'static str = "VARIANT";

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("BaseType.{:?}", self.inner)
    }
}

#[pyclass(name = "TwPrim")]
#[derive(Clone, Debug)]
pub struct PyTwPrim {
    inner: RustTwPrim,
}

#[pymethods]
impl PyTwPrim {
    #[staticmethod]
    fn boolean(value: bool) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::BOOLEAN(RustBaseType::BOOLEAN, value),
        })
    }

    #[staticmethod]
    fn integer(value: i32) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::INTEGER(RustBaseType::INTEGER, value),
        })
    }

    #[staticmethod]
    fn long(value: i64) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::LONG(RustBaseType::LONG, value),
        })
    }

    #[staticmethod]
    fn number(value: f64) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::NUMBER(RustBaseType::NUMBER, value),
        })
    }

    #[staticmethod]
    fn string(value: String) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::STRING(RustBaseType::STRING, value),
        })
    }

    #[staticmethod]
    fn datetime(value: i64) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::DATETIME(RustBaseType::DATETIME, value),
        })
    }

    #[staticmethod]
    fn blob(value: Vec<u8>) -> PyResult<Self> {
        Ok(PyTwPrim {
            inner: RustTwPrim::BLOB(RustBaseType::BLOB, Bytes::from(value)),
        })
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut content = BytesMut::new();
        match self.inner.to_bytes(&mut content) {
            Ok(_) => Ok(PyBytes::new_bound(py, &content)),
            Err(e) => Err(PyValueError::new_err(format!(
                "Binary serialization error: {}",
                e
            ))),
        }
    }

    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        match RustTwPrim::from_bytes(data) {
            Ok((prim, _consumed)) => Ok(PyTwPrim { inner: prim }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Binary deserialization error: {}",
                e
            ))),
        }
    }

    fn to_json(&self) -> PyResult<String> {
        let json_value = self.to_json_value()?;
        serde_json::to_string(&json_value)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
    }

    fn get_type(&self) -> String {
        match &self.inner {
            RustTwPrim::BOOLEAN(_, _) => "BOOLEAN".to_string(),
            RustTwPrim::INTEGER(_, _) => "INTEGER".to_string(),
            RustTwPrim::LONG(_, _) => "LONG".to_string(),
            RustTwPrim::NUMBER(_, _) => "NUMBER".to_string(),
            RustTwPrim::STRING(_, _) => "STRING".to_string(),
            RustTwPrim::DATETIME(_, _) => "DATETIME".to_string(),
            RustTwPrim::BLOB(_, _) => "BLOB".to_string(),
            RustTwPrim::LOCATION(..) => "LOCATION".to_string(),
            RustTwPrim::INFOTABLE(_, _) => "INFOTABLE".to_string(),
            RustTwPrim::VARIANT(_, _) => "VARIANT".to_string(),
            RustTwPrim::NOTHING(_) => "NOTHING".to_string(),
        }
    }

    fn get_value(&self, py: Python) -> PyResult<PyObject> {
        match &self.inner {
            RustTwPrim::BOOLEAN(_, v) => Ok(v.to_object(py)),
            RustTwPrim::INTEGER(_, v) => Ok(v.to_object(py)),
            RustTwPrim::LONG(_, v) => Ok(v.to_object(py)),
            RustTwPrim::NUMBER(_, v) => Ok(v.to_object(py)),
            RustTwPrim::STRING(_, v) => Ok(v.to_object(py)),
            RustTwPrim::DATETIME(_, v) => Ok(v.to_object(py)),
            RustTwPrim::BLOB(_, v) => Ok(PyBytes::new_bound(py, v.as_ref()).to_object(py)),
            RustTwPrim::NOTHING(_) => Ok(py.None()),
            _ => Err(PyTypeError::new_err("Unsupported type for get_value")),
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("TwPrim({:?})", self.inner)
    }
}

impl PyTwPrim {
    fn to_json_value(&self) -> PyResult<serde_json::Value> {
        match &self.inner {
            RustTwPrim::BOOLEAN(_, v) => Ok(serde_json::Value::Bool(*v)),
            RustTwPrim::INTEGER(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::LONG(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::NUMBER(_, v) => serde_json::Number::from_f64(*v)
                .map(serde_json::Value::Number)
                .ok_or_else(|| PyValueError::new_err("Invalid float value")),
            RustTwPrim::STRING(_, v) => Ok(serde_json::Value::String(v.clone())),
            RustTwPrim::DATETIME(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::BLOB(_, _) => Err(PyTypeError::new_err("Cannot convert BLOB to JSON")),
            RustTwPrim::NOTHING(_) => Ok(serde_json::Value::Null),
            _ => Err(PyTypeError::new_err("Cannot convert to JSON")),
        }
    }
}

#[pyclass(name = "TwxMessage")]
#[derive(Clone, Debug)]
pub struct PyTwxMessage {
    inner: RustTwxMsg,
}

#[pymethods]
impl PyTwxMessage {
    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        match RustTwxMsg::from_bytes(data) {
            Ok((msg, _consumed)) => Ok(PyTwxMessage { inner: msg }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Message deserialization error: {}",
                e
            ))),
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut content = BytesMut::new();
        match self.inner.to_bytes(&mut content) {
            Ok(_) => Ok(PyBytes::new_bound(py, &content)),
            Err(e) => Err(PyValueError::new_err(format!(
                "Message serialization error: {}",
                e
            ))),
        }
    }

    fn get_message_type(&self) -> String {
        match &self.inner {
            RustTwxMsg::Request(_, _) => "Request".to_string(),
            RustTwxMsg::Response(_, _) => "Response".to_string(),
            RustTwxMsg::Auth(_, _) => "Auth".to_string(),
            RustTwxMsg::Bind(_, _) => "Bind".to_string(),
        }
    }

    fn get_request_id(&self) -> u32 {
        self.inner.get_requestid()
    }

    fn get_session_id(&self) -> u32 {
        self.inner.get_sessionid()
    }

    fn get_endpoint(&self) -> u32 {
        self.inner.get_endpoint()
    }

    fn is_request(&self) -> bool {
        self.inner.is_request()
    }

    fn is_response(&self) -> bool {
        self.inner.is_response()
    }

    fn is_auth(&self) -> bool {
        matches!(self.inner, RustTwxMsg::Auth(_, _))
    }

    fn is_bind(&self) -> bool {
        self.inner.is_bind()
    }

    fn short_description(&self) -> String {
        self.inner.short_desc()
    }

    #[staticmethod]
    fn build_auth(request_id: u32, app_key: String) -> PyResult<Self> {
        let msg = RustTwxMsg::build_auth_msg(request_id, &app_key);
        Ok(PyTwxMessage { inner: msg })
    }

    fn __str__(&self) -> String {
        self.short_description()
    }

    fn __repr__(&self) -> String {
        format!("TwxMessage({})", self.get_message_type())
    }
}

#[pyclass(name = "TwxEvent")]
#[derive(Clone, Debug)]
pub struct PyTwxEvent {
    inner: RustTwxEvent,
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
    inner: RustTwxService,
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
    inner: RustTwxProperty,
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

#[pyclass(name = "AlwaysOnError")]
#[derive(Debug)]
pub struct PyAlwaysOnError {
    message: String,
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

/// Python bindings for ThingWorx AlwaysOn protocol codec
#[pymodule]
fn _native<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.setattr("__version__", "0.1.1")?;

    m.add_class::<PyBaseType>()?;
    m.add_class::<PyTwPrim>()?;
    m.add_class::<PyTwxMessage>()?;
    m.add_class::<PyTwxEvent>()?;
    m.add_class::<PyTwxService>()?;
    m.add_class::<PyTwxProperty>()?;
    m.add_class::<PyAlwaysOnError>()?;

    Ok(())
}
