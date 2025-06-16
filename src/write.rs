use crate::{Error, Json, Result};
use pyo3::{Bound, PyAny, PyResult, Python, pyfunction};
use pyo3_object_store::AnyObjectStore;
use serde_json::Value;
use stac::{Item, ItemCollection};
use stac_io::{Format, StacStore};

#[pyfunction]
#[pyo3(signature = (href, value, *, format=None, parquet_compression=None, store=None))]
pub fn write<'py>(
    py: Python<'py>,
    href: String,
    value: Bound<'_, PyAny>,
    format: Option<String>,
    parquet_compression: Option<String>,
    store: Option<AnyObjectStore>,
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
        let mut format = format
            .and_then(|f| f.parse::<Format>().ok())
            .or_else(|| Format::infer_from_href(&href))
            .unwrap_or_default();
        if matches!(format, Format::Geoparquet(_)) {
            if let Some(parquet_compression) = parquet_compression {
                tracing::debug!("setting parquet compression: {parquet_compression}");
                format =
                    Format::Geoparquet(Some(parquet_compression.parse().map_err(Error::from)?));
            }
        }
        let (stac_store, path) = if let Some(store) = store {
            (StacStore::from(store.into_dyn()), None)
        } else {
            stac_io::parse_href(&href)
                .map(|(store, path)| (store, Some(path)))
                .map_err(Error::from)?
        };
        let put_result = if let Some(path) = path {
            stac_store
                .put_format(path.as_ref(), value, format)
                .await
                .map_err(Error::from)?
        } else {
            stac_store
                .put_format(href, value, format)
                .await
                .map_err(Error::from)?
        };
        let put_result = serde_json::json!({
            "e_tag": put_result.e_tag,
            "version": put_result.version,
        });
        Ok(Some(Json(put_result)))
    })
}
