use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use alwayson_codec::{message::tw_message::TwxMsg as RustTwxMsg, BytesStream};
use bytes::BytesMut;

#[pyclass(name = "TwxMessage")]
#[derive(Clone, Debug)]
pub struct PyTwxMessage {
    pub(crate) inner: RustTwxMsg,
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
