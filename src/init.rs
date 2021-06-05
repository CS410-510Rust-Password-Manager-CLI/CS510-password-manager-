extern crate home;
extern crate keyring;
extern crate std;

use rpassword::read_password;
use std::error::Error;
use std::fs::{create_dir_all, write, DirEntry, self};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use text_io::read;

// Internal library
use common;
use errors;


/*
   Set a global password for the password store
*/
// pub fn set_global_password() -> Result<(), Box<dyn Error>> {
//     let service = "password_store_cli";
//
//     print!("Username: ");
//     std::io::stdout().flush().unwrap();
//     let username: String = read!("{}\n");
//     println!("The username is: {}", username);
//
//     let kr = keyring::Keyring::new(&service, &username);
//
//     print!("Password: ");
//     std::io::stdout().flush().unwrap();
//     let password = read_password().unwrap();
//
//     kr.set_password(&password)?;
//
//     Ok(())
// }
//
// pub fn get_password() -> Result<(), Box<dyn Error>> {
//     let service = "password_store_cli";
//
//     print!("Username: ");
//     std::io::stdout().flush().unwrap();
//     let username: String = read!("{}\n");
//     println!("The username is: {}", username);
//
//     let keyring = keyring::Keyring::new(&service, &username);
//
//     let password = keyring.get_password()?;
//     println!("The password is '{}'", password);
//
//     Ok(())
// }

/*
    Hashes store name and checks if the store name the user input can be created
 */
pub fn does_store_exist(store_name: &str) -> bool{
    let store_hash = super::common::calculate_store_name_hash(store_name);
    let store_path: &str = &format!("~/.passwordmanager/.store/{}.json", store_hash);
    return Path::new(store_path).exists()
}

pub fn init(store_name: &str) -> super::errors::Result<()>{
    // Check if store exists
    // If store exists, return error: this store exists
    // If not create a new store by name
    if does_store_exist(store_name){
        return Err(super::errors::PasswordStoreError::PasswordStoreExists(store_name))
    }
    setup(store_name);
    Ok(())
}


/*

*/
pub fn setup(store_name: &str) {
    // Check if .passwordmanager dir exists
    // check if a homedir env variable exits
    let hdir = home::home_dir();
    match hdir {
        Some(path) => {
            //a home env variable exists
            println!("Found home dir: {}", path.display());
            let mut hdirfinal = path.display().to_string();
            hdirfinal.push_str("/.passmanager");

            if !Path::new(&hdirfinal).is_dir() {
                // Create dir if path doesn't exist
                println!("Base path does not exist!");
                let created = create_dir_all(&hdirfinal);
                match created {
                    Ok(()) => println!("New base path created"),
                    Err(e) => println!("Error creating new path: {}", e),
                }
            }

            //creating path for new file
            let mut pathfilestring: String = "".to_owned();
            pathfilestring.push_str(&hdirfinal);
            pathfilestring.push('/');
            pathfilestring.push_str(store_name);
            pathfilestring.push_str(".txt");

            //write to file
            let mut path = PathBuf::new();
            path.push(pathfilestring);
            let written = write(path, "test");
            match written {
                Ok(()) => println!("Successfully written to file"),
                Err(e) => println!("Unable to write to file: {}", e),
            }
        }
        None => {
            println!("Impossible to get your home dir!");
            return;
        }
    }
}
