use pyo3::{exceptions::PyException, PyErr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Geojson(#[from] geojson::Error),

    #[error(transparent)]
    Pythonize(#[from] pythonize::PythonizeError),

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
    fn from(value: Error) -> Self {
        match value {
            Error::Geojson(err) => PyException::new_err(err.to_string()),
            Error::Pythonize(err) => PyException::new_err(err.to_string()),
            Error::SerdeJson(err) => PyException::new_err(err.to_string()),
            Error::Stac(err) => PyException::new_err(err.to_string()),
            Error::StacApi(err) => PyException::new_err(err.to_string()),
            Error::StacDuckdb(err) => PyException::new_err(err.to_string()),
        }
    }
}
