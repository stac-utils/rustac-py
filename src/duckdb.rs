use crate::Result;
use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyList},
};
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
