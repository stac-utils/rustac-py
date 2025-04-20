use cargo_lock::Lockfile;

fn main() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let package = lockfile
        .packages
        .into_iter()
        .find(|package| package.name.as_str() == "rustac")
        .unwrap();
    let precise = package.source.unwrap().precise().unwrap().to_string();
    println!("cargo:rustc-env=RUSTAC_SHA={}", precise);
}
