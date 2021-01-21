
use std::env;

fn main() {
    let base = env::current_dir().unwrap();
    let dir = base.join("src").join("c");
    println!("cargo:rustc-link-search={}", dir.to_str().unwrap());
}
