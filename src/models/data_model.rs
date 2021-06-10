use serde::{Deserialize, Serialize};

// Data is stored as an EntryStore
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct EntryStore {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub username: Vec<u8>, //encrypted
    pub password: Vec<u8>, //encrypted
}

impl EntryStore {
    pub fn new() -> Self {
        Self::default()
    }
}
