extern crate home;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::fs::File;

use crate::generic::errors::{
    Result,
    PasswordStoreError
};

use crate::models::data_model::{
    Entry,
    EntryStore
};
use std::io::BufReader;

// Global Configurations for the password manager
pub enum GlobalConfiguration {
    HomeDir,
    StoreDir,
    KeyStoreDir,
}

// Function to pass back home dir and store dir path locations
impl GlobalConfiguration {
    pub fn value(&self) -> Result<String> {
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
                    }
                    GlobalConfiguration::KeyStoreDir => {
                        hdirfinal.push_str("/.keys");
                        Ok(hdirfinal)
                    }
                }
            }
            None => return Err(PasswordStoreError::HomeDirError),
        }
    }
}

// Enum class for message templates for user message
pub enum UserMessage<'a> {
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
    // Inform user that entry has been successfully deleted from the manager
    DeletedEntrySuccessfully,
}

impl UserMessage<'_> {
    pub fn value(&self) -> String {
        let mut message = String::new();
        match *self {
            UserMessage::CreatingNewStore(store_name) => {
                message.push_str(&format!("Creating store with name: {}", store_name));
                message
            }
            UserMessage::StoreCreationSuccessful => "Store created successfully!".to_string(),
            UserMessage::CreatedBaseDir => "Base dir created!".to_string(),
            UserMessage::CreatedStoreDir => "Base store dir created!".to_string(),
            UserMessage::CreatedKeyStoreDir => "Base Key store dir created!".to_string(),
            UserMessage::CreatedEntrySuccessfully => "Entry created successfully!".to_string(),
            UserMessage::DeletedEntrySuccessfully => "Entry deleted successfully!".to_string(),
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
pub fn base_dir_exist() -> bool {
    match GlobalConfiguration::HomeDir.value() {
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false,
    }
}

// Check if the store dir exists
pub fn store_dir_exist() -> bool {
    match GlobalConfiguration::StoreDir.value() {
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false,
    }
}

// Check if the store dir exists
pub fn key_store_dir_exist() -> bool {
    match GlobalConfiguration::KeyStoreDir.value() {
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false,
    }
}

// Check if specific store file exists
pub fn does_store_exist(store_name: &str) -> bool{
    let store_hash = calculate_store_name_hash(store_name);
    let store_base_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_path: &str = &format!("{0}/{1}.json", store_base_path, store_hash);
    return Path::new(store_path).exists()
}

// Get all secrets from specific store
pub fn get_all_secrets<'a>(store_name: &str) -> Result<'static, Box<EntryStore>> {
    let base_store_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_hash = calculate_store_name_hash(store_name);
    let store_path = format!("{0}/{1}.json", base_store_path, store_hash);

    // Read data back to struct
    // Open the file in read-only mode with buffer.
    let file = File::open(Path::new(&store_path)).unwrap();
    let reader = BufReader::new(file);
    let secret_entries: EntryStore = serde_json::from_reader(reader).unwrap();

    return Ok(Box::new(secret_entries))
}