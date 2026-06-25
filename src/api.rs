use crate::{Error, Json, Result};
use futures_core::stream::BoxStream;
use futures_util::StreamExt;
use geojson::Geometry;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Bound, PyErr, PyResult, exceptions::PyValueError};
use serde_json::{Map, Value};
use stac::Bbox;
use stac::api::{Fields, Filter, Items, Search, Sortby, StreamItemsClient};
use stac_io::api::{Client, ClientBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::pin;
use tokio::sync::Mutex;

#[pyclass]
pub struct ApiClient(Client);

#[pyclass]
struct SearchIterator(Arc<Mutex<BoxStream<'static, stac_io::Result<Map<String, Value>>>>>);

#[pymethods]
impl ApiClient {
    #[new]
    #[pyo3(signature = (href, *, headers=None))]
    pub fn new(href: String, headers: Option<HashMap<String, String>>) -> Result<ApiClient> {
        let mut builder = ClientBuilder::new();
        if let Some(headers) = headers {
            builder = builder.default_headers((&headers).try_into()?);
        }
        let client = Client::with_client_builder(builder, &href)?;
        Ok(ApiClient(client))
    }

    #[pyo3(signature = (*, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, **kwargs))]
    #[allow(clippy::too_many_arguments)]
    pub fn search<'py>(
        &self,
        py: Python<'py>,
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
        kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
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
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = search_client(client, search, max_items).await?;
            Ok(Json(value.items))
        })
    }

    #[pyo3(signature = (*, intersects=None, ids=None, collections=None, max_items=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, **kwargs))]
    #[allow(clippy::too_many_arguments)]
    pub fn search_sync<'py>(
        &self,
        py: Python<'py>,
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
        kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Json<Vec<Map<String, Value>>>> {
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
        let client = self.0.clone();
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(async {
                let value = search_client(client, search, max_items).await?;
                Ok(Json(value.items))
            })
        })
    }

    #[pyo3(signature = (*, intersects=None, ids=None, collections=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, **kwargs))]
    #[allow(clippy::too_many_arguments)]
    pub fn iter_search<'py>(
        &self,
        py: Python<'py>,
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
        kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
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
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let stream = async_stream::try_stream! {
                let inner = client.search_stream(search).await?;
                futures_util::pin_mut!(inner);
                while let Some(item) = inner.next().await {
                    yield item?;
                }
            };
            Ok(SearchIterator(Arc::new(Mutex::new(Box::pin(stream)))))
        })
    }

    pub fn get_collection<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let collection = client.collection(&id).await.map_err(Error::from)?;
            Ok(Json(collection))
        })
    }

    pub fn get_collections<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut collections = Vec::new();
            let stream = client.collections().await.map_err(Error::from)?;
            pin!(stream);
            while let Some(result) = stream.next().await {
                let collection = result.map_err(Error::from)?;
                collections.push(Json(collection));
            }
            Ok(collections)
        })
    }
}

impl ApiClient {
    pub async fn search_item_collection(
        self,
        search: Search,
        max_items: Option<usize>,
    ) -> Result<stac::api::ItemCollection> {
        search_client(self.0, search, max_items).await
    }
}

#[pymethods]
impl SearchIterator {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut stream = stream.lock().await;
            if let Some(result) = stream.next().await {
                let item = result.map_err(Error::from)?;
                Ok(Some(Json(item)))
            } else {
                Ok(None)
            }
        })
    }
}

async fn search_client(
    client: Client,
    search: Search,
    max_items: Option<usize>,
) -> Result<stac::api::ItemCollection> {
    let stream = client.search_stream(search).await?;
    pin!(stream);
    let mut items = if let Some(max_items) = max_items {
        Vec::with_capacity(max_items)
    } else {
        Vec::new()
    };
    while let Some(result) = stream.next().await {
        let item = result?;
        items.push(item);
        if let Some(max_items) = max_items {
            if items.len() >= max_items {
                break;
            }
        }
    }
    Ok(items.into())
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

/// A string, dictionary, or list.
///
/// Used for fields.
#[derive(Debug, FromPyObject)]
pub enum StringOrDictOrList<'py> {
    /// Text
    String(String),

    /// Json
    Dict(Bound<'py, PyDict>),

    /// A list.
    List(Vec<String>),
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

impl<'py> TryFrom<StringOrDictOrList<'py>> for Fields {
    type Error = Error;
    fn try_from(value: StringOrDictOrList) -> Result<Fields> {
        match value {
            StringOrDictOrList::String(s) => Ok(s.parse().unwrap()),
            StringOrDictOrList::Dict(dict) => {
                pythonize::depythonize(&dict).map_err(|e| Error::Pythonize(e))
            }
            StringOrDictOrList::List(list) => {
                serde_json::from_value(serde_json::json!(list)).map_err(Error::from)
            }
        }
    }
}

/// Creates a [Search] from Python arguments.
#[allow(clippy::too_many_arguments)]
pub fn build_search<'py>(
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
    fields: Option<StringOrDictOrList<'py>>,
    kwargs: Option<Bound<'py, PyDict>>,
) -> PyResult<Search> {
    let mut fields = fields
        .map(|fields| fields.try_into())
        .transpose()?
        .unwrap_or_else(Fields::default);
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
                .map(|json| Geometry::new(json)),
            StringOrDict::String(s) => s
                .parse::<Geometry>()
                .map_err(|err| PyValueError::new_err(err.to_string())),
        })
        .transpose()?;
    let ids = ids.map(|ids| ids.into()).unwrap_or_default();
    let collections = collections.map(|ids| ids.into()).unwrap_or_default();
    let search = Search {
        items,
        intersects,
        ids,
        collections,
    };
    let search = search.normalize_datetimes().map_err(Error::from)?;
    Ok(search)
}
