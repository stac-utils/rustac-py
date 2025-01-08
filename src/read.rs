use crate::{Error, Json};
use pyo3::{pyfunction, types::PyAny, Bound, PyResult, Python};
use stac::{Format, Value};

#[pyfunction]
#[pyo3(signature = (href, *, format=None, options=None))]
pub fn read(
    py: Python<'_>,
    href: String,
    format: Option<String>,
    options: Option<Vec<(String, String)>>,
) -> PyResult<Bound<'_, PyAny>> {
    let format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(&href))
        .unwrap_or_default();
    let options = options.unwrap_or_default();
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let value = format
            .get_opts::<Value, _, _, _>(href, options)
            .await
            .map_err(Error::from)?;
        Ok(Json(value))
    })
}
