use crate::api::{
    ApiClient, PySortby, StringOrDict, StringOrDictOrList, StringOrList, build_search,
};
use crate::{Error, Json, Result};
use pyo3::prelude::*;
use pyo3::{Bound, PyResult, types::PyDict};
use pyo3_object_store::AnyObjectStore;
use serde_json::{Map, Value};
use stac::api::Search;
use stac::geoparquet::WriterOptions;
use stac_io::{Format, StacStore};
use std::collections::HashMap;

#[pyfunction]
#[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, headers=None, **kwargs))]
#[allow(clippy::too_many_arguments)]
pub fn iter_search<'py>(
    py: Python<'py>,
    href: String,
    intersects: Option<StringOrDict>,
    ids: Option<StringOrList>,
    collections: Option<StringOrList>,
    limit: Option<u64>,
    bbox: Option<Vec<f64>>,
    datetime: Option<String>,
    include: Option<StringOrList>,
    exclude: Option<StringOrList>,
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    fields: Option<StringOrDictOrList>,
    headers: Option<HashMap<String, String>>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = ApiClient::new(href, headers)?;
    client.iter_search(
        py,
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
        fields,
        kwargs,
    )
}

#[pyfunction]
#[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, headers=None, use_duckdb=None, **kwargs))]
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
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    fields: Option<StringOrDictOrList>,
    headers: Option<HashMap<String, String>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let use_duckdb = use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))));
    if use_duckdb {
        let search = build_search(
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
            fields,
            kwargs,
        )?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_duckdb(href, search, max_items)?;
            Ok(Json(value.items))
        })
    } else {
        let client = ApiClient::new(href, headers)?;
        client.search(
            py,
            intersects,
            ids,
            collections,
            max_items,
            limit,
            bbox,
            datetime,
            include,
            exclude,
            sortby,
            filter,
            query,
            fields,
            kwargs,
        )
    }
}

#[pyfunction]
#[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, headers=None, use_duckdb=None, **kwargs))]
#[allow(clippy::too_many_arguments)]
pub fn search_sync<'py>(
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
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    fields: Option<StringOrDictOrList>,
    headers: Option<HashMap<String, String>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Json<Vec<Map<String, Value>>>> {
    let use_duckdb = use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))));
    if use_duckdb {
        let search = build_search(
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
            fields,
            kwargs,
        )?;
        let value = search_duckdb(href, search, max_items)?;
        Ok(Json(value.items))
    } else {
        let client = ApiClient::new(href, headers)?;
        client.search_sync(
            py,
            intersects,
            ids,
            collections,
            max_items,
            limit,
            bbox,
            datetime,
            include,
            exclude,
            sortby,
            filter,
            query,
            fields,
            kwargs,
        )
    }
}

#[pyfunction]
#[pyo3(signature = (outfile, href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, headers=None, format=None, parquet_compression=None, store=None, use_duckdb=None, **kwargs))]
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
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    fields: Option<StringOrDictOrList>,
    headers: Option<HashMap<String, String>>,
    format: Option<String>,
    parquet_compression: Option<String>,
    store: Option<AnyObjectStore>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let mut format = format
        .map(|s| s.parse())
        .transpose()
        .map_err(Error::from)?
        .or_else(|| Format::infer_from_href(&outfile))
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
    let use_duckdb = use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))));
    let search = build_search(
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
        fields,
        kwargs,
    )?;
    let client = if use_duckdb {
        None
    } else {
        Some(ApiClient::new(href.clone(), headers)?)
    };
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let value = if let Some(client) = client {
            client.search_item_collection(search, max_items).await?
        } else {
            search_duckdb(href, search, max_items)?
        };
        let count = value.items.len();
        let value = serde_json::to_value(value).map_err(Error::from)?;
        if let Some(store) = store {
            StacStore::from(store)
                .put_format(outfile, value, format)
                .await
                .map_err(Error::from)?;
        } else {
            let (store, path) = stac_io::parse_href(outfile).map_err(Error::from)?;
            store
                .put_format(path, value, format)
                .await
                .map_err(Error::from)?;
        }
        Ok(count)
    })
}

fn search_duckdb(
    href: String,
    search: Search,
    max_items: Option<usize>,
) -> Result<stac::api::ItemCollection> {
    let value = stac_duckdb::search(&href, search, max_items)?;
    Ok(value)
}
