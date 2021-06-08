extern crate home;
extern crate std;

use std::fs::{create_dir_all};
use std::fs::File;

// Internal library
use crate::generic::common::{
    base_dir_exist,
    store_dir_exist,
    key_store_dir_exist,
    does_store_exist,
    calculate_store_name_hash,
    GlobalConfiguration,
    UserMessage
};

use crate::generic::errors::{
    Result,
    PasswordStoreError
};


// Sets up base dir if they do not exist
// Returns an error if the dir cannot be created
fn setup_base_dirs() -> Result<'static, ()>{
    let base_path = GlobalConfiguration::HomeDir.value().unwrap();
    match create_dir_all(&base_path){
        Ok(()) => Ok(()),
        Err(e) =>{
            println!("{:?}", e);
            Err(PasswordStoreError::ErrorCreatingBasePath)
        }
    }
}

// Sets up store dir if they do not exist
// Returns an error if the dir cannot be created
fn setup_store_dirs() -> Result<'static, ()>{
    let base_path = GlobalConfiguration::StoreDir.value().unwrap();
    match create_dir_all(&base_path){
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Err(PasswordStoreError::ErrorCreatingStorePath)
        }
    }
}

// Sets up key store dir if they do not exist
// Returns an error if the dir cannot be created
fn setup_key_store_dirs() -> Result<'static, ()>{
    let base_path = GlobalConfiguration::KeyStoreDir.value().unwrap();
    match create_dir_all(&base_path){
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{}", e.to_string());
            Err(PasswordStoreError::ErrorCreatingStorePath)
        }
    }
}


// Initialization for a new password store
pub fn setup(store_name: &str) -> Result<()>{
    // Setup base dirs if they do not exist
    if !base_dir_exist() {
        match setup_base_dirs() {
            Ok(()) => {
                println!("{}", UserMessage::CreatedBaseDir.value());
                Ok(())
            },
            Err(e) => Err(e),
        };
    }
    // Setup store dirs if they do not exist
    if !store_dir_exist(){
        match setup_store_dirs(){
            Ok(()) => {
                println!("{}", UserMessage::CreatedStoreDir.value());
                Ok(())
            },
            Err(e) => Err(e),
        };
    }

    if !key_store_dir_exist(){
        match setup_key_store_dirs(){
            Ok(()) => {
                println!("{}", UserMessage::CreatedKeyStoreDir.value());
                Ok(())
            },
            Err(e) => Err(e),
        };
    }
    // Return error if this store name already exists
    if does_store_exist(store_name) {
        return Err(PasswordStoreError::PasswordStoreExists(store_name))
    }

    //creating path for new file
    let base_store_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_hash = calculate_store_name_hash(store_name);
    let new_store_path = format!("{0}/{1}.json", base_store_path, store_hash);

    File::create(new_store_path);
    Ok(())
}
