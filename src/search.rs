use crate::{Error, Json, Result};
use pyo3::{prelude::*, types::PyDict};
use stac::Format;
use stac_api::{
    python::{StringOrDict, StringOrList},
    Search,
};

#[pyfunction]
#[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, use_duckdb=None, **kwargs))]
#[allow(clippy::too_many_arguments)]
pub fn search<'py>(
    py: Python<'py>,
    href: String,
    intersects: Option<StringOrDict>,
    ids: Option<StringOrList>,
    collections: Option<StringOrList>,
    max_items: Option<usize>,
    limit: Option<u64>,
    bbox: Option<Vec<f64>>,
    datetime: Option<String>,
    include: Option<StringOrList>,
    exclude: Option<StringOrList>,
    sortby: Option<StringOrList>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let search = stac_api::python::search(
        intersects,
        ids,
        collections,
        limit,
        bbox,
        datetime,
        include,
        exclude,
        sortby,
        filter,
        query,
        kwargs,
    )?;
    if use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))))
    {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_duckdb(href, search, max_items)?;
            Ok(Json(value.items))
        })
    } else {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_api(href, search, max_items).await?;
            Ok(Json(value.items))
        })
    }
}

#[pyfunction]
#[pyo3(signature = (outfile, href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, format=None, options=None, use_duckdb=None, **kwargs))]
#[allow(clippy::too_many_arguments)]
pub fn search_to<'py>(
    py: Python<'py>,
    outfile: String,
    href: String,
    intersects: Option<StringOrDict>,
    ids: Option<StringOrList>,
    collections: Option<StringOrList>,
    max_items: Option<usize>,
    limit: Option<u64>,
    bbox: Option<Vec<f64>>,
    datetime: Option<String>,
    include: Option<StringOrList>,
    exclude: Option<StringOrList>,
    sortby: Option<StringOrList>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    format: Option<String>,
    options: Option<Vec<(String, String)>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let search = stac_api::python::search(
        intersects,
        ids,
        collections,
        limit,
        bbox,
        datetime,
        include,
        exclude,
        sortby,
        filter,
        query,
        kwargs,
    )?;
    let format = format
        .map(|s| s.parse())
        .transpose()
        .map_err(Error::from)?
        .or_else(|| Format::infer_from_href(&outfile))
        .unwrap_or_default();
    if use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))))
    {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_duckdb(href, search, max_items)?;
            let count = value.items.len();
            let _ = format
                .put_opts(
                    outfile,
                    serde_json::to_value(value).map_err(Error::from)?,
                    options.unwrap_or_default(),
                )
                .await
                .map_err(Error::from)?;
            Ok(count)
        })
    } else {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_api(href, search, max_items).await?;
            let count = value.items.len();
            let _ = format
                .put_opts(
                    outfile,
                    serde_json::to_value(value).map_err(Error::from)?,
                    options.unwrap_or_default(),
                )
                .await
                .map_err(Error::from)?;
            Ok(count)
        })
    }
}

fn search_duckdb(
    href: String,
    search: Search,
    max_items: Option<usize>,
) -> Result<stac_api::ItemCollection> {
    let value = stac_duckdb::search(&href, search, max_items)?;
    Ok(value)
}

async fn search_api(
    href: String,
    search: Search,
    max_items: Option<usize>,
) -> Result<stac_api::ItemCollection> {
    let value = stac_api::client::search(&href, search, max_items).await?;
    Ok(value)
}
