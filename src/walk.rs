use std::collections::VecDeque;

use crate::{Error, Json, Result};
use pyo3::{
    exceptions::PyStopAsyncIteration, pyclass, pyfunction, pymethods, types::PyDict, Bound, Py,
    PyAny, PyResult, Python,
};
use stac::{Container, Node, Value};

#[pyfunction]
pub fn walk(container: Bound<'_, PyDict>) -> Result<Walk> {
    let value: Value = pythonize::depythonize(&container)?;
    let container: Container = value.try_into()?;
    let node = Node::from(container);
    let mut walks = VecDeque::new();
    walks.push_back(node);
    Ok(Walk(walks))
}

#[pyclass]
pub struct Walk(VecDeque<Node>);

#[pymethods]
impl Walk {
    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Some(node) = self.0.pop_front() {
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let node = node.resolve().await.map_err(Error::from)?;
                let mut children = Vec::with_capacity(node.children.len());
                for child in node.children {
                    children.push(Json(child.value.clone()));
                    self.0.push_back(child);
                }
                let items: Vec<_> = node.items.iter().map(|item| Json(item.clone())).collect();
                Ok((Json(node.value), children, items))
            })
        } else {
            Err(PyStopAsyncIteration::new_err("walk complete"))
        }
    }
}
