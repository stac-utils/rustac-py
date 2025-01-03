use crate::Error;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use stac::Format;
use stac_api::{
    python::{StringOrDict, StringOrList},
    BlockingClient, Item, ItemCollection,
};
use stac_duckdb::Client;
use tokio::runtime::Builder;

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
) -> PyResult<Bound<'py, PyList>> {
    let items = search_items(
        href,
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
        use_duckdb,
        kwargs,
    )?;
    pythonize::pythonize(py, &items)
        .map_err(PyErr::from)
        .and_then(|v| v.extract())
}

#[pyfunction]
#[pyo3(signature = (outfile, href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, format=None, options=None, use_duckdb=None, **kwargs))]
#[allow(clippy::too_many_arguments)]
pub fn search_to<'py>(
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
) -> PyResult<usize> {
    let items = search_items(
        href,
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
        use_duckdb,
        kwargs,
    )?;
    let format = format
        .map(|s| s.parse())
        .transpose()
        .map_err(Error::from)?
        .or_else(|| Format::infer_from_href(&outfile))
        .unwrap_or_default();
    let item_collection = ItemCollection::from(items);
    let count = item_collection.items.len();
    Builder::new_current_thread()
        .build()?
        .block_on(format.put_opts(
            outfile,
            serde_json::to_value(item_collection).map_err(Error::from)?,
            options.unwrap_or_default(),
        ))
        .map_err(Error::from)?;
    Ok(count)
}

#[allow(clippy::too_many_arguments)]
fn search_items<'py>(
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
    kwargs: Option<Bound<'py, PyDict>>,
) -> PyResult<Vec<Item>> {
    let mut search = stac_api::python::search(
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
        if let Some(max_items) = max_items {
            search.items.limit = Some(max_items.try_into()?);
        }
        let client = Client::new().map_err(Error::from)?;
        client
            .search_to_json(&href, search)
            .map(|item_collection| item_collection.items)
            .map_err(Error::from)
            .map_err(PyErr::from)
    } else {
        let client = BlockingClient::new(&href).map_err(Error::from)?;
        let items = client.search(search).map_err(Error::from)?;
        if let Some(max_items) = max_items {
            items
                .take(max_items)
                .collect::<Result<_, _>>()
                .map_err(Error::from)
                .map_err(PyErr::from)
        } else {
            items
                .collect::<Result<_, _>>()
                .map_err(Error::from)
                .map_err(PyErr::from)
        }
    }
}
