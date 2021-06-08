use crate::generic::common::{calculate_store_name_hash, does_store_exist, get_all_secrets, write_to_file, GlobalConfiguration, UserMessage, get_index};
use crate::generic::errors::{PasswordStoreError, Result};
use std::fs;
use std::io;
use std::io::Write;


//Todo: Clean up RSA private keys after entry or store deletion

// Deletes a secret from a secret store
// When deleting secret store, verify the store name before deleting
pub fn delete_secret_store(store_name: &str) -> Result<'static, ()> {
    // Hash store name, find file, verify, delete
    // Throw error if user inputs store name wrong in verify
    // Print UserMessage on successful deletion

    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    //hash the store name
    let hash_store_name = calculate_store_name_hash(store_name);

    //get file path of hashed store name
    let file_path = format!(
        "{}/{}.json",
        GlobalConfiguration::StoreDir.value().unwrap(),
        hash_store_name
    );

    //verify store name
    let mut buffer = String::new();
    print!("\nEnter storename again to verify: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    let verified = buffer.trim();
    if verified != store_name {
        return Err(PasswordStoreError::ErrorMisMatchStoreName);
    }

    //delete the file
    match fs::remove_file(file_path) {
        Err(_e) => return Err(PasswordStoreError::ErrorStoreDoesNotExist),
        Ok(_a) => {
            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
            Ok(())
        }
    }
}

// Deletes a secret entry in a store
// Todo: Enter secret again to delete
pub fn delete_entry(store_name: &str, entry_name: &str) -> Result<'static, ()> {
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    let failback_copy = get_all_secrets(store_name).unwrap();
    let mut all_secrets_final = get_all_secrets(store_name).unwrap();

    let store_hash = calculate_store_name_hash(store_name).to_string();
    match get_index(entry_name, &(*all_secrets_final)){
        Some(index) => {
            //remove entry from entries vector where name matches, if match found
            let mut entry_name_vec = Vec::new();
            entry_name_vec.push(all_secrets_final.entries[*index].name.clone());
            all_secrets_final.entries.remove(*index);
            match write_to_file(&all_secrets_final, &store_hash) {
                Ok(()) => {
                    match clean_up_rsa_keys(entry_name_vec){
                        Ok(()) => {
                            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
                            Ok(())
                        }
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(e)
            }
        }
        None => {
            match write_to_file(&failback_copy, &store_hash) {
                Ok(()) => Err(PasswordStoreError::ErrorNoEntryNameMatch),
                Err(e) => Err(e)
            }
        }
    }
}

fn clean_up_rsa_keys(key_names: Vec<String>) -> Result<'static, ()>{
    for key in key_names{
        let key_path = format!("{}/{}.json",
                               GlobalConfiguration::KeyStoreDir.value().unwrap(),
                                key);
    }

    Ok(())
}

#[cfg(test)]
fn delete_secret_store_test(store_name: &str) -> Result<'static, ()> {
    // Hash store name, find file, verify, delete
    // Throw error if user inputs store name wrong in verify
    // Print UserMessage on successful deletion

    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    //hash the store name
    let hash_store_name = calculate_store_name_hash(store_name);

    //get file path of hashed store name
    let file_path = format!(
        "{}/{}.json",
        GlobalConfiguration::StoreDir.value().unwrap(),
        hash_store_name
    );

    //delete the file
    match fs::remove_file(file_path) {
        Err(_e) => return Err(PasswordStoreError::ErrorStoreDoesNotExist),
        Ok(_a) => {
            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::init::*;
    use crate::generic::common::*;
    #[test]
    fn test_setup() {
        assert_eq!(setup("test123"), Ok(()));
        assert!(setup("test123").is_err());
        assert_eq!(base_dir_exist(), true);
        delete_secret_store_test("test123").expect("unable to delete");
    }
}