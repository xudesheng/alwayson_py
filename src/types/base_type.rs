use alwayson_codec::base::BaseType as RustBaseType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "BaseType")]
#[derive(Clone, Debug)]
pub struct PyBaseType {
    pub(crate) inner: RustBaseType,
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
