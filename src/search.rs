use crate::Error;
use geojson::Geometry;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use stac::{Bbox, Format};
use stac_api::{BlockingClient, Fields, Item, ItemCollection, Items, Search};
use stac_api::{Filter, Sortby};
use stac_duckdb::Client;
use tokio::runtime::Builder;

#[pyfunction]
#[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, use_duckdb=None))]
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
) -> PyResult<Bound<'py, PyList>> {
    let items = search_items(
        py,
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
    )?;
    pythonize::pythonize(py, &items)
        .map_err(PyErr::from)
        .and_then(|v| v.extract())
}

#[pyfunction]
#[pyo3(signature = (outfile, href, *, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, format=None, options=None, use_duckdb=None))]
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
) -> PyResult<usize> {
    let items = search_items(
        py,
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
) -> PyResult<Vec<Item>> {
    // TODO refactor to use https://github.com/gadomski/stacrs/blob/1528d7e1b7185a86efe9fc7c42b0620093c5e9c6/src/search.rs#L128-L162
    let mut fields = Fields::default();
    if let Some(include) = include {
        fields.include = include.into();
    }
    if let Some(exclude) = exclude {
        fields.exclude = exclude.into();
    }
    let fields = if fields.include.is_empty() && fields.exclude.is_empty() {
        None
    } else {
        Some(fields)
    };
    let query = query
        .map(|query| pythonize::depythonize(&query))
        .transpose()?;
    let bbox = bbox
        .map(|bbox| Bbox::try_from(bbox))
        .transpose()
        .map_err(Error::from)?;
    let sortby = sortby.map(|sortby| {
        Vec::<String>::from(sortby)
            .into_iter()
            .map(|s| s.parse::<Sortby>().unwrap()) // the parse is infallible
            .collect::<Vec<_>>()
    });
    let filter = filter
        .map(|filter| match filter {
            StringOrDict::Dict(cql_json) => {
                pythonize::depythonize(&cql_json.bind_borrowed(py)).map(Filter::Cql2Json)
            }
            StringOrDict::String(cql2_text) => Ok(Filter::Cql2Text(cql2_text)),
        })
        .transpose()?;
    let filter = filter
        .map(|filter| filter.into_cql2_json())
        .transpose()
        .map_err(Error::from)?;
    let items = Items {
        limit,
        bbox,
        datetime,
        query,
        fields,
        sortby,
        filter,
        ..Default::default()
    };

    let intersects = intersects
        .map(|intersects| match intersects {
            StringOrDict::Dict(json) => pythonize::depythonize(&json.bind_borrowed(py))
                .map_err(Error::from)
                .and_then(|json| Geometry::from_json_object(json).map_err(Error::from)),
            StringOrDict::String(s) => s.parse().map_err(Error::from),
        })
        .transpose()?;
    let ids = ids.map(|ids| ids.into());
    let collections = collections.map(|ids| ids.into());
    let mut search = Search {
        items,
        intersects,
        ids,
        collections,
    };
    if use_duckdb
        .unwrap_or_else(|| matches!(Format::infer_from_href(&href), Some(Format::Geoparquet(_))))
    {
        if let Some(max_items) = max_items {
            search.items.limit = Some(max_items.try_into()?);
        }
        let client = Client::from_href(href).map_err(Error::from)?;
        client
            .search_to_json(search)
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

#[derive(FromPyObject)]
pub enum StringOrDict {
    String(String),
    Dict(Py<PyDict>),
}

#[derive(FromPyObject)]
pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

impl From<StringOrList> for Vec<String> {
    fn from(value: StringOrList) -> Vec<String> {
        match value {
            StringOrList::List(list) => list,
            StringOrList::String(s) => vec![s],
        }
    }
}
