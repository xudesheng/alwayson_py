#![allow(non_local_definitions)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::uninlined_format_args)]

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{
    base::BaseType as RustBaseType, datashape::DataShape as RustDataShape,
    event::TwxEvent as RustTwxEvent, infotable::InfoTable as RustInfoTable,
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

#[pyclass(name = "InfoTable")]
#[derive(Clone, Debug)]
pub struct PyInfoTable {
    inner: RustInfoTable,
}

#[pymethods]
impl PyInfoTable {
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
}

impl PyInfoTable {
    fn to_json_with_depth(&self, depth: usize) -> PyResult<String> {
        let json_value = self
            .to_json_value_with_depth(depth)
            .map_err(|e| PyValueError::new_err(format!("JSON conversion error: {e}")))?;
        serde_json::to_string(&json_value)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {e}")))
    }

    fn to_json_value_with_depth(&self, depth: usize) -> Result<serde_json::Value, String> {
        const MAX_DEPTH: usize = 5;

        if depth >= MAX_DEPTH {
            // At max depth, return simplified representation
            return Ok(serde_json::json!({
                "type": "InfoTable",
                "rows": self.inner.rows.len(),
                "fields": self.inner.datashape.entries.len(),
                "note": "Max depth reached"
            }));
        }

        let mut json_table = serde_json::Map::new();

        // Add data shape information
        let mut fields = serde_json::Map::new();
        for (field_name, field_def) in &self.inner.datashape.entries {
            let mut field_info = serde_json::Map::new();
            field_info.insert(
                "baseType".to_string(),
                serde_json::Value::String(format!("{:?}", field_def.entry_type)),
            );
            field_info.insert(
                "description".to_string(),
                serde_json::Value::String(field_def.description.clone()),
            );
            fields.insert(field_name.clone(), serde_json::Value::Object(field_info));
        }
        json_table.insert("dataShape".to_string(), serde_json::Value::Object(fields));

        // Add rows data with full field expansion
        let mut rows_array = Vec::new();
        for row in &self.inner.rows {
            let mut row_obj = serde_json::Map::new();

            // Get field names from datashape for proper mapping
            let field_names: Vec<String> = self.inner.datashape.entries.keys().cloned().collect();

            for (field_idx, prim_value) in row.fields.iter().enumerate() {
                let field_name = if field_idx < field_names.len() {
                    field_names[field_idx].clone()
                } else {
                    format!("field_{}", field_idx)
                };

                // Convert TwPrim value to JSON with depth limiting
                match Self::twprim_to_json_value_with_depth(prim_value, depth + 1) {
                    Ok(json_val) => {
                        row_obj.insert(field_name, json_val);
                    }
                    Err(_) => {
                        // If conversion fails, use a string representation
                        row_obj.insert(
                            field_name,
                            serde_json::Value::String(format!("{:?}", prim_value)),
                        );
                    }
                }
            }
            rows_array.push(serde_json::Value::Object(row_obj));
        }
        json_table.insert("rows".to_string(), serde_json::Value::Array(rows_array));

        Ok(serde_json::Value::Object(json_table))
    }

    // Helper method to convert TwPrim to JSON with depth limiting
    fn twprim_to_json_value_with_depth(
        prim: &RustTwPrim,
        depth: usize,
    ) -> Result<serde_json::Value, String> {
        const MAX_DEPTH: usize = 5;

        match prim {
            RustTwPrim::BOOLEAN(_, v) => Ok(serde_json::Value::Bool(*v)),
            RustTwPrim::INTEGER(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::LONG(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::NUMBER(_, v) => serde_json::Number::from_f64(*v)
                .map(serde_json::Value::Number)
                .ok_or_else(|| "Invalid float value".to_string()),
            RustTwPrim::STRING(_, v) => Ok(serde_json::Value::String(v.clone())),
            RustTwPrim::DATETIME(_, v) => Ok(serde_json::Value::Number((*v).into())),
            RustTwPrim::BLOB(_, _) => Ok(serde_json::Value::String("[BLOB data]".to_string())),
            RustTwPrim::NOTHING(_) => Ok(serde_json::Value::Null),
            RustTwPrim::VARIANT(_, boxed_prim) => {
                // Recursively convert the variant content
                Self::twprim_to_json_value_with_depth(boxed_prim, depth)
            }
            RustTwPrim::INFOTABLE(_, nested_infotable) => {
                if depth >= MAX_DEPTH {
                    Ok(serde_json::json!({
                        "type": "InfoTable",
                        "rows": nested_infotable.rows.len(),
                        "fields": nested_infotable.datashape.entries.len(),
                        "note": "Max depth reached"
                    }))
                } else {
                    // Create a PyInfoTable wrapper and convert recursively
                    let nested_wrapper = PyInfoTable {
                        inner: (**nested_infotable).clone(),
                    };
                    nested_wrapper.to_json_value_with_depth(depth)
                }
            }
            _ => Err("Unsupported type in InfoTable row".to_string()),
        }
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
    m.setattr("__version__", "0.5.1")?;

    m.add_class::<PyBaseType>()?;
    m.add_class::<PyTwPrim>()?;
    m.add_class::<PyTwxMessage>()?;
    m.add_class::<PyTwxEvent>()?;
    m.add_class::<PyTwxService>()?;
    m.add_class::<PyTwxProperty>()?;
    m.add_class::<PyInfoTable>()?;
    m.add_class::<PyAlwaysOnError>()?;

    Ok(())
}
