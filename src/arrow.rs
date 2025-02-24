use pyo3::prelude::*;
use pyo3_arrow::PyTable;

#[pyfunction]
pub fn from_arrow(table: Bound<PyTable>) -> () {
    dbg!(table);
    todo!()
}
