use crate::{
    Result,
    search::{PySortby, StringOrDict, StringOrList},
};
use duckdb::Connection;
use pyo3::{
    IntoPyObjectExt,
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyList},
};
use pyo3_arrow::PyTable;
use stac_duckdb::Client;
use std::{path::PathBuf, sync::Mutex};

const REQUIRED_EXTENSIONS: [&str; 3] = ["spatial", "icu", "parquet"];

#[pyclass(frozen)]
pub struct DuckdbClient(Mutex<Client>);

#[pymethods]
impl DuckdbClient {
    #[new]
    #[pyo3(signature = (*, extension_directory=None, extensions=Vec::new(), install_extensions=true, use_hive_partitioning=false))]
    fn new(
        extension_directory: Option<PathBuf>,
        extensions: Vec<String>,
        install_extensions: bool,
        use_hive_partitioning: bool,
    ) -> Result<DuckdbClient> {
        let connection = Connection::open_in_memory()?;
        if let Some(extension_directory) = extension_directory {
            connection.execute(
                "SET extension_directory = ?",
                [extension_directory.to_string_lossy()],
            )?;
        }
        if install_extensions {
            for extension in REQUIRED_EXTENSIONS {
                connection.execute(&format!("INSTALL {extension}"), [])?;
            }
        }
        for extension in extensions {
            connection.execute(&format!("LOAD '{extension}'"), [])?;
        }
        for extension in REQUIRED_EXTENSIONS {
            connection.execute(&format!("LOAD {extension}"), [])?;
        }
        let mut client = Client::from(connection);
        client.use_hive_partitioning = use_hive_partitioning;
        Ok(DuckdbClient(Mutex::new(client)))
    }

    #[pyo3(signature = (sql, params = Vec::new()))]
    fn execute<'py>(&self, sql: String, params: Vec<String>) -> Result<usize> {
        let client = self
            .0
            .lock()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let count = client.execute(&sql, duckdb::params_from_iter(params))?;
        Ok(count)
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
        sortby: Option<PySortby<'py>>,
        filter: Option<StringOrDict>,
        query: Option<Bound<'py, PyDict>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<Bound<'py, PyList>> {
        let search = crate::search::build(
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
        let list = pythonize::pythonize(py, &item_collection.items)?;
        let list = list.extract()?;
        Ok(list)
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
        sortby: Option<PySortby<'py>>,
        filter: Option<StringOrDict>,
        query: Option<Bound<'py, PyDict>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<PyObject> {
        let search = crate::search::build(
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
            // FIXME this is awkward
            let convert_wkb = client.convert_wkb;
            client.convert_wkb = false;
            let result = client.search_to_arrow(&href, search);
            client.convert_wkb = convert_wkb;
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
