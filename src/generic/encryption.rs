use crate::generic::common::{calculate_store_name_hash, GlobalConfiguration};
use crate::models::data_model::Entry;

use rand::rngs::OsRng;
use rsa::{PaddingScheme, PrivateKeyPemEncoding, PublicKey, RSAPrivateKey, RSAPublicKey};
use std::fs::File;
use std::io::{Read, Write};

// Creates a new RSA private key for every password entry
// Saves the created private key to a pem file stored in the .keys
// Pem file name is based on the hashed value of the entry name
pub fn create_new_rsa_private_key(key_name: &str) -> std::io::Result<()> {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let new_key_file_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    let mut file = File::create(new_key_file_path)?;
    // Convert private key to PKCS1
    let key_buf = priv_key.to_pem_pkcs1().unwrap();
    // Write key to file
    file.write_all(key_buf.as_bytes())?;
    Ok(())
}

// Reads the private key and generates a public key to encrypt secrets
// Writes the secrets to the secrets store
pub fn encrypt_data_with_private_key(
    key_name: &str,
    username: &str,
    password: &str,
    entry_name: &str,
) -> std::io::Result<Box<Entry>> {
    let key_file_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    let mut file = File::open(key_file_path).unwrap();
    let mut priv_key_buf = String::new();
    file.read_to_string(&mut priv_key_buf)?;

    // Source: From example of how to read into memory PEM file as PKCS1
    let der_encoded = priv_key_buf
        .lines()
        .filter(|line| !line.starts_with('-'))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");

    // Create public key
    let pub_key = RSAPublicKey::from(&private_key);

    let mut rng = OsRng;
    // Encrypt data
    let enc_username_data = pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            username.as_bytes(),
        )
        .expect("failed to encrypt username");
    let enc_password_data = pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            password.as_bytes(),
        )
        .expect("failed to encrypt password");

    // Create new data entry from encrypted data
    let new_entry = Entry {
        name: entry_name.to_string(),
        username: enc_username_data,
        password: enc_password_data,
    };

    Ok(Box::new(new_entry))
}

// Gets a secret from the a secret store
pub fn decrypt_secret(entry_name: &str, raw_entry: &Entry) -> std::io::Result<()> {
    let key_name = calculate_store_name_hash(entry_name);
    let key_file_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    let mut file = File::open(key_file_path).unwrap();
    let mut priv_key_buf = String::new();
    file.read_to_string(&mut priv_key_buf)?;

    let der_encoded = priv_key_buf
        .lines()
        .filter(|line| !line.starts_with('-'))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");
    let dec_user_data = private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &raw_entry.username)
        .expect("Could not decrypt");
    let dec_pass_data = private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &raw_entry.password)
        .expect("Could not decrypt");

    println!("Username: {}", std::str::from_utf8(&dec_user_data).unwrap());
    println!("Password: {}", std::str::from_utf8(&dec_pass_data).unwrap());

    Ok(())
}
