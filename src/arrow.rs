use crate::{Error, Json, Result};
use duckdb::arrow::array::RecordBatchIterator;
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_arrow::PyTable;
use serde_json::Value;
use stac::geoarrow;
use stac::{Item, ItemCollection};

#[pyfunction]
pub fn from_arrow(py: Python, table: PyTable) -> Result<Bound<PyAny>> {
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
    let reader = RecordBatchIterator::new(record_batches.into_iter().map(Ok), schema);
    let item_collection = stac::geoarrow::from_record_batch_reader(reader)?;
    let item_collection = Json(item_collection).into_pyobject(py)?;
    Ok(item_collection)
}

#[pyfunction]
pub fn to_arrow(py: Python<'_>, items: Bound<PyAny>) -> Result<Py<PyAny>> {
    let value: Value = pythonize::depythonize(&items)?;
    let item_collection = if let Value::Array(array) = value {
        let items = array
            .into_iter()
            .map(|value| serde_json::from_value::<Item>(value).map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        ItemCollection::from(items)
    } else {
        serde_json::from_value(value)?
    };
    let (record_batch, _) = geoarrow::encode(item_collection.items)?;
    let record_batch = stac::geoarrow::with_wkb_geometry(record_batch, "geometry")?;
    let schema = record_batch.schema().clone();
    let table = PyTable::try_new(vec![record_batch], schema)?;
    let table = table.to_arro3(py)?;
    Ok(table.into_py_any(py)?)
}
