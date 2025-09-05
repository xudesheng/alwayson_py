#![allow(non_local_definitions)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::uninlined_format_args)]

mod types;

use pyo3::prelude::*;
use types::{
    PyAlwaysOnError, PyBaseType, PyInfoTable, PyTwPrim, PyTwxEvent, PyTwxMessage, PyTwxProperty,
    PyTwxService,
};

/// Python bindings for ThingWorx AlwaysOn protocol codec
#[pymodule]
fn _native<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.setattr("__version__", "0.6.0")?;

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
