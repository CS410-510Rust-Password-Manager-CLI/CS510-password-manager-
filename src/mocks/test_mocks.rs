#![allow(dead_code)]
use std::path::PathBuf;
use crate::models::data_model::EntryStore;

pub fn home_dir() -> Option<PathBuf> {
    let mut path = PathBuf::new();
    path.push("/home");
    Some(path)
}

pub fn get_all_secrets_from_store() -> Option<Box<EntryStore>>{
    println!("blah");
    None
}