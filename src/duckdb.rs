use crate::{
    Result,
    api::{PySortby, StringOrDict, StringOrDictOrList, StringOrList},
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

#[pyclass(frozen, subclass)]
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
    fn execute<'py>(&self, py: Python<'py>, sql: String, params: Vec<String>) -> Result<usize> {
        py.detach(|| -> Result<usize> {
            let client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            let count = client.execute(&sql, duckdb::params_from_iter(params))?;
            Ok(count)
        })
    }

    #[pyo3(signature = (sql, params = Vec::new()))]
    fn query_to_table<'py>(
        &self,
        py: Python<'py>,
        sql: String,
        params: Vec<String>,
    ) -> Result<Py<PyAny>> {
        let (record_batches, schema) = py.detach(|| -> Result<_> {
            let client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            let mut statement = client.prepare(&sql)?;
            let arrow = statement.query_arrow(duckdb::params_from_iter(params))?;
            let schema = arrow.get_schema();
            let record_batches: Vec<_> = arrow.collect();
            Ok((record_batches, schema))
        })?;
        let table = PyTable::try_new(record_batches, schema)?;
        let table = table.to_arro3(py)?;
        Ok(table.into_py_any(py)?)
    }

    #[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, limit=None, max_items=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, **kwargs))]
    fn search<'py>(
        &self,
        py: Python<'py>,
        href: String,
        intersects: Option<StringOrDict>,
        ids: Option<StringOrList>,
        collections: Option<StringOrList>,
        mut limit: Option<u64>,
        max_items: Option<u64>,
        bbox: Option<Vec<f64>>,
        datetime: Option<String>,
        include: Option<StringOrList>,
        exclude: Option<StringOrList>,
        sortby: Option<PySortby<'py>>,
        filter: Option<StringOrDict>,
        query: Option<Bound<'py, PyDict>>,
        fields: Option<StringOrDictOrList<'py>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<Bound<'py, PyList>> {
        if max_items.is_some() {
            if limit.is_some() {
                tracing::warn!("Both max_items and limit are set, overriding limit with max_items");
            }
            limit = max_items;
        }
        let search = crate::api::build_search(
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
        let items = py.detach(|| -> Result<_> {
            let client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Ok(client.search(&href, search)?.items)
        })?;
        let list = pythonize::pythonize(py, &items)?;
        let list = list.extract()?;
        Ok(list)
    }

    #[pyo3(signature = (href, *, intersects=None, ids=None, collections=None, limit=None, bbox=None, datetime=None, include=None, exclude=None, sortby=None, filter=None, query=None, fields=None, **kwargs))]
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
        fields: Option<StringOrDictOrList<'py>>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> Result<Py<PyAny>> {
        let search = crate::api::build_search(
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
        let record_batches = py.detach(|| -> Result<_> {
            let mut client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            // FIXME this is awkward
            let convert_wkb = client.convert_wkb;
            client.convert_wkb = false;
            let result = client
                .search_to_arrow(&href, search)
                .and_then(|arrow| arrow.collect::<std::result::Result<Vec<_>, _>>());
            client.convert_wkb = convert_wkb;
            Ok(result?)
        })?;
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
        let collections = py.detach(|| -> Result<_> {
            let client = self
                .0
                .lock()
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Ok(client.collections(&href)?)
        })?;
        let collections = pythonize::pythonize(py, &collections)?.extract()?;
        Ok(collections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn mutex_wait_allows_python_progress() {
        Python::initialize();
        Python::attach(|py| {
            let native = Client::from(Connection::open_in_memory().unwrap());
            let client = Py::new(py, DuckdbClient(Mutex::new(native))).unwrap();
            let borrowed = client.borrow(py);
            let mutex = &borrowed.0;
            let (locked_tx, locked_rx) = mpsc::channel();
            let (release_tx, release_rx) = mpsc::channel();

            thread::scope(|scope| {
                let holder = scope.spawn(move || {
                    let guard = mutex.lock().unwrap();
                    locked_tx.send(()).unwrap();
                    // Release even on regression, so a failure cannot deadlock the suite.
                    let progressed = release_rx.recv_timeout(Duration::from_secs(10)).is_ok();
                    drop(guard);
                    progressed
                });
                locked_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                let observer = scope.spawn(move || {
                    Python::attach(|_| {
                        let _ = release_tx.send(());
                    });
                });

                // This thread holds the GIL until execute detaches. The observer must
                // acquire it before the holder will release the client mutex.
                let result = client.bind(py).call_method1("execute", ("INVALID SQL",));
                let progressed = py.detach(|| {
                    observer.join().unwrap();
                    holder.join().unwrap()
                });
                assert!(progressed, "waiting for the client mutex held the GIL");
                assert!(result.is_err());
            });
            client
                .bind(py)
                .call_method1("execute", ("SELECT 1",))
                .unwrap();
        });
    }

    #[test]
    fn search_to_arrow_error_restores_convert_wkb() {
        Python::initialize();
        Python::attach(|py| {
            for convert_wkb in [true, false] {
                let mut client = Client::from(Connection::open_in_memory().unwrap());
                client.convert_wkb = convert_wkb;
                let client = Py::new(py, DuckdbClient(Mutex::new(client))).unwrap();
                // The unsupported query extension fails before any file or extension I/O.
                let kwargs = PyDict::new(py);
                kwargs.set_item("query", PyDict::new(py)).unwrap();
                let error = client
                    .bind(py)
                    .call_method("search_to_arrow", ("unused.parquet",), Some(&kwargs))
                    .unwrap_err();
                assert!(error.to_string().contains("query is not implemented"));
                // Both WKB and converted geometry produce valid Python results, so
                // inspect the setting itself rather than a subsequent search result.
                assert_eq!(client.borrow(py).0.lock().unwrap().convert_wkb, convert_wkb);
            }
        });
    }
}
