use std::io::Write;

use std::str;
use text_io::read;

// Internal libraries
use crate::generic::common::{calculate_store_name_hash, does_store_exist, write_to_file, UserMessage, get_all_secrets_from_store, get_path};
use crate::generic::encryption::{create_new_rsa_private_key, encrypt_data_with_private_key};
use crate::generic::errors::{PasswordStoreError, Result};
use crate::models::data_model::{Entry, EntryStore};

// Todo: Bug fix: Creating two entries with the same name

// Entry Point for creating a new data entry
pub fn create_entry_point(store_name: &str) -> Result<'static, ()> {
    // Check if the requested store exists
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    // Get user data from the command line
    let entry_name: String = *get_entry_name();
    let username: String = *get_username();
    match get_password() {
        Ok(b) => {
            let password: String = *b;
            let key_name = calculate_store_name_hash(&entry_name).to_string();
            match add_to_store(&key_name, &username, &password, store_name, &entry_name) {
                Ok(()) => {
                    println!(
                        "{}",
                        UserMessage::CreatedEntrySuccessfully(&entry_name).value()
                    );
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// Adds the new data to the specified store
pub fn add_to_store<'a>(
    key_name: &str,
    username: &str,
    password: &str,
    store_name: &str,
    entry_name: &str,
) -> Result<'a, ()> {

    let hashed_store_name = calculate_store_name_hash(store_name).to_string();
    let path = *get_path(store_name);
    match encrypt(key_name, username, password, entry_name) {
        Ok(encrypted_data) => {
            let new_entry = *encrypted_data;
            match get_all_secrets_from_store(&path) {
                Some(mut secret_entries) => {
                    secret_entries.entries.push(new_entry);
                    match write_to_file(&secret_entries, &hashed_store_name) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e),
                    }
                }
                None => {
                    let mut new_entry_store = EntryStore::new();
                    new_entry_store.entries.push(new_entry);
                    match write_to_file(&new_entry_store, &hashed_store_name) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e),
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
            Err(PasswordStoreError::ErrorEncryptionError)
        }
    }
}

// Encrypts the username and password with an RSA private key
fn encrypt<'a>(
    key_name: &str,
    username: &str,
    password: &str,
    entry_name: &str,
) -> Result<'a, Box<Entry>> {
    match create_new_rsa_private_key(&key_name) {
        Ok(()) => {
            // Once the key is successfully generated, encrypt the private data
            match encrypt_data_with_private_key(&key_name, &username, &password, &entry_name) {
                Ok(encrypted_data) => Ok(encrypted_data),
                Err(e) => {
                    println!("Error: {}", e.to_string());
                    Err(PasswordStoreError::ErrorEncryptionError)
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
            Err(PasswordStoreError::ErrorPrivateKeyGeneration)
        }
    }
}

// Read from stdin for the entry name
// Return a Box with the entry name
fn get_entry_name() -> Box<String> {
    println!("Enter a name for this new entry: ");
    let entry_name: String = read!("{}\n");
    std::io::stdout().flush().unwrap();
    Box::new(entry_name)
}

// Read from stdin for the username
// Return a Box with the username
pub fn get_username() -> Box<String> {
    println!("Username: ");
    let username: String = read!("{}\n");
    std::io::stdout().flush().unwrap();
    Box::new(username)
}

// Read password from stdin twice
// Verifies that the result is accurate before returning
// Stdin input is not revealed on the command line
// If the passwords do not match, throw error
pub fn get_password() -> Result<'static, Box<String>> {
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let pass_verify = rpassword::prompt_password_stdout("Password Verification: ").unwrap();

    if pass == pass_verify {
        Ok(Box::new(pass))
    } else {
        Err(PasswordStoreError::ErrorMisMatchPasswordCreation)
    }
}
