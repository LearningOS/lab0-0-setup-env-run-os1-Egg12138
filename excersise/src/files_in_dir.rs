use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;



pub fn visit_dirs(dir: &Path, cb: &dyn Fn())