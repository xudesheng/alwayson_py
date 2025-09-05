use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{
    base::BaseType as RustBaseType, datashape::DataShape as RustDataShape,
    infotable::InfoTable as RustInfoTable, primitive::TwPrim as RustTwPrim, BytesStream,
    SimpleJson,
};
use bytes::{Bytes, BytesMut};

#[pyclass(name = "TwPrim")]
#[derive(Clone, Debug)]
pub struct PyTwPrim {
    pub(crate) inner: RustTwPrim,
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

    #[staticmethod]
    fn infotable_empty() -> PyResult<Self> {
        // Create an empty InfoTable with no fields and no rows
        use indexmap::IndexMap;

        let datashape = RustDataShape {
            name: Some("EmptyDataShape".to_string()),
            entries: IndexMap::new(),
        };

        let infotable = RustInfoTable {
            datashape,
            rows: Vec::new(),
        };

        Ok(PyTwPrim {
            inner: RustTwPrim::INFOTABLE(RustBaseType::INFOTABLE, Box::new(infotable)),
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
        // Use the upstream to_json_typed method which requires BaseType
        let base_type = self.inner.base_type();
        match self.inner.to_json_typed(base_type) {
            Ok(json_value) => serde_json::to_string(&json_value)
                .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}"))),
            Err(e) => Err(PyValueError::new_err(format!(
                "JSON conversion error: {}",
                e
            ))),
        }
    }

    fn to_simple_json(&self) -> PyResult<String> {
        match self.inner.to_simple_json() {
            Ok(json_value) => serde_json::to_string(&json_value)
                .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}"))),
            Err(e) => Err(PyValueError::new_err(format!(
                "Simple JSON conversion error: {}",
                e
            ))),
        }
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
            RustTwPrim::VARIANT(_, boxed_prim) => {
                let wrapped_prim = PyTwPrim {
                    inner: (**boxed_prim).clone(),
                };
                wrapped_prim.get_type()
            }
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
            RustTwPrim::INFOTABLE(_, infotable) => {
                // For now, return a simple string representation of the InfoTable
                // TODO: Implement proper InfoTable Python wrapper
                let info_str = format!(
                    "InfoTable(rows={}, fields={})",
                    infotable.rows.len(),
                    infotable.datashape.entries.len()
                );
                Ok(info_str.to_object(py))
            }
            RustTwPrim::NOTHING(_) => Ok(py.None()),
            RustTwPrim::VARIANT(_, boxed_prim) => {
                // Recursively get the value from the wrapped primitive
                let wrapped_prim = PyTwPrim {
                    inner: (**boxed_prim).clone(),
                };
                wrapped_prim.get_value(py)
            }
            _ => Err(PyTypeError::new_err("Unsupported type for get_value")),
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("TwPrim({:?})", self.inner)
    }

    fn is_variant(&self) -> bool {
        matches!(self.inner, RustTwPrim::VARIANT(_, _))
    }

    fn unwrap_variant(&self, _py: Python) -> PyResult<PyTwPrim> {
        match &self.inner {
            RustTwPrim::VARIANT(_, boxed_prim) => Ok(PyTwPrim {
                inner: (**boxed_prim).clone(),
            }),
            _ => Err(PyTypeError::new_err("TwPrim is not a VARIANT type")),
        }
    }

    fn get_inner_type(&self) -> String {
        match &self.inner {
            RustTwPrim::VARIANT(_, boxed_prim) => {
                let wrapped_prim = PyTwPrim {
                    inner: (**boxed_prim).clone(),
                };
                wrapped_prim.get_type()
            }
            _ => self.get_type(),
        }
    }

    fn get_full_type(&self) -> String {
        match &self.inner {
            RustTwPrim::VARIANT(_, boxed_prim) => {
                let wrapped_prim = PyTwPrim {
                    inner: (**boxed_prim).clone(),
                };
                format!("VARIANT::{}", wrapped_prim.get_full_type())
            }
            _ => self.get_type(),
        }
    }
}
