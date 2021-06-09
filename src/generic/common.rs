use crate::generic::errors::{PasswordStoreError, Result};
use crate::models::data_model::EntryStore;
#[cfg(not(test))]
use home::home_dir;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::Path;

#[cfg(test)]
use crate::mocks::test_mocks::home_dir;

// Global Configurations for the password manager
pub enum GlobalConfiguration {
    HomeDir,
    StoreDir,
    KeyStoreDir,
}

// Function to pass back home dir and store dir path locations
impl GlobalConfiguration {
    pub fn value(&self) -> Result<String> {
        let hdir = home_dir();
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
            None => Err(PasswordStoreError::HomeDirError),
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
    CreatedEntrySuccessfully(&'a str),
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
            UserMessage::CreatedEntrySuccessfully(entry_name) => {
                message.push_str(&format!(
                    "Entry created successfully with name: {}",
                    entry_name
                ));
                message
            }
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
        Ok(_path) => {
            let result = Path::new(&_path).is_dir();
            result
        }
        _ => false,
    }
}

// Check if the store dir exists
pub fn store_dir_exist() -> bool {
    match GlobalConfiguration::StoreDir.value() {
        Ok(path) => Path::new(&path).is_dir(),
        _ => false,
    }
}

// Check if the store dir exists
pub fn key_store_dir_exist() -> bool {
    match GlobalConfiguration::KeyStoreDir.value() {
        Ok(path) => Path::new(&path).is_dir(),
        _ => false,
    }
}

// Check if specific store file exists
pub fn does_store_exist(store_name: &str) -> bool {
    let store_hash = calculate_store_name_hash(store_name);
    let store_base_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_path: &str = &format!("{0}/{1}.json", store_base_path, store_hash);
    return Path::new(store_path).exists();
}

// Get all secrets from a secret store
pub fn get_all_secrets_from_store(path: &str) -> Option<Box<EntryStore>> {
    // Read data back to struct
    // Open the file in read-only mode with buffer.
    let file = File::open(Path::new(path)).unwrap();
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(secret_entries) => Some(Box::new(secret_entries)),
        Err(_e) => None,
    }
}

//Write entry_store to file
pub fn write_to_file<'a>(entry_store: &EntryStore, hashed_store_name: &str) -> Result<'a, ()> {
    let serialized_data = serde_json::to_string(&entry_store).unwrap();
    let store_path = format!(
        "{0}/{1}.json",
        GlobalConfiguration::StoreDir.value().unwrap(),
        hashed_store_name
    );
    println!("Path: {}", store_path);
    println!("Data: {}", serialized_data);
    let store_file = File::create(Path::new(&store_path)).unwrap();
    serde_json::to_writer(store_file, &entry_store).unwrap();
    println!("Saved!");
    Ok(())
}

// Get the index that the entry_name exists at in the EntryStore
pub fn get_index(entry_name: &str, store: &EntryStore) -> Option<Box<usize>> {
    let mut index: usize = 0;
    let mut found = false;

    //loop through entries to find matching name
    for entry in &(*store).entries {
        if entry.name == entry_name {
            found = true;
            break;
        }
        index += 1;
    }
    if found {
        Some(Box::new(index))
    } else {
        None
    }
}

// Return a Vec of entry names that exist in the store: store_name
pub fn get_entry_names(path: &str) -> Option<Vec<String>> {
    match get_all_secrets_from_store(path) {
        Some(entry_store) => {
            let mut entry_names = Vec::new();
            for entry in entry_store.entries {
                entry_names.push(entry.name)
            }
            if entry_names.is_empty() {
                None
            } else {
                Some(entry_names)
            }
        }
        None => None,
    }
}

// Returns a string with the path to the store_name
pub fn get_path(store_name: &str) -> Box<String> {
    let base_store_path = GlobalConfiguration::StoreDir.value().unwrap();
    let store_hash = calculate_store_name_hash(store_name).to_string();
    let path = format!("{0}/{1}.json", base_store_path, store_hash);
    Box::new(path)
}

#[test]
fn get_home_dir() {
    let actual = GlobalConfiguration::HomeDir.value().unwrap();
    let expected = "/home/.passmanager";
    assert_eq!(actual, expected)
}
#[test]
fn get_store_dir() {
    let actual = GlobalConfiguration::StoreDir.value().unwrap();
    let expected = "/home/.passmanager/.store";
    assert_eq!(actual, expected)
}

#[test]
fn get_keystore_dir() {
    let actual = GlobalConfiguration::KeyStoreDir.value().unwrap();
    let expected = "/home/.passmanager/.keys";
    assert_eq!(actual, expected)
}

#[test]
fn getting_path_name() {
    let actual = *get_path("foobar");
    let expected = "/home/.passmanager/.store/13402334684424448340.json";
    assert_eq!(actual, expected)
}

#[test]
fn get_all_entries() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());
    let actual = *get_all_secrets_from_store(&path).unwrap();

    assert_eq!(3, actual.entries.len())
}

#[test]
#[should_panic]
fn get_all_entries_no_file() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/404", proj_root.to_str().unwrap());
    let _actual = *get_all_secrets_from_store(&path).unwrap();
}

#[test]
fn get_entries() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());
    let actual = get_entry_names(&path).unwrap();

    assert_eq!("foobar", actual[0]);
    assert_eq!("foo2", actual[1])
}

#[test]
fn get_index_with_entry() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());
    let actual = *get_all_secrets_from_store(&path).unwrap();

    let index = get_index("foobar", &actual).unwrap();

    assert_eq!(0, *index);
}

#[test]
#[should_panic]
fn get_index_with_entry_not_found() {
    let proj_root = project_root::get_project_root().unwrap();
    let path = format!("{}/resources/test.json", proj_root.to_str().unwrap());
    let actual = *get_all_secrets_from_store(&path).unwrap();

    let _index = get_index("not_found", &actual).unwrap();
}
