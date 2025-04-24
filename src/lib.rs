#![warn(unused_crate_dependencies)]

mod arrow;
mod cli;
mod collection;
mod duckdb;
mod error;
mod migrate;
mod read;
mod search;
mod version;
mod walk;
mod write;

use error::Error;
use pyo3::prelude::*;

type Result<T> = std::result::Result<T, Error>;

#[pymodule]
fn rustac(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add("RustacError", py.get_type::<error::RustacError>())?;

    m.add_class::<duckdb::DuckdbClient>()?;

    m.add_function(wrap_pyfunction!(arrow::from_arrow, m)?)?;
    m.add_function(wrap_pyfunction!(arrow::to_arrow, m)?)?;
    m.add_function(wrap_pyfunction!(cli::main, m)?)?;
    m.add_function(wrap_pyfunction!(
        collection::collection_from_id_and_items,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(migrate::migrate, m)?)?;
    m.add_function(wrap_pyfunction!(read::read, m)?)?;
    m.add_function(wrap_pyfunction!(search::search, m)?)?;
    m.add_function(wrap_pyfunction!(search::search_to, m)?)?;
    m.add_function(wrap_pyfunction!(version::sha, m)?)?;
    m.add_function(wrap_pyfunction!(version::version, m)?)?;
    m.add_function(wrap_pyfunction!(walk::walk, m)?)?;
    m.add_function(wrap_pyfunction!(write::write, m)?)?;

    Ok(())
}

struct Json<T: serde::Serialize>(T);

impl<'py, T: serde::Serialize> IntoPyObject<'py> for Json<T> {
    type Error = pythonize::PythonizeError;
    type Output = Bound<'py, PyAny>;
    type Target = PyAny;
    fn into_pyobject(self, py: Python<'py>) -> std::result::Result<Self::Output, Self::Error> {
        pythonize::pythonize(py, &self.0)
    }
}
