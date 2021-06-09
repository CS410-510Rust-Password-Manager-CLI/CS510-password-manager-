use crate::generic::common::{
    calculate_store_name_hash, does_store_exist, get_all_secrets_from_store, get_entry_names,
    get_index, get_path, write_to_file, GlobalConfiguration, UserMessage,
};
use crate::generic::errors::{PasswordStoreError, Result};
use std::fs::remove_file;
use std::io::{stdin, stdout, Write};

// Delete a secret store
// Delete entry by entry before deleting store to ensure that all
// RSA private keys and cleaned up and that password file can be
// restored in case of error
pub fn delete_secret_store(store_name: &str) -> Result<'static, ()> {
    let mut buffer = String::new();
    print!("\nEnter store name again to verify deletion: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    let verified = buffer.trim();
    if verified != store_name {
        return Err(PasswordStoreError::ErrorMisMatchStoreName);
    }

    let path = *get_path(store_name);
    match get_entry_names(&path) {
        Some(entry_names) => {
            for name in entry_names.iter() {
                if let Err(e) = delete_entry(store_name, name) {
                    return Err(e);
                }
            }
            //hash the store name
            let hash_store_name = calculate_store_name_hash(store_name);

            //get file path of hashed store name
            let file_path = format!(
                "{}/{}.json",
                GlobalConfiguration::StoreDir.value().unwrap(),
                hash_store_name
            );

            //delete the store file
            match remove_file(file_path) {
                Err(e) => {
                    println!("{}", e.to_string());
                    Err(PasswordStoreError::ErrorStoreDoesNotExist)
                }
                Ok(()) => {
                    println!("{}", UserMessage::DeletedEntrySuccessfully.value());
                    Ok(())
                }
            }
        }
        None => Err(PasswordStoreError::ErrorNoStoreName),
    }
}

// Deletes a secret entry in a store
pub fn delete_entry(store_name: &str, entry_name: &str) -> Result<'static, ()> {
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    let store_path = *get_path(store_name);
    // Create fail back copy.
    // If errors occur in operation, write the copy back to disk
    let failback_copy = get_all_secrets_from_store(&store_path).unwrap();
    let mut all_secrets_final = get_all_secrets_from_store(&store_path).unwrap();

    let store_hash = calculate_store_name_hash(store_name).to_string();
    match get_index(entry_name, &(*all_secrets_final)) {
        Some(index) => {
            // Get name of entry being removed
            //remove entry from entries vector where name matches, if match found
            all_secrets_final.entries.remove(*index);
            match write_to_file(&all_secrets_final, &store_hash) {
                Ok(()) => {
                    // If removal is successful, delete the RSA pem key
                    let hashed_entry_name = calculate_store_name_hash(entry_name).to_string();
                    match clean_up_rsa_keys(&hashed_entry_name) {
                        Ok(()) => {
                            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
                            Ok(())
                        }
                        // If key removal fails, restore to original
                        Err(e) => match write_to_file(&failback_copy, &store_hash) {
                            Ok(()) => Err(e),
                            Err(e) => Err(e),
                        },
                    }
                }
                Err(e) => Err(e),
            }
        }
        None => {
            // If anything goes wrong, failback to original copy
            match write_to_file(&failback_copy, &store_hash) {
                Ok(()) => Err(PasswordStoreError::ErrorNoEntryNameMatch),
                Err(e) => Err(e),
            }
        }
    }
}

// Clean up RSA keys of deleted entries
fn clean_up_rsa_keys(key_name: &str) -> Result<'static, ()> {
    let key_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    println!("{}", key_path);
    if let Err(e) = remove_file(key_path) {
        println!("{}", e.to_string());
        return Err(PasswordStoreError::ErrorRSAKeyDelete);
    }
    Ok(())
}
