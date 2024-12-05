#![deny(unused_crate_dependencies, warnings)]

mod error;
mod migrate;
mod read;
mod search;
mod version;
mod write;

use duckdb as _;
use error::Error;
use libduckdb_sys as _;
use openssl as _;
use pyo3::prelude::*;
pub use search::build_search;

type Result<T> = std::result::Result<T, Error>;

/// A collection of functions for working with STAC, using Rust under the hood.
#[pymodule]
fn stacrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(migrate::migrate, m)?)?;
    m.add_function(wrap_pyfunction!(migrate::migrate_href, m)?)?;
    m.add_function(wrap_pyfunction!(read::read, m)?)?;
    m.add_function(wrap_pyfunction!(search::search, m)?)?;
    m.add_function(wrap_pyfunction!(search::search_to, m)?)?;
    m.add_function(wrap_pyfunction!(version::version, m)?)?;
    m.add_function(wrap_pyfunction!(write::write, m)?)?;
    Ok(())
}
