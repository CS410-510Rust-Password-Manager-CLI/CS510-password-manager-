use crate::generic::common::{
    calculate_store_name_hash, does_store_exist, get_all_secrets, get_entry_names, get_index,
    write_to_file, GlobalConfiguration, UserMessage,
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
    print!("\nEnter storename again to verify: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    let verified = buffer.trim();
    if verified != store_name {
        return Err(PasswordStoreError::ErrorMisMatchStoreName);
    }

    match get_entry_names(store_name) {
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
// Todo: Enter secret again to delete
pub fn delete_entry(store_name: &str, entry_name: &str) -> Result<'static, ()> {
    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    // Create fail back copy.
    // If errors occur in operation, write the copy back to disk
    let failback_copy = get_all_secrets(store_name).unwrap();
    let mut all_secrets_final = get_all_secrets(store_name).unwrap();

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
        "{}/{}.json",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    if let Err(e) = remove_file(key_path) {
        println!("{}", e.to_string());
        return Err(PasswordStoreError::ErrorRSAKeyDelete);
    }
    Ok(())
}

// #[cfg(test)]
// fn delete_secret_store_test(store_name: &str) -> Result<'static, ()> {
//     // Hash store name, find file, verify, delete
//     // Throw error if user inputs store name wrong in verify
//     // Print UserMessage on successful deletion
//
//     if !does_store_exist(store_name) {
//         return Err(PasswordStoreError::ErrorStoreDoesNotExist);
//     }
//
//     //hash the store name
//     let hash_store_name = calculate_store_name_hash(store_name);
//
//     //get file path of hashed store name
//     let file_path = format!(
//         "{}/{}.json",
//         GlobalConfiguration::StoreDir.value().unwrap(),
//         hash_store_name
//     );
//
//     //delete the file
//     match fs::remove_file(file_path) {
//         Err(_e) => return Err(PasswordStoreError::ErrorStoreDoesNotExist),
//         Ok(_a) => {
//             println!("{}", UserMessage::DeletedEntrySuccessfully.value());
//             Ok(())
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::operations::init::*;
//     use crate::generic::common::*;
//     #[test]
//     fn test_setup_delete() {
//         if does_store_exist("test1") || does_store_exist("test2") || does_store_exist("test3") || does_store_exist("test4"){
//             return;
//         }
//         assert_eq!(setup("test1"), Ok(()));
//         assert!(setup("test1").is_err());
//         assert_eq!(base_dir_exist(), true);
//         assert_eq!(store_dir_exist(), true);
//         assert_eq!(key_store_dir_exist(), true);
//         assert_eq!(delete_secret_store_test("test1"), Ok(()));
//         assert_eq!(setup("test2"), Ok(()));
//         assert_eq!(setup("test3"), Ok(()));
//         assert_eq!(setup("test4"), Ok(()));
//         assert_eq!(delete_secret_store_test("test2"), Ok(()));
//         assert_eq!(delete_secret_store_test("test3"), Ok(()));
//         assert_eq!(delete_secret_store_test("test4"), Ok(()));
//         assert_eq!(does_store_exist("test1"), false);
//         assert_eq!(does_store_exist("test2"), false);
//         assert_eq!(does_store_exist("test3"), false);
//         assert_eq!(does_store_exist("test4"), false);
//     }
// }
