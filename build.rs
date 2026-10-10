use std::env;

fn main() {
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let target = env::var("TARGET").unwrap();
    println!("cargo::rustc-env=VERSION={version} ({target})");
}
