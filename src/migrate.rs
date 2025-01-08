use crate::{Error, Result};
use pyo3::{prelude::*, types::PyDict};
use stac::{Migrate, Value};

#[pyfunction]
#[pyo3(signature = (value, version=None))]
pub fn migrate<'py>(
    value: &Bound<'py, PyDict>,
    version: Option<&str>,
) -> Result<Bound<'py, PyDict>> {
    let py = value.py();
    let value: Value = pythonize::depythonize(value)?;
    let version = version
        .map(|version| version.parse().unwrap())
        .unwrap_or_default();
    let value = value.migrate(&version).map_err(Error::from)?;
    let value = pythonize::pythonize(py, &value)?;
    let value = value.extract()?;
    Ok(value)
}

#[pyfunction]
#[pyo3(signature = (href, version=None))]
pub fn migrate_href<'py>(
    py: Python<'py>,
    href: &str,
    version: Option<&str>,
) -> Result<Bound<'py, PyDict>> {
    let value: Value = stac::read(href).map_err(Error::from)?;
    let version = version
        .map(|version| version.parse().unwrap())
        .unwrap_or_default();
    let value = value.migrate(&version)?;
    let value = pythonize::pythonize(py, &value)?;
    let value = value.extract()?;
    Ok(value)
}
