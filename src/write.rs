use crate::{Error, Json, Result};
use pyo3::{Bound, PyAny, PyResult, Python, pyfunction};
use pyo3_object_store::AnyObjectStore;
use serde_json::Value;
use stac::{Item, ItemCollection, geoparquet::WriterOptions};
use stac_io::{Format, StacStore};

fn parse_value(value: Bound<'_, PyAny>) -> PyResult<stac::Value> {
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
    Ok(value)
}

fn parse_format(
    format: Option<String>,
    href: &str,
    parquet_compression: Option<String>,
) -> Result<Format> {
    let mut format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(href))
        .unwrap_or_default();
    if matches!(format, Format::Geoparquet(_)) {
        if let Some(parquet_compression) = parquet_compression {
            tracing::debug!("setting parquet compression: {parquet_compression}");
            format = Format::Geoparquet(WriterOptions {
                compression: Some(parquet_compression.parse().map_err(Error::from)?),
                ..Default::default()
            });
        }
    }
    Ok(format)
}

async fn write_inner(
    href: String,
    value: stac::Value,
    format: Format,
    store: Option<AnyObjectStore>,
) -> PyResult<Option<Json<Value>>> {
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
}

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
    let value = parse_value(value)?;
    let format = parse_format(format, &href, parquet_compression)?;
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        write_inner(href, value, format, store).await
    })
}

#[pyfunction]
#[pyo3(signature = (href, value, *, format=None, parquet_compression=None, store=None))]
pub fn write_sync(
    py: Python<'_>,
    href: String,
    value: Bound<'_, PyAny>,
    format: Option<String>,
    parquet_compression: Option<String>,
    store: Option<AnyObjectStore>,
) -> PyResult<Option<Json<Value>>> {
    let value = parse_value(value)?;
    let format = parse_format(format, &href, parquet_compression)?;
    py.detach(|| {
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(async { write_inner(href, value, format, store).await })
    })
}
