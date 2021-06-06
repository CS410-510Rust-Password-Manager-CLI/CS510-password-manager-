extern crate home;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use rsa::{RSAPublicKey, RSAPrivateKey, PaddingScheme, PrivateKeyPemEncoding, PublicKey};
use rand::rngs::OsRng;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use base64;

use crate::errors;
use crate::data_model;

// Global Configurations for the password manager
pub enum GlobalConfiguration{
    HomeDir,
    StoreDir,
    KeyStoreDir,
}

// Function to pass back home dir and store dir path locations
impl GlobalConfiguration {
    pub fn value(&self) -> errors::Result<String>{
        let hdir = home::home_dir();
        match hdir {
            Some(path) => {
                //a home env variable exists
                let mut hdirfinal = path.display().to_string();
                hdirfinal.push_str("/.passmanager");
                match *self {
                    GlobalConfiguration::HomeDir => Ok(hdirfinal),
                    GlobalConfiguration::StoreDir => {
                        hdirfinal.push_str("/.store");
                        Ok(hdirfinal)
                    },
                    GlobalConfiguration::KeyStoreDir => {
                        hdirfinal.push_str("/.keys");
                        Ok(hdirfinal)
                    }
                }
            }
            None => {
                return Err(super::errors::PasswordStoreError::HomeDirError)
            }
        }
    }
}

// Enum class for message templates for user message
pub enum UserMessage<'a>{
    // Inform user that they are creating a new store
    CreatingNewStore(&'a str),
    // Inform user that they store creation was successful
    StoreCreationSuccessful,
    // Inform user that base directory has been created
    CreatedBaseDir,
    // Inform user that store directory has been created
    CreatedStoreDir,
    // Inform user that key store directory has been created
    CreatedKeyStoreDir,
    // Inform user that new entry has been successfully saved into the manager
    CreatedEntrySuccessfully,
}

impl UserMessage<'_>{
    pub fn value(&self) -> String {
        let mut message = String::new();
        match *self {
            UserMessage::CreatingNewStore(store_name) => {
                message.push_str(&format!("Creating store with name: {}", store_name));
                message

            },
            UserMessage::StoreCreationSuccessful => "Store created successfully!".to_string(),
            UserMessage::CreatedBaseDir => "Base dir created!".to_string(),
            UserMessage::CreatedStoreDir => "Base store dir created!".to_string(),
            UserMessage::CreatedKeyStoreDir => "Base Key store dir created!".to_string(),
            UserMessage::CreatedEntrySuccessfully => "Entry created successfully!".to_string(),
        }
    }
}

// Hashes name input string
// Returns str reference to hashed str name
pub fn calculate_store_name_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// Check if the base dir exists
pub fn base_dir_exist() -> bool{
    match GlobalConfiguration::HomeDir.value(){
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false
    }
}

// Check if the store dir exists
pub fn store_dir_exist() -> bool{
    match GlobalConfiguration::StoreDir.value(){
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false
    }
}

// Check if the store dir exists
pub fn key_store_dir_exist() -> bool{
    match GlobalConfiguration::KeyStoreDir.value(){
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false
    }
}

// Creates a new RSA private key for every password entry
// Saves private key to pem file stored in the .keys
// Key name is based on the hashed value of the entry name
pub fn create_new_rsa_private_key(key_name: &str) -> std::io::Result<()>{
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    let new_key_file_path = format!("{}/{}.pem", GlobalConfiguration::KeyStoreDir.value().unwrap(), key_name);
    let mut file = File::create(new_key_file_path)?;
    let key_buf = priv_key.to_pem_pkcs1().unwrap();
    file.write_all(key_buf.as_bytes())?;
    Ok(())
}

pub fn encrypt_data_with_private_key(key_name: &str, username: &str, password: &str, store_name: &str, entry_name: &str) {
    let key_file_path = format!("{}/{}.pem", GlobalConfiguration::KeyStoreDir.value().unwrap(), key_name);
    let mut file = File::open(key_file_path).unwrap();
    let mut priv_key_buf = String::new();
    file.read_to_string(&mut priv_key_buf);

    let der_encoded = priv_key_buf
        .lines()
        .filter(|line| !line.starts_with("-"))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");
    let pub_key = RSAPublicKey::from(&private_key);

    let mut rng = OsRng;
    let enc_username_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), username.as_bytes()).expect("failed to encrypt username");
    let enc_password_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), password.as_bytes()).expect("failed to encrypt password");
    let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), username.as_bytes()).expect("failed to encrypt");

    //Write encrypted data to store file
    let store_path = format!("{0}/{1}.json", GlobalConfiguration::StoreDir.value().unwrap(), store_name);
    let mut store_file = File::open(store_path);

    println!("{}", str::from_utf8(&enc_username_data).unwrap());

    //Write encrypted data to store file
    // let store_path = format!("{0}/{1}.json", GlobalConfiguration::StoreDir.value().unwrap(), store_name);
    // let mut store_file = File::open(store_path)?;
    //
    // let new_entry = data_model::Entry{
    //     name: entry_name.to_string(),
    //
    // }

    //Ok()
}

