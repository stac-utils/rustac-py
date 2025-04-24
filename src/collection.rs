use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyAny, PyAnyMethods, PyDict},
};
use stac::{Collection, Item};

#[pyfunction]
pub fn collection_from_id_and_items<'py>(
    py: Python<'py>,
    id: String,
    items: Bound<'_, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let items: Vec<Item> = pythonize::depythonize(&items)?;
    let collection = Collection::from_id_and_items(id, &items);
    let collection = pythonize::pythonize(py, &collection)?.extract()?;
    Ok(collection)
}
