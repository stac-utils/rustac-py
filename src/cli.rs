use clap::Parser;
use pyo3::{pyfunction, types::PyAnyMethods, PyResult, Python};
use stac_cli::Stacrs;

#[pyfunction]
pub fn main(py: Python<'_>) -> PyResult<i64> {
    let signal = py.import("signal")?;
    let _ = signal
        .getattr("signal")?
        .call1((signal.getattr("SIGINT")?, signal.getattr("SIG_DFL")?))?;
    let args = Stacrs::parse_from(std::env::args_os().skip(1));
    std::process::exit(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                match args.run().await {
                    Ok(()) => 0,
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        1
                    }
                }
            }),
    )
}
