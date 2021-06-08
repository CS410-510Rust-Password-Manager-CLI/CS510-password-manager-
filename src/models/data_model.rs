use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct EntryStore{
    pub entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub username: Vec<u8>, //encrypted
    pub password: Vec<u8>, //encrypted
}

impl EntryStore{
    pub fn new() -> Self{
        Self::default()
    }
}