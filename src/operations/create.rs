use std::io::Write;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use std::str;
use text_io::read;
use rpassword;

use crate::generic::common::{
    calculate_store_name_hash,
};
use crate::generic::errors::{
    Result,
    PasswordStoreError
};
use crate::generic::encryption::{
    encrypt_data_with_private_key,
    create_new_rsa_private_key
};

//Uses the user passed store name to create a file in the .passmanager folder.
// The file will contain a generated password, date created and user entered name.

// Gather user info for new data entry
//TODO: Check if store exists first
pub fn create_menu(store_name: &str) -> Result<'static, ()>{
    let entry_name: String = *get_entry_name();
    let username: String = *get_username();
    match get_password(){
        Ok(b) => {
            let password: String = *b;
            let key_name = calculate_store_name_hash(&entry_name).to_string();
            println!("The Entry name is: {}", entry_name);
            println!("The user name is: {}", username);
            println!("Pass: {}", password);
            match create_new_rsa_private_key(&key_name){
                Ok(()) => {
                    // Once the key is successfully written, read the key and generate a pub key
                    // to encrypt and save to file
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
pub fn get_entry_name() -> Box<String>{
    print!("Entry Name: ");
    std::io::stdout().flush().unwrap();
    let entry_name: String = read!("{}\n");
    // String does not implement clone trait, must clone explicitly
    let b = Box::new(entry_name.clone());
    //println!("The username is: {}", username);
    return b
}

// Read from stdin for the username
pub fn get_username() -> Box<String>{
    print!("Username: ");
    std::io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    // String does not implement clone trait, must clone explicitly
    let b = Box::new(username.clone());
    //println!("The username is: {}", username);
    return b
}

// Read password from stdin twice
// Verifies that the result is accurate before returning
// If the passwords do not match, throw error
pub fn get_password() -> Result<'static, Box<String>>{
    // Need to encrypt here
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
fn genpass(length: u32) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
    rand_string
}
