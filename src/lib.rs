use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyTypeError};
use pyo3::types::PyBytes;

use alwayson_codec::{
    base::BaseType as RustBaseType,
    primitive::TwPrim as RustTwPrim,
};
use bytes::Bytes;

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
            _ => return Err(PyValueError::new_err(format!("Invalid base type: {}", type_name))),
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


    // TODO: Implement binary serialization once we understand the upstream API better
    // For now, using JSON serialization as a workaround
    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let json_str = self.to_json()?;
        Ok(PyBytes::new(py, json_str.as_bytes()))
    }

    #[staticmethod]
    fn from_bytes(_data: &[u8]) -> PyResult<Self> {
        // TODO: Implement binary deserialization once we understand the upstream API better
        Err(PyValueError::new_err("Binary deserialization not yet implemented"))
    }

    fn to_json(&self) -> PyResult<String> {
        let json_value = self.to_json_value()?;
        serde_json::to_string(&json_value)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {}", e)))
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
            RustTwPrim::BLOB(_, v) => Ok(PyBytes::new(py, v.as_ref()).to_object(py)),
            RustTwPrim::NOTHING(_) => Ok(py.None()),
            _ => Err(PyTypeError::new_err("Unsupported type for get_value"))
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
            RustTwPrim::NUMBER(_, v) => {
                serde_json::Number::from_f64(*v)
                    .map(serde_json::Value::Number)
                    .ok_or_else(|| PyValueError::new_err("Invalid float value"))
            }
            RustTwPrim::STRING(_, v) => Ok(serde_json::Value::String(v.clone())),
            RustTwPrim::DATETIME(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::BLOB(_, _) => Err(PyTypeError::new_err("Cannot convert BLOB to JSON")),
            RustTwPrim::NOTHING(_) => Ok(serde_json::Value::Null),
            _ => Err(PyTypeError::new_err("Cannot convert to JSON"))
        }
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
fn _native(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    
    m.add_class::<PyBaseType>()?;
    m.add_class::<PyTwPrim>()?;
    m.add_class::<PyAlwaysOnError>()?;
    
    Ok(())
}