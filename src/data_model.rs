use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct EntryStore{
    pub entries: Vec<Entry>
}

#[derive(Serialize, Deserialize)]
pub struct Entry{
    pub name: String,
    pub username: Vec<u8>, //encrypted
    pub password: Vec<u8>, //encrypted
}

