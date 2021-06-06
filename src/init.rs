extern crate home;
extern crate std;

use std::fs::{create_dir_all};
use std::path::{Path};
use std::fs::File;

// Internal library
use crate::common;
use crate::errors;
use crate::common::{GlobalConfiguration, UserMessage};

// Hashes store name and checks if the store name the user input can be created
pub fn does_store_exist(store_name: &str) -> bool{
    let store_hash = super::common::calculate_store_name_hash(store_name);
    let store_base_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_path: &str = &format!("{0}/{1}.json", store_base_path, store_hash);
    return Path::new(store_path).exists()
}

// Sets up base dir if they do not exist
// Returns an error if the dir cannot be created
fn setup_base_dirs() -> errors::Result<'static, ()>{
    let base_path = GlobalConfiguration::HomeDir.value().unwrap();
    match create_dir_all(&base_path){
        Ok(()) =>
            {
                Ok(())
            },
        Err(e) => Err(errors::PasswordStoreError::ErrorCreatingBasePath)
    }
}

// Sets up store dir if they do not exist
// Returns an error if the dir cannot be created
fn setup_store_dirs() -> errors::Result<'static, ()>{
    let base_path = GlobalConfiguration::StoreDir.value().unwrap();
    match create_dir_all(&base_path){
        Ok(()) =>
            {
                Ok(())
            },
        Err(e) => Err(errors::PasswordStoreError::ErrorCreatingStorePath)
    }
}

pub fn setup(store_name: &str) -> errors::Result<()>{
    // Check if .passwordmanager dir exists
    // Setup base dirs if they do not exist
    if !common::base_dir_exist(){
        match setup_base_dirs(){
            Ok(()) => {
                println!("{}", UserMessage::CreatedBaseDir.value());
                Ok(())
            },
            Err(e) => Err(e),
        };
        match setup_store_dirs(){
            Ok(()) => {
                println!("{}", UserMessage::CreatedStoreDir.value());
                Ok(())
            },
            Err(e) => Err(e),
        };
    }

    // Return error if this store name already exists
    if does_store_exist(store_name) {
        return Err(super::errors::PasswordStoreError::PasswordStoreExists(store_name))
    }

    //creating path for new file
    let base_store_path = common::GlobalConfiguration::StoreDir.value().unwrap();
    let store_hash = super::common::calculate_store_name_hash(store_name);
    let new_store_path = format!("{0}/{1}.json", base_store_path, store_hash);
    File::create(new_store_path);
    Ok(())
}
