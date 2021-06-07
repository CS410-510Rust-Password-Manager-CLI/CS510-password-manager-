use crate::errors;
use crate::common;
use crate::common::{GlobalConfiguration, UserMessage};
use std::fs;
use std::io;
use std::io::Write;



// Deletes a secret from a secret store
// When deleting secret store, verify the store name before deleting
pub fn delete_secret_store(store_name: &str) -> errors::Result<'static, ()>{
    // Hash store name, find file, verify, delete
    // Throw error if user inputs store name wrong in verify
    // Print UserMessage on successful deletion

    //check if store directory exists
    match common::store_dir_exist() {
        false => return Err(errors::PasswordStoreError::ErrorStoreDir),
        true => (),
    }

    //hash the store name
    let hash_store_name = common::calculate_store_name_hash(store_name);

    //get file path of hashed store name
    let file_path = format!("{}/{}.json", GlobalConfiguration::StoreDir.value().unwrap(), hash_store_name);


    //verify store name
    let mut buffer = String::new();
    print!("\nEnter storename again to verify: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    let verified = buffer.trim();
    if verified != store_name {
        return Err(errors::PasswordStoreError::ErrorMisMatchStoreName);
    }

    //delete the file
    match fs::remove_file(file_path) {
        Err(_e) => return Err(errors::PasswordStoreError::ErrorStoreExist),
        Ok(_a) => {
            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
            Ok(())
        }
    }
    
}

// Deletes a secret store
pub fn delete_secret(){
    todo!()
}