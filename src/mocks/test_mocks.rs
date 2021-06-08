#![allow(dead_code)]
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    let mut path = PathBuf::new();
    path.push("/home");
    Some(path)
}
