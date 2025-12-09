use crate::Error;
use object_store::local::LocalFileSystem;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3_object_store::AnyObjectStore;
use stac::{Item, geoarrow::Options};
use stac_io::store::geoparquet::StacGeoparquetObjectWriter;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
pub struct GeoparquetWriter(Arc<Mutex<Option<StacGeoparquetObjectWriter>>>);

#[pymethods]
impl GeoparquetWriter {
    #[staticmethod]
    #[pyo3(signature = (items, path, drop_invalid_attributes = true, store = None))]
    pub fn open<'py>(
        py: Python<'py>,
        items: Bound<PyAny>,
        path: &str,
        drop_invalid_attributes: bool,
        store: Option<AnyObjectStore>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let items: Vec<Item> = pythonize::depythonize(&items)?;
        let store = store
            .map(|store| store.into_dyn())
            .unwrap_or_else(|| Arc::new(LocalFileSystem::new()));
        let options = Options {
            drop_invalid_attributes,
            ..Default::default()
        };
        let path = path.into();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let writer =
                StacGeoparquetObjectWriter::new(store, path, items, options, Default::default())
                    .await
                    .map_err(Error::from)?;
            Ok(GeoparquetWriter(Arc::new(Mutex::new(Some(writer)))))
        })
    }

    pub fn write<'py>(
        &mut self,
        py: Python<'py>,
        items: Bound<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let items: Vec<Item> = pythonize::depythonize(&items)?;
        let writer = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = writer.lock().await;
            if let Some(writer) = writer.as_mut() {
                writer.write(items).await.map_err(Error::from)?;
                Ok(())
            } else {
                Err(PyErr::new::<PyRuntimeError, _>("Writer is closed"))
            }
        })
    }

    pub fn finish<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let writer = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = writer.lock().await;
            if let Some(writer) = writer.take() {
                writer.close().await.map_err(Error::from)?;
                Ok(())
            } else {
                Err(PyErr::new::<PyRuntimeError, _>("Writer is closed"))
            }
        })
    }
}
