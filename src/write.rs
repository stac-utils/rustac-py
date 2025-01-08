use crate::{Error, Json, Result};
use pyo3::{pyfunction, Bound, PyAny, PyResult, Python};
use serde_json::Value;
use stac::{Format, Item, ItemCollection};

#[pyfunction]
#[pyo3(signature = (href, value, *, format=None, options=None))]
pub fn write<'py>(
    py: Python<'py>,
    href: String,
    value: Bound<'_, PyAny>,
    format: Option<String>,
    options: Option<Vec<(String, String)>>,
) -> PyResult<Bound<'py, PyAny>> {
    let value: Value = pythonize::depythonize(&value)?;
    let value = if let Value::Array(array) = value {
        let items = array
            .into_iter()
            .map(|value| serde_json::from_value::<Item>(value).map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        stac::Value::ItemCollection(ItemCollection::from(items))
    } else {
        serde_json::from_value(value).map_err(Error::from)?
    };
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let format = format
            .and_then(|f| f.parse::<Format>().ok())
            .or_else(|| Format::infer_from_href(&href))
            .unwrap_or_default();
        let put_result = format
            .put_opts(href, value, options.unwrap_or_default())
            .await
            .map_err(Error::from)?;
        if let Some(put_result) = put_result {
            let put_result = serde_json::json!({
                "e_tag": put_result.e_tag,
                "version": put_result.version,
            });
            Ok(Some(Json(put_result)))
        } else {
            Ok(None)
        }
    })
}
