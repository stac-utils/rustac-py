#![deny(unused_crate_dependencies, warnings)]

mod duckdb;
mod error;
mod migrate;
mod read;
mod search;
mod version;
mod write;

use ::duckdb as _;
use error::Error;
use pyo3::prelude::*;

type Result<T> = std::result::Result<T, Error>;

/// A collection of functions for working with STAC, using Rust under the hood.
#[pymodule]
fn stacrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<duckdb::DuckdbClient>()?;
    m.add_function(wrap_pyfunction!(migrate::migrate, m)?)?;
    m.add_function(wrap_pyfunction!(migrate::migrate_href, m)?)?;
    m.add_function(wrap_pyfunction!(read::read, m)?)?;
    m.add_function(wrap_pyfunction!(search::search, m)?)?;
    m.add_function(wrap_pyfunction!(search::search_to, m)?)?;
    m.add_function(wrap_pyfunction!(version::version, m)?)?;
    m.add_function(wrap_pyfunction!(write::write, m)?)?;
    Ok(())
}
