use crate::generic::common::{does_store_exist, get_all_secrets};
use crate::generic::encryption::decrypt_secret;
use crate::generic::errors::{PasswordStoreError, Result};
use crate::models::data_model::Entry;

// Entry point for the get operation
pub fn display_secret(store_name: &str, entry_name: &str) -> Result<'static, ()> {
    // Check if store exists
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreEntryDoesNotExist);
    }

    match get_raw_secret(store_name, entry_name) {
        Some(raw_entry) => {
            decrypt_secret(entry_name, &(*raw_entry));
            Ok(())
        }
        None => Err(PasswordStoreError::ErrorEntryDoesNotExist),
    }
}

fn get_raw_secret<'a>(store_name: &str, entry_name: &str) -> Option<Box<Entry>> {
    match get_all_secrets(store_name) {
        Some(secrets) => {
            // Iterate through all entries and return an entry matching the entry name
            for entry in (*secrets).entries {
                if entry.name == entry_name {
                    let b = Box::new(entry);
                    return Some(b);
                }
            }
            None
        }
        None => None,
    }
}
