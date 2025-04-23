use pyo3::{
    PyErr, create_exception,
    exceptions::{PyException, PyIOError},
};
use thiserror::Error;

create_exception!(rustac, RustacError, PyException);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Duckdb(#[from] duckdb::Error),

    #[error(transparent)]
    Geojson(#[from] geojson::Error),

    #[error(transparent)]
    Geoarrow(#[from] geoarrow_array::error::GeoArrowError),

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
            _ => RustacError::new_err(err.to_string()),
        }
    }
}
