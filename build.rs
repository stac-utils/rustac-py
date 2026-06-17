use cargo_lock::Lockfile;

fn main() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let sha = lockfile
        .packages
        .into_iter()
        .find(|package| package.name.as_str() == "rustac")
        .and_then(|package| package.source)
        .and_then(|source| source.precise().map(ToString::to_string))
        .unwrap_or_else(|| "local".to_string());
    println!("cargo:rustc-env=RUSTAC_SHA={}", sha);
}
