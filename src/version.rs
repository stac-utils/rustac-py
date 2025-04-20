use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (name=None))]
pub fn version(name: Option<String>) -> Option<String> {
    if let Some(name) = name {
        if name.eq_ignore_ascii_case("stac") {
            Some(stac::version().to_string())
        } else if name.eq_ignore_ascii_case("stac-api") {
            Some(stac_api::version().to_string())
        } else if name.eq_ignore_ascii_case("stac-duckdb") {
            Some(stac_duckdb::version().to_string())
        } else {
            None
        }
    } else {
        Some(env!("CARGO_PKG_VERSION").to_string())
    }
}

#[pyfunction]
pub fn sha() -> String {
    // This environment variable is set in the build script
    env!("RUSTAC_SHA").to_string()
}
