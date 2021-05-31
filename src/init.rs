extern crate std;
extern crate home;
extern crate keyring;

use std::fs::{create_dir_all, write};
use std::path::{PathBuf, Path};
use std::error::{Error};
use std::io::{self, Write, Read};
use text_io::read;
use rpassword::read_password;

/*
    Set a global password for the password store
 */
pub fn set_global_password() -> Result<(), Box<dyn Error>> {
    let service = "password_store_cli";

    print!("Username: ");
    std::io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    println!("The username is: {}", username);

    let kr = keyring::Keyring::new(&service, &username);

    print!("Password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    kr.set_password(&password)?;

    Ok(())
}

pub fn get_password() -> Result<(), Box<dyn Error>>{
    let service = "password_store_cli";

    print!("Username: ");
    std::io::stdout().flush().unwrap();
    let username: String = read!("{}\n");
    println!("The username is: {}", username);

    let keyring = keyring::Keyring::new(&service, &username);

    let password = keyring.get_password()?;
    println!("The password is '{}'", password);

    Ok(())
}

pub fn initial_setup(){
    todo!();
}

pub fn create_user_configuration(){

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

    // TODO: Create store file
    // TODO: Store file encryption
}