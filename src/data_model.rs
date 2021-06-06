use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Entry{
    name: String,
    username: String, //encrypted
    password: String, //encrypted
    hint: String,
}

