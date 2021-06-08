use clap::{App, AppSettings, Arg, Clap, Subcommand};

use crate::common::{GlobalConfiguration, UserMessage};

mod operations {
    pub mod create;
    pub mod delete;
    pub mod get;
    pub mod init;
}
mod common;
mod data_model;
mod errors;

extern crate home;

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

fn main() {
    // Get all command line args
    let matches = App::new("Password Manager")
        .version("1.0")
        .author("Haohan Jiang <jiang4.pdx.edu>, Taraq Jallad <tajallad@pdx.edu>")
        .about(
            "Usage: \n\
        To initialize a new store: password_manager <NAME> init\n\
        To delete a secret from the store: password_manager <NAME> delete\n\
        To add a secret to the store: password_manager <NAME> create\n",
        )
        .arg(
            Arg::new("store_name")
                .short('s')
                .long("store_name")
                .value_name("NAME")
                .about("Store name for operation<op>.")
                .takes_value(true),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .value_name("OPERATION")
                .about(
                    "Action to perform on the password store. Available: init, create, get, delete",
                )
                .takes_value(true),
        )
        .arg(
            Arg::new("entry_name")
                .short('e')
                .long("entry_name")
                .about("Entry name for operation<op>.")
                .takes_value(true),
        )
        .get_matches();

    let store_name = matches.value_of("store_name").unwrap();
    if let Some(op) = matches.value_of("op") {
        //TODO: Check all arguments are set
        match op {
            "init" => {
                if let Err(e) = operations::init::setup(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "create" => {
                println!("Create");
                if let Err(e) = operations::create::create_menu(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "delete" => {
                if let Err(e) = operations::delete::delete_secret_store(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "get" => {
                let entry_name= matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::get::display_secret(store_name, entry_name){
                    println!("{}", e);
                    std::process::exit(1);
                }
            },
            "modify" => println!("modify!"),
            _ => println!("Must enter a valid operation"),
        }
    } else {
        println!("Must enter an operation");
        return;
    }
}
