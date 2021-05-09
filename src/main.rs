use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

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



fn init(store_name: &str){
    // Check if .passwordmanager dir exists
    let base_path: &str = "~/.passmanager";
    if !std::path::Path::new(base_path).is_dir(){
        // Create dir if path doesn't exist
        println!("Base path does not exist! Creating new one!")
    }
    // TODO: Create store file
    // TODO: Store file encryption
    return
}

fn get_stores(){
    let base_path: &str = "~/.passmanager";
    //TODO: Crawl through password dir and print all store names
    //TODO: Maybe store names should be encrypted as well?
    //TODO: Decrypt store names and print to screen
}

fn create(store_name: &str){
    //Need a store name and then add secrets to that store
    //This can use the CLI Menu format that we had in the menu function
    //Can add option to allow auto generation of secrets or to allow a user to use their own
}

fn main() {
    // Get all command line args
    let args: Vec<String> = std::env::args().collect();

    // List all password stores
    if args.len() == 1{
        println!("Getting all password stores!");
        get_stores();
        return
    }

    // Parse all other args
    match args[1].as_str() {
        "init" => {
            // TODO: Catch the panic here and just print message
            let store_name = args.get(2).expect("Did not get store name for option 'Init'");
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

    return
}
