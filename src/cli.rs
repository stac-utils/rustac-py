use clap::Parser;
use pyo3::{
    pyfunction,
    types::{PyAnyMethods, PyDict},
    PyResult, Python,
};
use stac_cli::Stacrs;
use tracing::Level;

#[pyfunction]
pub fn main(py: Python<'_>) -> PyResult<i64> {
    let signal = py.import("signal")?;
    let _ = signal
        .getattr("signal")?
        .call1((signal.getattr("SIGINT")?, signal.getattr("SIG_DFL")?))?;
    let args = Stacrs::parse_from(std::env::args_os().skip(1));
    let logging = py.import("logging")?;
    let kwargs = PyDict::new(py);
    kwargs.set_item(
        "format",
        "'%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'",
    )?;
    logging.getattr("basicConfig")?.call((), Some(&kwargs))?;
    let logger = logging.getattr("getLogger")?.call0()?;
    let level = args.log_level().unwrap_or(Level::INFO);
    logger.call_method1("setLevel", (level.to_string(),))?;
    std::process::exit(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                match args.run(false).await {
                    Ok(()) => 0,
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        1
                    }
                }
            }),
    )
}
