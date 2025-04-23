use crate::{Error, Json, Result};
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_arrow::PyTable;
use serde_json::Value;
use stac::geoarrow::Table;
use stac::{Item, ItemCollection};

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
    let table = Table::new(record_batches, schema);
    let item_collection =
        stac::geoarrow::from_record_batch_reader(table.into_reader()).map_err(Error::from)?;
    let item_collection = Json(item_collection).into_pyobject(py)?;
    Ok(item_collection)
}

#[pyfunction]
pub fn to_arrow(py: Python<'_>, items: Bound<PyAny>) -> PyResult<PyObject> {
    let value: Value = pythonize::depythonize(&items)?;
    let item_collection = if let Value::Array(array) = value {
        let items = array
            .into_iter()
            .map(|value| serde_json::from_value::<Item>(value).map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        ItemCollection::from(items)
    } else {
        serde_json::from_value(value).map_err(Error::from)?
    };
    // TODO we might want to just allow use to go WKB right when we got to table?
    let (record_batches, mut schema) = Table::from_item_collection(item_collection)
        .map_err(Error::from)?
        .into_inner();
    let record_batches = record_batches
        .into_iter()
        .map(|record_batch| {
            stac::geoarrow::with_wkb_geometry(record_batch, "geometry").map_err(Error::from)
        })
        .collect::<Result<Vec<_>>>()?;
    if !record_batches.is_empty() {
        schema = record_batches[0].schema();
    }
    let table = PyTable::try_new(record_batches, schema)?;
    let table = table.to_arro3(py)?;
    Ok(table.into_py_any(py)?)
}
