use crate::generic::common::{
    does_store_exist,
    calculate_store_name_hash,
    get_all_secrets,
    GlobalConfiguration
};
use crate::models::data_model::{
    Entry,
    EntryStore
};
use crate::generic::errors::{
    PasswordStoreError,
    Result
};
use crate::generic::encryption::{
    decrypt_secret
};

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use rsa::{
    PaddingScheme,
    RSAPrivateKey
};

// Entry point for the get operation
pub fn display_secret(store_name: &str, entry_name: &str) -> Result<'static, ()>{
    // Check if store exists
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist)
    }

    match get_raw_secret(store_name, entry_name){
        Ok(raw_entry) => {
            decrypt_secret(entry_name, &(*raw_entry));
            Ok(())
        },
        Err(e) => Err(e),
    }
}

pub fn get_raw_secret<'a>(store_name: &str, entry_name: &str) -> Result<'static, Box<Entry>> {
    match get_all_secrets(store_name){
        Ok(secrets) => {
            // Iterate through all entries and return an entry matching the entry name
            for entry in (*secrets).entries{
                if entry.name == entry_name{
                    let b = Box::new(entry);
                    return Ok(b)
                }
            }
            Err(PasswordStoreError::ErrorEntryDoesNotExist)
        }
        Err(e) => Err(e)
    }
}