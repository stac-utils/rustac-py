use pyo3::{
    create_exception,
    exceptions::{PyException, PyIOError},
    PyErr,
};
use thiserror::Error;

create_exception!(stacrs, StacrsError, PyException);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Geojson(#[from] geojson::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pythonize(#[from] pythonize::PythonizeError),

    #[error(transparent)]
    Py(#[from] PyErr),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Stac(#[from] stac::Error),

    #[error(transparent)]
    StacApi(#[from] stac_api::Error),

    #[error(transparent)]
    StacDuckdb(#[from] stac_duckdb::Error),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Py(err) => err,
            Error::Io(err) => PyIOError::new_err(err.to_string()),
            _ => StacrsError::new_err(err.to_string()),
        }
    }
}
