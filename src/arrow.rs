use crate::{Error, Json, Result};
use geoarrow::table::Table;
use pyo3::prelude::*;
use pyo3_arrow::PyTable;

#[pyfunction]
pub fn from_arrow(py: Python<'_>, table: PyTable) -> PyResult<Bound<PyAny>> {
    let (record_batches, mut schema) = table.into_inner();
    let record_batches = record_batches
        .into_iter()
        .map(|record_batch| {
            let record_batch = stac::geoarrow::with_native_geometry(record_batch, "geometry")?;
            Ok(record_batch)
        })
        .collect::<Result<Vec<_>>>()?;
    if !record_batches.is_empty() {
        schema = record_batches[0].schema();
    }
    let table = Table::try_new(record_batches, schema).map_err(Error::from)?;
    let item_collection = stac::geoarrow::from_table(table).map_err(Error::from)?;
    let item_collection = Json(item_collection).into_pyobject(py)?;
    Ok(item_collection)
}
