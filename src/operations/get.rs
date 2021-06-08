use crate::errors;
use crate::common;
use crate::data_model;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};
use rsa::{PaddingScheme, PrivateKeyPemEncoding, PublicKey, RSAPrivateKey, RSAPublicKey};
use crate::data_model::Entry;

// Entry point for the get operation
pub fn display_secret(store_name: &str, entry_name: &str) -> errors::Result<'static, ()>{
    // Check if store exists
    if !common::does_store_exist(store_name) {
        return Err(errors::PasswordStoreError::ErrorStoreDoesNotExist)
    }
    
    match get_raw_secret(store_name, entry_name){
        Ok(raw_entry) => {
            decrypt_secret(entry_name, &(*raw_entry));
            Ok(())
        },
        Err(e) => Err(e),
    }
}

// Gets a secret from the a secret store
pub fn decrypt_secret(entry_name: &str, raw_entry: &Entry) -> std::io::Result<()> {
    let key_name = common::calculate_store_name_hash(entry_name);
    let key_file_path = format!(
        "{}/{}.pem",
        common::GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
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
    let dec_user_data = private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &raw_entry.username).expect("Could not decrypt");
    let dec_pass_data = private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &raw_entry.password).expect("Could not decrypt");

    println!("Username: {}", std::str::from_utf8(&dec_user_data).unwrap());
    println!("Password: {}", std::str::from_utf8(&dec_pass_data).unwrap());

    Ok(())
}


// store function:
//      check hashed store name:
//          if not found or invalid: return error
//      check secret name:
//          if not valid or not found: return error
//      return secret to screen
pub fn get_raw_secret<'a>(store_name: &str, entry_name: &str) -> errors::Result<'static, Box<(Entry)>> {
    let base_store_path = common::GlobalConfiguration::StoreDir.value().unwrap();
    let store_hash = common::calculate_store_name_hash(store_name);
    let store_path = format!("{0}/{1}.json", base_store_path, store_hash);

    // Read data back to struct
    // Open the file in read-only mode with buffer.
    let file = File::open(Path::new(&store_path)).unwrap();
    let reader = BufReader::new(file);
    let secret_entries: data_model::EntryStore = serde_json::from_reader(reader).unwrap();

    // Iterate through all entries and return an entry matching the entry name
    for entry in secret_entries.entries{
        if entry.name == entry_name{
            println!("Found secret!");
            let b = Box::new(entry);
            return Ok(b);
        }
    }

    Err(errors::PasswordStoreError::ErrorEntryDoesNotExist)
}