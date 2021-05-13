use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
extern crate home;

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

/*
have a config file located at ~/.passstore/.config
* passmanager init <name> - create a new password store
                          - create a pgp key to encrypt the password file store with
                          - write the file to ~/.passwordstore/stores/<name>
  passmanager - ls of all password stores

  passmanager <name of store> --create --genpass --name <name of password>
  passmanager <name of store> --modify
  passmanager <name of store> --list

       password record:
            id:
            encrpytion:
            secret:
            access-time:
            last-modified:
            user-defined fields:
                - secret question

 */

/*
fn menu() {
    let mut again = true; //loop control
    let mut buffer = String::new(); //store user input
    let mut selection: u32;
    while again {
        println!("\n*PASSWORD MANAGER CLI*\n");
        println!("1. generate new password");
        println!("9. exit\n");
        print!("Selection: ");
        io::stdout().flush().unwrap();

        //read user input
        io::stdin()
            .read_line(&mut buffer)
            .expect("could not read input");

        //parse user input to u32
        selection = buffer
            .trim()
            .parse()
            .expect("invalid user input, expecting integer");
        buffer.clear();

        let length: u32 = 20;
        match selection {
            1 => {
                let password = genpass(length);
                println!("\nGenerated password: {}", password);
                println!("Date generated: {}", Utc::now().date().naive_utc());
            }
            9 => {
                again = false;
            }
            _ => {
                println!("\n~Invalid input~")
            }
        }
    }
}
*/

fn init(store_name: &str) {
    // Check if .passwordmanager dir exists
    // check if a homedir env variable exits
    let hdir = home::home_dir();
    match hdir {
        Some(path) => {
            //a home env variable exists
            println!("Found home dir: {}", path.display());
            let mut hdirfinal = path.display().to_string();
            hdirfinal.push_str("/.passmanager");

            if !std::path::Path::new(&hdirfinal).is_dir() {
                // Create dir if path doesn't exist
                println!("Base path does not exist!");
                let created = fs::create_dir_all(&hdirfinal);
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

            //write to file
            let mut path = PathBuf::new();
            path.push(pathfilestring);
            let written = std::fs::write(path, "test");
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

fn get_stores() {
    let hdir = home::home_dir();
    match hdir {
        Some(path) => {
            let mut hdirfinal = path.display().to_string();
            hdirfinal.push_str("/.passmanager");
            let testfiles = fs::read_dir(&hdirfinal);
            match testfiles {
                Ok(_v) => (),
                Err(_e) => {
                    println!("Error: the base path does not exist or the process lacks permissions to view the contents");
                    return;
                }
            }

            let files = fs::read_dir(&hdirfinal).unwrap();

            //print names of all files in the base directory
            for file in files {
                println!("Filename: {:?}", file.unwrap().file_name())
            }
            //TODO: Crawl through password dir and print all store names
            //TODO: Maybe store names should be encrypted as well?
            //TODO: Decrypt store names and print to screen
        }
        None => {
            println!("Impossible to get your home dir!");
        }
    }
}

fn create(store_name: &str) {
    //Need a store name and then add secrets to that store
    //This can use the CLI Menu format that we had in the menu function
    //Can add option to allow auto generation of secrets or to allow a user to use their own
}

fn main() {
    // Get all command line args
    let args: Vec<String> = std::env::args().collect();

    // List all password stores
    if args.len() == 1 {
        println!("Getting all password stores!");
        get_stores();
        return;
    }

    // Parse all other args
    match args[1].as_str() {
        "init" => {
            // TODO: Catch the panic here and just print message
            let store_name = args
                .get(2)
                .expect("Did not get store name for option 'Init'");
            println!("Init new password store: {}", store_name);
            init(store_name);
        }
        "create" => {
            println!("Creating new password")
        }
        _ => {
            println!("Unknown arg: {}", args[1])
        }
    }
}
