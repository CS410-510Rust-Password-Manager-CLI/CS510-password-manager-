use crate::generic::errors::{
    Result,
    PasswordStoreError
};
use crate::generic::common::{
    GlobalConfiguration,
    UserMessage,
    calculate_store_name_hash,
    does_store_exist,

};
use std::fs;
use std::io;
use std::io::Write;

// Deletes a secret from a secret store
// When deleting secret store, verify the store name before deleting
pub fn delete_secret_store(store_name: &str) -> Result<'static, ()>{
    // Hash store name, find file, verify, delete
    // Throw error if user inputs store name wrong in verify
    // Print UserMessage on successful deletion

    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist)
    }

    //hash the store name
    let hash_store_name = calculate_store_name_hash(store_name);

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
        return Err(PasswordStoreError::ErrorMisMatchStoreName);
    }

    //delete the file
    match fs::remove_file(file_path) {
        // TODO: Fix this
        Err(_e) => return Err(PasswordStoreError::ErrorStoreDoesNotExist),
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