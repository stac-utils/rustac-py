use crate::Result;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use stac::Item;
use stac::geoarrow::Options;
use stac::geoparquet::{Writer, WriterBuilder};
use std::{
    fs::File,
    sync::{Arc, Mutex},
};

#[pyclass]
pub struct GeoparquetWriter {
    writer: Arc<Mutex<Writer<File>>>,
}

#[pymethods]
impl GeoparquetWriter {
    #[new]
    #[pyo3(signature = (items, path, drop_invalid_attributes = true))]
    fn new(items: Bound<PyAny>, path: &str, drop_invalid_attributes: bool) -> Result<Self> {
        let items: Vec<Item> = pythonize::depythonize(&items)?;
        let writer = File::create(path)?;
        let writer = WriterBuilder::new(writer)
            .options(Options {
                drop_invalid_attributes,
            })
            .build(items)?;
        Ok(GeoparquetWriter {
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    pub fn write(&mut self, items: Bound<PyAny>) -> Result<()> {
        let items: Vec<Item> = pythonize::depythonize(&items)?;
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| PyException::new_err("could not lock the arrow writer"))?;
        writer.write(items)?;
        Ok(())
    }

    pub fn finish(&self) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| PyException::new_err("could not lock the arrow writer"))?;
        writer.finish()?;
        Ok(())
    }
}
