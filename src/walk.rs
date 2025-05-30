use crate::{Error, Json, Result};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, exceptions::PyStopAsyncIteration, pyclass, pyfunction,
    pymethods, types::PyDict,
};
use pyo3_object_store::AnyObjectStore;
use stac::{Item, Links, SelfHref, Value};
use stac_io::{Format, StacStore};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

#[pyfunction]
#[pyo3(signature = (container, store=None))]
pub fn walk(container: Bound<'_, PyDict>, store: Option<AnyObjectStore>) -> Result<Walk> {
    let mut value: Value = pythonize::depythonize(&container)?;
    if let Some(link) = value.link("self").cloned() {
        *value.self_href_mut() = Some(link.href);
    }
    let mut walks = VecDeque::new();
    walks.push_back(value);
    let store = if let Some(store) = store {
        Some(StacStore::from(store.into_dyn()))
    } else {
        None
    };
    Ok(Walk {
        values: Arc::new(Mutex::new(walks)),
        store,
    })
}

#[pyclass]
#[derive(Clone)]
pub struct Walk {
    values: Arc<Mutex<VecDeque<Value>>>,
    store: Option<StacStore>,
}

#[pymethods]
impl Walk {
    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let walk = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { walk.next().await })
    }
}

impl Walk {
    async fn next(self) -> PyResult<Json<WalkStep>> {
        let value = {
            let mut values = self.values.lock().await;
            values.pop_front()
        };
        match value {
            Some(value) => {
                let mut items = Vec::new();
                let mut children = Vec::new();
                let mut join_set = JoinSet::new();
                for mut link in value.links().iter().cloned() {
                    if link.is_child() || link.is_item() {
                        let store = self.store.clone();
                        if let Some(self_href) = value.self_href() {
                            link.make_absolute(self_href).map_err(Error::from)?;
                        }
                        join_set.spawn(async move {
                            if let Some(store) = store {
                                store.get_format(link.href.as_str(), Format::json()).await
                            } else {
                                let (store, path) = stac_io::parse_href(link.href)?;
                                store.get_format(path, Format::json()).await
                            }
                        });
                    }
                }
                {
                    let mut values = self.values.lock().await;
                    while let Some(result) = join_set.join_next().await {
                        let value = result.map_err(Error::from)?.map_err(Error::from)?;
                        if let Value::Item(item) = value {
                            items.push(item);
                        } else if value.is_catalog() || value.is_collection() {
                            children.push(value.clone());
                            values.push_back(value.clone());
                        }
                    }
                }
                Ok(Json((value, children, items)))
            }
            _ => Err(PyStopAsyncIteration::new_err("done walking")),
        }
    }
}

type WalkStep = (Value, Vec<Value>, Vec<Item>);
