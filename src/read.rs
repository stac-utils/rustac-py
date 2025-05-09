use crate::{Error, Json};
use pyo3::{Bound, PyResult, Python, pyfunction, types::PyAny};
use pyo3_object_store::AnyObjectStore;
use stac::{Format, Link, Links, SelfHref, Value};

#[pyfunction]
#[pyo3(signature = (href, *, format=None, store=None, set_self_link=true))]
pub fn read(
    py: Python<'_>,
    href: String,
    format: Option<String>,
    store: Option<AnyObjectStore>,
    set_self_link: bool,
) -> PyResult<Bound<'_, PyAny>> {
    let format = format
        .and_then(|f| f.parse::<Format>().ok())
        .or_else(|| Format::infer_from_href(&href))
        .unwrap_or_default();
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let mut value: Value = if let Some(store) = store {
            format
                .get_store(store.into_dyn(), href.as_str())
                .await
                .map_err(Error::from)?
        } else {
            format
                .get_opts(href.as_str(), [] as [(&str, &str); 0])
                .await
                .map_err(Error::from)?
        };
        if set_self_link {
            value.set_link(Link::self_(href.clone()));
        }
        *value.self_href_mut() = Some(href.into());
        Ok(Json(value))
    })
}
