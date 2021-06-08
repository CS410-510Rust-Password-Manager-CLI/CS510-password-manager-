use std::io::Write;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use std::str;
use text_io::read;
use rpassword;

// Internal libraries
use crate::generic::common::{
    calculate_store_name_hash,
    does_store_exist,
};
use crate::generic::errors::{
    Result,
    PasswordStoreError
};
use crate::generic::encryption::{
    encrypt_data_with_private_key,
    create_new_rsa_private_key
};


/**
    Entry point for the create operation.
**/
pub fn create_entry_point(store_name: &str) -> Result<'static, ()>{
    // Check if the requested store exists
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist)
    }

    // Get user data from the command line
    let entry_name: String = *get_entry_name();
    let username: String = *get_username();
    match get_password(){
        Ok(b) => {
            let password: String = *b;
            let key_name = calculate_store_name_hash(&entry_name).to_string();
            // If we get all data, create a new private RSA key from the entry name
            match create_new_rsa_private_key(&key_name){
                Ok(()) => {
                    // Once the key is successfully generated, encrypt the private data
                    let hashed_store_name = calculate_store_name_hash(store_name).to_string();
                    encrypt_data_with_private_key(&key_name, &username, &password, &hashed_store_name, &entry_name);
                    Ok(())
                },
                Err(e) => Err(PasswordStoreError::ErrorPrivateKeyGeneration),
            }
        },
        Err(e) => Err(e),
    }
}

// Read from stdin for the entry name
// Return a Box with the entry name
pub fn get_entry_name() -> Box<String>{
    std::io::stdout().flush().unwrap();
    let entry_name: String = read!("{}\n");
    // String does not implement clone trait, must clone explicitly
    let b = Box::new(entry_name.clone());
    return b
}

// Read from stdin for the username
// Return a Box with the username
pub fn get_username() -> Box<String>{
    std::io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    // String does not implement clone trait, must clone explicitly
    let b = Box::new(username.clone());
    return b
}

// Read password from stdin twice
// Verifies that the result is accurate before returning
// Stdin input is not revealed on the command line
// If the passwords do not match, throw error
pub fn get_password() -> Result<'static, Box<String>>{
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let pass_verify = rpassword::prompt_password_stdout("Password Verification: ").unwrap();

    if pass == pass_verify{
        let b = Box::new(pass.clone());
        Ok(b)
    }else{
        Err(PasswordStoreError::ErrorMisMatchPasswordCreation)
    }
}

// Create function:
//  hash store name
//  validate store name: -> If no name, throw new error
//  get user and password from command line
//  encrypt secrets
//  append to file with new user/password

///generates a random alphanumeric string, with a length of the passed in integer
/// random character code from: (https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html)
//Todo: Add as user function
fn genpass(length: u32) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
    rand_string
}
