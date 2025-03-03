use crate::{Error, Json, Result};
use pyo3::{
    exceptions::PyStopAsyncIteration, pyclass, pyfunction, pymethods, types::PyDict, Bound, Py,
    PyAny, PyResult, Python,
};
use stac::{Container, Item, Links, Node, SelfHref, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyfunction]
pub fn walk(container: Bound<'_, PyDict>) -> Result<Walk> {
    let mut value: Value = pythonize::depythonize(&container)?;
    if let Some(link) = value.link("self").cloned() {
        *value.self_href_mut() = Some(link.href);
    }
    let container: Container = value.try_into()?;
    let node = Node::from(container);
    let mut walks = VecDeque::new();
    walks.push_back(node);
    Ok(Walk(Arc::new(Mutex::new(walks))))
}

#[pyclass]
pub struct Walk(Arc<Mutex<VecDeque<Node>>>);

#[pymethods]
impl Walk {
    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let nodes = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { next_walk(nodes).await })
    }
}

type WalkStep = (Value, Vec<Container>, VecDeque<Item>);

async fn next_walk(nodes: Arc<Mutex<VecDeque<Node>>>) -> PyResult<Json<WalkStep>> {
    let mut nodes = nodes.lock().await;
    if let Some(node) = nodes.pop_front() {
        let mut node = node.resolve().await.map_err(Error::from)?;
        let items = std::mem::take(&mut node.items);
        let mut children = Vec::with_capacity(node.children.len());
        for child in node.children {
            children.push(child.value.clone());
            nodes.push_back(child);
        }
        Ok(Json((node.value.into(), children, items)))
    } else {
        Err(PyStopAsyncIteration::new_err("done walking"))
    }
}
