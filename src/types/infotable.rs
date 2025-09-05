use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{
    base::BaseType as RustBaseType,
    datashape::{DataShape as RustDataShape, DataShapeEntry},
    infotable::{InfoTable as RustInfoTable, InfoTableRow},
    primitive::TwPrim as RustTwPrim,
    BytesStream, SimpleJson,
};
use bytes::BytesMut;

#[pyclass(name = "InfoTable")]
#[derive(Clone, Debug)]
pub struct PyInfoTable {
    pub(crate) inner: RustInfoTable,
}

#[pymethods]
impl PyInfoTable {
    #[new]
    #[pyo3(signature = (name=None))]
    fn new(name: Option<String>) -> PyResult<Self> {
        use indexmap::IndexMap;

        let datashape = RustDataShape {
            name,
            entries: IndexMap::new(),
        };

        let infotable = RustInfoTable {
            datashape,
            rows: Vec::new(),
        };

        Ok(PyInfoTable { inner: infotable })
    }

    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        match RustInfoTable::from_bytes(data) {
            Ok((infotable, _consumed)) => Ok(PyInfoTable { inner: infotable }),
            Err(e) => Err(PyValueError::new_err(format!(
                "InfoTable deserialization error: {}",
                e
            ))),
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut content = BytesMut::new();
        match self.inner.to_bytes(&mut content) {
            Ok(_) => Ok(PyBytes::new_bound(py, &content)),
            Err(e) => Err(PyValueError::new_err(format!(
                "InfoTable serialization error: {}",
                e
            ))),
        }
    }

    fn get_row_count(&self) -> usize {
        self.inner.rows.len()
    }

    fn get_field_count(&self) -> usize {
        self.inner.datashape.entries.len()
    }

    fn get_datashape_name(&self) -> Option<String> {
        self.inner.datashape.name.clone()
    }

    fn to_json(&self) -> PyResult<String> {
        // Use upstream Serde serialization directly
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
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

    fn add_field_definition(
        &mut self,
        name: String,
        base_type: String,
        description: String,
    ) -> PyResult<()> {
        let rust_base_type = match base_type.to_uppercase().as_str() {
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
                    "Invalid base type: {base_type}"
                )))
            }
        };

        let entry = DataShapeEntry {
            name: name.clone(),
            description,
            entry_type: rust_base_type,
            aspects: Default::default(),
        };

        self.inner.datashape.entries.insert(name, entry);
        Ok(())
    }

    fn add_row(&mut self, py: Python, row_dict: PyObject) -> PyResult<()> {
        use pyo3::types::PyDict;

        let dict = row_dict.downcast_bound::<PyDict>(py)?;
        let mut row_fields = Vec::new();

        // Process fields in the order they appear in the data shape
        for (field_name, field_def) in &self.inner.datashape.entries {
            let py_value = dict
                .get_item(field_name)?
                .ok_or_else(|| PyValueError::new_err(format!("Missing field: {field_name}")))?;

            // Convert Python value to TwPrim based on field type
            let tw_prim = match field_def.entry_type {
                RustBaseType::STRING => {
                    let s: String = py_value.extract()?;
                    RustTwPrim::STRING(RustBaseType::STRING, s)
                }
                RustBaseType::NUMBER => {
                    let n: f64 = py_value.extract()?;
                    RustTwPrim::NUMBER(RustBaseType::NUMBER, n)
                }
                RustBaseType::INTEGER => {
                    let i: i32 = py_value.extract()?;
                    RustTwPrim::INTEGER(RustBaseType::INTEGER, i)
                }
                RustBaseType::LONG => {
                    let l: i64 = py_value.extract()?;
                    RustTwPrim::LONG(RustBaseType::LONG, l)
                }
                RustBaseType::DATETIME => {
                    let dt: i64 = py_value.extract()?;
                    RustTwPrim::DATETIME(RustBaseType::DATETIME, dt)
                }
                RustBaseType::BOOLEAN => {
                    let b: bool = py_value.extract()?;
                    RustTwPrim::BOOLEAN(RustBaseType::BOOLEAN, b)
                }
                _ => {
                    return Err(PyValueError::new_err(format!(
                        "Unsupported field type for field {field_name}: {:?}",
                        field_def.entry_type
                    )));
                }
            };

            row_fields.push(tw_prim);
        }

        let row = InfoTableRow { fields: row_fields };

        self.inner.rows.push(row);
        Ok(())
    }

    fn __str__(&self) -> String {
        format!(
            "InfoTable(rows={}, fields={})",
            self.inner.rows.len(),
            self.inner.datashape.entries.len()
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "InfoTable(rows={}, fields={}, name={:?})",
            self.inner.rows.len(),
            self.inner.datashape.entries.len(),
            self.inner.datashape.name
        )
    }
}
