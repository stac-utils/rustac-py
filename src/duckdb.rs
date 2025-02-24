use crate::Result;
use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyList},
    IntoPyObjectExt,
};
use pyo3_arrow::PyTable;
use stac_api::python::{StringOrDict, StringOrList};
use stac_duckdb::{Client, Config};
use std::sync::Mutex;

#[pyclass(frozen)]
pub struct DuckdbClient(Mutex<Client>);

#[pymethods]
impl DuckdbClient {
    #[new]
    #[pyo3(signature = (use_s3_credential_chain=true, use_hive_partitioning=false))]
    fn new(use_s3_credential_chain: bool, use_hive_partitioning: bool) -> Result<DuckdbClient> {
        let config = Config {
            use_s3_credential_chain,
            use_hive_partitioning,
            convert_wkb: true,
        };
        let client = Client::with_config(config)?;
        Ok(DuckdbClient(Mutex::new(client)))
    }

    #[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, **kwargs))]
    fn search<'py>(
        &self,
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
        sortby: Option<StringOrList>,
        filter: Option<StringOrDict>,
        query: Option<Bound<'py, PyDict>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<Bound<'py, PyDict>> {
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
        let item_collection = {
            let client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            client.search(&href, search)?
        };
        let dict = pythonize::pythonize(py, &item_collection)?;
        let dict = dict.extract()?;
        Ok(dict)
    }

    #[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, **kwargs))]
    fn search_to_arrow<'py>(
        &self,
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
        sortby: Option<StringOrList>,
        filter: Option<StringOrDict>,
        query: Option<Bound<'py, PyDict>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<PyObject> {
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
        let record_batches = {
            let mut client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            let convert_wkb = client.config.convert_wkb;
            client.config.convert_wkb = false;
            let result = client.search_to_arrow(&href, search);
            client.config.convert_wkb = convert_wkb;
            result?
        };
        if record_batches.is_empty() {
            Ok(py.None())
        } else {
            let schema = record_batches[0].schema();
            let table = PyTable::try_new(record_batches, schema)?;
            let table = table.to_arro3(py)?;
            Ok(table.into_py_any(py)?)
        }
    }

    fn get_collections<'py>(&self, py: Python<'py>, href: String) -> Result<Bound<'py, PyList>> {
        let client = self
            .0
            .lock()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let collections = client.collections(&href)?;
        let collections = pythonize::pythonize(py, &collections)?.extract()?;
        Ok(collections)
    }
}
