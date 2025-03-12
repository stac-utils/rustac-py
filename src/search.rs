use crate::{Error, Json, Result};
use geojson::Geometry;
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, types::PyDict, Bound, FromPyObject, PyErr, PyResult};
use stac::Bbox;
use stac::Format;
use stac_api::{Fields, Filter, Items, Search, Sortby};

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
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let search = build(
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
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict>,
    query: Option<Bound<'py, PyDict>>,
    format: Option<String>,
    options: Option<Vec<(String, String)>>,
    use_duckdb: Option<bool>,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let search = build(
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

/// Creates a [Search] from Python arguments.
#[allow(clippy::too_many_arguments)]
pub fn build<'py>(
    intersects: Option<StringOrDict<'py>>,
    ids: Option<StringOrList>,
    collections: Option<StringOrList>,
    limit: Option<u64>,
    bbox: Option<Vec<f64>>,
    datetime: Option<String>,
    include: Option<StringOrList>,
    exclude: Option<StringOrList>,
    sortby: Option<PySortby<'py>>,
    filter: Option<StringOrDict<'py>>,
    query: Option<Bound<'py, PyDict>>,
    kwargs: Option<Bound<'py, PyDict>>,
) -> PyResult<Search> {
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
    let bbox = bbox.map(Bbox::try_from).transpose().map_err(Error::from)?;
    let sortby: Vec<Sortby> = sortby
        .map(|sortby| match sortby {
            PySortby::ListOfDicts(list) => list
                .into_iter()
                .map(|d| pythonize::depythonize(&d).map_err(Error::from))
                .collect::<Result<Vec<_>>>(),
            PySortby::ListOfStrings(list) => list
                .into_iter()
                .map(|s| Ok(s.parse().unwrap())) // infallible
                .collect::<Result<Vec<_>>>(),
            PySortby::String(s) => Ok(vec![s.parse().unwrap()]),
        })
        .transpose()?
        .unwrap_or_default();
    let filter = filter
        .map(|filter| match filter {
            StringOrDict::Dict(cql_json) => pythonize::depythonize(&cql_json).map(Filter::Cql2Json),
            StringOrDict::String(cql2_text) => Ok(Filter::Cql2Text(cql2_text)),
        })
        .transpose()?;
    let filter = filter
        .map(|filter| filter.into_cql2_json())
        .transpose()
        .map_err(Error::from)?;
    let mut items = Items {
        limit,
        bbox,
        datetime,
        query,
        fields,
        sortby,
        filter,
        ..Default::default()
    };
    if let Some(kwargs) = kwargs {
        items.additional_fields = pythonize::depythonize(&kwargs)?;
    }

    let intersects = intersects
        .map(|intersects| match intersects {
            StringOrDict::Dict(json) => pythonize::depythonize(&json)
                .map_err(PyErr::from)
                .and_then(|json| {
                    Geometry::from_json_object(json)
                        .map_err(|err| PyValueError::new_err(err.to_string()))
                }),
            StringOrDict::String(s) => s
                .parse::<Geometry>()
                .map_err(|err| PyValueError::new_err(err.to_string())),
        })
        .transpose()?;
    let ids = ids.map(|ids| ids.into()).unwrap_or_default();
    let collections = collections.map(|ids| ids.into()).unwrap_or_default();
    Ok(Search {
        items,
        intersects,
        ids,
        collections,
    })
}

/// A string or dictionary.
///
/// Used for the CQL2 filter argument and for intersects.
#[derive(Debug, FromPyObject)]
pub enum StringOrDict<'py> {
    /// Text
    String(String),

    /// Json
    Dict(Bound<'py, PyDict>),
}

/// A string or a list.
///
/// Used for collections, ids, etc.
#[derive(Debug, FromPyObject)]
pub enum StringOrList {
    /// A string.
    String(String),

    /// A list.
    List(Vec<String>),
}

/// A sortby structure.
///
/// This can be a string, a list of strings, or a list of dictionaries.
#[derive(Debug, FromPyObject)]
pub enum PySortby<'py> {
    /// A string.
    String(String),

    /// A list.
    ListOfStrings(Vec<String>),

    /// A list.
    ListOfDicts(Vec<Bound<'py, PyDict>>),
}

impl From<StringOrList> for Vec<String> {
    fn from(value: StringOrList) -> Vec<String> {
        match value {
            StringOrList::List(list) => list,
            StringOrList::String(s) => vec![s],
        }
    }
}
