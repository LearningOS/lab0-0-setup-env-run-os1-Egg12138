
mod files_in_dir;

use std::path::Path;
use files_in_dir::trees;

fn main() {
    println!("Hello, world!");
    let filepath = Path::new(".");
    if Path::exists(filepath) {
        trees(Path::new("./target"));
    }
}
