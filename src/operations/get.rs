use crate::generic::common::{does_store_exist, get_all_secrets_from_store, get_path};
use crate::generic::encryption::decrypt_secret;
use crate::generic::errors::{PasswordStoreError, Result};
use crate::models::data_model::Entry;

// Entry point for the get operation
pub fn display_secret(store_name: &str, entry_name: &str) -> Result<'static, ()> {
    // Check if store exists
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreEntryDoesNotExist);
    }

    let path = *get_path(store_name);
    match get_raw_secret(&path, entry_name) {
        // Decrypt the secret and display to console
        Some(raw_entry) => match decrypt_secret(entry_name, &(*raw_entry)) {
            Ok(()) => Ok(()),
            Err(e) => {
                println!("{}", e.to_string());
                Err(PasswordStoreError::ErrorDataEncryption)
            }
        },
        None => Err(PasswordStoreError::ErrorEntryDoesNotExist),
    }
}

// Iterates through EntryStores and returns a Box of the raw, unencrypted secret
// matching the entry_name
fn get_raw_secret(store_path: &str, entry_name: &str) -> Option<Box<Entry>> {
    match get_all_secrets_from_store(store_path) {
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

#[test]
fn get_secrets() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());

    let actual = get_raw_secret(&path, "foobar").unwrap();

    assert_eq!("foobar", actual.name)
}

#[test]
#[should_panic]
fn secrets_not_found() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());

    let actual = get_raw_secret(&path, "notfound").unwrap();
}
